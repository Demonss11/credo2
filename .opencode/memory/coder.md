# Память: coder (код задачи T-XX)

- **Канон:** карточка `docs/tasks/T-XX-*/README.md` + `Dn` из «Источника»;
  сценарии `docs/features/*.feature`.
- **Правило:** чекпойнт до/после компиляции: карточка, изменённые файлы, что
  осталось, тесты (не запускались — R2). Кратко.

- **Знание:** `cargo`/тесты в моей зоне не запускаются (R2): компиляцию сверяю
  rust-analyzer diagnostics, формат — `rust-analyzer.format` (canonical rustfmt,
  при неоднозначности брать вывод форматтера). Кириллица в строковых литералах
  у rustfmt «широкая»: длинные строки ломаются раньше, чем кажется по числу
  символов — доверять форматтеру, а не подсчёту.

## Чекпойнты

- 2026-09-27 · T-03 (Run 4) · готово: `src/mcp.rs` (`create()` — обязательный
  `name`, сверка с `rule.name`, upsert/`{status,name}` без изменений;
  `tool_specs` — `name` + `required ["name","source"]`), `src/core.rs`
  (`parse_rule` — «отсутствует заголовок правила») + юнит-тесты в модулях.
  cargo не запускался; RA diagnostics чисто, формат сверен форматтером.
- 2026-09-27 · T-03 (Run 4, участок №4 — rework) · готово: одна строка
  `src/mcp.rs:563` — `check_create_tool_spec_requires_name_and_source`:
  `t.name.to_string() == "check.create"` → `t.name == "check.create"`
  (фикс `clippy::cmp_owned` по P1 из `docs/reviews/T-03-2026-09-27.md`).
  Иных правок нет, канон не тронут, cargo не запускался. RA diagnostics
  `src/mcp.rs` → 0/0/0; формат сверен `rust_analyzer_format`.
