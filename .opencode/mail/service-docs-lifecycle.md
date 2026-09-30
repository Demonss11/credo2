# Сервисная лента: service-docs-lifecycle (папки артефактов `research`/`reviews`/`analysis`)

**Назначение:** сервисная волна разбора трёх папок рабочих артефактов —
`docs/research/`, `docs/reviews/`, `docs/analysis/` (запрос владельца,
30.09.2026). Операция №1 — `research`.

**Установка владельца (30.09.2026):**
- архив — история git; неактуальные файлы удаляем (при нужде достанем из git);
- канон на эти папки не ссылается: факты — в Q/D, задачах, фичах; ссылки
  канона на артефакты снять (затронет D65, T-18/T-19, решения/задачи,
  receipts — найти).

**Маршрут волны:** №1 `research` (эта операция) → №2 `reviews` → №3
`analysis`; очередь и карта найденных ссылок — ниже.

---

## сервисная сессия · 30.09.2026 · операция №1 (`research`) — открытие

- База: `develop` = `origin/develop` = `9948ebf`; рабочее дерево — 1 `M`
  (`.opencode/state/current/progress.yaml`, запись волны `service-cleanup` —
  войдёт в пакет).
- Инвентаризация `docs/research/` (текущий коммит): единственный файл
  `doc-quality-checks-2026-09-29.md` (обзор doc-проверок соседнего проекта) —
  **отработано**: рекомендации приняты (Q62–Q64 → D66–D68), задача T-19 ⬜;
  файл удаляется (архив — git).
- Ссылки канона на файл — **18 в 7 файлах** (Q62×2, Q63×2, Q64×1, D66×4,
  D67×4, D68×3, T-19×2) — снимаются. Вне канона: receipts (историч.,
  append-only — не правим), отчёты `docs/reviews/*` (разберёт №2), память
  `researcher` (правит сервисная сессия).
- Политика: «Обновление 30.09.2026» в [D65](../../docs/decisions/D65-reference-policy.md)
  — `research`/`reviews`/`analysis` рабочие артефакты: неактуальные файлы
  удаляются (архив — git); канон на их файлы не ссылается — факты в Q/D,
  задачах, фичах; ссылки снимаются волнами; статус живого реестра
  `findings-registry.md` и синхронизация D64/T-18 — волна №3. Новых Q/D нет
  («Обновление» D65).
- Маршрут операции: `migrator` (D65 + снятие ссылок) → `docs-writer`
  (`docs/README.md`) → сервисная сессия (память `researcher`, удаление файла,
  запись `progress.yaml`) → `auditor` → `validator` → пакет `git` по гейту.
- Границы: `docs/**` (журнал, карточки, README), `.opencode/memory/researcher.md`,
  `.opencode/state/current/{progress,receipts}.yaml`, лента; `src/**`,
  `tests/**`, `Cargo.toml`, `.opencode/agents/**`, `.opencode/rules/**` не
  трогаются; `cargo` не запускается (D50).

## сервисная сессия · 30.09.2026 · очередь волны (`reviews`, `analysis`)

Карта ссылок канона на папки (по текущему коммиту; детали — `git grep`):

- **№2 `reviews`** (71 файл): инвентаризация «в работе / отработано»,
  удаление отработанных; ссылки канона к снятию: D49 (`T-11-closeout`),
  Q54 (там же), карточки T-01, T-11, T-12, T-13, T-16; receipts —
  `report:`-поля (~87) — append-only, не правим; отчёты ссылаются на
  удаляемый research-файл (разобрать здесь же). Зоны/исключения (T-18, T-19,
  `review.md`, роль-таблицы) — не ссылки, остаются.
- **№3 `analysis`** (24 файла + живой `findings-registry.md`): инвентаризация
  и удаление отработанных, статус реестра; ссылки канона к снятию: решения
  D16, D39–D49, D61, D63–D68, D76, D78, D81; вопросы Q45–Q54, Q59, Q61, Q72,
  Q74, Q77; карточки T-11, T-12, T-13, T-15, T-16, T-18; фичи `README` +
  пять `agents-*.feature` («Основание: mcp-ready-process.md»); `TRACEABILITY`
  :57; правила `.opencode/rules/dispatch-loop.md`, `review.md`; память
  `analyst`, `tester`; `docs/analysis/README.md`; синхронизация D64 п.8 /
  T-18 (допустимость ссылок на analysis, whitelist реестра).

