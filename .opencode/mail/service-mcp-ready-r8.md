# service-mcp-ready-r8 — лента операции: T-15, фаза C — C1 (схема состояния)

Открыта: 02.10.2026. **Сервисная операция** (программа T-15, фаза C, задача C1;
после C10/CCSN). Предмет: схема состояния процесса — поля, типы, инварианты,
писатель/читатель; `session_index`; `owner_response` дословно (карточка T-15
§C; записка §5 — проект к утверждению).

**Входы:** [`mcp-ready-process.md`](../../docs/tasks/T-15-mcp-ready-process/mcp-ready-process.md)
§5 (проект схемы), [D42](../../docs/decisions/D42-expect-iteration.md)
(`iteration` — только rework; участок — `progress_marker`), F15 (дрейф
`iteration`; «закрывается C1»), отчёты прогона 02.10
(`docs/analysis/T-15-run-2026-10-02-*`; рекомендации `analyst`/`validator` —
разделить `iteration`/участок), феча `agents-state-schema.feature`, фактический
формат `.opencode/state/current/*.yaml`.

**Рамка:** канон агентов — `.opencode/rules/**` (+ карта в `AGENTS.md`, если
файл новый); журнал — `migrator` (новый `Dn`); `src/**`, `tests/**`,
`Cargo.toml` не трогаются; `cargo` — только `validator` (docs_journal и полный
DoD); протокол канона: журнал → правки → аудит → приёмка → гейт → коммит.

**Хвост clean-logs (в дереве):** 17 удалённых лент + очищенная память ролей —
отдельный процессный коммит (прецедент `9948ebf`) либо подхват в пакет C1;
решение владельца на гейте.

## сервисная сессия · 02.10.2026 · открытие

- Срез: `develop` = `origin/develop` = `c754c97`; дерево — только хвост
  clean-logs; `state/current` — снимок закрытой T-23 (для сервисной операции
  не переинициализируется).
- План: owner-gate (модель `iteration`/участок; место схемы; хвост
  clean-logs) → `migrator` (`Dn` по схеме + карточка C1 🚧) → правки канона
  (схема) → `auditor` → `validator` (полный DoD, docs_journal) → owner-gate
  пакета → `git` → закрытие (F15, статус C1).
- Развилки владельцу — question (см. запись ниже).

## сервисная сессия · 02.10.2026 · гейт открытия — решения владельца

**question (3 вопроса, ответы — по рекомендованным вариантам):**

1. Модель `iteration`/участок → **по D42**: `iteration` — только rework-раунд
   (`rework = iteration − 1`); участок адресуется `progress_marker`/
   `resume_hint`; нового поля (`section`) нет; дрейф ловят схема и будущий
   `validate-state.mjs` (C2).
2. Место схемы → **новый канон `.opencode/rules/state-schema.md`** + ссылка из
   `dispatch-loop.md` + карта репозитория в `AGENTS.md`.
3. Хвост clean-logs → **отдельным коммитом**.

**Пакет очистки (подтверждён владельцем, точка R7; точные пути — по снимку
clean-logs):** коммит `chore(process): очистка логов (mail + memory)`; база
`develop` = `origin/develop` = `c754c97`; ветки не создаются, `master` не
трогается. К коммиту ровно:

- 17 `D` — `.opencode/mail/T-18.md`, `T-21.md`, `T-22.md`,
  `service-branch-align.md`, `service-docs-lifecycle.md`,
  `service-lifecycle-w1.md`, `service-lifecycle-w2-prep.md`,
  `service-mcp-ready-r5.md`, `service-mcp-ready-r6.md`, `service-mcp-ready-r7.md`,
  `service-question-kodaskills.md`, `service-review-links.md`,
  `service-rules-revision-w2.md`, `service-rules-revision.md`,
  `service-statuses-review.md`, `service-t15-run-review.md`,
  `service-traceability-closeout.md`;
- 9 `M` — `.opencode/memory/{analyst,auditor,coder,docs-writer,git,migrator,service,tester,validator}.md`
  + запись роли `git` в `memory/git.md` (F43, до `add`).

**Маршрут C1:** `migrator` (Q83 → D86 + карточка C1 🚧) → правки канона
(`state-schema.md` + ссылки `dispatch-loop.md`/`AGENTS.md`) → `auditor` →
`validator` (полный DoD) → owner-gate пакета → `git` → закрытие (F15, C1 ✅).
Начато: `dispatch git` (очистка) и `dispatch migrator` (журнал).

