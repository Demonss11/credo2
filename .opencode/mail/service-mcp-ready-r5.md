# service-mcp-ready-r5 — лента операции: T-15 B0-own BO-i2 (`wave0-guard`)

Открыта: 02.10.2026. **Сервисная операция** (продолжение мини-волны B0-own;
решение владельца: следующим шагом — BO-i2). Полигон:
`%TEMP%\opencode\wave0b-own` (temp, вне репозитория); канон и `.opencode/**`
репозитория не правятся.

**Предмет BO-i2** ([`wave0b-own.md`](../../docs/tasks/T-15-mcp-ready-process/wave0b-own.md)
§«Предложение BO-i2»): выбрать механизм страховки P2 `wave0-guard` —
(A) V2-плагин `permission.hook("evaluate")` + аудит `tool.execute.before`;
(B) `experimental.policies` (статический hard-deny в конфиге);
(C) комбинация; вердикт по протоколу B0, кандидат переноса — отдельным решением.

**Сценарии проб:** `git push --force`, `git reset --hard`, чтение `.env`,
`Remove-Item -Recurse -Force` (standard/paranoid), запись вне проекта;
allow-поток без изменений; поведение с `--auto`; аудит JSONL; ложные
срабатывания. Критерии: блокировка **до** запуска, allow-поток не меняется,
аудит пишется, `--auto` не обходит deny, ложных срабатываний нет.

**Улики:** `target/wave0b-own-i2/` (вне git) + JSONL плагина в полигоне;
`cargo` не запускается (D50). Модель проб — `opencode-go/deepseek-v4.1-flash`
(как у ролей). Аномалия «первый headless-прогон висит в CLI» — прогрев
(таймаут 240–300 с, повтор).

## сервисная сессия · 02.10.2026 · открытие

- Полигон жив: `.opencode/` с `node_modules`, плагины пусты
  (`_off/wave0b-own-probe.ts` — BO-i1), конфиг `opencode.json` (только `$schema`).
- План: A — плагин `wave0b-own-guard.ts` (режимы standard/paranoid через
  `wave0b-own-guard-mode.txt`); B — `experimental.policies` в конфиге полигона;
  C — совмещение (config-deny финален, хук не вызывается).
- Записи прогонов — ниже (append по мере исполнения).

## сервисная сессия · 02.10.2026 · BO-i2 исполнен — вердикт 🟢

- **Среда:** OpenCode **2.0.22** (в BO-i1 — 2.0.18), модель
  `opencode-go/deepseek-v4.1-flash`, прогоны `opencode run --auto`; полигон
  `%TEMP%\opencode\wave0b-own`; плагин загрузился (маркер, `loading plugin`).
- **Прогоны (13):** A1 force-push → BLOCKED (плагин); A2 reset --hard →
  BLOCKED; A3 read .env → BLOCKED; A4 Remove-Item standard → EXECUTED
  (`ask` + `--auto` = авто-одобрение); A5/A6 allow-поток — без изменений,
  ложных нет; A7 paranoid Remove-Item → BLOCKED; A8 запись вне проекта →
  BLOCKED; B1/B3 policies → «Blocked by configuration policy» (до запуска);
  B4 Remove-Item под policies → EXECUTED (шаблон `*--force*` не ловит
  `-Force`); C1 плагин+policies → policy финальна, хук вызывается; D1
  `permissions` deny → «Permission denied: shell», хук `evaluate` не
  вызывается (аудит `tool.before` пишется).
- **Вердикт:** 🟢 комбинация: `experimental.policies` (статический hard-контур
  выше репозитория) + `permissions` (ролевые правила репо) + плагин
  (контекстные deny + аудит JSONL). Одиночный плагин 🟢 (но `ask` не защищает
  при `--auto`); одиночные policies 🟢 с пробелами шаблонов.
- **Улики:** `target/wave0b-own-i2/` — плагин, конфиги, `wave0b-own-guard.jsonl`
  (42 записи: 21 `tool.before` + 21 `evaluate`, 6 deny — все целевые),
  маркер, режим. Журнал мини-волны обновлён (BO-i2, §факты/вердикт).
- **Наблюдения:** `opencode reload` в полигоне дважды сбросил MCP-каталог
  общей сессии (credo, восстановился) — сервис общий; модель сама отказалась
  от force-push в первой B1-пробе (нужна явная формулировка); scanner дробит
  составные команды.
- **Дальше:** BO-i3 — P1 `wave0-observe` (по подтверждению владельца);
  вердикт P2 — вход C10. Перенос механизмов — после аудита (не сейчас).

## сервисная сессия · 02.10.2026 · гейт пакета BO-i2 пройден

- Владелец подтвердил (**question**): «Коммит BO-i2 + BO-i3» — коммит записей
  BO-i2 (+F43-остаток) и push `develop`; затем BO-i3.
- Пакет (7 путей + запись роли `git`, F43): `M` — `.opencode/mail/service-t15-run-review.md`
  (F43-остаток), `.opencode/memory/migrator.md`, `.opencode/memory/service.md`,
  `.opencode/state/current/progress.yaml` (F43-остаток),
  `docs/tasks/T-15-mcp-ready-process/README.md`,
  `docs/tasks/T-15-mcp-ready-process/wave0b-own.md`; `??` —
  `.opencode/mail/service-mcp-ready-r5.md`; + `.opencode/memory/git.md` (F43).
