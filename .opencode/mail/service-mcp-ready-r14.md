# service-mcp-ready-r14 — лента операции: T-15, предикат «зачётного прогона»

Открыта: 03.10.2026. **Сервисная операция** (T-15, фаза C; после r13/C2).
Предмет: определить, что считать «чистым» (зачётным) прогоном — решение
владельца 03.10.2026 по обсуждению F26/F27/F15:

- **место:** `state-schema.md` (раздел);
- **ядро:** ведущий `lead` (атрибуция верна) + нет упоров лимитов `steps` +
  нет незапланированных прерываний владельца (каждый `owner_response` — гейт
  плана) + нет ручных восстановлений (пустые финалы/backfill/ручная
  реконструкция);
- **не считаются дефектами:** плановые гейты, `owner_override`, обязательный
  rework `validator`, CCSN-шаг, session-commit;
- **не ретроактивно:** зачёт — только новым прогонам;
- проверка — машинная (`progress.yaml` + экспорт сессии).

**Рамка:** канон — `.opencode/rules/state-schema.md`; журнал — Q89 → D92 +
карточка T-15 (`C17`; ссылки в B1-F26/F27/F15); протокол: журнал → правки →
`auditor` → `validator` (адресная + `docs_journal`) → гейт → `git` (develop) →
итог.

## сервисная сессия · 03.10.2026 · открытие

- Срез: `develop` = `origin/develop` = `eeac481` (r13/C2); в дереве — подхват
  (лента r13, память `service`; `.credo/sandbox.json` — продуктовые данные,
  вне пакета) + лента r14.
- Контекст: C2/D91 сделан (`validate-state.mjs`; 0 ошибок, 132 предупреждения —
  исторический мягкий режим); F26/F27/F15 ждут зачётного прогона.
- Наблюдения к находкам: `.credo/**` трекается в git (вопреки `AGENTS.md`
  «вне git»); live state — неизвестные поля `blocker`/`route_done`/`re_raise`
  (дрейф до схемы, мягкий режим).
- Следующее действие — `dispatch migrator` (Q89 → D92 + карточка + F83/F84).

## сервисная сессия · 03.10.2026 · правки канона r14 — готово

- **Журнал:** Q89 → D92 (`migrator`); каталоги, `TRACEABILITY` (Q89: D92,
  `in work`, T-15); карточка T-15: `C17` 🚧 + аннотации B1-F26/F27/F15
  («зачёт — по §„Зачётный прогон“»); находки F83 (`.credo/**` трекается
  в git), F84 (неизвестные поля live state).
- **Правка (сервисная сессия):** `state-schema.md` — новый раздел
  «Зачётный прогон»: ядро (ведущий `lead`; нет упоров `steps`; нет
  незапланированных прерываний владельца; нет ручных восстановлений), список
  не-дефектов, машинная проверка, одноразовость и неретроактивность.
- `.credo/sandbox.json` (продуктовые данные) — вне пакета (F83).
- Следующее действие — `auditor` (канон, до коммита).

## auditor · 03.10.2026 · аудит — расхождений нет (P3 ×2 закрыты)

- P1/P2 нет; D92 ↔ факт — ядро/не-дефекты/неретроактивность подтверждены;
  согласованность с `dispatch-loop`/D88/D89/D91/D42 — ок; журнал (Q89↔D92,
  `C17`, F83/F84) и границы чисты; `agents-perms.mjs` — «11 из 18».
- **P3 ×2 (закрыты):** (1) путь `metrics-report.mjs` — разнесён с
  `session-analysis/`; (2) условие 3 — уточнено («`surface_to_user`, стоявший
  в плане участка»); зеркала в D92 — `migrator`.
- Следующее действие — `validator` (адресная + `docs_journal`).

## validator · 03.10.2026 · приёмка — принято (docs_journal 14/0)

- P1/P2/P3 нет; D92 ↔ схема — совпадают (включая закрытые P3: путь
  `metrics-report.mjs`, условие 3); `docs_journal` **14 passed / 0 failed**;
  границы чисты; полный DoD не запускался (D50). Отчёт
  `docs/reviews/service-credited-run-2026-10-03.md`; квитанция
  `service-credited-run` (iteration 1, accepted).
