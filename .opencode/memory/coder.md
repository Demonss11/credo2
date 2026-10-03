# Память: coder (код задачи T-XX)

- **Канон:** карточка `docs/tasks/T-XX-*/README.md` + `Dn` из «Источника»;
  сценарии `docs/features/*.feature`.
- **Правило:** чекпойнт до/после компиляции: карточка, изменённые файлы, что
  осталось, тесты не запускались (R2 — прогон у `validator`). Кратко.

- **Знание:** `cargo test` не запускается (R2 — тесты только у `validator`);
  штатные для роли — `cargo fmt --check`, `cargo check`, `cargo clippy
  --all-targets -- -D warnings` (`agents/coder.md` §«Порядок», п. 5).
  RA-диагностики/формат — вспомогательные, не замена cargo. Кириллица в
  строковых литералах у rustfmt «широкая»: доверять форматтеру, а не подсчёту.

## Чекпойнты

- **T-16 (stale `check.test`) — чекпойнт до компиляции (2026-10-02).**
  Карточка T-16, D54 п.3, `test_draft.feature` «Тестирование устаревшего
  черновика». План: в `src/mcp.rs::test()` при `state.is_stale(&d)` разобрать и
  исполнить текст `rules/{name}.dar` (через `AppState::source_file_path` +
  `parse_rule`), иначе — текущее поведение. Контракт §4.5/D34 не меняется:
  `source_hash`/метки — от черновика. Ошибки: нечитаемый/невалидный файл —
  `evaluation_failed` (`ToolError::evaluation`) — решение в рамках D40.
  Правки: `src/lib.rs` (один хелпер `source_file_path`), `src/mcp.rs` (тело
  `test()` + юнит-тест). Тесты не запускались (R2); далее — `cargo fmt/check/
  clippy`. Чекпойнт «после» — ниже.

- **T-16 — после компиляции (2026-10-02).** Готово.
  - `src/lib.rs:811-816`: `AppState::source_file_path(name) -> PathBuf`;
    `is_stale` использует его (поведение то же).
  - `src/mcp.rs:224-246`: в `test()` при `state.is_stale(&d)` — чтение
    `rules/{name}.dar` и `parse_rule`; нечитаемый/невалидный файл →
    `ToolError::evaluation` (`evaluation_failed`). Метки `source_hash`/
    `last_test_checksum`/`tested_at` — от черновика (контракт §4.5/D34 цел).
  - Юнит-тесты: `stale_test_executes_file_text_q12_t16` (исполнение по файлу),
    `stale_test_invalid_file_is_evaluation_failed_t16` (код ошибки).
  - Проверки: `cargo fmt --check` — ok; `cargo check --all-targets` — ok;
    `cargo clippy --all-targets -- -D warnings` — ok. `cargo test` не
    запускался (R2/D50).
