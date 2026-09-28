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
- **T-04 · Run 5, итерация 1** (2026-09-28, `feature/T-04-mcp-errors` от `develop`
  `0437799`, рабочий файл дерева): чекпойнт **до прогона** — прочитаны лента,
  карточка, досье, `review.md`; `git status -sb`/`diff --stat`: правки
  `src/mcp.rs` (ToolError/ErrorCode, конверт §4.5, `call_tool` → `isError=true`),
  `src/lib.rs` (`DeprecateError`), `tests/mcp_draft.rs` + новые
  `tests/common/mod.rs`/`tests/mcp_errors.rs`. `src/rest.rs`/`src/core.rs` НЕ
  тронуты (REST Q23 граница цела). Коды 10, `delete_draft` идемпотентен,
  `draft_not_found` — только get/test/publish. Запускаю полный DoD
  (`fmt`/`clippy -D warnings`/`test --all`); боевая проверка B1 — жду маркер
  token-guard при выводе >12 КБ. Вердикт — в `docs/reviews/T-04-2026-09-28.md`.
- **T-04 · итог прогона:** `fmt` — pass; `clippy --all-targets -- -D warnings` —
  **pass**; `cargo test --all` — **107 passed / 0 failed** (lib 54 +
  `features_inventory` 4/4 + mcp_draft 18 + mcp_errors 8 + publish 12 + rest 11;
  счётчики 47/278). Вердикт **принято** (P1/P2/P3 нет); отчёт
  `docs/reviews/T-04-2026-09-28.md`, квитанция T-04 (iteration 1, accepted).
  B1: маркера `…[token-guard] срез` в выводе `cargo test` **нет** — ~7 КБ < 12 КБ;
  плагин активен (в этой сессии срезан `git diff src/mcp.rs` ≈21 КБ).
  Урок подтверждён: `clippy` после правок `src/**` обязателен (здесь pass).
- **W8-config · приёмка конфиг-пакета качества** (2026-09-28, `develop`
  HEAD `a7eac82` + рабочее дерево, снимок 2026-09-28): чекпойнт **до прогона** —
  прочитаны меморандум `memorandum-W8-run5.md` §0/§4.6, файлы `rustfmt.toml`
  (`edition="2024"`, `max_width=80`, `tab_spaces=4`, `match_block_trailing_comma`,
  `merge_derives`), `rust-toolchain.toml` (`channel="1.96.0"`), `.cargo/config.toml`
  (`rustflags = ["-D","warnings"]`), `.gitattributes` (`* text=auto eol=lf`),
  `opencode.json`; `rg --files -g clippy.toml` — пусто; `rg whitelist opencode.json`
  — нет (git diff: удалён legacy-блок `provider.opencode-go.whitelist`,
  −4 строки); `git status -sb`/`git diff --stat`: ровно 10 файлов
  `src/**`+`tests/**` (реформат 80: core 93, lib 134, main 4, mcp 97, rest 23,
  common/mod 52, features_inventory 28, mcp_draft 6, mcp_errors 14, rest 41),
  плюс канон/журнал/меморандум/`.cargo`/`rust*-*.toml` — **вне скоупа пакета**.
  Смоук-улика доступна чтением: `target/fmt-smoke-2026-09-28.rs` →
  `use serde_json::{Value, json};` (порядок style edition 2024). Запускаю полный
  DoD (`fmt`/`clippy -D warnings`/`test --all`); жду 107/0, `features_inventory`
  4/4 (47/278). Вердикт — в `docs/reviews/W8-config-2026-09-28.md`.
