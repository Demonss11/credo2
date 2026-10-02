# T-15 · B0-own — мини-волна своей обвязки (P1–P5)

- **Статус:** 🚧 BO-i1 (28.09.2026), BO-i2…BO-i4 (02.10.2026) выполнены;
  BO-i5…BO-i7 — по подтверждению владельца. Не канон; канон, `.opencode/**` и
  `opencode.json` репозитория не менялись.
- **Назначение:** рабочий журнал мини-волны B0-own к [`wave0b-report.md`](wave0b-report.md)
  §4/§6: собственные V2-плагины/скрипт вместо чужих V1-плагинов — P1
  `wave0-observe`, P2 `wave0-guard`, P3 `wave0-checkpoint`, P4
  `metrics-report.mjs`, P5 `wave0-attribution`.
- **Спутник:** [`wave0b-own-plugin-guide.md`](wave0b-own-plugin-guide.md) — шпаргалка
  для агентов по V2-плагинам (форма установки, шаблон, `ctx`, таблица хуков,
  проверенные формы данных, нативные источники, грабли).
- **Рамка:** пробы — в temp-полигоне; вердикты — по протоколу B0
  ([`wave0b-probes.md`](wave0b-probes.md)); перенос зелёного в
  `.opencode/plugins/`/`.opencode/scripts/` — только после аудита `auditor`;
  реестр — [`README.md`](README.md) §«Реестр задач T-15».
- **Полигон:** `%TEMP%\opencode\wave0b-own` (git init, dummy `.env`,
  `opencode.json`, `dummy.txt`); форма установки B0-i1: `.opencode/plugins/*.ts`
  (автозагрузка) + `.opencode/package.json` (`@opencode/plugin ^2.0.18`) +
  `npm install`; модель проб — `opencode-go/deepseek-v4.1-flash`.
- **Улики:** `target/wave0b-own-i*/` (вне git).

## Протокол итерации (шаблон; копировать на BO-iN)

### BO-iN · <тема> · <дата>

- **Цель:** …
- **Механика:** плагин/файл, форма установки, хук/источник.
- **Сценарий пробы:** сессия/промпт/роль.
- **Факты:** команда → вывод; маркеры, журнал, метрики.
- **Вердикт:** 🟢/🔴 + обоснование; перенос — только после аудита.
- **Откат:** шаг.
- **Дальше / риски:** …

## BO-i1 · Каркас и разведка V2-API · 28.09.2026

- **Цель:** свежий полигон с зафиксированной формой установки; зелёный startup;
  таблица «задача P1–P5 → доступный хук/источник V2» (где дёшево — проверка
  пробой).
- **База:** `develop @ cafc4c4`, дерево чистое (`git log -1`, `git status`).
- **Механика:** локальный плагин `.opencode/plugins/wave0b-own-probe.ts`
  (`Plugin.define({ id, setup })`, импорт `@opencode/plugin`); маркер
  `wave0b-own-probe.json` из setup; журнал `wave0b-own-events.jsonl`
  (`appendFileSync`, один JSONL-объект на запись); плагин этапа 2 регистрирует
  `permission.hook("evaluate")`, `tool.hook("execute.before"/"execute.after")`,
  `session.hook("context")`, `ctx.command.list()`, `ctx.agent.list()`,
  `ctx.event.subscribe()` (семплирование ≤5 записей на `(type, sessionID)`,
  фильтр чужого `location`).