- Сообщение коммита: `chore(process): T-15 B0-own BO-i2 — wave0-guard: пробы,
  вердикт, карточка P2`.
- База: `develop` = `origin/develop` = `fed94d6`; ветки нет (прямая правка);
  master не трогаем (merge — очередной волной).
- Следующее действие — `dispatch git`.

## git · 02.10.2026 · готово (пакет BO-i2)

- Коммит **`85658fd`** (8 путей: 7 `M` + 1 `A`; +268/−5) → push
  `fed94d6..85658fd` в `origin/develop`; ветки не создавались; master не
  трогался; дерево чистое.

## сервисная сессия · 02.10.2026 · BO-i3 открытие (P1 `wave0-observe`)

- **Цель:** наблюдаемость субагентов — события сессий (родитель/ребёнок) →
  JSONL + сводка по сессиям (вход C/D).
- **Механика:** плагин `wave0b-own-observe.ts` — `ctx.event.subscribe()` с
  фильтром по location/известным `sessionID` (BO-i1: поток серверный,
  `usage/execution` приходят с `location=null`); агрегат
  `wave0b-own-observe-summary.json`; сводка родителю (опция) — попытка
  `ctx.session.prompt` с логированием результата; проверка нативного
  `session.list` (фильтр `parentID`).
- **Пробы:** O1 — загрузка/маркер; O2 — субагент `general` + маркеры
  родителя; O3 — режим `journal+summary` (сводка родителю).
- **Улики:** `target/wave0b-own-i3/`; конфиг полигона не меняем (reload не
  нужен — правки плагинов подхватывает watcher).

## сервисная сессия · 02.10.2026 · BO-i3 исполнен — вердикт 🟢

- **Пробы:** O1 — полный жизненный цикл сессии в журнале (29 строк);
  O2 — субагент `general`: дочерний `session.created` с `parentID`+`agent`,
  агрегат родитель/ребёнок, tools `{subagent,shell}`; O3 — `journal+summary`:
  `summary.prompt ok:true`, доставка сводки подтверждена экспортом родителя.
- **Факты:** `ctx.session.list` у плагина **нет** (список — CLI/HTTP или
  события); `projectID` temp — `global`; registry-события — с location
  плагина; новые типы (`session.tool.*`, `shell.created/exited`).
- **Вердикт:** 🟢 (журнал + агрегат + сводка родителю). Доработки: `sh_*`
  не считать сессиями; корневой agent — из step/экспорта; дельты семплировать.
- **Улики:** `target/wave0b-own-i3/` (плагин, журнал 71 КБ, агрегат, маркер,
  экспорт родителя O3). Журнал — `wave0b-own.md` §BO-i3.
- **Дальше:** BO-i4 — P4 `metrics-report.mjs` (по подтверждению владельца);
  P5 — решить на BO-i7 (частично перекрыт).

## сервисная сессия · 02.10.2026 · гейт пакета BO-i3 пройден

- Владелец подтвердил (**question**): «Коммит BO-i3 + BO-i4» — коммит записей
  BO-i3 и push `develop`; затем BO-i4.
- Пакет (5 путей + запись роли `git`, F43): `.opencode/mail/service-mcp-ready-r5.md`,
  `.opencode/memory/{migrator,service}.md`, `docs/tasks/T-15-mcp-ready-process/README.md`,
  `docs/tasks/T-15-mcp-ready-process/wave0b-own.md`; + `.opencode/memory/git.md`.
- Сообщение коммита: `chore(process): T-15 B0-own BO-i3 — wave0-observe: пробы,
  вердикт, карточка P1`.
- База: `develop` = `origin/develop` = `85658fd`; master не трогаем.
- Следующее действие — `dispatch git`.

## git · 02.10.2026 · готово (пакет BO-i3)

- Коммит **`9a811b4`** (6 файлов: 6 `M`; +156/−3) → push `85658fd..9a811b4`
  в `origin/develop`. Замечание: прогноз «5 M + 1 A» был неверен —
  `memory/git.md` отслеживается (M); состав совпал.

## сервисная сессия · 02.10.2026 · BO-i4 открытие (P4 `metrics-report.mjs`)

- **Цель:** прототип отчёта метрик из нативных источников (`opencode stats
  --json`, `session list`, `session export`) — сводка проекта + свёртка
  цепочки (root → субагенты) по ролям/моделям (замена Telemetry для D,
  вход C6/D).
- **Механика:** `metrics-report.mjs` (CLI-only, без БД): stats → totals;
  list → сессии; `--chain <ses>` — экспорт root, поиск дочерних по маркеру
  `<subagent sessionID=…>`, экспорт детей, агрегат (agent/model/cost/tokens/
  wall/outcome); вывод markdown + JSON.
- **Пробы:** T1 — цепочка O2 в полигоне; T2 — цепочка прогона 02.10 в
  репозитории (31 ребёнок) со сверкой с числами B1-разбора.