## migrator · 30.09.2026 · операция №1 — готово

- Сделано: (1) [D65](../../docs/decisions/D65-reference-policy.md) — в конец
  «Следствий» добавлено «Обновление 30.09.2026» (рабочие артефакты
  `research`/`reviews`/`analysis`, снятие ссылок волнами, особый случай
  [`findings-registry.md`](../../docs/analysis/findings-registry.md)); (2) сняты
  18 ссылок на удаляемый `docs/research/doc-quality-checks-2026-09-29.md` в
  [Q62](../../docs/questions/Q62.md) (×2), [Q63](../../docs/questions/Q63.md)
  (×2), [Q64](../../docs/questions/Q64.md) (×1),
  [D66](../../docs/decisions/D66-doc-quality-checks.md) (×4),
  [D67](../../docs/decisions/D67-cspell-deferred.md) (×4),
  [D68](../../docs/decisions/D68-changelog-handwritten.md) (×3),
  [T-19](../../docs/tasks/T-19-doc-quality-checks/README.md) (×2).
- Проверки: `git grep -n "doc-quality-checks-2026-09-29" -- docs/questions docs/decisions docs/tasks`
  → пусто (exit 1); `git grep -n "research/" -- docs/questions docs/decisions docs/tasks`
  → 2 строки: T-19:35 (ожидаемая scope-строка) и D65:89 (упоминание
  `docs/research/**` в самом «Обновлении»); `git status --porcelain` → 8 файлов
  правок + `M .opencode/state/current/progress.yaml` + `?? .opencode/mail/service-docs-lifecycle.md`.
- Дальше / риски: ожидания команд в брифе разошлись с фактом (ссылок канона на
  файл нет): `git grep "doc-quality-checks" -- docs` непусто только на slug'ах
  `D66-doc-quality-checks.md`/`T-19-doc-quality-checks`; в D68 контекст имел
  другую разбивку строк — правка применена по смыслу с сохранением разметки;
  T-19:36→35 (сдвиг из-за удаления строки «опора — обзор»). Ссылки вне канона —
  в `docs/reviews/doc-tools-d66d68-2026-09-29.md` (волна №2) и памяти
  `researcher` (сервисная сессия).

## docs-writer · 30.09.2026 · операция №1 — готово

- Сделано: в [docs/README.md](../../docs/README.md) строки 25–27 —
  «первичные артефакты…на них ссылаются, их не дублируют» → «рабочие
  артефакты…канон на их файлы не ссылается — факты живут в Q/D, задачах,
  фичах; неактуальные файлы удаляются (архив — git-история;
  [D65](../../docs/decisions/D65-reference-policy.md))»; остальной файл (карта,
  стр. 22) не тронут.
- Проверки: `git diff -- docs/README.md` → изменён только блок 25–26 (2 → 4
  строки, разбивка из брифа); `git grep -n "на них ссылаются" -- docs/README.md`
  → пусто (exit 1); путь `docs/decisions/D65-reference-policy.md` существует
  (glob); `cargo` не запускался (D50).
- Дальше / риски: номер строки в брифе («2 → 3») разошёлся с разбивкой текста
  (получилось 4 строки) — правка применена по смыслу с сохранением окончаний
  строк; содержимое D65 — за `migrator`.

---

## сервисная сессия · 30.09.2026 · операция №1 — служебная зона — готово

- Сделано: `.opencode/memory/researcher.md` — строка «**Канон:** `docs/research/`»
  заменена на «**Файлы:** `docs/research/**` — рабочие обзоры: не канон, ссылки
  канона на них не ставятся; неактуальные файлы удаляются (архив — git)»;
  удалён `docs/research/doc-quality-checks-2026-09-29.md` (архив — git).
