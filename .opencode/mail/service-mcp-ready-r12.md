# service-mcp-ready-r12 — лента операции: T-15, C5+C7 (re-raise, самоотчёт/фичи)

Открыта: 03.10.2026. **Сервисная операция** (T-15, фаза C; после r11).
Предмет (решение владельца 03.10.2026, «Только C5+C7»):

- **C5 — re-raise:** объект (`id`, `category`, `origin`, `failed_clause`,
  `fix`, `blocking`, `resolved`) и категории (`expect_mismatch` ·
  `owner_override` · `plan_gap` · `role_failure`); зеркало `replan_reason` в
  `progress`; `owner_override` — вне метрики конвергенции; повтор
  `owner_override` на участке — сигнал к разбору плана (мемо §5.5).
- **C7 — шаблон самоотчёта сессии** (короткий формат сегментного итога) +
  **фичи/счётчики** `docs/features/README.md`: статусы `agents-*` — re-raise ✅,
  state-schema 🟡, session-checkpoint 🟡 (счётчики без изменений; зона
  `docs-writer`).

**Рамка:** канон — `.opencode/rules/dispatch-loop.md`,
`.opencode/rules/state-schema.md`, `.opencode/agents/{analyst,lead}.md`;
фичи — `docs-writer`; журнал — Q87 → D90 + карточка T-15 (`C5`/`C7` 🚧);
протокол: журнал → правки → `auditor` → `validator` (адресная; адресный
`docs_journal` — по решению) → гейт → `git` (develop) → итог.

## сервисная сессия · 03.10.2026 · открытие

- Срез: `develop` = `origin/develop` = `6cd75cf` (r11) + подхват (итог r11,
  память) — в дереве.
- Следующее действие — `dispatch migrator` (Q87 → D90 + карточка).

## сервисная сессия · 03.10.2026 · правки канона C5/C7 — готово

- **Журнал:** Q87 → D90 (`migrator`); каталоги, `TRACEABILITY` (Q87: D90,
  `in work`, T-15); карточка T-15: `C5`/`C7` → 🚧.
- **Правки (сервисная сессия):**
  - `dispatch-loop.md` — новый §«Re-raise» (объект, 4 категории, зеркало
    `replan_reason`, метрика конвергенции без `owner_override`); §«Каденция» —
    шаблон сегментного итога (`Задача/Фаза/Статус/Лента` + `Сделано/Дальше/
    Риски`);
  - `state-schema.md` — `re_raise.category` (enum — ссылка), `replan_reason`
    (enum 4 категории при `action: re-plan`);
  - `analyst.md` — при re-plan фиксировать объект `re_raise` в плане;
  - `lead.md` — запись `re-plan` содержит `replan_reason`.
- **Фичи (`docs-writer`):** `agents-re-raise` ⬜→✅; `agents-state-schema`
  ⬜→🟡; `agents-session-checkpoint` ⬜→🟡 (счётчики без изменений).
- Следующее действие — `auditor` (канон/фичи, до коммита).

## auditor · 03.10.2026 · аудит — расхождений нет

- P1/P2 нет; D90 ↔ факт — C5/C7 подтверждены (7 полей `re_raise`/4 категории,
  зеркало `replan_reason`, шаблон сегментного итога, статусы фич); журнал
  (Q87↔D90, карточка `C5`/`C7` 🚧) и границы чисты; фронтматтеры не менялись
  (reload не требуется); `agents-perms.mjs` — «11 из 18».
- **P3 (наблюдение):** лента r11 перекодирована UTF-16→UTF-8 при catch-up
  (размер 40 304 → 21 423 Б; содержание сохранено) — учесть `validator` при
  гейте.
- Следующее действие — `validator` (адресная + адресный `docs_journal`; второй
  случай практики F74).

## validator · 03.10.2026 · приёмка — rework (P1)

- **P1:** `docs/TRACEABILITY.md:91` (Q87, «Реализация») — вилдкард «(статусы
  `agents-*`)» запрещён тестом `features_are_named_in_traceability_and_exist`
  (D80); нужен поимённый перечень (или снять хвост). Единственное вхождение.
- **docs_journal: 13 passed / 1 failed** — красный тот же тест (F74-практика
  сработала: поймано до develop). Остальное — ок; P3 аудита (перекодировка
  ленты r11) — наблюдение. Отчёт `docs/reviews/service-c5c7-2026-10-03.md`;
  квитанция `rework` (iteration 1).