- **W8-config · итог прогона:** `cargo fmt --check` — pass;
  `cargo clippy --all-targets -- -D warnings` — pass (`Finished dev` 1m14s);
  `cargo test --all` — **107/0** (lib 54 + features_inventory 4 + mcp_draft 18 +
  mcp_errors 8 + publish 12 + rest 11), `features_inventory` 4/4 (47/278,
  README «47 файлов, 278 сценариев»). `clippy.toml` — нет (дерево и история);
  `whitelist` в `opencode.json` — нет (удалён legacy-блок, −4). Реформат
  `src/**`+`tests/**` — ровно 10 файлов (+335/−157), выборочные диффы —
  форматирование (`max_width` 80, импорты, trailing-comma в `match`). Вывод
  `cargo test` ~7 КБ — **без** маркера token-guard. Смоук
  `target/fmt-smoke-2026-09-28.rs` прочитан rg: `use serde_json::{Value, json};`
  (style edition 2024) — улика F20 согласуется. Вердикт **принято**
  (P1/P2/P3 нет); отчёт `docs/reviews/W8-config-2026-09-28.md`, квитанция
  `receipts.yaml` (W8-config, iteration 1, accepted). Урок: `git rev-parse`/
  `git ls-files` вне allowlist — HEAD берётся `git log --oneline -1`, состав
  файлов — `git diff --stat`/`git status`; `target/**` доступен точечным `rg`,
  но read-периметром не покрыт.
- **W8-canon · процессная приёмка канон-пакета 28.09.2026** (2026-09-28,
  `develop` HEAD `a7eac82` + рабочее дерево, снимок 2026-09-28): чекпойнт
  **до прогона** — прочитаны меморандум `memorandum-W8-run5.md` (§0–§8),
  `rights-probe-2026-09-28.md`, `findings-registry.md` (F1–F42),
  `run-checklist.md`, `W8-config-2026-09-28.md`, журнал Q47–Q53/D42–D48,
  SPEC §10 №42–48, `TRACEABILITY.md`, фронтматтеры 11 ролей,
  `dispatch-loop.md`/`review.md`/`git-workflow.md`, `token-guard.ts`,
  `.opencode/.gitignore`, память `coder`/`tester`/`auditor`/`migrator`,
  лента `service-mcp-ready-r2.md`. **Машинно:** `opencode reload` — отказ
  (вне allowlist `validator`; разрешено `auditor`); `opencode debug agents`
  ×2 — выводы идентичны (51378 Б, 13 `steps`), 11 ролей; `coder.steps 52`,
  `tester.steps 36`, `auditor.steps 32`, `git/migrator 28`, `lead 16`,
  `analyst/docs-writer/researcher 20`, `rust-expert 24`, `validator 36`;
  `edit .opencode/mail/**` ×11; `opencode reload` — только `auditor`;
  `findings-registry.md` deny у `analyst` (стр.78) + allow у `migrator`
  (стр.1635); `execute: deny` — ровно 2 (docs-writer/git); `git branch -vv`
  у `git`. `git status -sb`/`git diff --stat -- src tests`: ровно 10 файлов
  (+335/−157) — те же, что в W8-config, новых правок `src/**`/`tests/**` нет.
  Линки (меморандум/проба) резолвятся; дублей эскалации нет (канон — только
  `dispatch-loop` §Hard rules); `state/**` согласован в 4 текстах. Запускаю
  `cargo test --all` (страховка DoD на текущем дереве). Вердикт — в отчёте
  `docs/reviews/W8-canon-2026-09-28.md`.
- **W8-canon · итог:** `cargo test --all` — **107/0** (lib 54 + features_inventory
  4 + mcp_draft 18 + mcp_errors 8 + publish 12 + rest 11), `features_inventory`
  4/4 (47/278), вывод ~7 КБ без маркера среза. Машинная сверка прав (11 ролей,
  стабильность ×2) и адресные проверки журнала/ссылок — ок. Вердикт **принято**
  (P1/P2/P3 нет); отчёт `docs/reviews/W8-canon-2026-09-28.md`; квитанция
  `receipts.yaml` (W8-canon, iteration 1, accepted). Техническое: `opencode
  reload` — вне прав `validator` (отказ); заменено `debug agents` ×2. Пакет — к
  коммиту за `git`; F15/F26/F27 открыты.
