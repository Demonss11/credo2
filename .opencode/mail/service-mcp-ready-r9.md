# service-mcp-ready-r9 — лента операции: T-15, правки по итогам S-пилота T-24

Открыта: 02.10.2026. **Сервисная операция** (программа T-15, фаза C; после C1).
Предмет (решение владельца 02.10.2026, выбор «Сначала дешёвые правки»): права
ролей (F69), CCSN-процедура удаления ветки (F71), `tester` + `cargo clippy`,
state hygiene (F67-часть), модель `lead` (F70). C9/C12 (записи, re-plan, лимит
16) — отдельным решением.

**Входы:** разбор `docs/analysis/T-24-run-2026-10-02-lead-session.md`; реестр
находок (F26/F27/F58, F67–F72); лента `.opencode/mail/T-24.md`.

**Рамка:** канон — `.opencode/agents/{lead,git,analyst,tester}.md`,
`.opencode/rules/{review,git-workflow,dispatch-loop}.md`; журнал — `migrator`
(Q84 → D87 + строка T-15); протокол: журнал → правки → `opencode reload` →
`auditor` («инструкция ↔ права», agents-perms ×2) → `validator` (адресная,
без cargo) → гейт → `git` (develop, сервисный пакет без ветки) → итог.

## сервисная сессия · 02.10.2026 · открытие

- Срез: `develop` = `origin/develop` = `6c28e51` (T-24); дерево — подхват
  (записи T-24, разбор, findings) + лента этой операции.
- Состав правок (проект, к журналу):
  1. **Права:** `lead` + `git rev-parse --short HEAD`; `git` + `git branch
     --list *`, `git ls-remote --heads origin *`; `tester` + `cargo clippy *`;
     `analyst` — без расширения (заметка: ветку/HEAD проверяет косвенно,
     `git branch`/`rev-parse` — вне прав).
  2. **CCSN:** `git-workflow.md` §«Завершение задачи» — при блоке
     `git.push-delete` origin-ветку удаляет владелец вручную; роль `git` затем
     `branch -d`; шаг планируется заранее (гейт + ожидание), без re-plan-цикла.
  3. **State hygiene:** `dispatch-loop.md` — stale-детекция (`progress` =
     `complete` при противоречащем плане) → сразу `analyst`, без разведки.
  4. **Модель `lead`:** `#default` (F70), норма в `lead.md`.
- Следующее действие — `dispatch migrator` (Q84 → D87 + строка T-15).

## сервисная сессия · 02.10.2026 · правки канона — готово

- **Журнал:** Q84 → D87 (`migrator`): Q84 (права/CCSN/state/модель); D87 —
  4 пункта; каталоги, `TRACEABILITY` (Q84: D87, `in work`, T-15); карточка
  T-15: строка `C14` 🚧 + пометки F26/F27 (данные T-24).
- **Правки (сервисная сессия):**
  - `lead.md` — frontmatter + `git rev-parse --short HEAD`; тело — норма
    `#default` (F70);
  - `git.md` — frontmatter + `git branch --list *`, `git ls-remote --heads
    origin *`;
  - `analyst.md` — заметка: ветку/HEAD проверяет косвенно, `branch`/`rev-parse`
    вне прав;
  - `tester.md` — frontmatter + `cargo clippy *`; шаг 4 и шаблон отчёта —
    clippy;
  - `review.md` — списки команд lead/git/tester/analyst синхронизированы;
  - `git-workflow.md` §«Завершение задачи» — CCSN-пункт (ручное удаление
    origin-ветки владельцем; план заранее, без re-plan-цикла);
  - `dispatch-loop.md` §«Состояние и записи» — stale-детекция (`progress` =
    `complete` → сразу `analyst`).
- **`opencode reload`** выполнен; права подтверждены (`agents-perms --role
  lead/git/tester` — новые команды на месте); MCP-каталог после reload
  «шевельнулся» и восстановился (F65-факт: `credo`/`rust-analyzer` connected).
- Следующее действие — `auditor` («инструкция ↔ права», agents-perms ×2).

## auditor · 02.10.2026 · аудит — расхождений нет (P3)

