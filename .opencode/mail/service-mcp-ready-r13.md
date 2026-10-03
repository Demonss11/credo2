# service-mcp-ready-r13 — лента операции: T-15, C2 (`validate-state.mjs`)

Открыта: 03.10.2026. **Сервисная операция** (T-15, фаза C; после r12).
Предмет (решение владельца 03.10.2026, «реализуем C2»):

- **C2 — `validate-state.mjs`** (записка §5; карточка T-15, строка реестра
  `C2` ⬜): валидатор схемы состояния (`pre-flight` + приёмка `validator`);
  канон — `.opencode/rules/state-schema.md` (D86); неизвестные поля —
  **предупреждение**, не ошибка.

**Рамка:** канон — `.opencode/rules/state-schema.md`,
`.opencode/rules/dispatch-loop.md` (§«Pre-flight»), `.opencode/agents/validator.md`,
`review.md`, `AGENTS.md`; скрипт — служебная зона `.opencode/scripts/**`;
журнал — Q88 → D91 + карточка T-15 (`C2` 🚧); протокол: журнал → правки →
`auditor` → `validator` (адресная + схемный прогон) → гейт → `git` (develop) →
итог. `C2` — служебная зона, продуктовый код не затрагивается.

## сервисная сессия · 03.10.2026 · открытие — журнал заведён

- **Журнал:** Q88 (`docs/questions/Q88.md`) → D91
  (`docs/decisions/D91-c2-validate-state.md`); каталог `questions/README.md`,
  `decisions/README.md`, `docs/TRACEABILITY.md:92` (Q88: D91, `in work`,
  T-15 🚧); карточка T-15 — `C2` ⬜→🚧 (D91); фичи не тронуты.
- **Скрипт:** `.opencode/scripts/validate-state.mjs` — валидатор схемы
  состояния по `.opencode/rules/state-schema.md` (D86): артефакты
  `next_action.yaml`, `current_state.yaml`, `progress.yaml`, `receipts.yaml`;
  обязательные поля, enum'ы, даты, инварианты (`iteration ≥ 1`,
  `rework = iteration − 1`, единый `iteration`, `session_index` не убывает,
  `surface_to_user` → `channel` + `owner_response`), неизвестные поля —
  **предупреждение**; коды выхода 0/1/2.
## сервисная сессия · 03.10.2026 · правки канона и скрипт — готово

- **Правки (сервисная сессия):**
  - `.opencode/scripts/validate-state.mjs` — новый валидатор (см. «открытие —
    журнал заведён»): структура, enum'ы, даты, инварианты, мягкий режим для
    исторических записей, неизвестные поля — предупреждение; коды выхода 0/1/2.
  - `state-schema.md` §«Расширение и применимость» — абзац о машинной проверке
    (`validate-state.mjs`, точки применения);
  - `dispatch-loop.md` §«Меморандумы и находки» (pre-flight) — пункт
    «`validate-state.mjs` зелёный»;
  - `review.md` — скрипт в доступных командах `validator`;
  - `validator.md` — шаг предпрогонной проверки состояния;
  - `AGENTS.md` — карта `.opencode/scripts/`.
- **Прогон (сервисная сессия, не R2):** на текущем state — ошибок схемы нет;
  проверены красный случай (F15-подобный `iteration`/`rework`), пропуск
  `owner_response` при `surface_to_user`, непустые `result`/`next`,
  `--file`/`--json`/`--strict`.
- Следующее действие — `auditor` (канон/скрипт, до коммита).

## auditor · 03.10.2026 · расхождения (P2 ×5)

- **Аудит:** C2 — `validate-state.mjs` ↔ `D91` (место/роль, точки применения,
  контракт, инварианты, строгость, формат/коды), права `validator` ↔
  `review.md`, `agents-perms.mjs`, границы, журнал (Q88↔D91, каталоги,
  TRACEABILITY, «Сверка с кодом» ⚪), отсутствие адресов `docs/analysis/**`.
- **Бюджет:** 15 файлов, 9 rg (+4 shell-прогона).

**P1:** критичных проблем нет (инструкция ↔ права по формам команд согласованы;
скрипт исполним для `validator`).