- **Факты:**
  1. Сборка полигона: `npm install` в `.opencode/` — `added 283 packages in
     42s` (B0-i1: 60 с). Файлы полигона: `opencode.json` (`$schema`), `.env`,
     `dummy.txt`, `.opencode/package.json`, `.opencode/plugins/wave0b-own-probe.ts`.
  2. Startup (stage=minimal): `loading plugin …wave0b-own-probe.ts`
     (14:49:50.937Z); маркер из setup (14:49:56.057Z, `version 2.0.18`);
     WARN `failed to load plugin` для плагина нет.
  3. Прогон 1 (headless, «Ответь одним словом: ок»): CLI не завершился за 240 с —
     **аномалия B0-i1 повторилась**; сессия `ses_f1781a1b6ffeJQE5YHizEvsNKd`
     `outcome: succeeded` (`ок`, завершение ~5 мин, уже после kill CLI).
     Прогон 2 — норма: `> build · deepseek-v4.1-flash` → `ок`.
  4. Reload после атомарного write (stage=recon): `loading plugin`
     14:55:55.038Z; маркер stage=recon 14:55:55.090Z.
  5. Разведочный прогон (промпт: субагент + `echo PARENT-OK` + `read dummy.txt`):
     вывод `→ Read dummy.txt`, `$ echo PARENT-OK`, `✓ Run echo SUBAGENT-OK
     General Agent`; дочерняя сессия `ses_f177bdce2ffe2yW2P2a3I11d43`
     (agent `general`), родитель — `ses_f177bef6fffeBS6rutv2etd7Z5` (`build`).
  6. Журнал плагина (снимок ~15:00Z): 87 `event` · 4 `session.context` ·
     4 `permission.evaluate` · 4 `tool.before` · 4 `tool.after` · 1
     `command.list` · 1 `agent.list`.
  7. Проверка логов: счётчик `failed to load plugin` в логе искажается
     самореферентно — лог фиксирует spawn собственных `rg`-команд, содержащих
     шаблон поиска; надёжный критерий — адресный поиск по `wave0b-own-probe`
     (только `loading plugin`/watcher-строки).
- **Вердикт:** 🟢 — форма установки V2 воспроизведена на свежем полигоне, startup
  чистый, разведка дала подтверждённую таблицу хуков.
- **Откат:** удалить temp-полигон целиком; улики — `target/wave0b-own-i1/`
  (вне git); репозиторий не менялся.
- **Дальше:** BO-i2 (`wave0-guard`) — старт по подтверждению владельца.

### Таблица: задача P1–P5 → доступный хук/источник V2 (OpenCode 2.0.18)

| P | Задача по §4 | Механизм V2 | Подтверждение |
|---|---|---|---|
| P1 `wave0-observe` | события субагентских/родительских сессий → jsonl + сводка родителю | `ctx.event.subscribe()` — публичный поток сервера: `session.created` (в `data`: `parentID`, `agent`, `model`, `title`, `location`), `session.step.*`, `session.tool.*`, `session.text/reasoning.*`, `session.inbox.*`, `session.execution.*`, `session.usage.updated`; нативно: `session.list` (фильтр `parentID`, `null` — только корневые), `Session.Info.parentID` | **проба**: событие `session.created` дочерней сессии с `parentID` родителя и `agent=general`; поток серверный — события с `location=null` (usage/execution) приходят от любых сессий (нужен фильтр по project/session) |
| P2 `wave0-guard` | deny `--force`/`reset --hard`/секретов | `ctx.permission.hook("evaluate")` — меняет `effect` (`allow/ask/deny`), в событии `action`, `resources`, `agent`, `source` (deny из конфига финален — хук не вызывается); `ctx.tool.hook("execute.before")` — правка `input` до исполнения (reject-API нет); `ctx.shell.hook("create.before")` — правка `command`/`cwd`/`timeout`/`env`; `experimental.policies` — статический hard-deny в конфиге | **проба**: `evaluate` сработал (`subagent`/`shell`/`read`, `effect=allow`); `tool.before/after` — полные поля; policies — по докам, не проверялся (вход BO-i2) |
| P3 `wave0-checkpoint` | структурная сводка останова/обрыва | команды: `ctx.command.transform` (регистрация), config `commands`, `.opencode/commands/*.md`; `ctx.session.hook("prompt")` — admission-правка промпта; источник — `session export` (`info` + сообщения), продолжение — `session import`/`--continue`/`--fork` | `ctx.command.list()` — подтверждён (`init`, `review`); регистрация и запуск своей команды — не проверялись (следующая итерация) |
| P4 `metrics-report.mjs` | отчёты из `session export`/`stats` | CLI: `opencode stats --json [--days/--year/--full/--project/--models/--tools/--cost]`, `opencode session export [--sanitize]`; HTTP: `GET /api/experimental/session/stats`, `GET /api/experimental/session/{id}/export` (experimental) | **проба**: `stats --json` — sessions/subagents/prompts/steps/tokens/cost/tools/models/activity; `session export` — `info{agent, model, parentID, outcome, time, …}` + сообщения |
| P5 `wave0-attribution` | атрибуция `role`/модель крючком | нативно: `Session.Info.agent` + `model` (`list`/`export`), сообщения assistant (`agent`, `model`); хуки: `ctx.session.hook("context")` — `event.agent`, `event.model`; события `session.created`/`session.step.started` — `data.agent`, `data.model` | **проба**: context-хук дал `agent`/`model` родителя (`build`) и субагента (`general`); export — `agent`/`model`/`parentID` |

