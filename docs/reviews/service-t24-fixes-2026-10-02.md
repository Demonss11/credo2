# Приёмка: служебная операция r9 (T-15, фаза C) — правки по итогам S-пилота T-24 (Q84 → D87)

- **Тип:** сервисная операция (канон агентов + журнал), адресная документная сверка.
- **Артефакт:** Q84 → D87, права ролей, CCSN-хвост, state hygiene, модель `lead`.
- **Дата:** 2026-10-02.

**Проверка:** D87 ↔ факт (4 пункта), права (`agents-perms.mjs`), журнал
(Q84↔D87, каталоги, `TRACEABILITY`, карточка T-15), границы пакета, артефакты.

**Версия:** `develop` = `origin/develop`, HEAD `6c28e51` + рабочее дерево
(пакет не закоммичен) — снимок 2026-10-02.

**Вердикт:** принято (P1/P2 нет; P3 — одна, не блокер).

**P1:** — нет.
**P2:** — нет.
**P3:** `docs/tasks/T-15-mcp-ready-process/README.md:225` (строка реестра `C14`) —
«Источник» называет разбор `T-24-run-2026-10-02-lead-session.md` по имени, без
ссылки. Канон `dispatch-loop.md` §«Досье и улики» запрещает **ссылки** на
`docs/analysis/**` из задач; форма «имя без линка» соответствует существующему
стилю строк карточки (`:220` `W8 §7`, `:224` `F43`). Последствие отсутствует →
не блокер; закрыть при следующей правке карточки (зафиксировано аудитом r9).

## Что проверено и ок

### 1. D87 ↔ факт (4 пункта)

| Пункт D87 | Факт | Статус |
|---|---|---|
| 1a. `lead` + `git rev-parse --short HEAD` | `.opencode/agents/lead.md:19` (`git rev-parse --short HEAD`, allow); тело `:43-44` — норма `#default` (основание `D87`) | ok |
| 1b. `git` + `git branch --list *`, `git ls-remote --heads origin *` | `.opencode/agents/git.md:22-23` (обе команды, allow) | ok |
| 1c. `tester` + `cargo clippy *` | `.opencode/agents/tester.md:19` (allow); шаг 4 `:51-56` (линт `cargo clippy --all-targets -- -D warnings`); шаблон отчёта `:79` («Компиляция/линт … clippy — ok») | ok |
| 1d. `analyst` — без расширения, заметка | `.opencode/agents/analyst.md:49-50` — ветку/HEAD проверяет косвенно, `git branch`/`rev-parse` вне прав (`D87`); allowlist не расширен | ok |
| 2. CCSN в `git-workflow.md` §«Завершение задачи» | `.opencode/rules/git-workflow.md:117-121` — origin-ветку удаляет владелец вручную, `git` затем `branch -d`; шаг планируется заранее (гейт + ожидание), без re-plan-цикла (`D87`) | ok |
| 3. Stale-детекция в `dispatch-loop.md` §«Состояние и записи» | `.opencode/rules/dispatch-loop.md:110-113` — `progress = complete` при противоречащем плане → первым действием `analyst` (переинициализация; `D87`) | ok |
| 4. Норма `#default` в `lead.md` | `.opencode/agents/lead.md:43-44` — сессия `lead` на `#default`; `#max` только отдельным решением владельца (`D87`) | ok |

### 2. Права и `review.md`

- `node .opencode/scripts/agents-perms.mjs` → `agents: 11 из 18`; новые права
  видны: `lead` — `git rev-parse --short HEAD`; `git` — `git branch --list *`,
  `git ls-remote --heads origin *`; `tester` — `cargo clippy *`; `analyst` — без
  расширения (`reload` применён).
- `review.md` §«Доступные команды» (`:88-105`) сверен с фронтматтерами
  изменённых ролей — расхождений нет: `lead:96` (= `rev-parse --short HEAD`),
  `git:101-105` (read-only + `ask`-список), `tester:92` (+`clippy`),
  `analyst:93-94` (косвенно, вне прав).