- Следующее действие — `migrator` (фикс строки) → `validator` `-r2`.

## сервисная сессия · 03.10.2026 · фикс P1 — готово

- `migrator`: `docs/TRACEABILITY.md:91` — вилдкард `agents-*` → поимённый
  перечень (`agents-re-raise`/`agents-state-schema`/`agents-session-checkpoint`);
  `rg "agents-\*"` пусто; диф 1/0; ссылки живые.
- Следующее действие — `validator` `-r2` (адресная + `docs_journal`, ожидание
  14/0; квитанция — iteration 2).

## validator · 03.10.2026 · -r2 — принято

- P1 снят; `docs_journal` **14 passed / 0 failed** (в т.ч.
  `features_are_named_in_traceability_and_exist`); регресс C5/C7 ок; границы
  чисты; полный DoD не запускался (D50). Отчёт
  `docs/reviews/service-c5c7-2026-10-03-r2.md`; квитанция `service-c5c7`
  (iteration 2, accepted).
- Следующее действие — `migrator` (C5/C7 ✅; Q87 — остаётся `in work`, T-15 🚧;
  F61 — пометка), затем гейт пакета → `git`.

## сервисная сессия · 03.10.2026 · пакет r12 — гейт владельца

- Приёмка: `validator` `-r2` — **accepted** (`service-c5c7`, iteration 2;
  `docs_journal` 14/0). `C5`/`C7` ✅; Q87 остаётся `in work` (T-15 🚧); F61 —
  пометка (шаблон внесён; EN — F78).
- **Пакет (снимок `git status`, 17 M + 5 ?? = 22 пути + запись роли `git`
  F43 = 23):**
  - канон: `.opencode/rules/{dispatch-loop,state-schema}.md`,
    `.opencode/agents/{analyst,lead}.md`;
  - журнал: `docs/questions/Q87.md`,
    `docs/decisions/D90-c5-c7-re-raise-selfreport.md`,
    `docs/{questions,decisions}/README.md`, `docs/TRACEABILITY.md`;
  - фичи: `docs/features/README.md`; карточка T-15 + findings;
  - отчёты: `docs/reviews/service-c5c7-2026-10-03.md`,
    `docs/reviews/service-c5c7-2026-10-03-r2.md`;
  - лента r11 (перекодирована, P3 аудита) + лента r12;
  - state/память: `.opencode/state/current/receipts.yaml`,
    `.opencode/memory/{auditor,docs-writer,migrator,service,validator}.md`;
  - + `.opencode/memory/git.md` (запись роли, F43).
- Сообщение: `chore(process): T-15 r12 — C5/C7 (re-raise, самоотчёт/фичи; D90);
  приёмка`.
- База: `develop` = `origin/develop` = `6cd75cf`; ветки нет; master — по
  решению гейта.

## сервисная сессия · 03.10.2026 · гейт пакета — подтверждён

- **question:** owner-gate пакета r12 (23 пути).
- **owner_response (дословно):** «Коммит + push develop (Recommended)».
- Порядок: коммит в `develop` + `push origin develop`; `master` не трогаем.
- Следующее действие — `dispatch git`.

## сервисная сессия · 03.10.2026 · итог r12

- **Коммит:** `7e458fd` — `chore(process): T-15 r12 — C5/C7 (re-raise,
  самоотчёт/фичи; D90); приёмка` (23 пути: 18 M + 5 A; +880/−19); push
  `6cd75cf..7e458fd` → `origin/develop`; ветки/master не тронуты.
- **Итог:** C5 (re-raise: объект/категории/зеркало `replan_reason`) и C7
  (шаблон сегментного итога + статусы фич) внесены (D90); `C5`/`C7` ✅; Q87 —
  `in work` (T-15 🚧); F61 — пометка (шаблон внесён; EN-остаток — F78).
- **Остаток T-15 (закрываемо):** C2 (`validate-state.mjs`), C4+C3-остаток
  (`process/runN`/теги/фасад), C6 (метрики), C8 (кандидаты B0).
- Пост-пакетные записи — подхват.

## migrator · 03.10.2026 · готово

- Сделано: Q87 (`docs/questions/Q87.md`, статус `resolved by D90`) →
  D90 (`docs/decisions/D90-c5-c7-re-raise-selfreport.md`, 2 пункта: C5 re-raise
  объект/категории + `replan_reason`; C7 короткий самоотчёт + статусы фич);
  строки в `questions/README.md`, `decisions/README.md`, TRACEABILITY
  (Q87: D90, `in work`, T-15 🚧); карточка T-15 — `C5`/`C7` ⬜→🚧 (D90).
