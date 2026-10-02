# T-21. Unit-тесты `src/mcp.rs`: привести к struct-API `ToolError` и снять dead `required_str`

- **Статус:** ✅ сделана
- **Приоритет:** P3 (post-accept дефект T-04; v0.1.x — гигиена кода, продуктовое
  поведение не меняет)
- **Зависит от:** [T-04](../T-04-mcp-errors/README.md) (закрыта — исходная
  реализация конверта ошибок)
- **Источник:** T-04 (post-accept дефект); журнальная линия — Q29 (§4.5),
  [D34](../../decisions/D34-mcp-tool-contracts.md)

## Контекст

Задача [T-04](../T-04-mcp-errors/README.md) (MCP-ошибки: конверт `isError` и
10 стабильных кодов, Q29 §4.5) закрыта. При последующей сверке/компиляции
`src/mcp.rs` его **unit-тесты** остались на снятом enum-API `ToolError`: тип
превратился в `struct ToolError { code, message }`, а тесты по-прежнему
матчат enum-варианты и зовут отсутствующий метод. Компиляция `cargo test --all`
падает (E0223 / E0599); продуктовое поведение (`src/mcp.rs` не в тестовых
модулях) корректно — дефект локализован в `#[cfg(test)]`-модуле `src/mcp.rs`.

## Что сделать

1. **`validation_message`** (`src/mcp.rs`): matcher по enum-вариантам
   `ToolError::Envelope { code, message }` (E0223) и `ToolError::Message(_)`
   (E0599) заменить на struct-API — поля `code`/`message` структуры
   `ToolError`; сравнение кода — по `ErrorCode`/`code.as_str()` вместо строкового
   `assert_eq!(code, "validation_failed")`.
2. **Тест `validation_error_json_envelope_t03`** (`src/mcp.rs`): вызов снятого
   `ToolError::validation("…").into_json()` (E0599) заменить на существующий
   `to_json()` (конверт `{"error":{"code","message"}}`, Q29 §4.5).
3. **`required_str`** (`src/mcp.rs`): функция объявлена, но не вызывается
   (dead_code). Привести в порядок: либо задействовать по назначению (проверка
   обязательного непустого строкового параметра `check.create`, Q28) — с юнит-
   тестом, либо снять функцию; выбранный вариант отразить в отчёте.

## Критерий готовности

`cargo test --all` включает unit-тесты `src/mcp.rs` и он зелёный; DoD задачи
([`../README.md`](../README.md) §«DoD для любой задачи»). Поведение MCP-конверта
и кодов не меняется (Q29 §4.5).

## Примечания

- **Исполнитель** — `coder` (владелец `src/**`); правки — только `src/mcp.rs` в
  ветке T-21. `tests/**`, `Cargo.toml`, канон агентов не трогаются.
- **Прогон** `cargo test --all` выполняет `validator` (R2).
- Точные строки дефекта — снимок на 02.10.2026 (в теле задачи/ленте T-18);
  ссылки — по символам (`validation_message`,
  `validation_error_json_envelope_t03`, `required_str`), не по номерам строк
  (антипаттерн [§8](../../../.opencode/rules/journal.md)).
- Происхождение: post-accept находка при подготовке [T-18](../T-18-docs-journal-test/README.md);
  отдельного `Q`/`D`/`F` не заводится (дефект не журнальный — линия T-04
  прослеживается через `Источник` и строку `TRACEABILITY` Q29/D34).
