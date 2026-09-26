//! Независимые интеграционные тесты T-01 «Черновик: `source`, `source_hash`,
//! `stale`, `test_valid`» (Q12, Q29 §4.5, инварианты 2–5).
//!
//! Тесты гоняют **реальный** бинарник `credo2` по MCP-stdio (JSON-RPC 2.0),
//! а не приватный `McpServer::dispatch`, поэтому проверяют фактический контракт
//! ответов. Покрывают границы, которых нет в юнит-тестах `src/mcp.rs`:
//! пустой/бинарный `.dar`, `source_hash` после перезаписи, загрузка старого
//! `sandbox.json`, идемпотентное удаление и отсутствующий черновик в
//! `check.test`.

use serde_json::{Map, Value, json};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::mpsc::{Receiver, RecvTimeoutError};
use std::time::Duration;

const SRC: &str = "Правило МинимальныйВозраст { Если (Клиент.Возраст < 21) { \
                   Решение = Отказ; Причина = \"Возраст меньше 21\"; } }";

const SRC_V2: &str = "Правило МинимальныйВозраст { Если (Клиент.Возраст < 18) { \
                      Решение = Отказ; Причина = \"Возраст меньше 18\"; } }";

const NAME: &str = "МинимальныйВозраст";
const DAR: &str = "rules/МинимальныйВозраст.dar";

/// Минимальный MCP-клиент поверх stdio дочернего процесса.
struct Mcp {
    child: Child,
    stdin: ChildStdin,
    rx: Receiver<String>,
    next_id: i64,
}