## сервисная сессия · 02.10.2026 · правки канона C1 — готово

- **Очистка логов** (отдельный коммит): `2f5544c` `chore(process): очистка
  логов (mail + memory)` — 26 путей (17 D + 9 M), push `c754c97..2f5544c` в
  `origin/develop`; ветки/master не тронуты.
- **Журнал:** `migrator` — `docs/questions/Q83.md`; `docs/decisions/D86-state-schema.md`
  (Resolves Q83; Спека `—`, D70; Affects — `state-schema.md`, `dispatch-loop.md`,
  `AGENTS.md`, T-15; Сверка ⚪); строки в каталогах Q83/D86; `TRACEABILITY`
  (Q83: D86, `in work`, T-15 🚧); карточка T-15: строка `C1` ⬜→🚧
  (service-mcp-ready-r8, 02.10). F15 не закрыт (после приёмки, отдельно).
- **Правки канона (сервисная сессия):** создан `.opencode/rules/state-schema.md`
  (4 артефакта: поля/типы/обязательность, писатель/читатель, инварианты,
  `session_index`, `owner_response` дословно, расширение/применимость);
  `dispatch-loop.md` — ссылка на схему (§«Состояние и записи»), `owner_response`
  в §«Владелец: канал и фиксация», `session_index` в §«Память, лента,
  терминология»; `AGENTS.md` — карта (`.opencode/rules/`, `.opencode/state/`);
  роли: `analyst.md` (enum `status` `idle` вместо `done`; `session_index`),
  `lead.md` (`session_index`; `channel`+`owner_response`+`result`).
- **Дальше:** `auditor` (L, до коммита) → `validator` (полный DoD) → гейт
  пакета → `git`.

## сервисная сессия · 02.10.2026 · аудиты и приёмка — готово

- **Аудит №1:** P1 нет; P2-1 (`Affects` D86 без ролей), P2-2 (enum `action`
  без `wait_for_user`), P3 (`idle` без определения) — **закрыты**: D86 дополнен
  (`migrator`), схема правлена (сервисная сессия).
- **Аудит №2 (свежая сессия):** P1/P2/P3 нет; P3-рекомендация (шаблон отчёта
  `lead.md` — `done`→`idle`) закрыта.
- **Приёмка:** `validator` — **accepted**, P1/P2/P3 нет; DoD: `cargo fmt --check`
  pass, `cargo clippy --all-targets -- -D warnings` pass, `cargo test --all`
  **135 passed / 0 failed** (docs_journal 14/14, features_inventory 4/4);
  отчёт `docs/reviews/T-15-c1-2026-10-02.md`; квитанция `T-15-c1` (iteration 1)
  в `receipts.yaml`.
- **Закрытие:** F15 — закрыт (схема D86; чистая проверка инкремента — за первым
  живым rework, B1-F15); карточка T-15: строка C1 ✅ 02.10 (`T-15-c1`).

## сервисная сессия · 02.10.2026 · пакет C1 — гейт владельца

- **Пакет (снимок `git status`, 13 M + 5 ?? = 18 путей + запись роли `git`
  F43 = 19):**
  - `M`: `.opencode/agents/analyst.md`, `.opencode/agents/lead.md`,
    `.opencode/memory/auditor.md`, `.opencode/memory/migrator.md`,
    `.opencode/memory/validator.md`, `.opencode/rules/dispatch-loop.md`,
    `.opencode/state/current/receipts.yaml`, `AGENTS.md`, `docs/TRACEABILITY.md`,
    `docs/analysis/findings-registry.md`, `docs/decisions/README.md`,
    `docs/questions/README.md`, `docs/tasks/T-15-mcp-ready-process/README.md`;
  - `??`: `.opencode/mail/service-mcp-ready-r8.md`,
    `.opencode/rules/state-schema.md`, `docs/decisions/D86-state-schema.md`,
    `docs/questions/Q83.md`, `docs/reviews/T-15-c1-2026-10-02.md`;
  - + `.opencode/memory/git.md` (запись роли, F43).
- Сообщение: `chore(process): T-15 C1 — схема состояния (state-schema.md, D86);
  приёмка`.