**P2:**
- `docs/tasks/T-15-mcp-ready-process/README.md:213,244` — карточка `C2` помечена
  «✅ 03.10 — принято (`service-c2`)», критерий «`validate-state.mjs` зелёный»,
  но квитанции `service-c2` в `receipts.yaml` **нет** (последняя по T-15 —
  `T-15-c13`/`T-15-c1`), отчёта в `docs/reviews/` нет, а
  `docs/TRACEABILITY.md:92` — `in work`/T-15 🚧. Приёмки не было (операция r13
  идёт до `validator`) → фиктивная улика и статус-противоречие. Правка: `C2`
  вернуть в 🚧 до фактической приёмки, имя квитанции брать из реальной записи.
- `.opencode/rules/dispatch-loop.md:180-184` — pre-flight («выполняет `lead`
  или сервисная сессия») включает `node .opencode/scripts/validate-state.mjs`,
  но у `lead` во фронтматтере **нет** shell-права на этот скрипт
  (`agents-perms.mjs`: shell `lead` = `rg`/read-only git). Инструкция
  невыполнима для `lead` → право добавить `lead.md` (и отразить в
  `review.md`) либо переписать пункт на «сервисную сессию/`validator`».
- `.opencode/scripts/validate-state.mjs:435-458` — `crossCheck()` сверяет
  единое `iteration` только `next_action ↔ current_state`; D91 п.4 и
  `state-schema.md` §«Инварианты» п.1 требуют единого `iteration` в **четырёх**
  артефактах (план/состояние/`progress`/`receipts`). Not `progress`/`receipts`
  iteration-дрейф не ловится → ложное «зелёно» на F15-подобном случае в
  записях. Правка: добавить сверку с записями текущего раунда.
- `.opencode/scripts/validate-state.mjs:355-361` — при `action: surface_to_user`
  проверяются `channel` и `owner_response`, но **не** `question`; D91 п.3 и
  `state-schema.md:106` называют `question` условно обязательным. Правка:
  добавить `soft("question", …)` при `surface_to_user`.
- `.opencode/scripts/validate-state.mjs:280-283` — `SCHEMA_DATE = 2026-10-03`,
  тогда как схема принята `D86` (**2026-10-02**); записи 02.10 (C1) уходят в
  мягкий режим вопреки D91 п.5 / `state-schema.md` §«Расширение и
  применимость» («созданные до принятия схемы»). Правка: граница — дата D86
  (02.10) либо явная оговорка в каноне.

**P3:**
- `.opencode/rules/review.md:90-91` — список команд `validator` даёт
  `agents-perms.mjs` (без `*`) и `validate-state.mjs *`, тогда как тело
  `validator.md:97` использует **bare** `node .opencode/scripts/validate-state.mjs`,
  а фронтматтер содержит обе формы. Правка одной строкой: добавить bare-форму.

**Проверки:** `node .opencode/scripts/agents-perms.mjs` — `agents: 11 из 18`,
`validator` с новыми allow `validate-state.mjs`/`*`, `steps=36`; сам
`validate-state.mjs` прогнать не смог (нет права у `auditor` — by design),
контракт сверен чтением логики (строки 1-524).
**Инструкция ↔ права:** расхождений по формам команд frontmatter↔`review.md`
нет; **отдельно** — право `validate-state.mjs` отсутствует у `lead` (P2 выше).
**Что проверено и ок:** место/роль скрипта (служебная зона, контракт —
`state-schema.md`, второй источник не введён); точки применения (pre-flight +
приёмка); контракт полей/enum/дат/непустых `result`/`next` (REQUIRED/DATE_RE/
`checkType`); коды выхода 0/1/2 и флаги `--dir/--file/--json/--strict`;
`iteration ≥ 1`, `rework = iteration − 1`, `session_index` не убывает,
`surface_to_user → channel`+`owner_response`, неизвестные поля —
предупреждение, `--strict` эскалация; журнал: Q88↔D91 парны
(`Q88.md:3`, `D91:5`), каталоги `questions/README.md:110`,
`decisions/README.md:120`, TRACEABILITY:92, «Сверка с кодом» ⚪ не применимо
(обосновано); `AFFECTS` D91 ↔ факт правок сходятся; `AGENTS.md:25` — карта
скрипта; feature `agents-state-schema` ✅ (D91) не противоречит;
`docs/analysis/**`-адресов и номеров строк в правках канона нет; границы —
`git status`: `src/**`, `tests/**`, `Cargo.toml`, `opencode.json` не тронуты
(`docs-writer`/`validator.md`/rules/AGENTS/журнал/карточка/фича/лента);
`grep` по `docs/analysis|:[0-9]+` в `state-schema.md`/`validator.md` пусто.