- Проверки: ID свободны (последние — Q86/D89; Q87/D90 отсутствовали);
  `git diff --check` пусто; ссылки Q87/D90 и каталогов резолвятся (grep/glob):
  T-15, `mcp-ready-process.md` (§5.5/§7), `findings-registry.md` (F58/F61),
  `features/README.md`, D88/D89/D86/D50/D70, `journal.md`. `cargo` не
  запускался (D50; адресный `docs_journal` — `validator`).
- Дальше / риски: правки канона сервисной сессией
  (`.opencode/rules/{dispatch-loop,state-schema}.md`,
  `.opencode/agents/{analyst,lead}.md`) + статусы фич `docs-writer`
  (re-raise ✅, state-schema 🟡, session-checkpoint 🟡) → `reload` → `auditor`
  → `validator` (адресная). Новых находок нет.

## docs-writer · 03.10.2026 · готово

- Сделано: статусы фич `agents-*` в `docs/features/README.md` (D90, C7/C5):
  `agents-re-raise` ⬜→✅ (канон C5: объект `re_raise`, категории, зеркало
  `replan_reason`; `owner_override` вне метрики), `agents-state-schema` ⬜→🟡
  (C1 ✅/D86, C2 ⬜), `agents-session-checkpoint` ⬜→🟡 (C3 инкремент 1/D89 + C9 ✅;
  остаток `process/runN`/теги/фасад) — строки 298–300. `agents-metrics` и
  `agents-mcp-readiness` не тронуты; шапки `.feature` не тронуты.
- Проверки: `git diff -- docs/features/README.md` — ровно три строки (298–300);
  счётчики без изменений (строка 315: 47 файлов / 278 сценариев); пути
  `agents-{re-raise,state-schema,session-checkpoint}.feature` резолвятся
  (glob). `cargo` не запускался (D50).
- Дальше / риски: адресная приёмка `validator` (согласованность с
  `tests/features_inventory.rs`). Открытых вопросов нет.

## auditor · 03.10.2026 · готово

- **Предмет:** служебная операция r12 (T-15, C5+C7) — аудит «инструкция ↔ права»
  и D90↔факт до коммита; канон не правлю.
- Сделано: сверены D90 ↔ записка §5.5/§7 и правки канона
  (`dispatch-loop.md` §«Re-raise»/§«Каденция»; `state-schema.md`;
  `analyst.md`; `lead.md`), журнал Q87↔D90, `TRACEABILITY`, карточка T-15,
  `features/README.md`; границы и фронтматтеры; машинная сверка прав.
- Проверки: `node .opencode/scripts/agents-perms.mjs` — ok (11/18, стабильно
  дважды); `git diff --check` — пусто; фронтматтеры не менялись; чужие файлы
  (`AGENTS.md`, `opencode.json`, `src/**`, `tests/**`, `Cargo.toml`) не тронуты;
  `cargo` не запускался (D50).
- **Вердикт: P1/P2 — нет; расхождений «инструкция ↔ права» нет.**
  - C5: 7 полей объекта `re_raise` и 4 категории совпадают D90↔dispatch-loop↔
    state-schema↔analyst/lead; `owner_override` вне метрики; повтор — сигнал.
  - C7: шаблон сегментного итога в §«Каденция»; фичи `agents-re-raise` ✅,
    `agents-state-schema` 🟡, `agents-session-checkpoint` 🟡 — заметки, счётчики
    47/278 без изменений.
  - Журнал: Q87 `resolved by D90`; TRACEABILITY (Q87→D90, `in work`, T-15 🚧);
    карточка `C5`/`C7` 🚧; ссылки живые; номеров строк нет.
- **P3 (наблюдение, не требует правки):**
  - `.opencode/mail/service-mcp-ready-r11.md` — файл r11 при catch-up
    перекодирован UTF-16→UTF-8 (40 304 → 21 423 Б); контент сохранён
    (добавлена секция «итог r11»), правка в рамках «лента/память». Замечание
    для `validator`: при пакетном подтверждении учесть размер правки.
- Дальше / риски: `validator` (адресная приёмка D50/docs_journal) → гейт →
  `git`. Расхождений, блокирующих коммит, нет.

