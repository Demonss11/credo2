//! Общая поддержка интеграционных тестов MCP: клиент поверх stdio реального
//! бинарника `credo2` (JSON-RPC 2.0) и проверки единого конверта ошибок
//! (Q29, §4.5, T-04).
//!
//! Модуль подключается тестовыми крейтами как `mod common;`, поэтому часть
//! помощников может не использоваться в каждом из них.

#![allow(dead_code)]

use serde_json::json;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::mpsc::{Receiver, RecvTimeoutError};
use std::time::Duration;

/// Минимальный MCP-клиент поверх stdio дочернего процесса.
pub struct Mcp {
    child: Child,
    stdin: ChildStdin,
    rx: Receiver<String>,
    next_id: i64,
}

impl Mcp {
    pub fn start(workspace: &Path) -> Self {
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
                    },
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

    fn send(&mut self, msg: &serde_json::Value) {
        writeln!(self.stdin, "{}", serde_json::to_string(msg).unwrap())
            .expect("write stdin");
        self.stdin.flush().expect("flush stdin");
    }

    pub fn notify(&mut self, method: &str, params: serde_json::Value) {
        self.send(
            &json!({ "jsonrpc": "2.0", "method": method, "params": params }),
        );
    }

    pub fn request(
        &mut self,
        id: i64,
        method: &str,
        params: serde_json::Value,
    ) -> serde_json::Value {
        self.send(&json!({
            "jsonrpc": "2.0",
            "id": id,
            "method": method,
            "params": params,
        }));
        loop {
            let line = match self.rx.recv_timeout(Duration::from_secs(30)) {
                Ok(l) => l,
                Err(RecvTimeoutError::Timeout) => {
                    panic!("таймаут ответа на {method} (id={id})")
                },
                Err(RecvTimeoutError::Disconnected) => {
                    panic!("сервер закрыл stdout")
                },
            };
            let v: serde_json::Value =
                serde_json::from_str(line.trim()).expect("ответ — JSON");
            // Пропускаем уведомления сервера (без id).
            if v.get("id").and_then(serde_json::Value::as_i64) == Some(id) {
                return v;
            }
        }
    }

    fn next_id(&mut self) -> i64 {
        self.next_id += 1;
        self.next_id
    }

    /// Возвращает `(isError, payload)`; payload — JSON из `content[0].text`.
    pub fn call(
        &mut self,
        tool: &str,
        args: serde_json::Value,
    ) -> (bool, serde_json::Value) {
        let id = self.next_id();
        let resp = self.request(
            id,
            "tools/call",
            json!({ "name": tool, "arguments": args }),
        );
        let result = &resp["result"];
        assert!(result.is_object(), "нет result в ответе: {resp}");
        let is_error = result["isError"].as_bool().unwrap_or(false);
        let text = result["content"][0]["text"]
            .as_str()
            .unwrap_or_else(|| panic!("нет content[0].text: {resp}"));
        let payload: serde_json::Value = serde_json::from_str(text)
            .unwrap_or_else(|e| {
                panic!("content[0].text — не JSON ({e}): {text}");
            });
        (is_error, payload)
    }

    /// Успешный `check.create` для `name`/`source`.
    pub fn create(&mut self, name: &str, source: &str) -> serde_json::Value {
        let (err, payload) = self
            .call("check.create", json!({ "name": name, "source": source }));
        assert!(!err, "check.create вернул ошибку: {payload}");
        payload
    }

    pub fn get_draft(&mut self, name: &str) -> (bool, serde_json::Value) {
        self.call("check.get_draft", json!({ "name": name }))
    }
}

impl Drop for Mcp {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

pub fn temp_workspace() -> tempfile::TempDir {
    tempfile::tempdir().expect("tempdir")
}

/// Текст ошибки инструмента: принимает и старый `{"error":"<текст>"}`, и
/// единый конверт T-04 `{"error":{"code","message"}}`.
pub fn error_text(payload: &serde_json::Value) -> String {
    payload["error"]
        .as_str()
        .map(str::to_owned)
        .or_else(|| payload["error"]["message"].as_str().map(str::to_owned))
        .unwrap_or_else(|| panic!("нет текста ошибки: {payload}"))
}

/// Проверяет единый конверт ошибки MCP (Q29, §4.5): `isError = true`,
/// `content[0].text = {"error":{"code","message"}}`; `code` — латиница
/// `snake_case`, `message` — непустой русский текст. Возвращает `message`.
pub fn assert_error_envelope(
    is_error: bool,
    payload: &serde_json::Value,
    expected_code: &str,
) -> String {
    assert!(is_error, "ожидалась ошибка isError=true: {payload}");
    let error = payload
        .get("error")
        .unwrap_or_else(|| panic!("нет error в ответе: {payload}"))
        .as_object()
        .unwrap_or_else(|| {
            panic!("error — не объект (плоский конверт?): {payload}")
        });
    let mut keys: Vec<&str> = error.keys().map(String::as_str).collect();
    keys.sort_unstable();
    assert_eq!(
        keys,
        ["code", "message"],
        "конверт §4.5: только code/message"
    );
    let code = error["code"]
        .as_str()
        .unwrap_or_else(|| panic!("code — не строка: {payload}"));
    assert_eq!(code, expected_code, "код ошибки: {code}");
    assert!(
        code.chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_'),
        "code не латиница snake_case: {code}"
    );
    let message = error["message"]
        .as_str()
        .unwrap_or_else(|| panic!("message — не строка: {payload}"))
        .to_string();
    assert!(!message.is_empty(), "message пуст");
    assert!(
        message
            .chars()
            .any(|ch| matches!(ch, 'А'..='я' | 'Ё' | 'ё')),
        "message не русский (Q11): {message}"
    );
    message
}

/// Код ошибки из единого конверта T-04 (`{"error":{"code","message"}}`).
pub fn error_code(payload: &serde_json::Value) -> &str {
    payload["error"]["code"].as_str().unwrap_or_else(|| {
        panic!("нет error.code в конверте ошибки: {payload}")
    })
}