### Находки для прототипов

- **P1:** поток событий серверный, не location-scoped: реализация обязана
  фильтровать по проекту/сессии, иначе журнал засорится чужими сессиями
  (наблюдено: `usage.updated` этого репозитория попал в temp-журнал).
  Связка «родитель → субагент» доступна в `session.created.data.parentID`;
  состав типа `session.idle` в потоке не наблюдался (в `session export` есть
  сообщение `type: "idle"` c `outcome`).
- **P2:** `permission.evaluate` даёт действие и ресурс до исполнения:
  `action` ∈ {`subagent`, `shell`, `read`, …}, `resources` — имя агента/команда/
  путь; удобен как точка deny. `tool.execute.*` — наблюдательная точка
  (правка входа/результата), отказа нет. `create.before` у shell — правка
  команды/окружения.
- **P3:** `command.list()` вернул `{location, data:[{name, description}]}`
  (встроенные `init`, `review`); источники для сводки — `session export`
  (структура `info`+`messages`), snapshot-поля `snapshot.start/end/files`.
- **P4:** `--project .` на temp-полигоне дал 20 сессий (projectID `global` —
  у temp-проектов без remote; ожидаемо); на реальном репозитории фильтр
  проверить в BO-i5. HTTP-эндпоинты помечены experimental — в `metrics-report.mjs`
  опереться на CLI JSON.
- **P5:** атрибуция полностью нативна (`Session.Info`), плагин нужен только для
  онлайн-крючка (`context` hook даёт `agent`+`model` на каждый запрос).

### Наблюдения и риски

- Аномалия «первый headless-прогон после подъёма проекта зависает в CLI, сессия
  завершается `succeeded`» повторилась (B0-i1 и BO-i1): контроль таймаута,
  повтор прогона; на вердикт о плагине не влияет (startup-лог и маркер чистые).
- Журнал probe продолжает пополняться после итерации (плагин жив, поток
  серверный): в BO-i2 ограничить фильтром по сессии/проекту либо глушить по
  завершении пробы; temp-данные при откате удаляются целиком.
- Кириллица в консольных чтениях JSONL отображается как mojibake (кодировка
  консоли); файлы — UTF-8, значения в журнале корректны.

### Ревью BO-i1 (сервисная сессия, 28.09.2026)

- Независимая проверка: база `cafc4c4`; реестр (`B0-i8` ✅ `cafc4c4`, `B0-own-i1`
  ✅, `P1–P5` ⬜), память, лента (r3 = 322 строки → том 4); маркер `stage=recon`
  (14:55:55Z); комплект улик `target/wave0b-own-i1/` (журнал, экспорт дочерней
  сессии, `stats`, вырезка лога); сессии: `ses_f177bdce2ffe…` (`parentID`,
  `agent=general`, `succeeded`), родитель `ses_f177bef6fffe…`. Вердикт BO-i1
  подтверждён — **принято**.
- Аномалия «первый прогон после подъёма локации висит в CLI» (2/2: B0-i1, BO-i1):
  приём — таймаут 60–90 с → kill → повтор (сессия уже `succeeded`; либо resume по
  `sessionID`); в BO-i2 первый прогон — «прогрев» полигона. Кандидат в находку
  (F/D) — оформить в отчёте BO-i7.