- Следующее действие — `migrator` (`C17` ✅; Q89 остаётся `in work`, T-15 🚧),
  затем гейт пакета → `git`.

## сервисная сессия · 03.10.2026 · пакет r14 — гейт владельца

- **Приёмка:** `validator` — **принято** (`service-credited-run`, iteration 1);
  `docs_journal` 14/0. `C17` ✅; Q89 остаётся `in work` (T-15 🚧, D82);
  F83/F84 открыты.
- **Пакет (снимок `git status`, 12 M + 4 ?? = 16 + запись роли `git` F43):**
  - канон: `.opencode/rules/state-schema.md`;
  - журнал: `docs/questions/Q89.md` (новый),
    `docs/decisions/D92-credited-run-predicate.md` (новый), каталоги
    (`docs/questions/README.md`, `docs/decisions/README.md`),
    `docs/TRACEABILITY.md`;
  - приёмка: `docs/reviews/service-credited-run-2026-10-03.md` (новый),
    `.opencode/state/current/receipts.yaml`;
  - карточка/реестр: `docs/tasks/T-15-mcp-ready-process/README.md`,
    `docs/analysis/findings-registry.md`;
  - ленты/память: `.opencode/mail/service-mcp-ready-r14.md` (новый),
    `.opencode/mail/service-mcp-ready-r13.md` (подхват),
    `.opencode/memory/{auditor,migrator,service,validator}.md`.
- **Вне пакета:** `.credo/sandbox.json` (продуктовые данные; F83).
- Следующее действие — подтверждение владельца → `dispatch git`.

## migrator · 03.10.2026 · готово

- Сделано: заведены Q89 (`docs/questions/Q89.md`, `resolved by D92`) и
  D92 (`docs/decisions/D92-credited-run-predicate.md`) — ядро-предикат зачётного
  прогона (4 условия + список не-дефектов, машинная проверка, неретроактивно);
  `Affects` — `.opencode/rules/state-schema.md` (раздел «Зачётный прогон»),
  `Tasks` — T-15 (`C17`; B1-F26/F27/F15).
- Карточка T-15: строка `C17` 🚧 (service-mcp-ready-r14, 03.10); в пунктах
  B1-F26/F27/F15 добавлено «зачёт — по §„Зачётный прогон“ (`state-schema.md`,
  D92)».
- Реестр находок: добавлены F83 (`.credo/**` трекается в git вопреки
  `AGENTS.md`/`.gitignore`) и F84 (live state — неизвестные схеме поля
  `blocker`/`route_done`/`re_raise`, мягкий режим) — оба открыты.
- Каталоги/связи: `questions/README.md`, `decisions/README.md`, TRACEABILITY
  (Q89: D92, `in work`, T-15 🚧).
- Проверки: ID свободны (Q88/D91 — последние); ссылки резолвятся (D58, D86,
  D89, D91, T-15, findings); `cargo` не запускался (D50). `git diff --check`
  — за ролью `git` (shell вне прав migrator).
- Файлы: Q89, D92, 2 каталога, TRACEABILITY, карточка T-15, findings-registry
  (+ лента r14/память).
- Дальше / риски: правки канона сервисной сессией
  (`.opencode/rules/state-schema.md` — раздел «Зачётный прогон») → `reload` →
  `auditor` → `validator` (адресная + `docs_journal`).

## auditor · 03.10.2026 · готово

- **Аудит:** D92↔факт (4 условия + не-дефекты + машинная проверка +
  неретроактивность), согласованность с `dispatch-loop`/D88/D89/D91/D42,
  журнал Q89↔D92/TRACEABILITY/карточка/F83/F84, границы.
- **Проверки:** `node .opencode/scripts/agents-perms.mjs` — 11/18, фронтматтеры
  не менялись; `git status --porcelain` — только объявленный периметр;
  `git diff --check` — пусто. `validate-state.mjs` — вне прав `auditor`
  (by design), контракт сверен чтением: под предикат правок не требует
  (мета-критерий, новых полей/enum нет; «вердикт — записью» ложится в
  существующий `note`). `cargo` не запускался (D50).