- База: `develop` = `origin/develop` = `2f5544c`; ветки нет; master — по
  решению гейта.

## сервисная сессия · 02.10.2026 · гейт пакета — ответ владельца

- **question (owner-gate):** коммит пакета C1 (19 путей) + push.
- **owner_response (дословно):** «не делаем коммит».
- **Статус:** пакет **deferred_by_owner** — `git` не вызывался, коммита нет;
  принятая работа остаётся в дереве; возобновление — новым подтверждением;
  `complete` не исполняется до closeout.

## сервисная сессия · 02.10.2026 · замечания владельца — канон и тесты

- **Читаемость `state-schema.md`:** «180 строк 5+ агентам — лишние токены;
  `lead` — тонкий, `git` — только git». Принято: читатели — `analyst`
  (разделы артефактов + инварианты) и `validator` (при приёмке); `auditor` —
  при аудите схемы; `lead`/`git`/прочие файл не читают. Хранитель канона —
  сервисная сессия (правки) + `auditor` (независимый аудит); машинный
  контроль — `validate-state.mjs` (C2); отдельная роль не вводится.
- **Тесты:** канон — `D50`/`review.md`: пакет без правок
  `src/**`/`tests/**`/`Cargo.toml` — cargo не запускается. Бриф приёмки C1
  ошибочно требовал полный DoD — `validator` прогнал его сверх канона
  (135/0; безвредно, но отход; урок — брифы приёмки по составу пакета).
  Исключение — счётчики `docs/features/**` → адресный `features_inventory`.
- **Правки по замечанию:** `state-schema.md` — сужено «Когда читать»
  (analyst + validator + auditor-по-потребности); `lead.md` — минимум inline
  (номер сессии, поля записи), отсылка к файлу снята.
- **Дальше:** адресная переприёмка (validator, без cargo) → пакет остаётся до
  нового подтверждения коммита.

## сервисная сессия · 02.10.2026 · гейт возобновлён — «коммить»

- **owner_response (дословно, инициировано владельцем):** «коммить» —
  возобновление гейта пакета C1 (после «не делаем коммит»).
- **Пакет (финальный снимок `git status`, 14 M + 6 ?? = 20 путей + запись
  роли `git` F43 = 21):**
  - `M`: `.opencode/agents/analyst.md`, `.opencode/agents/lead.md`,
    `.opencode/memory/auditor.md`, `.opencode/memory/migrator.md`,
    `.opencode/memory/service.md`, `.opencode/memory/validator.md`,
    `.opencode/rules/dispatch-loop.md`, `.opencode/state/current/receipts.yaml`,
    `AGENTS.md`, `docs/TRACEABILITY.md`, `docs/analysis/findings-registry.md`,
    `docs/decisions/README.md`, `docs/questions/README.md`,
    `docs/tasks/T-15-mcp-ready-process/README.md`;
  - `??`: `.opencode/mail/service-mcp-ready-r8.md`,
    `.opencode/rules/state-schema.md`, `docs/decisions/D86-state-schema.md`,
    `docs/questions/Q83.md`, `docs/reviews/T-15-c1-2026-10-02.md`,
    `docs/reviews/T-15-c1-2026-10-02-r2.md`;
  - + `.opencode/memory/git.md` (запись роли, F43).
- Сообщение: `chore(process): T-15 C1 — схема состояния (state-schema.md, D86);
  приёмка`.
- База: `develop` = `origin/develop` = `2f5544c`; ветки нет; master не трогаем.
- Следующее действие — `dispatch git`.

## сервисная сессия · 02.10.2026 · итог C1

- **Коммит:** `d414998` — `chore(process): T-15 C1 — схема состояния
  (state-schema.md, D86); приёмка` (21 путь: 15 M + 6 A; +1063/−20); push
  `2f5544c..d414998` → `origin/develop`; ветки/master не тронуты; дерево
  чистое, `develop` = `origin/develop`.
- **Итог:** схема состояния канонизирована (`.opencode/rules/state-schema.md`,
  D86/Q83): поля/типы/инварианты, писатель/читатель, `session_index`,
  `owner_response`; читатели — `analyst` + `validator` (+`auditor` по
  потребности); F15 закрыт; карточка C1 ✅; приёмка `T-15-c1` (переприёмка
  r2 — адресная, без cargo по D50).
