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
- **T-03 · Run 4, итерация 2** (2026-09-27, `feature/T-03-check-create`,
  HEAD `bf56a4a`, рабочий файл дерева): чекпойнт **до прогона** — прочитаны
  лента, карточка T-03/Q28, `draft.feature`, `agent_minimal.feature`,
  `review.md`; `git status -sb`/`git diff`: правки `src/mcp.rs`
  (`create()` name→parse→сверка; `tool_specs` name+required), `src/core.rs`
  (`parse_rule` «отсутствует заголовок правила») + юнит-тесты в модулях,
  `tests/mcp_draft.rs` (+6 сценариев). Канон не тронут. Граница T-04 (D40):
  сцена «невалидный source» — текст есть, код `validation_failed` позже.
  Запускаю полный DoD (`fmt`/`clippy -D warnings`/`test --all`); окно 3/259
  ожидаю 42/259. Вердикт — в `docs/reviews/T-03-2026-09-27.md`.
- **T-03 · Run 4, итог прогона:** `fmt` — pass; `clippy --all-targets -D warnings`
  — **fail** (1 ошибка `cmp_owned`, `src/mcp.rs:563`: `t.name.to_string() ==
  "check.create"` в новом юнит-тесте; подсказка `t.name`); `cargo clippy
  --all-targets` без `-D` — ровно 1 warning, фикс одиночный; `cargo test --all`
  — **90 passed / 0 failed** (lib 49 + features_inventory 4/4 + mcp_draft 14 +
  publish 12 + rest 11), счётчики 42/259. Вердикт: **отклонено (rework)**, P1 —
  красный DoD. Отчёт `docs/reviews/T-03-2026-09-27.md`, квитанция T-03
  (iteration 2, verdict rework). Урок: rust-analyzer diagnostics не ловят
  clippy-lints — `cmp_owned` виден только `cargo clippy` (tester запускал
  `cargo check`, не clippy).
- **T-03 · Run 4, итерация 2, раунд `-r2`** (2026-09-27,
  `feature/T-03-check-create`, HEAD `bf56a4a` + рабочий файл дерева): чекпойнт
  **до прогона** — прочитаны лента (участок №4), отчёт `-r1`, план, карточка,
  `draft.feature` (сцены check.create), `review.md`; `git status -sb`/`diff
  --stat`: правка `src/mcp.rs:563` на месте (`t.name == "check.create"`),
  changed-файлы те же, что на `-r1` (нет новых); канон не тронут. Запускаю
  полный DoD (`fmt`/`clippy -D warnings`/`test --all`); окно 3/259 ожидаю
  42/259. Вердикт — в `docs/reviews/T-03-2026-09-27-r2.md`.
- **T-03 · Run 4, итерация 2, `-r2` — итог прогона:** `fmt` — pass;
  `clippy --all-targets -- -D warnings` — **pass** (`Finished`, прежний
  `cmp_owned` снят); `cargo test --all` — **90 passed / 0 failed**
  (lib 49 + features_inventory 4/4 + mcp_draft 14 + publish 12 + rest 11),
  счётчики 42/259. Вердикт **принято** (P1/P2/P3 нет); отчёт
  `docs/reviews/T-03-2026-09-27-r2.md`, квитанция T-03 — append-запись
  (iteration 2, verdict accepted). Урок подтверждён: `cmp_owned` ловится
  только `cargo clippy`; повторный DoD после правки `src/**` — обязателен.
- **Сервисная приёмка MCP-ready / W8 Run 4** (2026-09-27, HEAD `89ebd42`,
  дерево `develop` + рабочие доки): чекпойнт **до прогона** — прочитаны лента
  `service-mcp-ready.md`, своя память, `tests/features_inventory.rs`.
  `git status --porcelain`: изменены `docs/features/README.md`,
  `docs/tasks/README.md`, `.opencode/mail/T-03.md`,
  `.opencode/memory/git.md`; untracked — `.opencode/commands/`, две service-ленты,
  `docs/analysis/{mcp-ready-process,memorandum-W8-run4}.md`, 5 `agents-*.feature`,
  `docs/tasks/T-15-mcp-ready-process/`. **`src/**` и `tests/**` не тронуты**
  (после merge `89ebd42`). Кода нет → `fmt`/`clippy` не требуются; запускаю
  `cargo test --all`. Ожидание: всё зелёное, `features_inventory` 4/4,
  счётчики `docs/features/README.md` = 47 файлов / 278 сценариев
  (новые 4+4+4+3+4 = 19, процесс 11/55). Вердикт — в отчёте ленты.
- **Сервисная приёмка MCP-ready / W8 Run 4 — итог прогона:** `git status`
  подтвердил отсутствие правок `src/**`/`tests/**`; `cargo test --all` —
  **90 passed / 0 failed** (lib 49 + `features_inventory` 4/4 + `mcp_draft` 14 +
  `publish` 12 + `rest` 11). Счётчики сверены машинно (`readme_totals_match_files`)
  и вручную: 47 `.feature`, 278 сценариев (процесс 11/55, новые 19).
  Вердикт **accepted** (P1/P2/P3 нет), отчёт — append в
  `.opencode/mail/service-mcp-ready.md`. Изменены только два моих файла:
  эта память и лента; ничего не коммитил, канон/код не правил.