- **Улики:** `target/wave0b-own-i4/`.

## сервисная сессия · 02.10.2026 · BO-i4 исполнен — вердикт 🟢

- **T1** (полигон): цепочка O2 — 2 сессии, $0.0008; **T2** (репозиторий):
  прогон 02.10 — 32 сессии, **$1.4133**, вх. 4 666 766, вых. 531 193,
  214.1 мин; роли/модели — **1:1 с B1-разбором** (build $0.4604; analyst
  13/$0.3320; tester 2/$0.2218; migrator 3/$0.1495; validator 4/$0.1404;
  git 5/$0.0646; docs-writer 2/$0.0200; coder 1/$0.0126; auditor 1/$0.0119;
  #default 31/$0.9529, #max 1/$0.4604).
- **Факты/грабли:** `opencode` — npm-шим (нужен `shell: true`); `stats.tokens`
  — объект; `stats` без `--days` — всё (132 сессии/$32.28); дети — по маркеру
  в экспорте root; экспорт 31 ребёнка ≈ 30 с.
- **Вердикт:** 🟢 (CLI-only отчёт цепочек воспроизводит метрики без БД; вход
  C6/D). Улики: `target/wave0b-own-i4/` (скрипт, T1/T2-отчёты). Журнал —
  `wave0b-own.md` §BO-i4.
- **Дальше:** BO-i5 — P3 `wave0-checkpoint` (по подтверждению владельца);
  P5 — решение на BO-i7.

## сервисная сессия · 02.10.2026 · гейт пакета BO-i4 пройден

- Владелец подтвердил (**question**): «Коммит BO-i4 + BO-i5» — коммит записей
  BO-i4 и push `develop`; затем BO-i5.
- Пакет (5 путей + запись роли `git`, F43): `.opencode/mail/service-mcp-ready-r5.md`,
  `.opencode/memory/{migrator,service}.md`, `docs/tasks/T-15-mcp-ready-process/README.md`,
  `docs/tasks/T-15-mcp-ready-process/wave0b-own.md`; + `.opencode/memory/git.md`.
- Сообщение коммита: `chore(process): T-15 B0-own BO-i4 — metrics-report: пробы,
  вердикт, карточка P4`.
- База: `develop` = `origin/develop` = `9a811b4`; master не трогаем.
- Следующее действие — `dispatch git`.

## git · 02.10.2026 · готово (пакет BO-i4)

- Коммит **`11110bd`** (6 файлов: 6 `M`; +147/−3) → push `9a811b4..11110bd`
  в `origin/develop`.

## сервисная сессия · 02.10.2026 · BO-i5 открытие (P3 `wave0-checkpoint`)

- **Цель:** структурная сводка останова/обрыва (Handoff): цель · статус ·
  файлы · риски/открытые вопросы · resume-путь (вход C9).
- **Механика:** скрипт `session-checkpoint.mjs` (`session export` → сводка md);
  проба нативного resume (`--session`/`--continue`/`--fork`); попытка
  регистрации команды плагином (`ctx.command.transform`, формы — defensive).
- **Пробы:** T1 — полигон (цепочка O2); T2 — `tester` T-18 (стоп по лимиту);
  T3 — resume `--session`; C1 — команда.
- **Улики:** `target/wave0b-own-i5/`.

## сервисная сессия · 02.10.2026 · BO-i5 исполнен — вердикт 🟢

- **C1 (команда):** `ctx.command.transform` → `editor.add({name, description,
  template})` — ok; `command.list()` возвращает `["init","review","checkpoint",
  "checkpoint3"]`; форма `{info, template}` ломает `command.list`
  («Schema validation failed») — не использовать; запуск команды из headless
  не проверялся.
- **T1 (полигон):** сводка O2 — цель/статус/файлы (snapshot)/инструменты/
  resume — корректно.
- **T2 (реальный стоп, `tester` T-18):** сводка поймала цель (бриф), статус
  (rework -r2), 44 файла, 18 строк-маркеров, инструменты (read×60, shell×32,
  edit×23…), resume-команду — пригодно для C9.
- **T3 (resume):** `opencode run --session <id>` продолжил ту же сессию
  (2 промпта, succeeded) — нативный путь восстановления подтверждён.
- **Вердикт:** 🟢 (сводка + resume; регистрация команды — форма подтверждена,
  запуск — за кадром headless).
- **Доработки для переноса:** разделять файлы «изменённые/прочитанные»;
  уточнить маркеры (ловят «открытых задач»); статус обрезается; первый
  промпт субагента содержит преамбулу.
- **Улики:** `target/wave0b-own-i5/` (скрипт, T1/T2-сводки, журнал C1,
  плагин, экспорт resume). Журнал — `wave0b-own.md` §BO-i5.
- **Дальше:** BO-i6 — P5 `wave0-attribution` (решить: подтвердить/свернуть —
  агент/модель/иерархия уже в P1/P4) либо подготовка BO-i7 (отчёт+ревью);
  решение владельца.

## сервисная сессия · 02.10.2026 · гейт пакета BO-i5 пройден (коммит; пауза)

- Владелец подтвердил (**question**): «Коммит BO-i5, пауза» — коммит записей
  BO-i5 и push `develop`; BO-i6/BO-i7 — позже.
- Пакет (5 путей + запись роли `git`, F43): `.opencode/mail/service-mcp-ready-r5.md`,
  `.opencode/memory/{migrator,service}.md`, `docs/tasks/T-15-mcp-ready-process/README.md`,
  `docs/tasks/T-15-mcp-ready-process/wave0b-own.md`; + `.opencode/memory/git.md`.
- Сообщение коммита: `chore(process): T-15 B0-own BO-i5 — wave0-checkpoint:
  пробы, вердикт, карточка P3`.
- База: `develop` = `origin/develop` = `11110bd`; master не трогаем.
- Следующее действие — `dispatch git`; после — пауза (BO-i6/BO-i7 позже).

## сервисная сессия · 02.10.2026 · пауза: комментарий к рекомендациям ревью BO-i1

- По запросу владельца — сверка рекомендаций ревью BO-i1 с фактами BO-i2…BO-i5:
  - «прогрев первым прогоном» — в BO-i2/BO-i3/BO-i5 первые прогоны прошли
    штатно (2.0.22; локация уже была поднята), прогрев использован как
    startup-проба (маркер + `loading plugin`); kill — только fallback при
    зависании (таймаут 240–300 с, сессия после kill проверяется экспортом);
  - «фильтр журнала по project/session» — нужен event-stream-плагинам
    (`ctx.event.subscribe`), не hook-плагинам; `projectID` temp-полигона =
    `global` (фильтр по project ненадёжен) — рабочий фильтр: `location` +
    множество известных `sessionID` (реализовано в `wave0-observe`: 188 строк,
    чужих нет);
  - «выключение probe-плагина по завершении пробы» — реализовано переносом в
    `_off/`: guard — последняя запись 11:59:08, observe — 12:17:52 (после
    переноса журналы не растут); при паузе выключен и `wave0-checkpoint`
    (перенесён в `_off/`); service restart/reload не используем (сервис общий;
    reload в полигоне уже давал churn MCP-каталога общей сессии).
- Всё — вход BO-i7 (отчёт мини-волны): рекомендации считаются отработанными
  с уточнениями (фильтр по location/sessionID; bounded-журнал для переноса).

## сервисная сессия · 02.10.2026 · BO-i6 открытие (P5 `wave0-attribution`)

- **Цель:** проверить онлайновую атрибуцию role/model (`session.hook("context")`)
  на 2.0.22 для родителя и субагента; решить — подтвердить P5 или свернуть
  (атрибуция уже покрыта P1/P4).
- **Механика:** плагин `wave0b-own-attribution.ts` — context-хук пишет
  `agent`/`model`/`tools` по каждому агентскому запросу; прогон с субагентом
  `general`; сверка с экспортом/`session list` (известный дефект: у root в
  хранилище `agent=build`, у export root — `agent` отсутствует).
- **Улики:** `target/wave0b-own-i6/`.

## сервисная сессия · 02.10.2026 · BO-i6 исполнен — вердикт 🟢 (свернуть P5)

- **Проба:** context-хук — родитель `agent=build`, субагент `agent=general`
  (`#default`, tools 12/10) — онлайн-атрибуция работает; export: ребёнок
  `info.agent=general`, root — без `agent`; `session list` без поля `agent`;
  хранилище top-level — `build` (F60).
- **Вердикт:** P5 **свернуть** — покрыто P1/P4 (онлайн-хук при нужде встроить
  в observe; офлайн — export/события/БД с оговоркой F60).
- **Улики:** `target/wave0b-own-i6/` (плагин, журнал, маркер, экспорты).
  Журнал — `wave0b-own.md` §BO-i6.
- **Дальше:** BO-i7 — отчёт мини-волны, ревью; перенос — после аудита
  и решения владельца.

## сервисная сессия · 02.10.2026 · BO-i7 — отчёт и финализация (готово)

- **Отчёт:** [`wave0b-own-report.md`](../../docs/tasks/T-15-mcp-ready-process/wave0b-own-report.md) —
  итоги BO-i1…BO-i6 (P1–P5), факты, рекомендации по переносу (§3), наблюдения
  (кандидаты F, §4), чек-лист готовности (§5), вопросы владельцу (§6).
- **Ревью сервисной сессии:** журнал ↔ улики ↔ лента ↔ коммиты
  (`85658fd`, `9a811b4`, `11110bd`, `e740603`) — расхождений нет.
- **Дальше:** аудит `auditor` (перед переносом) → гейт владельца (состав
  переноса + коммит записей BO-i6/BO-i7) → перенос/откат полигона — по решению.

## auditor · 02.10.2026 · готово (аудит B0-own перед переносом) — запись по поручению роли

> Роль `auditor` остановлена лимитом шагов; отчёт и чекпойнт оформила
> сервисная сессия по её поручению (вердикт и проверки — из ответа роли).

- **Вердикт:** **P1/P2 — расхождений нет**; **P3** — хрупкость P4 (поиск
  детей по маркеру `<subagent sessionID=…>`) не вынесена в отчёт §3 →
  **закрыт правкой отчёта** (строка P4 в §3).
- **Проверки (числа):** P1 observe — 183 непустых строки, `session.created` с
  `parentID`+`agent=general`, чужих нет, `session.list` отсутствует;
  P2 guard — 42 строки/6 deny (все целевые); P3 checkpoint — 1:1 с журналом,
  resume — 2 промпта/succeeded; P4 metrics — 32 сессии/$1.4133/вх. 4 666 766/
  вых. 531 193/214.1 мин — 1:1; P5 attribution — родитель build/субагент
  general, root без `info.agent`, child с `parentID`; дифф — 5 заявленных
  путей, `git diff --check` пусто; `src/tests/Cargo.toml/AGENTS.md/opencode.json`
  не тронуты; 4 коммита (`85658fd`, `9a811b4`, `11110bd`, `e740603`);
  `agents-perms.mjs` ×2 → `11 из 18`; канон/фронтматтеры не менялись.
- **Риски переноса (из аудита):** bounded-журналы; идемпотентность
  трансформаций; F60 (атрибуция top-level); DEP0190/`shell:true`; хрупкость
  маркера (учтена в §3 отчёта).
- **Дальше:** гейт владельца — коммит записей BO-i6/BO-i7 + аудита и решение
  о переносе (§6 отчёта).

## сервисная сессия · 02.10.2026 · гейт BO-i6/i7 пройден (коммит + перенос)

- Владелец подтвердил (**question**): «Без P2 до C10» — коммит записей
  BO-i6/i7 + аудита; перенос **P1 (+опция P5), P3, P4**; **P2 (guard) — после
  решения C10**; полигон сохраняем до заморозки.
- Пакет записей (7 путей + запись роли `git`, F43): `M` —
  `.opencode/mail/service-mcp-ready-r5.md`, `.opencode/memory/{auditor,migrator,service}.md`,
  `docs/tasks/T-15-mcp-ready-process/{README.md,wave0b-own.md}`; `??` —
  `docs/tasks/T-15-mcp-ready-process/wave0b-own-report.md`; + `.opencode/memory/git.md`.
- Сообщение коммита: `chore(process): T-15 B0-own BO-i6/i7 — wave0-attribution
  (свёрнут), отчёт, аудит`.
- База: `develop` = `origin/develop` = `e740603`; master не трогаем.
- Следующее действие — `dispatch git` (пакет записей), затем перенос P1/P3/P4
  в `.opencode/plugins|scripts` (сервисная сессия) и отдельный пакет переноса.

## сервисная сессия · 02.10.2026 · перенос P1/P3/P4 исполнен

- **P1** → `.opencode/plugins/wave0-observe.ts` (id `wave0-observe`): журнал
  `target/wave0-observe.jsonl` (bounded: ротация в `.1` при > 8 МБ), агрегат
  `target/wave0-observe-summary.json`; сводка родителю — только при
  `WAVE0_OBSERVE_SUMMARY=1`; фильтр location/known-sessionID. Проверено:
  `loading plugin` + журнал пишет события текущей сессии репозитория.
- **P3** → `.opencode/scripts/session-checkpoint.mjs` (доработка: файлы
  «изменённые/прочитанные»); проверено на `tester` T-18 (3 изменённых / 44
  прочитанных, resume-путь, метрики).
- **P4** → `.opencode/scripts/metrics-report.mjs`; проверено (stats/list).
- **P2** — не переносится (ждёт решения C10); **P5** — свёрнут (опция в P1).
- Полигон `%TEMP%\opencode\wave0b-own` сохранён до заморозки (решение
  владельца).
- Дальше: `validator` (приёмка переноса) → коммит пакета переноса.

## validator · 02.10.2026 · принято (перенос B0-own) — см. секцию роли ниже

- Вердикт: **принято, P1/P2/P3 нет**; отчёт
  `docs/reviews/service-b0-own-transfer-2026-10-02.md`, квитанция
  `service-b0-own-transfer` (iteration 1, accepted) в `receipts.yaml`.
- Проверки: артефакты соответствуют заявленному; смоук-улики in-repo
  (`plugin.start` + `event` с sid текущей сессии; checkpoint 3/44); границы
  чисты; `cargo` не запускался (D50); лог вне прав — компенсирован in-repo.

## сервисная сессия · 02.10.2026 · гейт пакета переноса пройден

- Владелец подтвердил (**question**): «Коммит + push develop (Recommended)».
- Пакет — 12 путей + запись роли `git` (F43) = 13: `??` —
  `.opencode/plugins/wave0-observe.ts`, `.opencode/scripts/{session-checkpoint,metrics-report}.mjs`,
  `docs/reviews/service-b0-own-transfer-2026-10-02.md`; `M` —
  `.opencode/mail/service-mcp-ready-r5.md`,
  `.opencode/memory/{migrator,service,validator}.md`,
  `.opencode/state/current/receipts.yaml`, `docs/tasks/T-15-mcp-ready-process/{README.md,wave0b-own.md,wave0b-own-report.md}`,
  `.opencode/memory/git.md`.
- Сообщение коммита: `chore(process): T-15 B0-own — перенос P1/P3/P4 в служебную
  зону (wave0-observe, session-checkpoint, metrics-report)`.
- База: `develop` = `origin/develop` = `ac5d382`; master не трогаем.
- Следующее действие — `dispatch git`.

## migrator · 02.10.2026 · готово (BO-i2: карточка P2)

- Сделано: в `docs/tasks/T-15-mcp-ready-process/README.md` одна строка
  сводной таблицы «Реестр задач T-15» — **B0-own-P2** статус
  `⬜ · предложена BO-i2` → `🟢 BO-i2 (02.10) — вердикт: комбинация
  experimental.policies + permissions + плагин; перенос — после аудита (вход C10)`.
- Проверки: `git diff -- <README>` → ровно один ханк/одна строка (192–196);
  `git diff --check` пусто. `cargo` не запускал (D50).
- Дальше / риски: коммит — за ролью `git` (гейт); иных строк/файлов не трогал.

## git · 02.10.2026 · план пакета

- Сделано (до `add`, F43): чекпойнт в `.opencode/memory/git.md` и эта запись —
  часть пакета. Подтверждение сверено по §«гейт пакета BO-i2 пройден» (:62–76),
  ответ владельца «Коммит BO-i2 + BO-i3».
- База: `develop` = `origin/develop` = `fed94d6`; ветки нет (прямая правка),
  master не трогаем. Снимок 6 `M` + 1 `??` = ровно пакет 7 путей.
- К коммиту — 8 путей: `.opencode/mail/service-t15-run-review.md`,
  `.opencode/memory/{migrator,service,git}.md`,
  `.opencode/state/current/progress.yaml`,
  `docs/tasks/T-15-mcp-ready-process/README.md`,
  `docs/tasks/T-15-mcp-ready-process/wave0b-own.md`,
  `.opencode/mail/service-mcp-ready-r5.md`.
- Сообщение: `chore(process): T-15 B0-own BO-i2 — wave0-guard: пробы, вердикт,
  карточка P2`. Ожидаемо: 7 `M` + 1 `A`; далее `commit` → `push origin develop`.
- Дальше / риски: хеши — в ответе `lead` (F43); после `push` в отслеживаемые
  файлы не пишу; BO-i3 — отдельным действием.

## migrator · 02.10.2026 · готово (BO-i3: карточка P1)

- Сделано: в `docs/tasks/T-15-mcp-ready-process/README.md` одна строка
  сводной таблицы «Реестр задач T-15» — **B0-own-P1** статус
  `⬜` → `🟢 BO-i3 (02.10) — вердикт: журнал событий сессий + агрегат +
  сводка родителю; перенос — после аудита`.
- Проверки: `git diff -- <README>` → ровно один ханк/одна строка (194);
  `git diff --check` пусто. `cargo` не запускал (D50).
- Дальше / риски: коммит — за ролью `git` (гейт); иных строк/файлов не трогал.

## git · 02.10.2026 · план пакета (BO-i3)

- Сделано (до `add`, F43): чекпойнт в `.opencode/memory/git.md` и эта запись —
  часть пакета. Подтверждение сверено по §«гейт пакета BO-i3 пройден»
  (:115–125), ответ владельца «Коммит BO-i3 + BO-i4».
- База: `develop` = `origin/develop` = `85658fd`; ветки нет (прямая правка),
  master не трогаем. Снимок 5 `M` = ровно пакет 5 путей (+ мой чекпойнт).
- К коммиту — 6 путей: `.opencode/mail/service-mcp-ready-r5.md`,
  `.opencode/memory/{migrator,service,git}.md`,
  `docs/tasks/T-15-mcp-ready-process/README.md`,
  `docs/tasks/T-15-mcp-ready-process/wave0b-own.md`.
- Сообщение: `chore(process): T-15 B0-own BO-i3 — wave0-observe: пробы, вердикт,
  карточка P1`. Ожидаемо: 5 `M` + 1 `A`; далее `commit` → `push origin develop`.
- Дальше / риски: хеши — в ответе `lead` (F43); после `push` в отслеживаемые
  файлы не пишу.

## migrator · 02.10.2026 · готово (BO-i4: карточка P4)

- Сделано: в `docs/tasks/T-15-mcp-ready-process/README.md` одна строка
  сводной таблицы «Реестр задач T-15» — **B0-own-P4** статус
  `⬜` → `🟢 BO-i4 (02.10) — вердикт: CLI-отчёт цепочек (stats/export)
  воспроизвёл метрики прогона; перенос — после аудита (вход C6/D)`.
- Проверки: `git diff -- <README>` → ровно один ханк/одна строка (197);
  `git diff --check` пусто. `cargo` не запускал (D50).
- Дальше / риски: коммит — за ролью `git` (гейт); иных строк/файлов не трогал.

## git · 02.10.2026 · план пакета (BO-i4)

- Сделано (до `add`, F43): чекпойнт в `.opencode/memory/git.md` и эта запись —
  часть пакета. Подтверждение сверено по §«гейт пакета BO-i4 пройден» (:164–174),
  ответ владельца «Коммит BO-i4 + BO-i5». База: `develop` = `origin/develop` =
  `9a811b4`; снимок `git status` = 5 `M` = ровно пакет 5 путей (+ мой чекпойнт).
- К коммиту — 6 путей (`./`-префикс, без `--`): `.opencode/mail/service-mcp-ready-r5.md`,
  `.opencode/memory/{migrator,service,git}.md`,
  `docs/tasks/T-15-mcp-ready-process/README.md`,
  `docs/tasks/T-15-mcp-ready-process/wave0b-own.md`.
- Сообщение: `chore(process): T-15 B0-own BO-i4 — metrics-report: пробы, вердикт,
  карточка P4`. Ожидаемо: 5 `M` + 1 `A`; далее `commit` → `push origin develop`.
- Дальше / риски: хеши — в ответе `lead` (F43); после `push` в отслеживаемые
  файлы не пишу.

## migrator · 02.10.2026 · готово (BO-i5: карточка P3)

- Сделано: в `docs/tasks/T-15-mcp-ready-process/README.md` одна строка
  сводной таблицы «Реестр задач T-15» — **B0-own-P3** статус
  `⬜` → `🟢 BO-i5 (02.10) — вердикт: сводка останова
  (`session-checkpoint.mjs`) + resume `--session`; перенос — после аудита
  (вход C9)`.
- Проверки: `git diff -- <README>` → ровно один ханк/одна строка (196);
  `git diff --check` пусто. `cargo` не запускал (D50).
- Дальше / риски: коммит — за ролью `git` (гейт); иных строк/файлов не трогал.

## git · 02.10.2026 · план пакета (BO-i5)

- Сделано (до `add`, F43): чекпойнт в `.opencode/memory/git.md` и эта запись —
  часть пакета. Подтверждение сверено по §«гейт пакета BO-i5 пройден (коммит;
  пауза)» (:217–227), ответ владельца «Коммит BO-i5, пауза». База: `develop` =
  `origin/develop` = `11110bd`; ветки нет (прямая правка), master не трогаем.
  Снимок `git status` = 5 `M` = ровно пакет 5 путей (+ мой чекпойнт).
- К коммиту — 6 путей (`./`-префикс, без `--`):
  `.opencode/mail/service-mcp-ready-r5.md`,
  `.opencode/memory/{migrator,service,git}.md`,
  `docs/tasks/T-15-mcp-ready-process/README.md`,
  `docs/tasks/T-15-mcp-ready-process/wave0b-own.md`.
- Сообщение: `chore(process): T-15 B0-own BO-i5 — wave0-checkpoint: пробы,
  вердикт, карточка P3`. Ожидаемо: 6 `M`; далее `commit` → `push origin develop`.
- Дальше / риски: хеши — в ответе `lead` (F43); после `push` в отслеживаемые
  файлы не пишу; BO-i6/BO-i7 — пауза.

## migrator · 02.10.2026 · готово (BO-i6/i7: карточка P5 + отчёт)

- Сделано: `docs/tasks/T-15-mcp-ready-process/README.md` — три правки:
  (1) сводная таблица «Реестр задач T-15», **B0-own-P5**: `⬜` → `🟢 BO-i6 (02.10)
  — вердикт: свернуть (покрыто P1/P4 + context-хук)`; (2) после строки P5 —
  новая строка `| B0-own-i7 | Отчёт и финализация мини-волны | B0-own | ✅ 02.10 |
  wave0b-own-report.md |`; (3) «Примечания» → блок «Связанные артефакты» — пункт
  со ссылкой на `wave0b-own-report.md` (итоги BO-i1…BO-i6; перенос — после
  аудита и решения владельца).
- Проверки: `git diff -- <README>` → ровно 3 правки (P5 1+/1−, +i7 1, +пункт 2);
  `git diff --check` пусто. `cargo` не запускал (D50).
- Дальше / риски: коммит — за ролью `git` (гейт); `wave0b-own-report.md`
  (`??`) в пакет не входит — вне моей правки; иных строк/файлов не трогал.

## git · 02.10.2026 · план пакета (BO-i6/i7)

- Сделано (до `add`, F43): чекпойнт в `.opencode/memory/git.md` и эта запись —
  часть пакета. Подтверждение сверено по §«гейт BO-i6/i7 пройден (коммит +
  перенос)» (:306–319), ответ владельца «Без P2 до C10». База: `develop` =
  `origin/develop` = `e740603`; ветки нет (прямая правка), master не трогаем.
  Снимок `git status` = 6 `M` + 1 `??` = ровно пакет 7 путей (+ мой чекпойнт).
- К коммиту — 8 путей (`./`-префикс, без `--`):
  `.opencode/mail/service-mcp-ready-r5.md`,
  `.opencode/memory/{auditor,migrator,service,git}.md`,
  `docs/tasks/T-15-mcp-ready-process/README.md`,
  `docs/tasks/T-15-mcp-ready-process/wave0b-own.md`,
  `docs/tasks/T-15-mcp-ready-process/wave0b-own-report.md`.
- Сообщение: `chore(process): T-15 B0-own BO-i6/i7 — wave0-attribution (свёрнут),
  отчёт, аудит`. Ожидаемо: 7 `M` + 1 `A`; далее `commit` → `push origin develop`.
- Дальше / риски: хеши — в ответе `lead` (F43); после `push` в отслеживаемые
  файлы не пишу; перенос P1/P3/P4 — отдельным пакетом (сервисная сессия).

## migrator · 02.10.2026 · готово (перенос P1/P3/P4: карточка)

- Сделано: в `docs/tasks/T-15-mcp-ready-process/README.md` три строки сводной
  таблицы «Реестр задач T-15» — в ячейки статуса дописан хвост (существующий
  текст сохранён): **B0-own-P1** `· перенесён 02.10 → .opencode/plugins/wave0-observe.ts`;
  **B0-own-P3** `· перенесён 02.10 → .opencode/scripts/session-checkpoint.mjs`;
  **B0-own-P4** `· перенесён 02.10 → .opencode/scripts/metrics-report.mjs`.
- Проверки: `git diff -- <README>` → ровно 3 строки (P1/P3/P4, по 1+/1−);
  `git diff --check` пусто. `cargo` не запускал (D50).
- Дальше / риски: коммит — за ролью `git` (гейт); иных строк/файлов не трогал.

## validator · 02.10.2026 · принято (перенос B0-own)

- **Вердикт:** принято; P1/P2/P3 нет. Отчёт
  `docs/reviews/service-b0-own-transfer-2026-10-02.md`; квитанция `accepted`
  (task `service-b0-own-transfer`, iteration 1) записана в
  `.opencode/state/current/receipts.yaml`.
- **База/версия:** `develop` = `origin/develop` = `HEAD` = `ac5d382` (ветки
  нет, прямая правка; снимок 02.10.2026).
- **Артефакты = заявленному:** плагин `Plugin.define({id:"wave0-observe"})`,
  ротация `.1` при `>8 МБ` (каждые 200 appends), агрегат
  `target/wave0-observe-summary.json`, сводка только `WAVE0_OBSERVE_SUMMARY=1`,
  фильтр `loc===dir`/known-sid; P3 — changed (`edit/write/patch`+`snapshot.files`)
  /read + resume + `shell:win32`; P4 — stats/list/`--chain` + `shell:win32`.
- **Улики in-repo:** `target/wave0-observe.jsonl` — `plugin.start` (v2.0.22,
  directory = repo) + `event` (root `ses_f03ba898…`, дети `migrator`/`validator`
  с `parentID`); агрегат жив; `checkpoint-tester-T18.md` — «изменённые (3)/
  прочитанные (44)». Лог `~/.local/share/opencode/log/opencode.log` — вне прав
  (`external_directory`), компенсирован in-repo уликами.
- **Границы чисты:** статус — только 6 `M` (лента, `memory/{migrator,service}`,
  карточка T-15, `wave0b-own.md`, `wave0b-own-report.md`) + 3 `??`
  (`.opencode/plugins/wave0-observe.ts`, `scripts/session-checkpoint.mjs`,
  `scripts/metrics-report.mjs`); `src/tests/Cargo.toml/AGENTS.md/opencode.json/
  .opencode/agents|rules/docs/decisions|questions/docs/TRACEABILITY.md` — не
  затронуты; `git diff --check` пусто; `target/` в `.gitignore:1`.
- **Права:** `agents-perms.mjs` ×2 → `11 из 18`. `cargo` не запускался (D50).
- **Дальше:** гейт пакета подтверждается владельцем (состав — в отчёте
  §«Состав пакета для гейта»), затем `dispatch git` (коммит + push `develop`).

## git · 02.10.2026 · план пакета (перенос B0-own)

- Сделано (до `add`, F43): чекпойнт в `.opencode/memory/git.md` и эта запись —
  часть пакета. Подтверждение сверено по §«гейт пакета переноса пройден»
  (:346–359), ответ владельца «Коммит + push develop».
- База: `develop` = `origin/develop` = `ac5d382`; ветки нет (прямая правка),
  master не трогаем. Снимок `git status` = 8 `M` + 4 `??` = ровно пакет
  12 путей (+ мой чекпойнт, F43).
- К коммиту — 13 путей (`./`-префикс, без `--`):
  `.opencode/plugins/wave0-observe.ts`,
  `.opencode/scripts/session-checkpoint.mjs`,
  `.opencode/scripts/metrics-report.mjs`,
  `docs/reviews/service-b0-own-transfer-2026-10-02.md`,
  `.opencode/mail/service-mcp-ready-r5.md`,
  `.opencode/memory/{migrator,service,validator,git}.md`,
  `.opencode/state/current/receipts.yaml`,
  `docs/tasks/T-15-mcp-ready-process/{README.md,wave0b-own.md,wave0b-own-report.md}`.
- Сообщение: `chore(process): T-15 B0-own — перенос P1/P3/P4 в служебную зону
  (wave0-observe, session-checkpoint, metrics-report)`. Ожидаемо: 9 `M` + 4 `A`;
  далее `commit` → `push origin develop`.
- Дальше / риски: хеши — в ответе `lead` (F43); после `push` в отслеживаемые
  файлы не пишу.