- Журнал probe продолжал расти после BO-i1 (87 → 123 строки, 55.7 КБ; остановлен
  15:10:57Z): probe-плагин погашен — файл убран из `.opencode/plugins/` полигона в
  `_off/` (репозиторий и канон не тронуты); контроль — после снятия новый прогон
  в полигоне мгновенный (без зависания), журнал не растёт; в BO-i2 — фильтр по
  `project`/`session` и выключение по завершении пробы.

### Предложение BO-i2 · `wave0-guard`: плагин vs `experimental.policies`

- **Цель:** выбрать механизм страховки P2 и оформить прототип для переноса.
- **Варианты:** (A) V2-плагин — `permission.hook("evaluate")` (динамические
  правила: `agent` + `action` + `resources`), аудит `tool.execute.before`,
  правка `shell.create.before`; (B) `experimental.policies` — статический
  hard-deny из конфига (`permission:shell:*--force*` и т.п.), нельзя отменить
  репозиторию; (C) комбинация: policies — грубый статический контур, плагин —
  контекстный слой и аудит.
- **Сценарии проб:** `git push --force`, `git reset --hard`, `read .env`,
  `Remove-Item -Recurse -Force` (standard/paranoid), запись вне проекта;
  критерии — блокировка до запуска команды, allow-поток без изменений, аудит
  JSONL, поведение с `--auto`, ложные срабатывания.
- **Результат:** вердикт по протоколу B0 → кандидат переноса (CC Safety Net —
  отдельное решение владельца, пресет standard).

### BO-i2 · `wave0-guard`: плагин vs `experimental.policies` · 02.10.2026

- **Цель:** выбрать механизм страховки P2 и оформить прототип для переноса
  (вход C10); проверить критерии: блокировка до запуска, allow-поток без
  изменений, аудит, `--auto`, ложные срабатывания.
- **База/среда:** полигон `%TEMP%\opencode\wave0b-own`; OpenCode **2.0.22**
  (в BO-i1 было 2.0.18); модель проб `opencode-go/deepseek-v4.1-flash`;
  прогоны `opencode run --auto`.
- **Механика проб:**
  - **A — плагин** `.opencode/plugins/wave0b-own-guard.ts`:
    `permission.hook("evaluate")` (правила: `--force`→deny, `reset --hard`→deny,
    `read *.env`→deny, `Remove-Item -Recurse -Force`→ask/deny по режиму,
    `edit` вне location→deny) + аудит `tool.execute.before`; режим
    standard/paranoid — файлом `wave0b-own-guard-mode.txt` (без перезагрузки).
  - **B — `experimental.policies`** (конфиг полигона): `shell:*--force*`,
    `shell:*reset --hard*`, `read:*.env` → deny.
  - **D — `permissions`** (правило репо): `shell:*reset --hard*` → deny.
  - **C — комбинация** (плагин + policies).
- **Факты (прогоны; улики — `target/wave0b-own-i2/`, журнал
  `wave0b-own-guard.jsonl`: 42 записи, 6 deny — все целевые, ложных нет):**

| # | Сценарий | Механизм | Результат |
|---|---|---|---|
| A1 | `git push --force origin develop` | плагин | **BLOCKED** (`wave0b-guard: force-flag`), до запуска git |
| A2 | `git reset --hard HEAD` | плагин | **BLOCKED** (`reset-hard`) |
| A3 | чтение `.env` | плагин | **BLOCKED** (`secrets-env`), содержимое не раскрыто |
| A4 | `Remove-Item -Recurse -Force probe-dir` | плагин, standard | **EXECUTED** — `ask` при `--auto` авто-одобряется |
| A5 | `git status` + `echo` | плагин | EXECUTED — allow-поток не изменён |
| A6 | `git log -1` | плагин | EXECUTED — ложных срабатываний нет |
| A7 | `Remove-Item -Recurse -Force probe-dir2` | плагин, paranoid | **BLOCKED** (`recursive-force-delete`) |
| A8 | запись вне location | плагин | **BLOCKED** (`outside-location`) |
| B1 | `git push --force …` | policies | **BLOCKED** (`Blocked by configuration policy`) до запуска |
| B3 | чтение `.env` | policies | **BLOCKED** (policy) |
| B4 | `Remove-Item -Recurse -Force probe-dir3` | policies | **EXECUTED** — шаблон `*--force*` не ловит `-Force` (пробел статических шаблонов) |
| C1 | `git push --force …` | плагин + policies | **BLOCKED** (policy); хук `evaluate` при этом вызывается (policy применяется после хука) |
| D1 | `git reset --hard HEAD` | `permissions` deny | **BLOCKED** (`Permission denied: shell`); хук `evaluate` **не** вызывается, аудит `tool.before` пишется |