## validator · 03.10.2026 · rework (P1)

- **Проверка:** адресная документная сверка + адресный
  `cargo test --test docs_journal` (решение владельца 03.10.2026; F74); полный
  DoD не запускался (D50 — `src/**`, `tests/**`, `Cargo.toml` не менялись).
  Версия: `develop` = `origin/develop` @ `6cd75cf` + рабочее дерево.
- **Вердикт: отклонено (rework, P1).**
  - `cargo test --test docs_journal` → **13 passed / 1 failed** (ожидание 14/0).
    Красный: `features_are_named_in_traceability_and_exist`
    (`tests/docs_journal.rs:642-647`) — `realization.contains("agents-*")`.
  - **P1:** `docs/TRACEABILITY.md:91` (Q87, колонка «Реализация») содержит
    «`(статусы `agents-*`)`» — вилдкард-обобщение запрещено (D80).
    **Правка (migrator, одна строка):** поимённый перечень
    `[agents-re-raise.feature](features/agents-re-raise.feature)`,
    `[agents-state-schema.feature](…)`, `[agents-session-checkpoint.feature](…)`
    (как Q74/D74 `:78`) либо снять хвост. Единственное вхождение `agents-*` в
    `TRACEABILITY.md` (`rg`).
- **Проверено и ок:** C5/C7 ↔ D90 (7 полей/4 категории, зеркало
  `replan_reason`, метрика; `state-schema.md` enum-ссылка; `analyst.md:106-107`;
  `lead.md:62-63`; шаблон сегментного итога `dispatch-loop.md:34-36`); журнал
  Q87↔D90, каталоги, карточка `C5`/`C7` 🚧; фичи — `agents-re-raise` ✅,
  `agents-state-schema`/`agents-session-checkpoint` 🟡, счётчики 47/278, шапки
  `.feature` не тронуты; границы (`--numstat -- src tests Cargo.toml` пусто,
  `git diff --check` пусто). P2 нет.
- **P3 (наблюдение):** перекодировка ленты r11 UTF-16→UTF-8 (аудитор) —
  подтверждено, не блокер.
- **Дальше / риски:** правка P1 (`migrator`) → повторная приёмка `-r2`
  (адресный `docs_journal`, ожидание 14/0) → гейт → `git`. Отчёт
  `docs/reviews/service-c5c7-2026-10-03.md`; квитанция `service-c5c7`
  iteration 1, verdict `rework`. Канон и статусы не менялись (закрытие —
  `migrator` после приёмки).

## migrator · 03.10.2026 · готово (фикс P1)

- Сделано: `docs/TRACEABILITY.md:91` (Q87, «Реализация») — вилдкард
  «`(статусы `agents-*`)`» заменён на поимённый перечень (стиль Q74/D78):
  `agents-re-raise.feature`, `agents-state-schema.feature`,
  `agents-session-checkpoint.feature`. Иных строк/файлов не тронуто.
- Проверки: `rg -n "agents-\*" docs/TRACEABILITY.md` — пусто;
  `git diff --numstat -- docs/TRACEABILITY.md` = `1 0` (ровно одна строка),
  `git diff --check` пусто; три `.feature` существуют (glob). `cargo` не
  запускался (D50; адресный `docs_journal` — `validator`, ожидание 14/0).
- Дальше / риски: повторная приёмка `validator` `-r2` (адресный `docs_journal`)
  → гейт → `git`. Новых находок нет.

## validator · 03.10.2026 · `-r2` — до прогона

- **Проверка:** адресная документная сверка + адресный
  `cargo test --test docs_journal` (F74); полный DoD не запускается (D50 —
  `src/**`, `tests/**`, `Cargo.toml` не менялись). Версия: `develop` =
  `origin/develop` @ `6cd75cf` + рабочее дерево.
- **Фикс P1 — подтверждён чтением:** `docs/TRACEABILITY.md:91` — поимённый
  перечень `agents-re-raise`/`agents-state-schema`/`agents-session-checkpoint`;
  `rg "agents-\*" docs/TRACEABILITY.md` — пусто; три `.feature` существуют;
  диф 1/0.
- Границы чисты: `--numstat -- src tests Cargo.toml` пусто; `git diff --check` —
  только предупреждение CRLF ленты r11 (не ошибка).
- Ориентир — 14 passed / 0 failed (в т.ч. `features_are_named_in_traceability_and_exist`).
  Запускаю `cargo test --test docs_journal`.