impl Mcp {
    fn start(workspace: &Path) -> Self {
        let exe = env!("CARGO_BIN_EXE_credo2");
        let mut child = Command::new(exe)
            .arg("--workspace")
            .arg(workspace)
            .arg("--no-rest")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .expect("spawn credo2");
        let stdin = child.stdin.take().expect("stdin");
        let stdout = child.stdout.take().expect("stdout");
        let (tx, rx) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            for line in BufReader::new(stdout).lines() {
                match line {
                    Ok(l) => {
                        if tx.send(l).is_err() {
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }
        });

        let mut mcp = Self {
            child,
            stdin,
            rx,
            next_id: 0,
        };
        let init = mcp.request(
            1,
            "initialize",
            json!({
                "protocolVersion": "2024-11-05",
                "capabilities": {},
                "clientInfo": { "name": "tester", "version": "0" },
            }),
        );
        assert_eq!(init["result"]["serverInfo"]["name"], "credo", "{init}");
        mcp.notify("notifications/initialized", json!({}));
        mcp.next_id = 1;
        mcp
    }

    fn send(&mut self, msg: &Value) {
        writeln!(self.stdin, "{}", serde_json::to_string(msg).unwrap()).expect("write stdin");
        self.stdin.flush().expect("flush stdin");
    }

    fn notify(&mut self, method: &str, params: Value) {
        self.send(&json!({ "jsonrpc": "2.0", "method": method, "params": params }));
    }

    fn request(&mut self, id: i64, method: &str, params: Value) -> Value {
        self.send(&json!({
            "jsonrpc": "2.0",
            "id": id,
            "method": method,
            "params": params,
        }));
        loop {
            let line = match self.rx.recv_timeout(Duration::from_secs(30)) {
                Ok(l) => l,
                Err(RecvTimeoutError::Timeout) => panic!("таймаут ответа на {method} (id={id})"),
                Err(RecvTimeoutError::Disconnected) => panic!("сервер закрыл stdout"),
            };
            let v: Value = serde_json::from_str(line.trim()).expect("ответ — JSON");
            // Пропускаем уведомления сервера (без id).
            if v.get("id").and_then(Value::as_i64) == Some(id) {
                return v;
            }
        }
    }

    fn next_id(&mut self) -> i64 {
        self.next_id += 1;
        self.next_id
    }

    /// Возвращает `(isError, payload)`; payload — JSON из `content[0].text`.
    fn call(&mut self, tool: &str, args: Value) -> (bool, Value) {
        let id = self.next_id();
        let resp = self.request(id, "tools/call", json!({ "name": tool, "arguments": args }));
        let result = &resp["result"];
        assert!(result.is_object(), "нет result в ответе: {resp}");
        let is_error = result["isError"].as_bool().unwrap_or(false);
        let text = result["content"][0]["text"]
            .as_str()
            .unwrap_or_else(|| panic!("нет content[0].text: {resp}"));
        let payload: Value = serde_json::from_str(text).unwrap_or_else(|e| {
            panic!("content[0].text — не JSON ({e}): {text}");
        });
        (is_error, payload)
    }

    fn create(&mut self, source: &str) -> Value {
        let (err, payload) = self.call("check.create", json!({ "name": NAME, "source": source }));
        assert!(!err, "check.create вернул ошибку: {payload}");
        payload
    }

    fn get_draft(&mut self, name: &str) -> (bool, Value) {
        self.call("check.get_draft", json!({ "name": name }))
    }
}

impl Drop for Mcp {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

fn temp_workspace() -> tempfile::TempDir {
    tempfile::tempdir().expect("tempdir")
}

/// Пустой файл и бинарные (не-UTF-8) байты — это «хэш отличается → `stale`.
#[test]
fn stale_empty_and_binary_dar_q29_inv3() {
    let t = temp_workspace();
    let mut mcp = Mcp::start(t.path());
    mcp.create(SRC);

    let rules = t.path().join("rules");
    std::fs::create_dir_all(&rules).unwrap();
    let file = t.path().join(DAR);

    // Файла нет → stale = false.
    let (_, d) = mcp.get_draft(NAME);
    assert_eq!(d["draft"]["stale"], json!(false), "{d}");

    // Пустой файл: хэш отличается от source_hash → stale = true.
    std::fs::write(&file, b"").unwrap();
    let (_, d) = mcp.get_draft(NAME);
    assert_eq!(d["draft"]["stale"], json!(true), "пустой .dar: {d}");

    // Бинарные (не-UTF-8) байты → stale = true (чтение байт, а не строки).
    std::fs::write(&file, [0xFF, 0xFE, 0x00, 0x9C]).unwrap();
    let (_, d) = mcp.get_draft(NAME);
    assert_eq!(d["draft"]["stale"], json!(true), "бинарный .dar: {d}");

    // Ровно исходные байты → stale = false.
    std::fs::write(&file, SRC.as_bytes()).unwrap();
    let (_, d) = mcp.get_draft(NAME);
    assert_eq!(d["draft"]["stale"], json!(false), "совпадающий .dar: {d}");
}

/// `source_hash` после перезаписи: файл уже устарел относительно нового текста.
#[test]
fn stale_and_hash_after_overwrite_q12() {
    let t = temp_workspace();
    let mut mcp = Mcp::start(t.path());
    mcp.create(SRC);

    let rules = t.path().join("rules");
    std::fs::create_dir_all(&rules).unwrap();
    std::fs::write(t.path().join(DAR), SRC.as_bytes()).unwrap();

    let (_, d) = mcp.get_draft(NAME);
    let hash_v1 = d["draft"]["source_hash"].as_str().unwrap().to_string();
    assert_eq!(hash_v1, credo2::core::source_hash(SRC));
    assert_eq!(d["draft"]["stale"], json!(false), "{d}");

    // «Сохранить = обновить»: повторный check.create перезаписывает текст.
    let created = mcp.create(SRC_V2);
    assert_eq!(
        created,
        json!({ "status": "ok", "name": NAME }),
        "{created}"
    );

    let (_, d) = mcp.get_draft(NAME);
    assert_eq!(d["draft"]["source"], SRC_V2);
    let hash_v2 = d["draft"]["source_hash"].as_str().unwrap();
    assert_eq!(hash_v2, credo2::core::source_hash(SRC_V2));
    assert_ne!(hash_v1, hash_v2);
    // Файл остался старым → stale = true.
    assert_eq!(d["draft"]["stale"], json!(true), "{d}");
    // created_at не сбрасывается при перезаписи.
    assert!(d["draft"]["created_at"].is_string());
    // Исходный .dar не меняется.
    assert_eq!(std::fs::read_to_string(t.path().join(DAR)).unwrap(), SRC);
}

/// `check.create`/`check.get_draft`: поля по §4.5, без внутреннего `rule`,
/// `size`, `format`; `source_hash` формата `sha256:<hex>`.
#[test]
fn get_draft_canonical_fields_and_no_internals_q29_inv2_inv5() {
    let t = temp_workspace();
    let mut mcp = Mcp::start(t.path());

    let created = mcp.create(SRC);
    // Q28/§4.5: успех check.create — ровно {status, name}.
    let obj = created.as_object().unwrap();
    let mut keys: Vec<&str> = obj.keys().map(String::as_str).collect();
    keys.sort_unstable();
    assert_eq!(keys, ["name", "status"], "ответ check.create: {created}");
    assert_eq!(created["status"], "ok");
    assert_eq!(created["name"], NAME);

    let (err, got) = mcp.get_draft(NAME);
    assert!(!err, "{got}");
    assert_eq!(got["status"], "ok");
    let d = &got["draft"];

    assert_eq!(d["source"], SRC, "текст хранится без изменений");
    let hash = d["source_hash"].as_str().expect("source_hash");
    assert_eq!(hash, credo2::core::source_hash(SRC));
    let hex = hash.strip_prefix("sha256:").expect("префикс sha256:");
    assert_eq!(hex.len(), 64, "hex sha256: {hash}");
    assert!(hex.chars().all(|c| c.is_ascii_hexdigit()), "{hash}");
    assert_eq!(d["condition"], "Клиент.Возраст < 21");
    assert_eq!(d["decision"], "Отказ");
    assert_eq!(d["reason"], "Возраст меньше 21");
    assert!(d["created_at"].is_string() && d["updated_at"].is_string());
    assert!(d["last_test_checksum"].is_null());
    assert!(d["tested_at"].is_null());
    assert_eq!(d["test_valid"], json!(false));
    assert_eq!(d["stale"], json!(false));

    // Инвариант 2: внутренний Rule/Condition/Action не публикуется.
    for forbidden in ["rule", "condition_", "action", "size", "format"] {
        assert!(d.get(forbidden).is_none(), "лишнее поле {forbidden}: {d}");
    }
}

/// `check.list_drafts`: канонические поля и отсутствие `rule`/`size`/`format`.
#[test]
fn list_drafts_canonical_fields_q29() {
    let t = temp_workspace();
    let mut mcp = Mcp::start(t.path());
    mcp.create(SRC);

    let (err, resp) = mcp.call("check.list_drafts", json!({}));
    assert!(!err, "{resp}");
    assert_eq!(resp["status"], "ok");
    assert_eq!(resp["count"], json!(1));
    let item = &resp["drafts"][0];
    let obj = item.as_object().unwrap();
    let mut keys: Vec<&str> = obj.keys().map(String::as_str).collect();
    keys.sort_unstable();
    assert_eq!(
        keys,
        ["condition", "decision", "name", "stale", "updated_at"],
        "поля list_drafts: {item}"
    );
    assert_eq!(item["name"], NAME);
    assert_eq!(item["condition"], "Клиент.Возраст < 21");
    assert_eq!(item["decision"], "Отказ");
    assert_eq!(item["stale"], json!(false));
}

/// Успешный `check.test` фиксирует метку; смена текста делает `test_valid`
/// ложным (Q16/Q34, инвариант 4).
#[test]
fn test_records_checksum_and_invalidates_on_change_q29_inv4() {
    let t = temp_workspace();
    let mut mcp = Mcp::start(t.path());
    mcp.create(SRC);

    let input: Map<String, Value> =
        serde_json::from_value(json!({ "Клиент.Возраст": 19 })).unwrap();
    let (err, tested) = mcp.call("check.test", json!({ "name": NAME, "input": input }));
    assert!(!err, "{tested}");
    assert_eq!(tested["status"], "ok");
    assert_eq!(tested["rule_name"], NAME);
    assert_eq!(tested["condition"], "Клиент.Возраст < 21");
    assert_eq!(tested["matched"], json!(true));
    assert_eq!(tested["decision"], "Отказ");
    assert_eq!(tested["reason"], "Возраст меньше 21");
    assert_eq!(tested["source_hash"], credo2::core::source_hash(SRC));
    assert_eq!(tested["last_test_checksum"], tested["source_hash"]);
    assert!(tested["tested_at"].is_string());
    assert!(tested.get("explanation").is_none(), "{tested}");

    let (_, d) = mcp.get_draft(NAME);
    assert_eq!(d["draft"]["test_valid"], json!(true), "{d}");
    assert_eq!(d["draft"]["last_test_checksum"], tested["source_hash"]);
    assert_eq!(d["draft"]["tested_at"], tested["tested_at"]);

    // Новый текст → метка остаётся, но недействительна.
    mcp.create(SRC_V2);
    let (_, d) = mcp.get_draft(NAME);
    assert_eq!(d["draft"]["source_hash"], credo2::core::source_hash(SRC_V2));
    assert_eq!(d["draft"]["last_test_checksum"], tested["source_hash"]);
    assert_eq!(d["draft"]["test_valid"], json!(false), "{d}");
}

/// Старый `sandbox.json` (без `source`/`source_hash`/меток теста) грузится,
/// черновик читается и не считается stale/валидным.
#[test]
fn old_sandbox_entry_loads_and_is_readable() {
    let t = temp_workspace();
    std::fs::create_dir_all(t.path().join(".credo")).unwrap();
    let old = r#"{"drafts":{"МинимальныйВозраст":{"name":"МинимальныйВозраст",
        "rule":{"name":"МинимальныйВозраст",
                "condition":{"field":"Клиент.Возраст","op":"<","value":21.0},
                "action":{"decision":"Отказ","reason":"старое"}},
        "created_at":"t0","updated_at":"t0"}}}"#;
    std::fs::write(t.path().join(".credo/sandbox.json"), old).unwrap();

    let mut mcp = Mcp::start(t.path());
    let (err, got) = mcp.get_draft(NAME);
    assert!(!err, "старый черновик не читается: {got}");
    let d = &got["draft"];
    assert_eq!(d["source"], "");
    assert_eq!(d["source_hash"], "");
    assert!(d["last_test_checksum"].is_null());
    assert!(d["tested_at"].is_null());
    assert_eq!(d["test_valid"], json!(false));
    assert_eq!(d["stale"], json!(false));
    // Правило из старой записи всё ещё исполняется.
    let input: Map<String, Value> =
        serde_json::from_value(json!({ "Клиент.Возраст": 19 })).unwrap();
    let (err, tested) = mcp.call("check.test", json!({ "name": NAME, "input": input }));
    assert!(!err, "{tested}");
    assert_eq!(tested["reason"], "старое");
}