- Проверки: `git grep -n "research/" -- docs .opencode/memory` → markdown-ссылок
  на удалённый файл нет; остаются упоминания зон/скоупа (`docs/README.md:22,25`,
  `D65:89`, `T-19:35`; `docs/analysis/README.md:39` — волна №3), чекпойнт
  `memory/migrator.md` и отчёты `docs/reviews/**` (волна №2); `git status` —
  удаление отражено (` D`).
- Дальше / риски: `auditor` → `validator`; запись волны в `progress.yaml` — до
  пакета `git` (по гейту).

## auditor · 30.09.2026 · операция №1 — аудит

- Сделано/проверено (независимо, по дереву, не по записям ленты):
  - **Ссылки:** `git grep -n "doc-quality-checks-2026-09-29" -- docs` → 2 строки,
    обе в `docs/reviews/doc-tools-d66d68-2026-09-29.md:17,69` (зона волны №2) —
    ожидаемо; `git grep -n -e "\.\./research/" -- docs/questions docs/decisions
    docs/tasks docs/features docs/README.md` → пусто (exit 1); `git grep -n
    "research/"` по тем же путям → 4 строки: `docs/README.md:22,25` (зона
    `research/`, не адрес файла), `D65:89` (текст «Обновления»), `T-19:35`
    (scope-строка) — допустимы.
  - **D65:** «Обновление 30.09.2026» на месте (`D65:88–99`), уточняет п.4 явно
    (рабочие артефакты не канон; архив — git; ссылки снимаются волнами; особый
    случай `findings-registry` → волна `analysis`); противоречий с остальным
    телом нет: п.4 (`D65:46–55`) отдельно трактует `.opencode/state/**`/`mail/**`
    как удаляемые данные и держит `findings-registry` исключением — согласовано.
  - **Память (F35):** `.opencode/memory/researcher.md:3–5` — «Файлы:
    `docs/research/**`… не канон»; канон роли `.opencode/agents/researcher.md`
    (`edit docs/research/**`, тело «Пишешь только `docs/research/**`») — без
    расхождений. Чекпойнты `migrator`/`docs-writer` — их собственная зона, не
    шаблонные правки; чужих записей не переписывали.
  - **Дерево/границы:** `git status --porcelain` → ровно ожидаемое (12 `M` +
    ` D docs/research/doc-quality-checks-2026-09-29.md` + `?? .opencode/mail/
    service-docs-lifecycle.md`); `git diff --stat -- src tests Cargo.toml` пусто;
    `git status --porcelain .opencode/agents` / `.opencode/rules` / `AGENTS.md` /
    `docs/features` — пусто (не тронуты).
  - **Права:** `node .opencode/scripts/agents-perms.mjs --all` → 18 из 18,
    расхождений нет; прогон дважды (11/18, затем 18/18). `opencode reload` **не
    проводился** — агенты/правила не менялись, перезагрузка не требуется.
  - **Запреты:** в правках номеров строк не появилось (номера только в контексте
    диффа); миграционных маркеров нет; чужие записи ленты/памяти не тронуты
    (только append).
  - **Лента:** порядок записей (открытие → migrator → docs-writer → служебная
    зона) сходится с фактами дерева; расчёт «18 ссылок в 7 файлах» совпал с
    диффом.
- Находки: P1 — нет; P2 — нет; P3 — нет.
- Замечания (не находки): удаление файла ещё в рабочем дереве — ожидаемо, архив
  остаётся в истории git.
- Дальше / риски: `validator` → пакет `git` по гейту. Канон не правил; ничего,
  кроме этой записи и `memory/auditor.md`, не менял. `cargo` не запускался (D50).

## validator · 30.09.2026 · операция №1 — принято

- Сделано: приёмка волны `research` по `.opencode/rules/review.md`; отчёт
  [service-docs-lifecycle-2026-09-30.md](../../docs/reviews/service-docs-lifecycle-2026-09-30.md);
  квитанция в `receipts.yaml` (task `service-docs-lifecycle`, iteration 1,
  accepted).
