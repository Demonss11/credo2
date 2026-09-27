//! Интеграционные тесты T-04/Q29: единый конверт ошибок MCP (§4.5) для
//! инструментов публикации и депрекации (`publish_failed`, `validation_failed`,
//! `draft_not_found`, `version_not_found`, `deprecation_conflict`,
//! `unknown_tool`).
//!
//! Тесты гоняют **реальный** бинарник `credo2` по MCP-stdio и проверяют
//! `isError = true` + `{"error":{"code","message"}}`. Репозиторий публикаций
//! сидируется напрямую через библиотечные хелперы (по образцу `tests/rest.rs`).
//!
//! Вне scope (отдельные задачи): test-гейт публикации («черновик не
//! протестирован», «контрольная сумма не совпадает») — T-02; success-схема
//! `check.delete_draft` (`deleted: bool`) — T-05; `check.run` /
//! `version_deprecated` — вне MVP (Q33).

mod common;

use common::*;
use credo2::core::{contract_from_rule, parse_rule};
use credo2::{commit_tree, ensure_repo, publish, rev_parse, update_ref, write_index_with_parent};
use serde_json::json;
use std::path::Path;

const SRC: &str = "Правило CreditAgeMin { Если (Клиент.Возраст < 21) { \
                   Решение = Отказ; Причина = \"Возраст меньше 21\"; } }";
const NAME: &str = "CreditAgeMin";

/// Сидирует `.credo/published-repo`: публикует версию и сливает ветку в `main`
/// (эмуляция merge PR, как в `tests/rest.rs`).
fn seed_published(workspace: &Path, source: &str, version: &str) {
    let repo = workspace.join(".credo").join("published-repo");
    ensure_repo(&repo).unwrap();
    let rule = parse_rule(source).unwrap();
    let contract = contract_from_rule(&rule, version);
    let outcome = publish(&repo, &rule, &contract, version, "test").unwrap();
    let tree = write_index_with_parent(&repo, &outcome.branch, &[]).unwrap();
    let main = rev_parse(&repo, "main").unwrap();
    let commit = commit_tree(&repo, &tree, &main, "merge").unwrap();
    update_ref(&repo, "refs/heads/main", &commit).unwrap();
}

/// Сценарий `mcp_tools.feature` «check.publish с невалидной версией
/// отклоняется»: `v.0.0.1` → `validation_failed`, «невалидная версия».
#[test]
fn publish_invalid_version_is_validation_failed() {
    let t = temp_workspace();
    let mut mcp = Mcp::start(t.path());

    let (err, payload) = mcp.call(
        "check.publish",
        json!({ "name": NAME, "version": "v.0.0.1" }),
    );
    let message = assert_error_envelope(err, &payload, "validation_failed");
    assert!(
        message.contains("невалидная версия"),
        "сообщение: {message}"
    );
}

/// `check.publish` по отсутствующему черновику → `draft_not_found`
/// (`coder` размечает путь публикации; проверка кода, а не текста).
#[test]
fn publish_missing_draft_is_draft_not_found() {
    let t = temp_workspace();
    let mut mcp = Mcp::start(t.path());

    let (err, payload) = mcp.call("check.publish", json!({ "name": NAME, "version": "1.0.0" }));
    let message = assert_error_envelope(err, &payload, "draft_not_found");
    assert!(
        message.contains("черновик не найден"),
        "сообщение: {message}"
    );
}

/// Сценарий `immutability.feature` / `publish.feature`: повторная публикация
/// той же версии (ветка уже существует) → `publish_failed`.
#[test]
fn publish_duplicate_version_is_publish_failed() {
    let t = temp_workspace();
    let mut mcp = Mcp::start(t.path());
    mcp.create(NAME, SRC);

    let (err, first) = mcp.call("check.publish", json!({ "name": NAME, "version": "1.0.0" }));
    assert!(!err, "первая публикация отклонена: {first}");

    let (err, payload) = mcp.call("check.publish", json!({ "name": NAME, "version": "1.0.0" }));
    let message = assert_error_envelope(err, &payload, "publish_failed");
    assert!(message.contains("уже существует"), "сообщение: {message}");
}

/// Сценарий `mcp_tools.feature`/`deprecation.feature` «check.deprecate с пустым
/// reason отклоняется» → `validation_failed`.
#[test]
fn deprecate_empty_reason_is_validation_failed() {
    let t = temp_workspace();
    seed_published(t.path(), SRC, "1.0.0");
    let mut mcp = Mcp::start(t.path());

    let (err, payload) = mcp.call(
        "check.deprecate",
        json!({ "name": NAME, "version": "1.0.0", "reason": "" }),
    );
    let message = assert_error_envelope(err, &payload, "validation_failed");
    assert!(message.contains("reason"), "сообщение: {message}");
}

/// `check.deprecate` с невалидной версией → `validation_failed`
/// (`mcp_tools.feature`).
#[test]
fn deprecate_invalid_version_is_validation_failed() {
    let t = temp_workspace();
    let mut mcp = Mcp::start(t.path());

    let (err, payload) = mcp.call(
        "check.deprecate",
        json!({ "name": NAME, "version": "v.0.0.1", "reason": "заменена" }),
    );
    let message = assert_error_envelope(err, &payload, "validation_failed");
    assert!(
        message.contains("невалидная версия"),
        "сообщение: {message}"
    );
}

/// `check.deprecate` по отсутствующей версии → `version_not_found`.
#[test]
fn deprecate_missing_version_is_version_not_found() {
    let t = temp_workspace();
    seed_published(t.path(), SRC, "1.0.0");
    let mut mcp = Mcp::start(t.path());

    let (err, payload) = mcp.call(
        "check.deprecate",
        json!({ "name": NAME, "version": "9.9.9", "reason": "нет такой" }),
    );
    let message = assert_error_envelope(err, &payload, "version_not_found");
    assert!(
        message.contains("версия не найдена"),
        "сообщение: {message}"
    );
}

/// Сценарий `deprecation.feature` «Повторная депрекация отклоняется» →
/// `deprecation_conflict`.
#[test]
fn deprecate_twice_is_deprecation_conflict() {
    let t = temp_workspace();
    seed_published(t.path(), SRC, "1.0.0");
    let mut mcp = Mcp::start(t.path());

    let (err, first) = mcp.call(
        "check.deprecate",
        json!({ "name": NAME, "version": "1.0.0", "reason": "заменена на 1.0.1" }),
    );
    assert!(!err, "первая депрекация отклонена: {first}");

    let (err, payload) = mcp.call(
        "check.deprecate",
        json!({ "name": NAME, "version": "1.0.0", "reason": "заменена на 1.0.2" }),
    );
    let message = assert_error_envelope(err, &payload, "deprecation_conflict");
    assert!(message.contains("уже помечена"), "сообщение: {message}");
}

/// Неизвестный инструмент → `unknown_tool` (один из 10 стабильных кодов Q29).
#[test]
fn unknown_tool_is_unknown_tool() {
    let t = temp_workspace();
    let mut mcp = Mcp::start(t.path());

    let (err, payload) = mcp.call("check.nope", json!({}));
    let message = assert_error_envelope(err, &payload, "unknown_tool");
    assert!(
        message.contains("неизвестный инструмент"),
        "сообщение: {message}"
    );
}
