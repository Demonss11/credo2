# Память: tester (тесты задачи T-XX)

- **Канон:** `docs/features/*.feature` (сценарии задачи),
  `.opencode/rules/review.md`.
- **Правило:** чекпойнт — какие тесты добавлены, каким сценариям/кейсам
  соответствуют, что компилируется. Прогон — у `validator` (R2). Кратко.

## Чекпойнты

- Чекпойнтов ещё не было.

## 2026-09-27 · T-03 `check.create` `{name, source}` — тесты добавлены

- Добавлено в `tests/mcp_draft.rs` (интеграционные, поверх бинарника по MCP-stdio):
  - `create_twice_overwrites_draft_without_flag_q28` — (b) upsert: повторный
    create = тот же `{status:ok,name}`, текст перезаписан, `count == 1`;
  - `create_name_mismatch_is_error_and_no_draft_q28` — (c) mismatch
    `name`↔заголовок → isError, черновик не создан (`get_draft`/`list_drafts`);
  - `create_missing_name_or_source_is_error_q28` — (d) нет `name` (и нет
    `source`) → isError, count 0;
  - `create_source_without_header_is_error_q28` — (e) source без заголовка →
    isError, сообщение содержит «отсутствует заголовок правила», черновик нет.
  - вспомогательный `error_message(&Value)` — извлечение текста из
    `{"error": …}` (строка или `{message}`).
- (a) положительный контракт уже покрыт существующими
  `get_draft_canonical_fields_and_no_internals_q29_inv2_inv5` и
  `stale_and_hash_after_overwrite_q12` — не дублировал.
- Компиляция: `cargo check --all-targets` → ok; `cargo fmt --check` → ok.
- Не проверено: прогон тестов — за `validator`. Сомнение: не отвергает ли
  rmcp вызов без required-параметра до `dispatch` (тогда (d) получит
  JSON-RPC error вместо tool-result isError).
- Существующие тесты `tests/mcp_draft.rs` новому контракту не противоречат.