- **Инструкция ↔ права:** расхождений нет.
- **Вердикт:** P1/P2 нет. **P3×2:**
  1. `state-schema.md:181-182` — `metrics-report.mjs` перечислен внутри
     `.opencode/scripts/session-analysis/`, тогда как фактический путь —
     `.opencode/scripts/metrics-report.mjs`. Правка: разнести пути
     (`session-analysis/` и `metrics-report.mjs` — соседние) либо указать
     полный путь. (То же по смыслу — `D92:40-41`.)
  2. `state-schema.md:171-172` (зеркало `D92:33-34`) — условие 3 («каждый
     `owner_response` соответствует гейту плана») тавтологично: инвариант 5
     уже допускает `owner_response` только при `surface_to_user`. Различающий
     признак «гейт был в плане `next[]`» не назван. Правка: одна строка
     «(поверхность присутствовала в `next[]` плана участка)».
- **Ок:** 4 условия ядра совпадают D92↔`state-schema.md`; список не-дефектов
  (плановые гейты, `owner_override`, `-rN`, CCSN, session-commit) — реальные
  понятия `dispatch-loop`/D42/D88/D89/D90; неретроактивность заявлена в решении
  и в схеме; машинная проверка опирается на существующие `progress.yaml` +
  `session-analysis/`/`metrics-report.mjs`; `Affects` D92 = `state-schema.md`
  (Q41, второй канон не заводится); Q89 ⬌ D92 (`resolved by`/`Resolves`),
  `questions/README:111`/`decisions/README:121` в порядке, TRACEABILITY:93
  (D92/`in work`/T-15 🚧), карточка `C17` 🚧 + аннотации B1-F26/F27/F15,
  F83/F84 уникальны и ссылаются на D58/D86/D91; номера строк и адреса
  `docs/analysis/**` (кроме реестра) в каноне/решении отсутствуют.
- **Границы:** изменены только `.opencode/rules/state-schema.md`,
  `docs/{questions,decisions}/**`, `docs/TRACEABILITY.md`, карточка T-15,
  findings-registry, лента/память; `src/**`, `tests/**`, `Cargo.toml`,
  `opencode.json`, иной канон — не тронуты; `.credo/sandbox.json` — вне пакета
  (F83). Фронтматтеры не менялись.
- **Следующие шаги:** `validator` (адресная + `docs_journal`); P3-1/P3-2 —
  на усмотрение сервисной сессии (не блокируют).

## migrator · 03.10.2026 · готово (P3-1/P3-2 аудита r14)

- Сделано: в `docs/decisions/D92-credited-run-predicate.md` устранены два P3
  аудита r14 (зеркала правок схемы; существо решения не менялось):
  1. условие 3 ядра — вместо тавтологического «соответствует гейту плана»
     «соответствует `surface_to_user`, стоявшему в плане участка (`next[]`);
     ответов вне плана и „продолжай“ нет»;
  2. путь инструмента — вместо `metrics-report.mjs` без пути разнесены
     «экспорт сессии (`.opencode/scripts/session-analysis/`) и
     `.opencode/scripts/metrics-report.mjs`».
- Проверки: чтением — diff локализован (Решение п.1.3 и п.3, строки 33–35,
  41–43); формулировки ядра/не-дефектов/неретроактивности сохранены; `cargo`
  не запускался (D50).
- Дальше / риски: `validator` (адресная + `docs_journal`); прочие строки D92
  не трогались.

## validator · 03.10.2026 · до прогона

