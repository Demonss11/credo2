//! Независимые интеграционные тесты T-01 «Черновик: `source`, `source_hash`,
//! `stale`, `test_valid`» (Q12, Q29 §4.5, инварианты 2–5).
//!
//! Тесты гоняют **реальный** бинарник `credo2` по MCP-stdio (JSON-RPC 2.0),
//! а не приватный `McpServer::dispatch`, поэтому проверяют фактический контракт
//! ответов. Покрывают границы, которых нет в юнит-тестах `src/mcp.rs`:
//! пустой/бинарный `.dar`, `source_hash` после перезаписи, загрузка старого
//! `sandbox.json`, идемпотентное удаление и отсутствующий черновик в
//! `check.test`.
//!
//! T-03/Q28 дополняет файл сценариями `check.create` `{name, source}` из
//! `draft.feature` и `agent_minimal.feature`; T-04/Q29 фиксирует коды ошибок
//! единого конверта (`validation_failed`, `draft_not_found`,
//! `evaluation_failed`) и идемпотентность `check.delete_draft`.

mod common;

use common::*;
use serde_json::json;

const SRC: &str = "Правило МинимальныйВозраст { Если (Клиент.Возраст < 21) { \
                   Решение = Отказ; Причина = \"Возраст меньше 21\"; } }";

const SRC_V2: &str = "Правило МинимальныйВозраст { Если (Клиент.Возраст < 18) { \
                      Решение = Отказ; Причина = \"Возраст меньше 18\"; } }";

const DAR: &str = "rules/МинимальныйВозраст.dar";
const NAME: &str = "МинимальныйВозраст";

/// Пустой файл и бинарные (не-UTF-8) байты — это «хэш отличается → `stale`.
#[test]
fn stale_empty_and_binary_dar_q29_inv3() {
    let t = temp_workspace();
    let mut mcp = Mcp::start(t.path());
    mcp.create(NAME, SRC);

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
    mcp.create(NAME, SRC);

    let rules = t.path().join("rules");
    std::fs::create_dir_all(&rules).unwrap();
    std::fs::write(t.path().join(DAR), SRC.as_bytes()).unwrap();

    let (_, d) = mcp.get_draft(NAME);
    let hash_v1 = d["draft"]["source_hash"].as_str().unwrap().to_string();
    assert_eq!(hash_v1, credo2::core::source_hash(SRC));
    assert_eq!(d["draft"]["stale"], json!(false), "{d}");

    // «Сохранить = обновить»: повторный check.create перезаписывает текст.
    let created = mcp.create(NAME, SRC_V2);
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
    // Q28: перезапись — не добавление второй записи («сохранить = обновить»).
    let (_, drafts) = mcp.call("check.list_drafts", json!({}));
    assert_eq!(drafts["count"], json!(1), "{drafts}");
}

/// `check.create`/`check.get_draft`: поля по §4.5, без внутреннего `rule`,
/// `size`, `format`; `source_hash` формата `sha256:<hex>`.
#[test]
fn get_draft_canonical_fields_and_no_internals_q29_inv2_inv5() {
    let t = temp_workspace();
    let mut mcp = Mcp::start(t.path());

    let created = mcp.create(NAME, SRC);
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
    mcp.create(NAME, SRC);

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
    mcp.create(NAME, SRC);

    let (err, tested) = mcp.call(
        "check.test",
        json!({ "name": NAME, "input": { "Клиент.Возраст": 19 } }),
    );
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
    mcp.create(NAME, SRC_V2);
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
    let (err, tested) = mcp.call(
        "check.test",
        json!({ "name": NAME, "input": { "Клиент.Возраст": 19 } }),
    );
    assert!(!err, "{tested}");
    assert_eq!(tested["reason"], "старое");
}

/// Сценарий `draft.feature` «Удаление черновика» + «Повторное удаление
/// черновика не является ошибкой»: первый вызов убирает черновик, обращение к
/// нему даёт `draft_not_found`, повторное удаление — идемпотентный успех
/// (`isError=false`, без конверта `draft_not_found`).
///
/// Примечание: поле `deleted: bool` (§4.5 стр.531) — success-схема задачи
/// T-05, здесь не проверяется (T-04 отвечает только за ошибки).
#[test]
fn delete_is_idempotent() {
    let t = temp_workspace();
    let mut mcp = Mcp::start(t.path());
    mcp.create(NAME, SRC);

    let (err, first) = mcp.call("check.delete_draft", json!({ "name": NAME }));
    assert!(!err, "первое удаление — ошибка: {first}");
    assert!(
        first.get("error").is_none(),
        "первое удаление вернуло конверт ошибки: {first}"
    );

    // Сценарий «Удаление черновика»: обращение к удалённому — `draft_not_found`.
    let (err, got) = mcp.get_draft(NAME);
    assert_error_envelope(err, &got, "draft_not_found");

    let (err, second) = mcp.call("check.delete_draft", json!({ "name": NAME }));
    assert!(
        !err,
        "повторное удаление — ошибка (не идемпотентно): {second}"
    );
    assert!(
        second.get("error").is_none(),
        "повторное удаление вернуло конверт ошибки (ожидался успех deleted:false): {second}"
    );
}