- **Дальше по T-15 (выбор владельца):** C2 (`validate-state.mjs`), C3
  (session-commit), C5/C6 (re-raise/метрики), чистый S-прогон (F26/F27).
- Запись итога — пост-пакетная (F43): подхват следующим плановым пакетом.

## migrator · 02.10.2026 · готово (Q83 → D86 + C1 🚧)

- Сделано: [Q83](../../docs/questions/Q83.md) (`resolved by` [D86](../../docs/decisions/D86-state-schema.md))
  — схема состояния процесса: место и модель (F15); [D86](../../docs/decisions/D86-state-schema.md)
  — отдельный канон, модель D42; карточка T-15, строка C1 ⬜ → 🚧.
- Проверки: `git diff --check` пусто; `git status` — ровно ожидаемые файлы
  (+ чужая untracked лента r8); относительные ссылки живые; ID Q83/D86 свободны
  (Q82/D85 — последние). `cargo` не запускался (D50).
- Сверка D86: ⚪ (процесс/документы; продуктовый код не затронут).
- Дальше / риски: правки канона (`state-schema.md` + ссылки
  `dispatch-loop.md`/`AGENTS.md`) — сервисная сессия; далее `auditor` →
  `validator` (полный DoD). F15 **не** закрыт — после приёмки C1, отдельно.

## migrator · 02.10.2026 · готово (P2-1 аудита: Affects D86)

- Сделано: в `docs/decisions/D86-state-schema.md` поле `Affects` дополнено
  `.opencode/agents/analyst.md`, `.opencode/agents/lead.md` (P2-1: оба изменены
  пакетом C1 — enum `status` `done`→`idle` + `session_index` у `analyst`;
  `session_index`/`owner_response` у `lead`).
- Проверки: `git diff --check` пусто; D86 ещё untracked (`??`) → `git diff --`
  по нему пуст, правка подтверждена чтением (строки 8–11). `cargo` не
  запускался (D50).
- Дальше / риски: Q83 и статусы не трогались; новых записей нет. P2-2
  (`wait_for_user` в схеме) и P3 — вне моего периметра (канон / сервисная
  сессия).

## git · 02.10.2026 · в работе (записи до `add`, F43)

- Сделано: прочитаны память, правило, лента; пакет сверен со снимком
  `git status --porcelain` — ровно 17 `D` `.opencode/mail/` + 9 `M`
  `.opencode/memory/` + untracked лента r8 (в пакет не входит); база
  `HEAD` = `develop` = `origin/develop` = `c754c97`. Чекпойнт памяти
  (`memory/git.md`) и эта запись — до `add` (F43), входят в коммит.
- Проверки: `git status --porcelain` → 17 D + 9 M + 1 `??`; `git status -sb` →
  `## develop...origin/develop` (без расхождения); `git log -1 --oneline` →
  `c754c97`.
- Дальше / риски: `add` точными путями → `diff --cached --name-status`
  (ожидание ровно 17 D + 9 M) → `commit -m "chore(process): очистка логов
  (mail + memory)"` → `push origin develop`. Расхождение staged — стоп и вопрос.
  Хеши будут в ответе `lead` (после push в отслеживаемые файлы не пишу).

## auditor · 02.10.2026 · C1 — аудит канона (L, до коммита)

- **Проверено:** состав/границы (`git status` — ровно 9 `M` + 4 `??` из ожидания;
  `git diff -- src tests Cargo.toml` пусто); полнота `state-schema.md` против
  карточки T-15 §C и фечи `agents-state-schema.feature`; противоречия
  D42/D50/`dispatch-loop.md`/`journal.md`; журнал Q83↔D86 (парность, `Resolves`,
  каталоги, `TRACEABILITY` Q83: D86/`in work`/T-15 🚧; номера строк и адреса
  `.opencode/mail|state` в Q/D — нет; ссылки живые); «инструкция ↔ права»;
  память `migrator`; F15 (не закрыт — корректно).
- **Проверки:** `node .opencode/scripts/agents-perms.mjs` — ok (agents: 11 из 18);
  список команд совпадает с `review.md` §«Доступные команды»; фронтматтеры не
  менялись. Технический факт: `read` больших файлов (T-15 README, `progress.yaml`,
  `receipts.yaml`, `TRACEABILITY.md`) срезан `token-guard`; решение — по
  важным строкам/точечным срезам, отчёт не затронут.
