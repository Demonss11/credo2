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

- **2026-10-02 · T-21 (feature/T-21-mcp-test-struct-api) · чекпойнт до компиляции.**
  Задача: починить `src/mcp.rs` до `cargo check --all-targets` зелёного (post-accept
  дефект T-04), только этот файл. Сделано:
  (1) `validation_message` — enum-матч `ToolError::Envelope{..}`/`Message(_)`
  заменён на struct-API: `assert_eq!(err.code, ErrorCode::ValidationFailed); err.message`;
  (2) `validation_error_json_envelope_t03` — `into_json()` → `to_json()`;
  (3) `required_str` — задействован в `create` для `name`/`source` (вариант
  «использовать», не удалять): `None`-ветка даёт тот же текст `"Нужен параметр
  '<key>'"`, тест `create_requires_name_param` цел, новый отказ — пустая
  непустая по `trim()` строка (расширение валидации Q28; тесты 717/720/726 ждут
  `contains("name")`).
  Осталось: fmt/check/clippy. `cargo test` не запускается (R2). Риск: если
  validator ужесточит DoD по пустому `source` — это ожидаемо и не ломает прод.

- **2026-10-02 · T-21 · результат.** Компиляция: `cargo check --lib --profile test`
  ok; `cargo clippy --lib -- -D warnings` ok; `cargo check --all-targets` — по
  `src/mcp.rs` ноль диагностик, осталось только `tests/**` (T-18, не моя зона).
  `cargo fmt --check` — диффы только в `tests/mcp_draft.rs` (T-18), по `src/mcp.rs`
  чисто. `cargo test` не запускался (R2). Отчёт — лента `.opencode/mail/T-21.md`.
  Следующему (validator): ждать закрытия T-18 для зелёных `tests/**`.