- **Наблюдения:**
  - порядок слоёв: `permissions` deny → хук не вызывается; `policies` → хук
    вызывается, но policy финальна; `ask` + `--auto` = выполнение (для защиты
    нужен именно `deny`);
  - модель сама отказалась выполнять force-push на первом промпте B1 — для проб
    нужна явная формулировка «техническая проба, выполнить обязательно»;
  - `opencode reload` в полигоне дважды вызвал churn MCP-каталога общей сессии
    (credo временно пропадал, восстановился) — сервис общий, осторожно в
    служебной зоне (кандидат в наблюдения BO-i7);
  - scanner дробит составные команды (`git remote -v; git status` → два
    resource) — правила матчатся по каждому.
- **Вердикт:** 🟢 — **комбинация**: `experimental.policies` — статический
  hard-контур (живёт в конфигурации выше репозитория — глобальный конфиг/Console;
  из репозитория не снимается), `permissions` — ролевые правила репо (дёшево,
  deny финален), плагин — контекстные deny (секреты, вне-проекта, варианты
  команд) + аудит JSONL. Одиночный плагин — тоже 🟢 (но `ask` не защищает при
  `--auto`); одиночные policies — 🟢 с оговоркой о пробелах шаблонов.
- **Откат:** полигон удаляется целиком после BO-i7; улики `target/wave0b-own-i2/`
  (плагин, конфиги, JSONL, маркер) — вне git.
- **Дальше:** BO-i3 — следующая P из очереди (§4): P1 `wave0-observe`
  (старт — по подтверждению владельца); вердикт P2 — вход C10 (решение
  «включить/отклонить» — фаза C/владелец).

### BO-i3 · `wave0-observe`: наблюдаемость субагентов · 02.10.2026

- **Цель:** события субагентских/родительских сессий → JSONL + сводка
  (вход C/D); проверить фильтр потока (BO-i1: серверный, часть событий с
  `location=null`), агрегат и доставку сводки родителю.
- **Механика:** плагин `.opencode/plugins/wave0b-own-observe.ts` —
  `ctx.event.subscribe()`; фильтр «location == каталог полигона или известный
  `sessionID`»; агрегат `wave0b-own-observe-summary.json`; сводка родителю —
  `ctx.session.prompt` (режим `journal+summary`, дебаунс 5 с тишины); маркер
  `wave0b-own-observe.json`, журнал `wave0b-own-observe.jsonl`. Полигон,
  OpenCode 2.0.22, модель та же; конфиг полигона не менялся.
- **Факты (O1–O3; улики `target/wave0b-own-i3/`):**
  - O1 (одиночная сессия): полный цикл в журнале (29 строк) — `session.created`
    → `inbox.enqueued/delivered` → `execution.started` → `instructions.updated`
    → `usage.updated` → `renamed` → `step.started/streamed/ended` →
    `reasoning.*` → `text.*` → `execution.succeeded`.
  - `ctx.session.list` — **не функция** (ошибка в журнале): список сессий у
    плагина недоступен — только CLI/HTTP (§7 шпаргалки) или события.
  - O2 (субагент `general`): дочерний `session.created` несёт **`parentID` и
    `agent=general`** (+`model`); дальнейшие события ребёнка без `parentID`
    (связка — по `created`); агрегат: родитель — tools `{subagent:1, shell:1}`,
    ребёнок — `{shell:1}`; новые типы: `session.tool.input.started/ended`,
    `session.tool.called/progress/success`, `shell.created`, `shell.exited`;
    registry-события (`model.updated` и др.) приходят с location = каталог
    плагина.
  - O3 (`journal+summary`): через 5 с тишины — `summary.prompt ok:true`
    (shape `text`); **доставка подтверждена** экспортом родителя
    (`target/wave0b-own-i3/ses_parent-O3.json` — текст сводки в транскрипте).
  - Чужие сессии не попали (фильтр работает); `projectID` temp-полигона —
    `global` (фильтр по проекту ненадёжен; только location/`sessionID`).