- **Инструкция ↔ права:** расхождений нет. `analyst` (next_action/current_state,
  `session_index`) и `lead` (`progress.yaml`, `session_index`/`owner_response`) —
  права в YAML покрывают; новые команды не вводились.
- **P1:** нет.
- **P2:**
  - `docs/decisions/D86-state-schema.md:8-11` — `Affects` не содержит
    `.opencode/agents/analyst.md` и `.opencode/agents/lead.md`, хотя оба правлены
    этим пакетом (тела: enum `done`→`idle` + `session_index` у `analyst`;
    `session_index`/`owner_response` у `lead`; лента r8:85-86). Прецедент —
    D41/D43/D44/D45/D48/D49 перечисляют `.opencode/agents/*.md` в `Affects`.
    Ожидание/факт: добавить оба файла → иначе неполная трассируемость правки
    канона.
  - `.opencode/rules/state-schema.md:97` — enum `action` в `progress` (dispatch ·
    surface_to_user · re-plan · complete) не содержит `wait_for_user`, хотя
    `dispatch-loop.md:24` объявляет его действием очереди, а каденция
    (`dispatch-loop.md:31`) требует записи после каждого действия. Ожидание/факт:
    включить `wait_for_user` либо явно оговорить, что запись не делается → иначе
    C2 (`validate-state.mjs`) отметит легитимную запись, а `lead` не знает,
    фиксировать ли остановку.
- **P3:**
  - `.opencode/rules/state-schema.md:48` — значение `idle` в `next_action.status`
    введено без определения/триггера: нет в `dispatch-loop.md`, в D86 и в
    практике (`rg "status: idle" .opencode` — пусто). Ожидание/факт: описать
    условие или убрать (правка — одна строка).
- **Вердикт:** до коммита нужны правки P2 (Affects D86; `wait_for_user` в enum
  схемы); P3 — по желанию. После правок — свежий аудит.
- **Дальше:** находки переданы сервисной сессии (канон не правлю); при внесении
  правок — повторный аудит, затем `validator` (полный DoD).

## validator · 02.10.2026 · C1 — приёмка, до прогона (R5)

- **Проверено чтением:** состав пакета (`git status --porcelain` — 10 `M` +
  4 `??`, ровно по снимку; `src/`, `tests/`, `Cargo.toml` не тронуты);
  `state-schema.md` (4 артефакта, поля/типы/обязательность, писатель/читатель,
  инварианты, `session_index`, `owner_response` дословно; `action` включает
  `wait_for_user`); `dispatch-loop.md:96-97,154-155` ссылается на схему;
  D86 `Affects:8-12` включает `analyst.md`/`lead.md`; Q83:3 `resolved by D86`;
  каталоги (`questions/README:105`, `decisions/README:115`); `TRACEABILITY:87`
  (Q83: D86 / `in work` / T-15 🚧); карточка T-15 `C1 🚧` (:204); `AGENTS.md`
  карта (:24, :27); `analyst.md:64` enum `status`; `lead.md:88` — `idle`
  (P3-1 аудита закрыт).
- **Запускаю:** `cargo fmt --check`; `cargo clippy --all-targets -- -D warnings`;
  `cargo test --all` (R2/D50). Ориентир: 135 passed / 0 failed; docs_journal
  14/14, features_inventory 4/4.
- **Дальше:** результат прогона и вердикт — отдельной записью.

## validator · 02.10.2026 · C1 — приёмка: принято

- **DoD (R2/D50):** `cargo fmt --check` pass; `cargo clippy --all-targets --
  -D warnings` pass; `cargo test --all` — **135 passed / 0 failed**
  (docs_journal 14/14; features_inventory 4/4). Сверка состава: `git status` —
  ровно 10 `M` + 4 `??`; `git diff --numstat -- src tests Cargo.toml` пусто;
  `git diff --check` пусто.