- P1/P2 нет. «Инструкция ↔ права»: `agents-perms.mjs` ×2 — «11 из 18»;
  runtime подтверждает новые права (`lead` — rev-parse; `git` — `branch
  --list`/`ls-remote`; `tester` — clippy; `analyst` — без расширения);
  `review.md` синхронен фронтматтерам. D87 ↔ факт: 4 пункта внесены; иных
  правок канона нет. Границы чисты; `cargo` не запускался.
- **P3 (не блокер):** карточка T-15, строка `C14` — имя разбора в «Источнике»
  (канон: без ссылок на `docs/analysis/**` из задач; форма — имя, не линк;
  стиль совпадает с существующими строками). Закрыть при следующей правке
  карточки.
- Следующее действие — `validator` (адресная приёмка, без cargo).

## сервисная сессия · 02.10.2026 · пакет r9 — гейт владельца

- Приёмка: `validator` — **accepted** (`service-t24-fixes`; отчёт
  `docs/reviews/service-t24-fixes-2026-10-02.md`; P3 аудита закрыт правкой
  карточки — `migrator`). Находки F69/F70/F71 закрыты; F67/F68 — остаток
  C9/C12.
- **Пакет (снимок `git status`, 23 M + 5 ?? = 28 путей + запись роли `git`
  F43 = 29):**
  - канон: `.opencode/agents/{analyst,git,lead,tester}.md`,
    `.opencode/rules/{dispatch-loop,git-workflow,review}.md`;
  - журнал: `docs/questions/Q84.md`, `docs/decisions/D87-t24-pilot-fixes.md`,
    `docs/{questions,decisions}/README.md`, `docs/TRACEABILITY.md`;
  - карточка T-15 + findings; разбор `docs/analysis/T-24-run-2026-10-02-lead-session.md`,
    досье `docs/analysis/T-24-2026-10-02.md`; отчёт приёмки
    `docs/reviews/service-t24-fixes-2026-10-02.md`;
  - подхват T-24: `.opencode/mail/T-24.md`, `.opencode/state/current/**`,
    `.opencode/memory/{analyst,auditor,migrator,service,validator}.md`;
  - лента r9 `.opencode/mail/service-mcp-ready-r9.md`;
  - + `.opencode/memory/git.md` (запись роли, F43).
- Сообщение: `chore(process): T-15 r9 — правки по S-пилоту T-24 (права/CCSN/state/модель, D87); приёмка`.
- База: `develop` = `origin/develop` = `6c28e51`; ветки нет; master — по
  решению гейта.

## сервисная сессия · 02.10.2026 · гейт пакета — подтверждён

- **question:** owner-gate пакета r9 (29 путей).
- **owner_response (дословно):** «Коммит + push develop (Recommended)».
- Порядок: коммит в `develop` + `push origin develop`; `master` не трогаем.
- Следующее действие — `dispatch git`.

## migrator · 02.10.2026 · готово

- Сделано: заведены [Q84](../../docs/questions/Q84.md) (resolved by D87) и
  [D87](../../docs/decisions/D87-t24-pilot-fixes.md) (`Resolves` Q84; `Спека` —;
  `Affects` — роли `{lead,git,analyst,tester}` + правила
  `{review,git-workflow,dispatch-loop}`; `Tasks` — T-15, C14). Решение — 4
  пункта (права/CCSN/state/модель `#default`). Обновлены каталоги
  `questions/README.md`/`decisions/README.md`, `TRACEABILITY` (Q84: `in work`,
  T-15 🚧), карточка T-15 (строка реестра `C14` 🚧 + данные T-24 в F26/F27).
- Проверки: ID свободны (Q83/D86 — последние; Q84/D87 созданы); `git diff
  --check` пусто; относительные ссылки Q84/D87 живы (`.opencode/agents/*`,
  `.opencode/rules/*`, D47/D50/D70/D78, findings, T-15, `journal.md`,
  TRACEABILITY). Сверка D87 — ⚪ (процесс/документы); `cargo` не запускался
  (D50); адресный прогон — `validator`.