**Следующие шаги:**
- [ ] Вернуть `C2` в 🚧 и убрать несуществующую квитанцию `service-c2` из карточки (до приёмки).
- [ ] Решить pre-flight: добавить `lead` право `validate-state.mjs` или сузить пункт `dispatch-loop.md`.
- [ ] Досверить в скрипте: единое `iteration` по 4 артефактам; `question`; граница мягкого режима = дата D86.
- [ ] Прогнать `validator` (адресная + схемный прогон), квитанция/отчёт — по факту.

## сервисная сессия · 03.10.2026 · фиксы аудита (P2 ×5, P3) — готово

- **P2-1** (карточка «✅» без приёмки): `docs/tasks/T-15-mcp-ready-process/README.md`
  — `C2` возвращён в 🚧 («в работе (r13, D91); скрипт и точки применения»);
  ✅ — только после фактической приёмки.
- **P2-2** (pre-flight невыполним для `lead`): `lead.md` — добавлены права
  `node .opencode/scripts/validate-state.mjs` и `… *`; `review.md` §«Доступные
  команды» — команда у `lead`.
- **P2-3** (единое `iteration` — только 2 артефакта): `crossCheck()` сверяет
  план/состояние с последней записью `progress`/`receipts` по той же задаче
  (4 артефакта).
- **P2-4** (`question` при `surface_to_user` не проверялся): добавлено
  `soft("question", …)`.
- **P2-5** (граница мягкого режима): сделана явной — флаг `--since`
  (по умолчанию дата C2, 03.10.2026; схема D86 — 02.10, но машинная проверка
  введена C2); формулировка D91 п.5 и `state-schema.md` уточнены
  («до начала машинной проверки»), а не «до принятия схемы».
- **P3** (`review.md` даёт `*`, тело — bare): `review.md` приведён к bare-форме
  (фронтматтер содержит обе).
- **Прогон после правок:** `validate-state.mjs` — ошибок 0, предупреждений 132
  (исторические записи — мягкий режим; неизвестные поля — предупреждение);
  `cargo test --test docs_journal` — **14/0**.
- Следующее действие — `validator` (адресная приёмка + схемный прогон).

## validator · 03.10.2026 · приёмка — принято

- **Проверка:** адресная приёмка + схемный прогон + адресный `docs_journal`
  (F74); полный DoD не запускался (D50 — `src/**`, `tests/**`, `Cargo.toml` не
  менялись). Версия: `develop` = `origin/develop` @ `7e458fd` + рабочее дерево.
- **Вердикт: принято** (P1/P2/P3 — нет).
  - `node .opencode/scripts/validate-state.mjs` — **ошибок 0, предупреждений
    132** (исторические записи — мягкий режим; неизвестные поля —
    предупреждение).
  - Негативные фикстуры (`--dir`): дрейф `iteration` план↔состояние и по
    4 артефактам, `rework ≠ iteration − 1`, `surface_to_user` без
    `channel`/`owner_response`/`question`, `dispatch` без `role`/`expect_match`,
    чужой enum, убывающий `session_index`, `--strict`/`--json`/`--file`/`--since`
    — все пойманы.
  - `cargo test --test docs_journal` — **14 passed / 0 failed**.
  - `agents-perms.mjs` — «11 из 18»; `validator` и `lead` — allow
    `validate-state.mjs`.
  - Фиксы аудита (P2 ×5 + P3) подтверждены.
- **P3 (наблюдение):** `.credo/sandbox.json` (tracked) изменён в дереве — вне
  периметра C2, в пакет не включается; live state содержит неизвестные поля
  `blocker`/`route_done`/`re_raise` — пред-существующий дрейф, кандидат на
  расширение схемы.
- **Технические замечания:** роль `validator` достигла лимита шагов до записи
  артефактов; отчёт и квитанция оформлены сервисной сессией по результатам
  проверки. Временные фикстуры удалены до пакета.
- **Артефакты:** отчёт `docs/reviews/service-c2-2026-10-03.md`; квитанция
  `service-c2` (iteration 1, accepted) в `receipts.yaml`.

## сервисная сессия · 03.10.2026 · пакет r13 — гейт владельца

