# CHANGELOG — credo2

> Фиксируются только изменения кода продукта (решение
> [D72](decisions/D72-changelog-full-cleanup.md)); правки документации и процесса
> — в журнале Q/D (`.opencode/rules/journal.md`).

## 0.1.0 (в разработке)

- **T-21** (`src/mcp.rs`): unit-тесты приведены к struct-API `ToolError`
  (`validation_message` — поля `code`/`message` вместо enum-вариантов;
  `validation_error_json_envelope_t03` — `to_json()` вместо снятого
  `into_json()`); dead `required_str` задействован в `check.create` (Q28).
  Поведение MCP-конверта и кодов не меняется (Q29 §4.5). Отчёт приёмки —
  `docs/reviews/T-21-2026-10-02-r3.md`.
- **T-18** (`tests/docs_journal.rs`): новый тест целостности журнала Q/D —
  ID, парность `Q`↔`D`, таблицы индексов, запреты ссылок и запрет номеров строк
  (D64/Q60, D65, D77, D80); 14 проверок. Отчёт приёмки —
  `docs/reviews/T-18-2026-10-02-r2.md`.
- **T-22** (`tests/mcp_draft.rs`): сняты предсуществующие ошибки компиляции —
  E0425 (`Value` в импорте `serde_json`) и E0061 ×3 (`mcp.create(SRC)` →
  `mcp.create(NAME, SRC)`); тест не ослаблен, контракт `check.create`/`check.test`
  (T-03/Q28, Q29 §4.5) не менялся. Подтверждено в отчёте приёмки —
  `docs/reviews/T-21-2026-10-02-r3.md`.
