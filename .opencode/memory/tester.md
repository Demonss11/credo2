# Память: tester (тесты задачи T-XX)

- **Канон:** `docs/features/*.feature` (сценарии задачи),
  `.opencode/rules/review.md`.
- **Правило:** чекпойнт — какие тесты добавлены, каким сценариям/кейсам
  соответствуют, что компилируется. Прогон — у `validator` (R2). Кратко.
- **Знание:** `git -C <путь>` отклоняется всегда (проба 28.09.2026,
  `docs/analysis/rights-probe-2026-09-28.md`) — рабочая директория задаётся
  полем `workdir`, с `-C` не экспериментировать.

## Чекпойнты

> D41: оперативная хроника задач — в `state/` и ленте; здесь — знание роли и
> аварийные чекпойнты. Записи ниже — история.

- **Каркас тестов MCP:** `tests/mcp_draft.rs` — `Mcp` (реальный `credo2`,
  stdio), `mcp.call(tool,args)->(isError,payload)`; ошибки до T-04 — `{"error":"текст"}`,
  helper `error_text` принимает и `{"error":{"message":…}}`.

## Чекпойнты

- **T-03 (2026-09-27):** добавлены в `tests/mcp_draft.rs`:
  `create_scenario_saves_draft_to_sandbox_with_hash` (draft.feature «Создание
  черновика через MCP» + персистентность после рестарта + `.dar` не создаётся),
  `create_rejects_source_without_rule_header` (сцена «невалидный source»),
  `create_rejects_name_title_mismatch`, `create_without_name_param_is_error`,
  `create_without_source_param_is_error`, `agent_create_appears_in_drafts_list`
  (agent_minimal). Расширен `stale_and_hash_after_overwrite_q12` (перезапись →
  `count==1`). Компиляция: `cargo check --all-targets` ok, `cargo fmt --check`
  ok. Прогон — за `validator`.

- **T-04 (2026-09-28), чекпойнт «продолжение»:** конверт ошибок §4.5 —
  `{"error":{"code","message"}}`, коды латиница snake_case / русский текст.
  Рефактор общего каркаса: создан `tests/common/mod.rs` (`Mcp`, `error_text`,
  `error_code`, `assert_error_envelope`, `temp_workspace`); `tests/mcp_draft.rs`
  переведён на `mod common` (в работе — см. ленту T-04).
  Разделение scope: success-схема `check.delete_draft` (`deleted: bool`) — T-05;
  test-гейт публикации — T-02; `version_deprecated`/`check.run` — вне MVP (Q33).
  Компиляция/прогон — за `validator`.

- **T-04 (2026-09-28), чекпойнт «готово»:** интеграционные тесты ошибочных
  сценариев. `tests/common/mod.rs` (новый) — общий каркас MCP (`Mcp`,
  `temp_workspace`, `error_text`, `error_code`, `assert_error_envelope`).
  `tests/mcp_draft.rs` — на `common`, зафиксированы коды (`validation_failed`,
  `draft_not_found`, `evaluation_failed`), `delete_is_idempotent` усилен
  (`get_draft` после удаления → `draft_not_found`; повторное удаление — успех),
  добавлены Q8/Q9 и отсутствие `input`. `tests/mcp_errors.rs` (новый) —
  `publish_failed`/`publish` невалидная версия/без черновика, `deprecate`
  (пустой reason, невалидная/отсутствующая версия, повтор) → `deprecation_conflict`,
  `unknown_tool`; сидирование `.credo/published-repo` по образцу `rest.rs`.
  Проверки: `cargo fmt --check` ok, `cargo check --all-targets` ok; `cargo test`
  не запускался (за `validator`). Грабли: автоформаттер при сохранении сортирует
  mixed-case импорты иначе, чем `cargo fmt` (style edition) — писать однотипные
  импорты (`use common::*;`, полное `serde_json::Value`) или сверять
  `cargo fmt --check` после правок.
