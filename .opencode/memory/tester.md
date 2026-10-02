# Память: tester (тесты задачи T-XX)

- **Канон:** `docs/features/*.feature` (сценарии задачи),
  `.opencode/rules/review.md`.
- **Правило:** чекпойнт — какие тесты добавлены, каким сценариям/кейсам
  соответствуют, что компилируется. Прогон — у `validator` (R2). Кратко.
- **Знание:** `git -C <путь>` отклоняется всегда (проба 28.09.2026) — рабочая
  директория задаётся полем `workdir`, с `-C` не экспериментировать.

## Чекпойнты

- **T-18 (2026-10-02, участок №2) — до компиляции.** Реализую
  `tests/docs_journal.rs` (Rust, только `std`; образец — `features_inventory.rs`),
  v0.1: проверки (1)–(8) карточки T-18/D64 (+D77/D80/D82/D65). Разбор форматов:
  Q-статус `resolved by Dn` (все, кроме Q81 `open`); D `Resolves: [Qn]` либо `—`
  (ретро-D D1–D5, D7–D11, D13); TRACEABILITY-колонки
  `Q | D | Жизненный цикл | Задачи | Реализация`; реестры `tasks/README`,
  `features/README`. Скоуп запретов — канон журнала; «Сверка с кодом» D-файлов —
  датированный снимок (аналог историч. зон, карточка T-18 §Примечания), номера
  строк в ней не гейтятся; миграционные фразы — только в шапке/таблице, `OPEN_QUESTIONS.md` —
  только как markdown-ссылка; адреса — только directory-qualified пути в
  удаляемые зоны (whitelist `findings-registry.md`). Прогон — `validator` (R2).
  Далее: `cargo check --all-targets`, `cargo fmt`.
  **Результат (участок №5):** dead_code `TraceRow.d` снят усилением — тест
  `traceability_rows_reference_existing_decisions` читает `row.d` (D-колонка →
  существующее решение; пусто только при `open`, Q81). `cargo clippy --test
  docs_journal -- -D warnings` — ok; `cargo fmt --check` — ok (правок не
  потребовалось). D85 (б): `live_text` — снимок только для номеров строк,
  адресный гейт по сырому тексту. Прогон `cargo test --all` — `validator` (R2).
  **Результат (участок №8, rework -r2):** исправлены 4 дефекта разбора
  (P1-1..P1-4 отчёта `docs/reviews/T-18-2026-10-02.md`), проверки не ослаблены:
  (1) `field_line` триммит значение; (2) `cell_task_statuses` понимает `T-XX`
  (опц. `-`) и ищет статус по всей ячейке после markdown-ссылки; (3) `section()`
  ищет заголовок только в начале строки (inline `## …` не ловит); (4)
  `normalize_feature` срезает `features/`/`docs/features/` в `feature_tokens`
  и `feature_registry`. `cargo clippy --test docs_journal -- -D warnings` — ok
  (снят clippy collapsible_if let-chain); `fmt --check` — ok. Канон-дефект D39
  снят `migrator`. Прогон — `validator` T-18 -r2 (R2).

- **T-22 (2026-10-02) — до компиляции.** Снятие предсуществующих ошибок
  `tests/mcp_draft.rs`: (1) E0425 — `Value` не в scope → импорт
  `use serde_json::{json, Value};`; (2) E0061 ×3 (`create_name_mismatch...`,
  `create_invalid_source...`, `create_upsert...`) — `mcp.create(SRC)` →
  `mcp.create(NAME, SRC)` под сигнатуру `common::Mcp::create(name, source)`.
  Тест не ослаблен (T-03/Q28 сохранён). Только `tests/mcp_draft.rs`; чужие
  пути T-18/T-21 не тронуты. Далее: `cargo check --test mcp_draft`, `cargo fmt`.
  Прогон `cargo test --all` — `validator` (R2/D50).
  **Результат:** `cargo check --test mcp_draft` — ok (7.03s, E0425/E0061 сняты);
  `cargo fmt --check` — ok (rustfmt также перенормировал 4 длинные строки в том
  же файле). Правки только в `tests/mcp_draft.rs`; T-18/T-21 не тронуты. Прогон
  не делал (R2).
