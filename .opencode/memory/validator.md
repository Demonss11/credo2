# Память: validator (приёмка)

- **Канон:** `.opencode/rules/review.md`; `docs/BRIEF.md` §5.3/§7.
- **Правило:** чекпойнт **до** прогона тестов (что проверено, что запускаю) и
  **после** (результат, вердикт) — тесты прерывают сессию (R5). Кратко.

## Чекпойнты

- **T-12** (2026-09-27, `feature/T-12-agent-loop`, HEAD `f06ff82`, дерево):
  чекпойнт **до прогона** — прочитаны лента/карточка D39/Q44, `dispatch-loop.md`,
  роли, `review.md`, `git-workflow.md`, фичи; счётчики 42/259 (процесс 36);
  противоречий канона и остатков старого цикла нет. Запускаю DoD
  (`fmt`/`clippy`/`test --all`). Вердикт — в отчёте `docs/reviews/T-12-2026-09-27.md`.
- **T-12 · итог прогона:** `fmt` — ok, `clippy -D warnings` — ok,
  `cargo test --all` — **78 passed / 0 failed** (`features_inventory` 4/4,
  42/259). Вердикт **принято** (P1/P2/P3 нет); отчёт
  `docs/reviews/T-12-2026-09-27.md`, квитанция в `receipts.yaml`
  (task T-12, iteration 1). `cargo test` калькуляция: lib 43 + int 35 = 78
  (не 88 — 88 = базис Run 2, D39 §7).
- **T-03** (2026-09-27, `feature/T-03-check-create`, HEAD `c212156`, дерево):
  чекпойнт **до прогона** — прочитаны лента `mail/T-03.md`, досье
  `docs/analysis/T-03-2026-09-27.md`, `draft.feature`, `agent_minimal.feature`;
  код `src/mcp.rs:44-72` (оба параметра обязательны, сверка name↔rule.name,
  `tool_specs` required ["name","source"]), `src/core.rs:444` («отсутствует
  заголовок правила»), тесты `tests/mcp_draft.rs:411-526` (+4 кейса, helper
  `error_message`), юнит-тесты `mcp.rs:514-596`. Границы T-03/T-04 соблюдены
  (конверт `{error:{code,message}}`/`validation_failed` не внедрён). Открытый
  вопрос tester: поведение rmcp при нарушении `required` (JSON-RPC error vs
  tool-result isError) — проверит прогон кейса (d). Запускаю DoD
  (`fmt`/`clippy`/`test --all`). Вердикт — ниже и в ленте.
- **T-03 · итог прогона:** `fmt` — pass, `clippy -D warnings` — pass,
  `cargo test --all` — **86 passed / 0 failed** (lib 47 + features_inventory 4 +
  mcp_draft 12 + publish 12 + rest 11). Вердикт **принято**, P1/P2/P3 нет;
  отчёт `docs/reviews/T-03-2026-09-27.md`, квитанция (T-03, iteration 1,
  accepted). Открытый вопрос tester закрыт фактом: кейс (d)
  `create_missing_name_or_source_is_error_q28` проходит без адаптации — rmcp не
  отклоняет по `required` до `dispatch`, отказ приходит как tool-result
  `isError`. Границы T-03/T-04 соблюдены (конверт/`validation_failed` — T-04).
- **T-12 · H5 (адресная проверка после приёмки, 2026-09-27):** механические
  docs-правки (статус ✅ карточки и сводки, ссылка на отчёт, примечание Run 3).
  `cargo test --test features_inventory` 4/4 ok, счётчики 42/259 (процесс 36),
  ссылка резолвится, `src/**`/`tests/**` не тронуты, вне периметра правок нет.
  Вердикт: **пройдено**; 1 P3 (не блокер) — в `features/README.md:286`
  местоимение «её» читается как T-03 (по смыслу — T-12); рекомендация — «T-12».
  Новый файл отчёта не создавался.