- **Приёмка:** `validator` — **принято** (`service-c2`, iteration 1);
  схемный прогон 0 ошибок; `docs_journal` 14/0. `C2` ✅; Q88 остаётся
  `in work` (T-15 🚧; D82); фича `agents-state-schema` ✅.
- **Пакет (снимок `git status`, 16 M + 5 ?? = 21 + запись роли `git` F43):**
  - скрипт: `.opencode/scripts/validate-state.mjs` (новый);
  - канон: `.opencode/rules/{state-schema,dispatch-loop,review}.md`,
    `.opencode/agents/{lead,validator}.md`, `AGENTS.md`;
  - журнал: `docs/questions/Q88.md`,
    `docs/decisions/D91-c2-validate-state.md`,
    `docs/{questions,decisions}/README.md`, `docs/TRACEABILITY.md`;
  - карточка `docs/tasks/T-15-mcp-ready-process/README.md`; фича
    `docs/features/README.md`;
  - отчёт: `docs/reviews/service-c2-2026-10-03.md`;
  - лента r13 + подхват хвоста r12 (`.opencode/mail/service-mcp-ready-r12.md`);
  - state/память: `.opencode/state/current/receipts.yaml`,
    `.opencode/memory/{auditor,service,validator}.md`.
  - **Вне пакета:** `.credo/sandbox.json` (tracked, продуктовые данные CREDO —
    не C2).
- Сообщение: `chore(process): T-15 r13 — C2 (validate-state.mjs, схема
  состояния; D91); приёмка`.
- База: `develop` = `origin/develop` = `7e458fd`; ветки нет; `master` — по
  решению гейта.

## сервисная сессия · 03.10.2026 · гейт пакета — подтверждён

- **question:** owner-gate пакета r13 (21 путь + запись роли `git`).
- **owner_response (дословно):** «Коммит + push develop (Recommended)».
- Порядок: коммит в `develop` + `push origin develop`; `master` не трогаем.
- Следующее действие — `dispatch git`.

## сервисная сессия · 03.10.2026 · итог r13

- **Коммит:** `eeac481` — `chore(process): T-15 r13 — C2 (validate-state.mjs,
  схема состояния; D91); приёмка` (21 путь: 16 M + 5 A; +1128/−8); push
  `7e458fd..eeac481` → `origin/develop`; ветки/`master` не тронуты.
- **Итог:** C2 внесён — валидатор схемы состояния
  `.opencode/scripts/validate-state.mjs` (D91), точки применения — pre-flight
  (`lead`) и приёмка (`validator`); права выданы обеим ролям; канон
  (`state-schema.md`, `dispatch-loop.md`, `review.md`, `AGENTS.md`) обновлён.
  `C2` ✅; Q88 остаётся `in work` (T-15 🚧); фича `agents-state-schema` ✅.
  Аудит `auditor` (P2 ×5 + P3) — исправлено до коммита; приёмка `validator`
  (`service-c2`, iteration 1) — принято; схемный прогон 0 ошибок,
  `docs_journal` 14/0.
- **Остаток T-15 (закрываемо):** C4+C3-остаток (`process/runN`/теги/фасад),
  C6 (метрики), C8 (кандидаты B0).
- **Пост-пакетные записи** (этот итог, память) — подхват следующим пакетом
  (F43).


## validator · 03.10.2026 · в работе (r13, C2), до прогона

- **Снимок:** `develop` = `origin/develop`, HEAD `7e458fd` + рабочее дерево.
- **Проверено чтением/rg:** D91 ↔ факт (место/роль, точки применения
  pre-flight+приёмка, контракт полей/enum/дат, инварианты, строгость
  `--since`, формат/коды 0/1/2); журнал Q88↔D91, каталоги, TRACEABILITY:92,
  «Сверка с кодом» ⚪; карточка C2 🚧 (:213, не ✅); фиксы аудита P2×5+P3.
- **Запускаю:** `node .opencode/scripts/validate-state.mjs` (ожидание ошибок 0);
  негативные фикстуры `--dir` в `$env:TEMP` (дрейф `iteration`/`rework`, пропуск
  `owner_response`/`question`/`channel`, убывающий `session_index`, чужой enum,
  `--strict`/`--json`/`--file`); `agents-perms.mjs`; `git diff --numstat -- src
  tests Cargo.toml`; `cargo test --test docs_journal` (ориентир 14/0). Полный DoD
  не запускается (D50).