- **Вердикт:** 🟢 — журнал + агрегат + сводка родителю работают; фильтр
  location/`sessionID` корректен.
- **Доработки для переноса:** `sh_*` (shell) не считать сессиями; корневой
  `agent` — из `step.started`/экспорта (в `session.created` только `model`);
  сводка — эвристика по тишине (события `idle` в потоке нет); дельты
  (`reasoning.delta`/`text.delta`) для продакшена семплировать (журнал
  O1–O3 — ~71 КБ).
- **Откат:** полигон целиком после BO-i7; улики — вне git.
- **Дальше:** BO-i4 — P4 `metrics-report.mjs` (отчёты из `stats`/`export`,
  свёртка по цепочкам роль/модель); P5 `wave0-attribution` частично перекрыт
  (агент/модель/иерархия — уже в P1) — решить на BO-i7.

### BO-i4 · `metrics-report.mjs`: отчёты из `stats`/`export` · 02.10.2026

- **Цель:** прототип отчёта метрик из нативных источников (замена Opencode
  Telemetry для D; вход C6/D): сводка проекта + свёртка цепочки
  root→субагенты по ролям/моделям.
- **Механика:** скрипт `metrics-report.mjs` (CLI-only, без БД):
  `opencode stats --json`; `session list --format json` (фильтр по cwd — у
  temp `projectID=global`); `--chain <ses>` — экспорт root, поиск детей по
  маркеру `<subagent sessionID=…>`, экспорт детей, агрегат
  (agent/model/cost/tokens/wall/outcome), вывод markdown + `--out`.
- **Факты:**
  - T1 (полигон, цепочка O2): 2 сессии (root + `general`), $0.0008; таблицы
    роли/модель/сессии — ок.
  - T2 (репозиторий, прогон 02.10): 32 сессии (root + 31 ребёнок), стоимость
    **$1.4133**, вх. 4 666 766, вых. 531 193, окно 214.1 мин; по ролям —
    build $0.4604, analyst 13/$0.3320, tester 2/$0.2218, migrator 3/$0.1495,
    validator 4/$0.1404, git 5/$0.0646, docs-writer 2/$0.0200, coder
    1/$0.0126, auditor 1/$0.0119; модели — `#default` 31/$0.9529,
    `#max` 1/$0.4604 — **совпадает с B1-разбором** (агрегат из БД).
  - Грабли: `opencode` в Node — npm-шим (`.ps1/.cmd`), прямой spawn → ENOENT
    (нужен `shell: true`; DEP0190 — косметика); `stats.tokens` — объект;
    `stats` без `--days` агрегирует всё (132 сессии/$32.28 — не прогон);
    для прогона — цепочка или `--days`.
- **Вердикт:** 🟢 — цепочки/роли/модели/стоимость воспроизводятся
  CLI-средствами без БД; пригодно для D/C6.
- **Ограничения:** дети находятся по маркеру в экспорте root (хрупко при
  смене формата); экспорт детей последовательный (31 ребёнок ≈ 30 с);
  `stats` без фильтров — все проекты.
- **Откат:** полигон после BO-i7; улики `target/wave0b-own-i4/`.
- **Дальше:** BO-i5 — P3 `wave0-checkpoint` (сводка останова/обрыва);
  P5 — решение на BO-i7.

## Откат полигона (после BO-i7)

Удалить `%TEMP%\opencode\wave0b-own` целиком; улики `target/wave0b-own-i*/` —
вне git и удаляются вместе с рабочей копией по решению владельца.