## validator · 03.10.2026 · `-r2` — принято

- **Проверка:** адресная документная сверка + адресный
  `cargo test --test docs_journal` (F74); полный DoD не запускался (D50).
  Версия: `develop` = `origin/develop` @ `6cd75cf` + рабочее дерево.
- **Вердикт: принято** (P1 снят; P2/P3 нет).
  - **docs_journal: 14 passed / 0 failed** (0.06s) — ожидание выполнено,
    `features_are_named_in_traceability_and_exist` ok.
  - **Фикс P1:** `docs/TRACEABILITY.md:91` — поимённый перечень
    (`agents-re-raise`/`agents-state-schema`/`agents-session-checkpoint`);
    `rg "agents-\*"` пусто; три `.feature` существуют; диф 1/0.
  - **Регресс C5/C7:** `dispatch-loop.md:150-160` §«Re-raise» (7 полей/4
    категории/зеркало/метрика), `state-schema.md:62,:109-110`, `analyst.md:106`,
    `lead.md:62`; §«Каденция» `:34-36` — шаблон; `features/README.md:298-300`
    (✅/🟡/🟡), счётчики 47/278; Q87↔D90, каталоги `:109`/`:119`, карточка
    `C5`/`C7` 🚧 `:216`/`:218`. Границы чисты.
- **P3 (наблюдение):** перекодировка ленты r11 UTF-16→UTF-8 (аудитор) —
  подтверждено; `git diff --check` даёт CRLF-предупреждение для этого файла
  (не ошибка); учесть при гейте пакета.
- **Дальше / риски:** гейт пакета → `git` (develop) → `migrator` (C5/C7 ✅,
  Q87 → `done` + видимая `[T-15] ✅` в TRACEABILITY) → `complete`. Отчёт
  `docs/reviews/service-c5c7-2026-10-03-r2.md`; квитанция `service-c5c7`
  iteration 2, verdict `accepted`. Канон и статусы не менялись.

## migrator · 03.10.2026 · закрытие C5/C7 — готово

- **Статус:** готово. **Тип:** закрытие после приёмки (`service-c5c7`, `-r2`).
- **Запись:** без новых Q/D; закрытие T-15 (`C5`/`C7`) по D90.
- Сделано:
  - карточка `docs/tasks/T-15-mcp-ready-process/README.md:216,:218` —
    `C5`/`C7` → «✅ 03.10 — принято (`service-c5c7`, `-r2`); D90»;
  - `docs/TRACEABILITY.md:91` (Q87) — **не тронута**: `in work` / `[T-15] 🚧`
    (T-15 остаётся 🚧; `done` не допускает открытых задач — D82, прецеденты
    Q83/Q85/Q86);
  - `docs/analysis/findings-registry.md:73` (F61) — пометка «03.10.2026: короткий
    шаблон самоотчёта внесён (C7/D90); EN-префикс обёртки харнесса — остаток
    (F78)», статус «открыт»; иные находки не тронуты.
- Проверки: `git diff -U0` — карточка ровно 2 строки (`C5`/`C7`), findings ровно
  1 строка (F61); `git diff --numstat -- docs/TRACEABILITY.md` = `1 0` (baseline
  untracked r12-пакета; строка Q87 прочитана и совпадает — правок нет);
  `git diff --check` — только CRLF-предупреждение r11-ленты (не мой файл);
  ссылки/ID живые; `cargo` не запускался (D50; адресный `docs_journal` 14/0 —
  `validator`). **Грабля:** baseline карточки был `⬜` (🚧 не был закоммичен) —
  правка сразу на ✅.
- Дальше / риски: гейт пакета → `git` (develop) → `complete`. Новых Q/D/задач
  нет.

## git · 03.10.2026 · пакет r12 — до `add` (F43)

- **Статус:** готово (записи до `add`).
- **База:** `develop` = `origin/develop` = `HEAD` = `6cd75cf`; ветки нет.
- **Подтверждение:** лента, секция «гейт пакета — подтверждён» (`owner_response`
  дословно «Коммит + push develop (Recommended)») — сверено.
- Снимок `git status --porcelain` совпал с пакетом — 17 `M` + 5 `??` = 22
  + запись роли `git` (F43) = 23 (ожидание staged: 18 M + 5 A).
- Следующее: `add` точными путями (23) → `diff --cached --name-status` →
  `commit` → `push origin develop`.