/// Сценарий `draft.feature` «Получение содержимого черновика» (граница):
/// `check.get_draft` по отсутствующему имени → `draft_not_found`.
#[test]
fn get_draft_missing_is_draft_not_found() {
    let t = temp_workspace();
    let mut mcp = Mcp::start(t.path());

    let (err, payload) = mcp.get_draft("НесуществующееПравило");
    let message = assert_error_envelope(err, &payload, "draft_not_found");
    assert!(
        message.contains("черновик не найден"),
        "сообщение: {message}"
    );
    assert!(
        message.contains("НесуществующееПравило"),
        "сообщение должно называть черновик: {message}"
    );
}

/// Сценарий `test_draft.feature` «Тестирование несуществующего черновика»:
/// код `draft_not_found`, сообщение содержит «черновик не найден».
#[test]
fn test_missing_draft_is_draft_not_found() {
    let t = temp_workspace();
    let mut mcp = Mcp::start(t.path());

    let (err, payload) = mcp.call(
        "check.test",
        json!({ "name": "НесуществующееПравило", "input": {} }),
    );
    let message = assert_error_envelope(err, &payload, "draft_not_found");
    assert!(
        message.contains("черновик не найден"),
        "сообщение: {message}"
    );
}

/// Сценарий `test_draft.feature` «Тестирование черновика с отсутствующим
/// полем» (Q8): пустой `input` → `evaluation_failed`, сообщение называет поле.
#[test]
fn test_missing_field_is_evaluation_failed_q8() {
    let t = temp_workspace();
    let mut mcp = Mcp::start(t.path());
    mcp.create(NAME, SRC);

    let (err, payload) = mcp.call("check.test", json!({ "name": NAME, "input": {} }));
    let message = assert_error_envelope(err, &payload, "evaluation_failed");
    assert!(message.contains("Неизвестное поле"), "сообщение: {message}");
    assert!(
        message.contains("Клиент.Возраст"),
        "сообщение должно называть поле: {message}"
    );
}

/// Q9: несовместимый тип значения поля → `evaluation_failed`, не `0`.
#[test]
fn test_incompatible_type_is_evaluation_failed_q9() {
    let t = temp_workspace();
    let mut mcp = Mcp::start(t.path());
    mcp.create(NAME, SRC);

    let (err, payload) = mcp.call(
        "check.test",
        json!({ "name": NAME, "input": { "Клиент.Возраст": "много" } }),
    );
    let message = assert_error_envelope(err, &payload, "evaluation_failed");
    assert!(
        message.contains("Несовместимые типы"),
        "сообщение: {message}"
    );
    assert!(
        message.contains("Клиент.Возраст"),
        "сообщение должно называть поле: {message}"
    );
}

/// Граница входа `check.test`: параметр `input` обязателен → `validation_failed`.
#[test]
fn test_without_input_param_is_validation_failed() {
    let t = temp_workspace();
    let mut mcp = Mcp::start(t.path());
    mcp.create(NAME, SRC);

    let (err, payload) = mcp.call("check.test", json!({ "name": NAME }));
    let message = assert_error_envelope(err, &payload, "validation_failed");
    assert!(message.contains("input"), "сообщение: {message}");
}

// ---------- T-03/Q28: `check.create` — `{name, source}` ----------

/// Сценарий `draft.feature` «Создание черновика через MCP»: `check.create` с
/// `name`+`source` парсит текст, сверяет заголовок, сохраняет черновик в
/// песочнице и возвращает `{status, name}`; текст хранится без изменений, есть
/// `source_hash`; `.dar`-файл не создаётся (draft-first, Q33).
#[test]
fn create_scenario_saves_draft_to_sandbox_with_hash() {
    let t = temp_workspace();
    let mut mcp = Mcp::start(t.path());

    let created = mcp.create(NAME, SRC);
    // Q28/§4.5: успех — ровно {status, name}.
    assert_eq!(
        created,
        json!({ "status": "ok", "name": NAME }),
        "{created}"
    );

    let (err, got) = mcp.get_draft(NAME);
    assert!(!err, "{got}");
    let d = &got["draft"];
    assert_eq!(d["source"], SRC, "текст правила хранится без изменений");
    assert_eq!(d["source_hash"], credo2::core::source_hash(SRC));
    // Q33: `check.create` не материализует файл правила.
    assert!(
        !t.path().join(DAR).exists(),
        "check.create не должен создавать {DAR}"
    );

    // «Сохраняется в песочнице»: новая сессия на том же workspace видит черновик.
    drop(mcp);
    let mut mcp = Mcp::start(t.path());
    let (err, reloaded) = mcp.get_draft(NAME);
    assert!(!err, "черновик не пережил перезапуск: {reloaded}");
    assert_eq!(reloaded["draft"]["source"], SRC);
    assert_eq!(
        reloaded["draft"]["source_hash"],
        credo2::core::source_hash(SRC)
    );
}