- Дальше / риски: правки канона ролей/правил вносит сервисная сессия (зона
  `.opencode/**`), затем `opencode reload` → `auditor` («инструкция ↔ права»,
  agents-perms ×2) → `validator`. F69/F70/F71 закрываются после приёмки
  правок; пункты C9/C12 (цена шага, re-plan, лимит 16) — отдельным решением.

## auditor · 02.10.2026 · готово

- Сделано: независимый аудит r9 «инструкция ↔ права» + D87 ↔ факт до коммита.
  **P1/P2 нет.** P3 (одна): карточка T-15:225 (C14) ссылается на разбор
  `docs/analysis/**` именем — адресных ссылок и номеров строк нет,
  соответствует существующему стилю карточки; не блокирует.
- Проверки:
  - `node .opencode/scripts/agents-perms.mjs` ×2 — «agents: 11 из 18»,
    идентичные прогоны; runtime-вывод подтвердил применение фронтматтеров:
    `lead` + `git rev-parse --short HEAD`, `git` + `git branch --list *`/
    `git ls-remote --heads origin *`, `tester` + `cargo clippy *`; `analyst`
    без расширения; `steps` на месте.
  - `review.md` §«Доступные команды» сверен с фронтматтерами изменённых ролей —
    расхождений нет (`git diff ./.opencode/rules/review.md` подтвердил синхрон
    списков lead/git/tester/analyst).
  - D87 ↔ факт: 4 пункта решения = внесённые правки (lead.md frontmatter+тело
    `#default`; git.md frontmatter ×2; analyst.md заметка; tester.md frontmatter
    +шаг 4+шаблон отчёта; review.md; git-workflow.md CCSN-пункт; dispatch-loop.md
    stale-детекция). Иных правок в каноне нет.
  - Журнал: Q84↔D87 (`Resolves`/`resolved by`), каталоги `questions/README` и
    `decisions/README` содержат строки; TRACEABILITY Q84 = D87/`in work`/T-15 🚧;
    карточка T-15 — строка `C14` 🚧 + пометки F26/F27 (данные T-24), без
    `.opencode/mail|state`-адресов и номеров строк; ссылки D87/Q84 живы
    (D47/D50/D70/D78/D86, T-15, findings, journal, TRACEABILITY — существуют).
  - Границы: `git status --porcelain` — только `.opencode/{agents,rules,memory,
    mail,state}` + `docs/**` (+ новые Q84/D87/разбор/ленты); `src/**`, `tests/**`,
    `Cargo.toml` не тронуты.
- Дальше / риски: канон не правил. Не проверено: `opencode mcp list` — вне прав
  auditor (`opencode reload` only), F65-факт восстановления MCP-каталога взят с
  ленты r9 (ожидание/факт совпало по записи). `cargo` не запускался (D50).
  Правок не требуется; при согласии владельца — `validator` (адресная, без cargo).

## validator · 02.10.2026 · принято

- Сделано: адресная документная приёмка r9 (D50 + решение владельца
  02.10.2026 «тесты только при правках кода» — `cargo` **не запускался**;
  `src/**`, `tests/**`, `Cargo.toml` не тронуты). Снимок: `develop` =
  `origin/develop`, HEAD `6c28e51` + рабочее дерево (пакет не закоммичен).
- Проверки:
  - **D87 ↔ факт** — 4 пункта внесены: `lead.md:19` (`git rev-parse --short
    HEAD`) + `:43-44` (`#default`); `git.md:22-23` (`branch --list *`,
    `ls-remote --heads origin *`); `analyst.md:49-50` (заметка, без
    расширения); `tester.md:19` (`cargo clippy *`) + шаг 4 `:51-56` + шаблон
    `:79`. CCSN — `git-workflow.md:117-121`; stale — `dispatch-loop.md:110-113`.
  - **Права** — `node .opencode/scripts/agents-perms.mjs` → `agents: 11 из
    18`; новые права видны (reload применён); `review.md:88-105` синхронен
    фронтматтерам lead/git/analyst/tester.
  - **Журнал** — Q84↔D87 парны (`Q84.md:3`, `D87:5`); каталоги
    `questions/README.md:106`, `decisions/README.md:116`; `TRACEABILITY.md:88`
    Q84: D87 / `in work` / T-15 🚧; карточка `T-15:225` C14 🚧 + F26/F27;
    ссылки живы; адресных номеров строк нет.
  - **Статические гейты** `docs_journal.rs` (D50, без прогона) — Q↔D,
    каталоги, TRACEABILITY lifecycle + task parity зелёные по чтению;
    реестр `tasks/README.md:56` 🚧 = `[T-15] 🚧`.
  - **Границы** — `git diff --numstat -- src tests Cargo.toml` пусто;
    `git diff --check` пусто; изменены только разрешённые пути канона/журнала/
    записей.
