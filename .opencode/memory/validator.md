# Память: validator (приёмка)

- **Канон:** `.opencode/rules/review.md`; `docs/BRIEF.md` §5.3/§7.
- **Правило:** чекпойнт **до** прогона тестов (что проверено, что запускаю) и
  **после** (результат, вердикт) — тесты прерывают сессию (R5). Кратко.

## Чекпойнты

> D41: оперативная хроника задач — в `state/` и ленте; здесь — знание роли и
> аварийные чекпойнты. Записи ниже — история.

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
- **T-12 · H5 (адресная проверка после приёмки, 2026-09-27):** механические
  docs-правки (статус ✅ карточки и сводки, ссылка на отчёт, примечание Run 3).
  `cargo test --test features_inventory` 4/4 ok, счётчики 42/259 (процесс 36),
  ссылка резолвится, `src/**`/`tests/**` не тронуты, вне периметра правок нет.
  Вердикт: **пройдено**; 1 P3 (не блокер) — в `features/README.md:286`
  местоимение «её» читается как T-03 (по смыслу — T-12); рекомендация — «T-12».
  Новый файл отчёта не создавался.
- **T-13** (2026-09-27, `feature/T-13-agent-hardening`, HEAD `c212156`, дерево):
  чекпойнт **до прогона** — прочитаны лента/карточка T-13, D40/D41, Q45/Q46,
  `dispatch-loop.md`, `lead.md`/`analyst.md`, `AGENTS.md`, `git-workflow.md`,
  `review.md`, память 11 ролей (пометка D41), артефакты
  (`rights-matching-2026-09-27.md` §6, `findings-registry.md` F1–F13/H7,
  `run-checklist.md`), SPEC §10 №40/№41, TRACEABILITY, карточки T-13/T-14,
  сводка, ссылка T-03. `git status`/`git diff --stat`: `src/**`/`tests/**`
  не тронуты (только `.opencode/**`, `AGENTS.md`, `docs/**`). Запускаю DoD
  (`fmt`/`clippy`/`test --all`). Вердикт — в отчёте
  `docs/reviews/T-13-2026-09-27.md`.
- **T-13 · итог прогона:** `fmt` — ok, `clippy -D warnings` — ok,
  `cargo test --all` — **78 passed / 0 failed** (`features_inventory` 4/4,
  42/259). `git status`: `src/**`/`tests/**` не тронуты, вне периметра правок
  нет. Вердикт **принято** (P1/P2/P3 нет); отчёт
  `docs/reviews/T-13-2026-09-27.md`, квитанция в `receipts.yaml`
  (task T-13, iteration 1). Калькуляция: lib 43 + mcp_draft 8 + publish 12 +
  rest 11 + features_inventory 4 = 78.
- **T-13 · H5 (адресная проверка после приёмки, 2026-09-27):** механические
  docs-правки — карточка T-13 ⬜ → ✅ + ссылка на отчёт (стр. 11, резолвится в
  `docs/reviews/T-13-2026-09-27.md`), сводка `docs/tasks/README.md` T-13 → ✅
  (T-14 ⬜), `docs/CHANGELOG.md` — запись W7/T-13 в «### Процесс» (стр. 82–99).
  `docs/features/README.md` не менялся (42/259); вне периметра правок нет
  (новый tracked-диф — только CHANGELOG). Вердикт: **ок**; новых P1/P2/P3 нет,
  новый файл отчёта не создавался.