- **Адресно:** `state-schema.md` покрывает C1 (4 артефакта, поля/типы/
  обязательность, писатель/читатель, инварианты, `session_index`,
  `owner_response` дословно), не противоречит D42 (`iteration` — rework;
  участок — `progress_marker`/`resume_hint`; поля `section` нет) и
  `dispatch-loop.md` (канал/`owner_response`, авторитет `receipts`); феча
  `agents-state-schema.feature` покрыта; Q83↔D86 парны, каталоги и
  `TRACEABILITY` синхронны (Q83: D86 / `in work` / T-15 🚧); карточка C1 🚧;
  роли `analyst.md`/`lead.md` синхронны; карта `AGENTS.md`; адресов
  `файл:строка` и `.opencode/mail|state/**` нет (D65/D85); ссылки живые.
  P3 аудитов закрыты: `state-schema.md:48` определяет `idle`;
  `lead.md:88` — `in_progress / awaiting_user / blocked / idle`.
- **Вердикт:** принято (P1/P2/P3 нет). Отчёт:
  `docs/reviews/T-15-c1-2026-10-02.md`; квитанция `T-15-c1` в
  `receipts.yaml`. F15 не закрыт (зона `migrator` после приёмки).
- **Дальше:** owner-gate пакета → `git` (коммит C1) → `migrator` (закрытие
  F15, C1 ✅).

## auditor · 02.10.2026 · C1 — повторный аудит (P2/P3 после правок)

- **Проверено:** закрытие P2-1/P2-2/P3; состав пакета C1 (`git status --porcelain`,
  `git diff --numstat`, полный `git diff`); `src/tests/Cargo.toml` не тронуты;
  регресс к D42/`dispatch-loop.md`/журналу; живые ссылки; отсутствие номеров строк.
- **P2-1 — закрыт.** `docs/decisions/D86-state-schema.md:8-12` — `Affects`
  содержит `.opencode/agents/analyst.md`, `.opencode/agents/lead.md` (плюс
  `state-schema.md`, `dispatch-loop.md`, `AGENTS.md`, `T-15`) — совпадает с
  фактически изменённым набором C1. D86 untracked → исторического diff нет,
  сверка чтением; иных расхождений в D86 не видно.
- **P2-2 — закрыт.** `.opencode/rules/state-schema.md:97` — enum `action`
  (`dispatch · surface_to_user · wait_for_user · re-plan · complete`) совпадает с
  `dispatch-loop.md:24` и с `next[].kind` (схема:55). Легитимная запись остановки
  больше не отвергается.
- **P3 — закрыт.** `state-schema.md:48` — `idle` определён («активной работы
  нет»); `analyst.md:64` enum `status` синхронизирован (`in_progress |
  awaiting_user | blocked | idle`).
- **Регресс — чисто.** Состав: 10 `M` (8 правок C1 + `memory/migrator.md` —
  запись P2-1 + `memory/auditor.md` — чекпойнт первого аудита) + 4 `??`
  (`state-schema.md`, `D86`, `Q83`, лента r8). `git diff -- src tests Cargo.toml`
  пусто (`git diff --numstat` без строк по этим путям); `git diff --check` пусто;
  адресов `файл:строка` в `state-schema.md`/D86 нет (журнал §7); ссылки
  `state-schema.md`↔`dispatch-loop.md`↔`AGENTS.md` живые; Q83↔D86 парны,
  `TRACEABILITY` Q83: D86/`in work`/T-15 🚧; F15 не закрыт — корректно.
- **Новое — P3 (одна строка):** `.opencode/agents/lead.md:88` — шаблон отчёта
  пользователю перечисляет статус `done`, тогда как схема переименовала это
  состояние в `idle` (`state-schema.md:48`, `analyst.md:64`). Последствие:
  пользователь увидит не-энумный ярлык; правка — `done` → `idle` или снять
  перечень. Не блокирует.
- **Проверки:** `node .opencode/scripts/agents-perms.mjs` — ok (agents: 11 из 18,
  дважды не требовалось — состав полный); список команд совпадает с `review.md`
  §«Доступные команды»; фронтматтеры не менялись. Срезов `token-guard` в этом
  аудите не было.
- **Инструкция ↔ права:** расхождений нет.
- **Вердикт:** P1/P2 расхождений нет; P2-1 и P2-2 закрыты; P3 (первый) закрыт;
  единственная новая P3 — `lead.md:88` (`done`→`idle`), по желанию до коммита.
  Канон не правил.

## migrator · 02.10.2026 · закрытие после приёмки C1 (`T-15-c1`)