- **Вердикт: принято, P1/P2 нет**; P3 — `T-15:225` (C14) «Источник» называет
  разбор по имени без ссылки (стиль карточки, не блокер; закрыть при следующей
  правке). Отчёт `docs/reviews/service-t24-fixes-2026-10-02.md`; квитанция
  `service-t24-fixes` (iteration 1, accepted) в `state/current/receipts.yaml`.
- Дальше / риски: `cargo` не запускался (D50); `opencode mcp list` — вне прав;
  F65-факт взят из ленты. Гейт пакета → `git` (develop, сервисный пакет без
  ветки) → `complete`.

## migrator · 02.10.2026 · готово

- Сделано: закрытие служебной операции r9 по приёмке `validator` (accepted,
  `service-t24-fixes`). **Реестр находок** — F69/F70/F71 → «закрыт 02.10.2026»
  с основанием и ссылкой на [D87](../../docs/decisions/D87-t24-pilot-fixes.md):
  F69 — права расширены (`lead` + `git rev-parse --short HEAD`; `git` +
  `branch --list *`/`ls-remote --heads origin *`; `tester` + `cargo clippy *`;
  `analyst` без расширения); F70 — норма `#default` в `lead.md`; F71 — CCSN-
  процедура в `git-workflow.md` §«Завершение задачи». F67/F68 — не закрыты,
  дополнена пометка «остаток — C9/C12 (решения отдельно)». F26/F27/F58 не
  тронуты. **Карточка T-15** — строка `C14` → «✅ 02.10 — принято
  (`service-t24-fixes`); права/CCSN/state/модель (D87)». **P3 аудита** закрыт:
  в «Источнике» C14 убрано имя разбора `T-24-run-2026-10-02-lead-session.md`,
  оставлено «F67–F72 (реестр находок)» (канон: задачи не ссылаются на
  `docs/analysis/**`).
- Проверки: `git diff --check` пусто; ссылки D87/`service-t24-fixes` живы
  (файлы существуют); правки локализованы в двух файлах (реестр + карточка);
  `cargo` не запускался (D50). Замечание: `git diff` по реестру/карточке тянет
  и незакоммиченные правки того же пакета (F26/F27/F58, добавление C14) —
  судить о своих правках по локализации строк.
- Дальше / риски: новых Q/D/задач нет. Остаток — гейт пакета → `git` (develop,
  сервисный пакет); C9/C12 — отдельным решением.

## git · 02.10.2026 · в работе

- Сделано: сверил подтверждение (лента, секция «гейт пакета — подтверждён»,
  `owner_response` дословно «Коммит + push develop (Recommended)»); снимок
  `git status` совпал с пакетом — 23 `M` + 5 `??` = 28; база `develop` =
  `origin/develop` = `HEAD` = `6c28e51`, ветки нет. Чекпойнт памяти `git.md` —
  до `add` (F43).
- Проверки: `git status` → 23 M + 5 ?? (совпадает); `git log -1 --oneline` →
  `6c28e51`.
- Дальше / риски: `add` точными путями (29) → staged-сверка (ожидание 24 M +
  5 A) → коммит `chore(process): T-15 r9 — правки по S-пилоту T-24
  (права/CCSN/state/модель, D87); приёмка` → `push origin develop`. Ветки не
  создаются, `master` не трогается; после `push` в отслеживаемые файлы не
  пишу — хеш верну `lead` ответом.