/// Сценарий `draft.feature` «check.create с невалидным source отклоняется»:
/// `source` без заголовка «Правило …» → ошибка `validation_failed`, сообщение
/// содержит «отсутствует заголовок правила», черновик не создаётся.
#[test]
fn create_rejects_source_without_rule_header() {
    let t = temp_workspace();
    let mut mcp = Mcp::start(t.path());

    let bad = "Если (Клиент.Возраст < 21) { Решение = Отказ; }";
    let (err, payload) = mcp.call("check.create", json!({ "name": NAME, "source": bad }));
    let message = assert_error_envelope(err, &payload, "validation_failed");
    assert!(
        message.contains("отсутствует заголовок правила"),
        "сообщение: {message}"
    );

    // Черновик не создаётся.
    let (err, got) = mcp.get_draft(NAME);
    assert!(err, "черновик не должен быть создан: {got}");
    let (_, drafts) = mcp.call("check.list_drafts", json!({}));
    assert_eq!(drafts["count"], json!(0), "{drafts}");
}

/// T-03/Q28: `name` не совпадает с заголовком `Правило …` из `source` →
/// ошибка, черновик не создаётся ни под каким именем.
#[test]
fn create_rejects_name_title_mismatch() {
    let t = temp_workspace();
    let mut mcp = Mcp::start(t.path());

    let (err, payload) = mcp.call(
        "check.create",
        json!({ "name": "ДругоеИмя", "source": SRC }),
    );
    let message = assert_error_envelope(err, &payload, "validation_failed");
    assert!(message.contains("не совпадает"), "сообщение: {message}");
    assert!(
        message.contains(NAME),
        "сообщение должно называть заголовок: {message}"
    );

    // Черновик не создан ни под запрошенным именем, ни под заголовком.
    let (err, got) = mcp.get_draft("ДругоеИмя");
    assert!(err, "создан черновик 'ДругоеИмя': {got}");
    let (err, got) = mcp.get_draft(NAME);
    assert!(err, "создан черновик '{NAME}' при расхождении: {got}");
    let (_, drafts) = mcp.call("check.list_drafts", json!({}));
    assert_eq!(drafts["count"], json!(0), "{drafts}");
}

/// T-03/Q28: параметр `name` обязателен — без него ошибка, черновик не создан.
#[test]
fn create_without_name_param_is_error() {
    let t = temp_workspace();
    let mut mcp = Mcp::start(t.path());

    let (err, payload) = mcp.call("check.create", json!({ "source": SRC }));
    let message = assert_error_envelope(err, &payload, "validation_failed");
    assert!(message.contains("name"), "сообщение: {message}");

    let (_, drafts) = mcp.call("check.list_drafts", json!({}));
    assert_eq!(drafts["count"], json!(0), "{drafts}");
}

/// T-03/Q28: параметр `source` обязателен — без него ошибка, черновик не создан.
#[test]
fn create_without_source_param_is_error() {
    let t = temp_workspace();
    let mut mcp = Mcp::start(t.path());

    let (err, payload) = mcp.call("check.create", json!({ "name": NAME }));
    let message = assert_error_envelope(err, &payload, "validation_failed");
    assert!(message.contains("source"), "сообщение: {message}");

    let (_, drafts) = mcp.call("check.list_drafts", json!({}));
    assert_eq!(drafts["count"], json!(0), "{drafts}");
}

/// Сценарий `agent_minimal.feature` «Создание правила через естественный
/// язык»: агент вызывает `check.create` с `name`+`source`, черновик
/// появляется в `check.list_drafts`.
#[test]
fn agent_create_appears_in_drafts_list() {
    let t = temp_workspace();
    let mut mcp = Mcp::start(t.path());

    let created = mcp.create(NAME, SRC);
    assert_eq!(created["status"], "ok", "{created}");

    let (err, drafts) = mcp.call("check.list_drafts", json!({}));
    assert!(!err, "{drafts}");
    assert_eq!(drafts["status"], "ok", "{drafts}");
    assert_eq!(drafts["count"], json!(1), "{drafts}");
    assert_eq!(drafts["drafts"][0]["name"], NAME, "{drafts}");
}