- Проверки (независимо, по дереву): `git status --porcelain` = 12 `M` + ` D
  docs/research/doc-quality-checks-2026-09-29.md` + `?? .../service-docs-lifecycle.md`
  (сходится с брифом); `git diff --stat -- src tests Cargo.toml` пусто;
  `git status --porcelain .opencode/agents .opencode/rules AGENTS.md docs/features`
  пусто; `git grep -n "doc-quality-checks-2026-09-29" -- docs` → только
  `docs/reviews/doc-tools-d66d68-2026-09-29.md:17,69` (волна №2); `git grep -n
  "\.\./research/"` по канону → пусто; 18 снятых ссылок без «висячих» хвостов;
  D65:88–99 уточняет п.4, ссылки живые; каталоги/`TRACEABILITY`/ID/статусы не
  затронуты; `node .opencode/scripts/agents-perms.mjs` ×2 → `agents: 11 из 18`,
  расхождений нет; архив файла есть в git (`develop:` 160 строк). `cargo`
  **не запускался** (D50).
- Дальше / риски: P1/P2/P3 нет — критичных проблем нет; пакет `git` по гейту.
  Техническое: `git rev-parse develop origin/develop HEAD` отклонён движком прав
  → заменён на `git log --oneline -1` (база `9948ebf` подтверждена).

---

## сервисная сессия · 30.09.2026 · операция №1 — подтверждение пакета

- Гейт пройден: владелец подтвердил **коммит + push** (`question`, ответ
  «Коммит + push (Recommended)») — волна `service-docs-lifecycle`, операция №1
  (`research`); один коммит «канон + записи» (D75).
- База: `develop` @ `9948ebf` (= `origin/develop`); режим — коммит прямо в
  `develop`, затем `git push origin develop`.
- Пакет (19 путей операции + запись роли `git` по F43 = 20):
  - `.opencode/mail/service-docs-lifecycle.md`
  - `.opencode/memory/auditor.md`
  - `.opencode/memory/docs-writer.md`
  - `.opencode/memory/git.md` (чекпойнт роли `git`, новый до `add`)
  - `.opencode/memory/migrator.md`
  - `.opencode/memory/researcher.md`
  - `.opencode/memory/validator.md`
  - `.opencode/state/current/progress.yaml`
  - `.opencode/state/current/receipts.yaml`
  - `docs/README.md`
  - `docs/decisions/D65-reference-policy.md`
  - `docs/decisions/D66-doc-quality-checks.md`
  - `docs/decisions/D67-cspell-deferred.md`
  - `docs/decisions/D68-changelog-handwritten.md`
  - `docs/questions/Q62.md`
  - `docs/questions/Q63.md`
  - `docs/questions/Q64.md`
  - `docs/research/doc-quality-checks-2026-09-29.md` (удаление)
  - `docs/reviews/service-docs-lifecycle-2026-09-30.md`
  - `docs/tasks/T-19-doc-quality-checks/README.md`
- Сообщение коммита:
  `docs(D65): рабочие артефакты research/reviews/analysis — удаление по отработке, ссылки канона сняты (service-docs-lifecycle №1)`.
- Замечание: запись роли `git` (лента + `memory/git.md`) формируется **до**
  `add` и входит в коммит (F43); после `push` в отслеживаемые файлы не пишем
  (хеши — ответом).

## git · 30.09.2026 · операция №1 — пакет к коммиту (F43)

- Подтверждение сверено по записи «сервисная сессия · 30.09.2026 · операция №1
  — подтверждение пакета» (эта лента): 20 путей, режим «коммит + push» прямо в
  `develop`, сообщение коммита — оттуда.
- Снимок дерева до `add` сходится с ожиданием: 16 `M` + ` D
  docs/research/doc-quality-checks-2026-09-29.md` + 2 `??`
  (лента, отчёт); `HEAD` = `9948ebf` = база = `origin/develop`.
- Сделано (F43, до `add`): чекпойнт в `memory/git.md`; настоящая запись.
- Дальше: `add` 20 точными путями (`./`-префикс, без `--`) → сверка staged
  (`diff --cached --name-status`, ожидание 17 `M` + 1 `D` + 2 `A`) → `commit`
  → `git push origin develop`. После `push` git-команд нет (D75); хеши —
  ответом.