### 3. Журнал и связи

- Q84↔D87 парны: Q84 «resolved by [D87]» (`Q84.md:3`); D87 `Resolves: [Q84]`
  (`D87:5`); D87 — 4 пункта, «Сверка с кодом» с вердиктом ⚪
  (`D87:61-80`, процесс/документы; `cargo` не запускался, D50).
- Каталоги: `questions/README.md:106` (Q84 → D87) и `decisions/README.md:116`
  (D87 → Q84, `accepted`, 2026-10-02).
- `TRACEABILITY.md:88` — Q84: D87, `in work`, `[T-15] 🚧`, реализация = периметр
  правок.
- Карточка T-15: строка `C14` 🚧 (`README.md:225`); данные T-24 в F26/F27
  (`:78-89`) и findings F67–F72.
- Ссылки Q84/D87 живы (проверены целиком: `D47`,`D50`,`D70`,`D78`,`D86`, T-15,
  findings-registry, разбор, `journal.md`, `TRACEABILITY`); номеров строк
  адресных нет (`rg ':[0-9]{2,}'` по Q84/D87 — пусто).

### 4. Статические гейты `tests/docs_journal.rs` (D50 — без прогона)

Проверено чтением тела тестов; все зелёные по состоянию артефактов:

- `questions_have_decision_or_exception` (:378) — Q84 `resolved by`, ровно один
  существующий D87;
- `decisions_have_existing_question_or_retro_exception` (:412) — D87 `Resolves`
  на существующий Q84;
- `decisions_are_listed_in_catalog` (:444) / `questions_are_listed_in_catalog_and_traceability`
  (:464) — Q84/D87 в каталогах и TRACEABILITY;
- `every_decision_has_code_review_verdict` (:504) — «Вердикт:» в «Сверке с кодом»;
- `traceability_lifecycle_uses_canon_dictionary` (:524) — `in work`;
- `traceability_rows_reference_existing_decisions` (:544) — D87 существует;
- `traceability_lifecycle_matches_task_openness` (:564) — `in work` ∧ T-15 🚧;
- `traceability_tasks_exist_and_match_registry` (:592) — T-15: реестр
  `tasks/README.md:56` 🚧 = TRACEABILITY `[T-15] 🚧` (строки Q74/Q83/Q84).

### 5. Границы

- `git diff --numstat -- src tests Cargo.toml` — пусто; `git diff --check` — пусто.
- `git status --porcelain` — изменены только разрешённые пути:
  `.opencode/agents/{lead,git,analyst,tester}.md`,
  `.opencode/rules/{review,git-workflow,dispatch-loop}.md`,
  `docs/{questions,decisions}/**` (+`README` каталогов), `docs/TRACEABILITY.md`,
  карточка T-15, `docs/analysis/findings-registry.md`,
  `docs/analysis/T-24-2026-10-02.md` (досье), лента `T-24`, `state/current/*`,
  память ролей; новые `??`: лента r9, разбор, Q84, D87. `src/**`, `tests/**`,
  `Cargo.toml`, иной канон не тронуты.
- Другие правки канона (сверх D87) не обнаружены.

### 6. Артефакты

- Отчёт приёмки — `docs/reviews/service-t24-fixes-2026-10-02.md` (этот файл;
  имя по стандарту `service-<тема>-<дата>.md`).
- Квитанция — append в `.opencode/state/current/receipts.yaml`
  (`task: service-t24-fixes`, `iteration: 1`, `accepted`).
- Отчёт в ленту r9 + чекпойнт в память.

## Не проверено

- `cargo fmt|clippy|test` — **не запускались** (D50 + решение владельца
  02.10.2026 «тесты только при правках кода»): пакет не трогает `src/**`,
  `tests/**`, `Cargo.toml`. Счётчики/состав `docs/features/**` не менялись —
  адресный `features_inventory` не требуется.
- `opencode mcp list` — вне прав `validator`; факт восстановления MCP-каталога
  после `reload` взят из ленты r9 (аудит).