- Сделано: адресная документная сверка r14 завершена чтением/rg/git —
  D92↔`state-schema.md` §«Зачётный прогон» (ядро/не-дефекты/машинная
  проверка/неретроактивность + F26/F27/F15) совпадают; P3-1 (путь
  `metrics-report.mjs`) и P3-2 (тавтология условия 3 → «стоявшему в плане
  участка (`next[]`)») закрыты и в схеме, и в D92; журнал Q89↔D92, каталоги
  (`README:111`/`:121`), TRACEABILITY:93, карточка `C17` 🚧 :230 + аннотации
  B1-F26/F27/F15, F83/F84 — согласованы. Границы чисты (`--numstat` по
  src/tests/Cargo.toml пусто; `--check` пусто; `.credo/sandbox.json` — вне
  пакета, F83).
- Проверки: чтение/rg/glob/git; ориентир `docs_journal` — 14 passed / 0 failed.
- Дальше / риски: запускаю `cargo test --test docs_journal`; чекпойнт в память
  записан.

## validator · 03.10.2026 · принято

- Сделано: приёмка r14 завершена. Отчёт
  `docs/reviews/service-credited-run-2026-10-03.md`; квитанция
  `service-credited-run` (iteration 1, accepted) — append в
  `.opencode/state/current/receipts.yaml`.
- Проверки: `cargo test --test docs_journal` — **14 passed / 0 failed** (0.06s).
  D92 ↔ `state-schema.md` §«Зачётный прогон» — ядро/не-дефекты/машинная
  проверка/неретроактивность совпадают; P3-1 (путь `metrics-report.mjs`) и
  P3-2 (тавтология условия 3) закрыты. Журнал Q89↔D92, каталоги,
  TRACEABILITY:93, карточка `C17` 🚧 :230 + B1-F26/F27/F15, F83/F84 —
  согласованы. Границы чисты.
- Дальше / риски: P1/P2/P3 нет. Закрытие статусов (`C17` ✅, Q89 → `done`) —
  `migrator`; канон не правил, статусы не менял.

## migrator · 03.10.2026 · готово

- Сделано: закрытие статусов r14 (приёмка `validator` accepted
  `service-credited-run`, iteration 1). Карточка T-15 — строка `C17` (`:230`)
  🚧 → «✅ 03.10 — принято (`service-credited-run`); D92». **`docs/TRACEABILITY.md`
  НЕ переведён**: Q89 (`:93`) остаётся `in work` / `[T-15] 🚧` (T-15 🚧, `done`
  не допускает открытых задач — D82; прецеденты Q83/Q85–Q88). F83/F84 не
  тронуты (открыты).
- Проверки: чтением — diff локализован (`:230`, ровно 1 строка); ссылки живые
  (D92 `docs/decisions/D92-credited-run-predicate.md`, отчёт
  `docs/reviews/service-credited-run-2026-10-03.md`, TRACEABILITY `:93`);
  F83/F84 — `findings-registry.md:95-96` открыты; `cargo` не запускался (D50).
- Дальше / риски: новых Q/D/задач нет; судьба Q89 (перевод в `done`) — вне
  r14 (T-15 🚧). Иных файлов не тронуто.

## git · 03.10.2026 · готово

- Сделано: сервисный пакет r14 — подтверждение владельца сверено по ленте
  (секция «пакет r14 — гейт владельца»); ветки нет, коммит в `develop`.
  База `develop` = `origin/develop` = `HEAD` = `eeac481` (`T-15 r13`).
  Пакет — 17 путей (12 `M` + 4 `??` + запись роли `git` F43); вне пакета —
  `.credo/sandbox.json` (F83). Записи (эта лента + чекпойнт памяти `git.md`) —
  до `add` (F43).
- Проверки (перед `add`): `git status -sb` — 13 M + 4 ?? (13-я M —
  `.credo/sandbox.json`, вне пакета); `git log -1 --oneline` = `eeac481`;
  `git-check.mjs` — ветка `develop` в синхроне с `origin/develop`.
- Дальше / риски: `add` точными путями (17) → `diff --cached --check` (пусто)
  → `diff --cached --name-status` (13 M + 4 A) → коммит `chore(process): T-15
  r14 — предикат «зачётного прогона» (state-schema; D92); приёмка` →
  `push origin develop`. `master` не трогается; после `push` в отслеживаемые
  файлы не писать (хеши — в ответе `lead`, F43).
