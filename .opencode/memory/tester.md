# Память: tester (тесты задачи T-XX)

- **Канон:** `docs/features/*.feature` (сценарии задачи),
  `.opencode/rules/review.md`.
- **Правило:** чекпойнт — какие тесты добавлены, каким сценариям/кейсам
  соответствуют, что компилируется. Прогон — у `validator` (R2). Кратко.

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