/// Удаление идемпотентно: первый вызов убирает черновик, повторный не ошибка.
#[test]
fn delete_is_idempotent() {
    let t = temp_workspace();
    let mut mcp = Mcp::start(t.path());
    mcp.create(SRC);

    let (err, first) = mcp.call("check.delete_draft", json!({ "name": NAME }));
    assert!(!err, "первое удаление — ошибка: {first}");

    let (err, got) = mcp.get_draft(NAME);
    assert!(err, "черновик не удалён: {got}");

    let (err, second) = mcp.call("check.delete_draft", json!({ "name": NAME }));
    assert!(
        !err,
        "повторное удаление — ошибка (не идемпотентно): {second}"
    );
}

/// `check.test` по отсутствующему черновику — ошибка (не паника).
#[test]
fn test_missing_draft_is_error_not_panic() {
    let t = temp_workspace();
    let mut mcp = Mcp::start(t.path());

    let (err, payload) = mcp.call(
        "check.test",
        json!({ "name": "НесуществующееПравило", "input": {} }),
    );
    assert!(err, "ожидалась ошибка: {payload}");
    let message = payload["error"]
        .as_str()
        .map(str::to_owned)
        .or_else(|| payload["error"]["message"].as_str().map(str::to_owned))
        .unwrap_or_else(|| panic!("нет текста ошибки: {payload}"));
    assert!(message.contains("найден"), "сообщение: {message}");
}
