# T-22. Интеграционные тесты `tests/mcp_draft.rs`: снять предсуществующие ошибки компиляции (E0425, E0061)

- **Статус:** ✅ сделана
- **Приоритет:** P3 (post-accept дефект T-04; v0.1.x — гигиена тестов, контракт
  не меняется)
- **Зависит от:** —
- **Источник:** T-04 (post-accept дефект); журнальная линия — Q29 (§4.5),
  [D34](../../decisions/D34-mcp-tool-contracts.md) (линия контракта T-03/T-04)

## Контекст

Тесты `tests/mcp_draft.rs` (контракт `check.create`/`check.test`: T-03/Q28,
T-04/Q29 §4.5) **не компилируются** — это предсуществующий дефект рабочего
дерева (`tracked = HEAD/develop` на момент находки; не введён ни T-18, ни
T-21). Полный прогон `cargo test --all` падает; продуктовое поведение
(`src/**`) корректно — дефект локализован в тестовом файле.

Точные ошибки (снимок на 02.10.2026, атрибуция — отчёт `validator` T-21):

1. **E0425** в `create_rejected` (`tests/mcp_draft.rs`): тип `Value` в сигнатуре
   не в scope — импортирован только `serde_json::json`, а `Value` — нет.
2. **E0061 ×3** (`create_name_mismatch_keeps_existing_draft_t03`,
   `create_invalid_source_keeps_existing_draft_t03`,
   `create_upsert_replaces_single_draft_t03`): вызов `mcp.create(SRC)` не
   совпадает с сигнатурой `Mcp::create(&mut self, name, source)`
   (`tests/common/mod.rs`) — пропущен аргумент `name`.

## Что сделать

1. **E0425 (`Value` не в scope):** добавить `Value` в импорт
   (`use serde_json::{json, Value};` либо отдельная строка `use
   serde_json::Value;`). Сигнатура `create_rejected(mcp: &mut Mcp, args: Value)`
   сохраняется.
2. **E0061 ×3 (`mcp.create(SRC)`):** привести вызовы к сигнатуре
   `Mcp::create(&mut self, name, source)` (`tests/common/mod.rs`) —
   `mcp.create(NAME, SRC)`. Затронуты три теста из п. «Контекст» (в файле уже
   есть корректная форма `mcp.create(NAME, SRC)` — привести к ней).

**Тест не ослабляется:** правка — только снятие ошибок компиляции (импорт и
аргумент); смысл контракта T-03/Q28 и проверяемые утверждения не меняются.

## Критерий готовности

`tests/mcp_draft.rs` компилируется; полный DoD задачи
([`../README.md`](../README.md) §«DoD для любой задачи»). Контракт
`check.create`/`check.test` (T-03/Q28, Q29 §4.5) не меняется.

## Примечания

- **Исполнитель** — `tester` (владелец `tests/**`); правки — только
  `tests/mcp_draft.rs` в ветке T-22. `src/**`, `Cargo.toml`, канон агентов не
  трогаются.
- **Прогон** `cargo test --all` выполняет `validator` (R2/D50).
- Точные строки дефекта — снимок на 02.10.2026 (в отчёте `validator` T-21 и
  ленте T-21); ссылки — по символам (`create_rejected`, имена тестов,
  сигнатура `Mcp::create`), не по номерам строк (антипаттерн
  [§8](../../../.opencode/rules/journal.md)).
- Происхождение: post-accept находка при приёмке [T-21](../T-21-mcp-test-struct-api/README.md);
  отдельного `Q`/`D`/`F` не заводится (дефект не журнальный — линия T-04
  прослеживается через `Источник` и строку `TRACEABILITY` Q29/D34).
- F43: пакеты T-18/T-21/T-22 раздельны — T-22 коммитится своей веткой
  `feature/T-22-<слаг>`; чужие рабочие пути в пакет не подмешивать.