- Сделано: [F15](../../docs/analysis/findings-registry.md) — статус «открыт» →
  «закрыт 02.10.2026» (C1: канон `.opencode/rules/state-schema.md` (D86);
  `iteration` = только rework, участок — `progress_marker`/`resume_hint`; дрейф
  ловят схема и `validate-state.mjs` (C2); чистая проверка инкремента `1→2`
  (B1-F15) — за первым живым rework); карточка T-15 — строка `C1` 🚧 → ✅
  (`T-15-c1`, схема `state-schema.md`), пометка B1-F15 уточнена (F15 закрыт C1,
  предмет — чистая механика `1→2`).
- Проверки: `git diff --check` пусто; `git diff` — ровно две целевые правки
  (F15 — 1 строка; карточка — `C1` + пометка B1-F15); ссылки живые; номеров
  строк нет. `cargo` не запускался (D50).
- Дальше / риски: F-записей не заведено, новых Q/D нет, `docs/features/**` не
  тронуты (статусы фич — C7).

## validator · 02.10.2026 · T-15/C1 — адресная переприёмка (r2): принято

- **Проверка:** адресная переприёмка после правок канона по замечаниям
  владельца (порог `review.md` §«Возврат на доработку»: канон/права → новый
  отчёт + машинная сверка прав + адресные проверки). Пакет не закоммичен
  (`deferred_by_owner`). **`cargo` не запускался** (D50 — `src/**`,
  `tests/**`, `Cargo.toml` не тронуты); предыдущий полный прогон C1 (135/0) —
  сверх канона, отход брифа зафиксирован в ленте.
- **Дельта (проверено чтением):** `state-schema.md:3-7` — «Когда читать»
  сужено до `analyst` + `validator` (+`auditor` по потребности схемы);
  `lead`/`git`/прочие не читают. `lead.md` — отсылка к схеме снята,
  добавлен inline-минимум: `:54-57` `session_index` (старт с 1, новый
  запуск +1, resume сохраняет), `:69-71` `channel`/`owner_response`
  (дословно)/`result`. В силе: `state-schema.md:98` `wait_for_user` в enum
  `action`; `:49` `idle` определён; D86:11 `Affects` с ролями
  `analyst.md`/`lead.md`; `lead.md:90` шаблон статуса `idle`.
- **Проверки:** `node .opencode/scripts/agents-perms.mjs` — ok, **11 из 18**,
  список команд = `review.md` §«Доступные команды» (frontmatter не менялся →
  `reload` не вызывался, F65); `git status --porcelain` — состав пакета +
  `M memory/service.md` (чекпойнт сервисной сессии) + `??` лента r8;
  `git diff --numstat -- src tests Cargo.toml` пусто; `git diff --check`
  пусто; `rg state-schema .opencode/agents` — только `analyst.md:67,70`;
  адресов `файл:строка` в `state-schema.md`/D86/Q83 нет; ссылки живые;
  Q83↔D86 парны, `TRACEABILITY` Q83: D86 / `in work` / T-15 🚧.
- **Вердикт:** принято (P1/P2/P3 нет). Отчёт
  `docs/reviews/T-15-c1-2026-10-02-r2.md`; квитанция `T-15-c1` (iteration 1,
  report `-r2`) в `receipts.yaml`.
- **Дальше / риски:** пакет остаётся до нового подтверждения коммита; F15/C1
  статусы не менялись (закрыты).

## git · 02.10.2026 · пакет C1 — чекпойнт (до add)

- Сделано: подтверждение владельца сверено (лента, «гейт возобновлён — „коммить"»,
  `owner_response` «коммить»); снимок `git status --porcelain` совпал с пакетом —
  14 `M` + 6 `??`; база `develop` = `origin/develop` = `2f5544c`; ветки нет,
  `master` не трогается. Запись памяти `git.md` (F43) и этот отчёт — до `add`.
- Проверки: `git status --porcelain` → 14 M + 6 ??; `git log -1 --oneline develop`
  → `2f5544c`; `git rev-parse --abbrev-ref HEAD` → `develop`.
- Дальше / риски: `add` точными путями → `diff --cached --name-status`
  (ожидание 15 M + 6 A) → `commit` →
  `push origin develop`; сообщение
  `chore(process): T-15 C1 — схема состояния (state-schema.md, D86); приёмка`.
