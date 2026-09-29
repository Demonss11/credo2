# Память: сервисная сессия (контур владельца)

- **Назначение:** сервисные операции без задачи (T-15 B0 «wave 0 фазы B», вне
  канона; MCP-ready, процесс агентов).
- **Канон:** `AGENTS.md` §«Память и почта», §«Сервисные операции»;
  `.opencode/rules/dispatch-loop.md` (hard rules).
- **Правило:** чекпойнт — волна/итерация, выполненные шаги, улики, что осталось;
  кратко, со ссылками.

## Чекпойнты

- 28.09.2026 · T-15 B0: старт (решения владельца — модель проб
  `opencode-go/deepseek-v4.1-flash`; пробы в temp; перенос зелёного —
  `.opencode/plugins` + `.opencode/package.json`; `state/current` не трогаем —
  находка в отчёт). Выполнен **B0-i1**: temp-проект
  `%TEMP%\opencode\wave0b-probe`; форма 1 — `.opencode/plugins/*.ts` + `npm install
  @opencode/plugin` (283 пакета, 60 с), маркер setup 13:53:02; форма 2 — config
  `plugins[]` (каталог), маркер 13:53:48; startup чистый (`failed to load plugin`
  нет). Аномалия: прогон 1 — CLI завис на `/wait`, сессия `succeeded` (kill).
  Улики — `target/wave0b-i1/`; протокол — `wave0b-probes.md`; лента — r3.
  Осталось: **B0-i2…i8** (i2 — Shell Strategy: `instructions` в V2 не загружается,
  контент — рекомендация в C).
- 28.09.2026 · **B0-i2** (Shell Strategy, v1.1.0, MIT): `instructions` в V2 не
  загружается (проба A: `ок`), `AGENTS.md` — рабочий путь (проба B: `ПАНТЕРА-9137`);
  сверка: `git --no-pager` в начале команды конфликтует с префиксными allowlist'ами,
  Linux-части вне pwsh; `GIT_TERMINAL_PROMPT=0` — учесть. Вердикт: 🔴 установка /
  🟢 контент → C. Улики — `target/wave0b-i2/`. Осталось: i3…i8.
- 28.09.2026 · **B0-i3** (Opencode Telemetry, v0.2.0, MIT): V1 Plugin API →
  WARN `failed to load plugin` в V2 («must export a default definition with an id
  and an effect or setup function»); config-пути (абс/отн/файл) не подхватываются;
  `.opencode/plugins/<name>/` грузится только с корневым `index.ts`. CLI `octm`
  работает (Bun 1.4.2), но без плагина пуст. Вердикт 🔴; D — нативные
  `stats`/`session export`. Улики — `target/wave0b-i3/`. Осталось: i4…i8.
- 28.09.2026 · **B0-i4** (Subagent Reporter + Agent Identity, v3.1.1): оба V1 API →
  WARN `failed to load plugin` (default-определение / `@opencode-ai/plugin`);
  нативный stdout субагента — только `✓ … General Agent`; вердикт 🔴×2;
  рекомендации C (`--format json`/свой V2-плагин; своя атрибуция). Улики —
  `target/wave0b-i4/`. Осталось: i5…i8.
- 28.09.2026 · **B0-i5** (CC Safety Net v2.4.11): 🟢 — V2-энтрипоинт работает;
  live: `.env` read BLOCKED (`secret.basename.env`), `git push --force` BLOCKED
  (`git.push-force`), `git status` прошёл; audit JSONL с sessionId (копия в
  `target/wave0b-i5/`); `reset --hard` BLOCKED в репо / ALLOWED в temp;
  `Remove-Item -Recurse -Force` — только paranoid. Перенос — решение владельца +
  auditor. Осталось: i6…i8.
- 28.09.2026 · **B0-i6** (snip): 🔴 — V1 API, CLI отсутствует на Windows (нет
  snip/go/brew); замер экономии невозможен; префикс `snip` конфликтует с
  allowlist'ами ролей. Политика C — B1/token-guard. Улики — `target/wave0b-i6/`.
  Осталось: i7, i8.
- 28.09.2026 · **B0-i7** (Handoff v0.5.0): 🔴 — V1 API + runtime `@opencode-ai/plugin`
  (`src/tools.ts`), без default; проба в temp зафиксировала WARN + повтор ~16 раз;
  данные C: resume по `sessionID` (полный контекст) vs continuation-сводка
  (дешевле) + `session export`. Улики — `target/wave0b-i7/`. Осталось: i8 (отчёт).
- 28.09.2026 · **B0-i8**: отчёт `wave0b-report.md` заполнен; вердикты: 🟢 — CC
  Safety Net (перенос — решение владельца + auditor), 🔴 — Telemetry/Reporter/
  Identity/snip/Handoff, Shell Strategy — 🔴 установка/🟢 контент; рекомендации
  C/D/E/F переданы; коммит-пакет ждёт подтверждения (develop, как c13_records).
  Пробы завершены; канон не тронут; состояние — у владельца.
- 28.09.2026 · решения владельца: **своя обвязка B0-own** (P1 observe, P2 guard,
  P3 checkpoint, P4 metrics, P5 attribution) — после коммита B0 (прототипы в temp,
  перенос после аудита); **перенос CC Safety Net — позже** (пресет standard);
  **S/M-прогон** — после B0-own. Отчёт дополнен (§4 «Своя обвязка», §6 «Решения
  владельца»); реестр — строка `B0-own`. Далее: пакет `wave0b_records` → роль git (✅ коммит `cafc4c4`).
- 28.09.2026 · **BO-i1** (T-15 B0-own, каркас и разведка V2-API): полигон
  `%TEMP%\opencode\wave0b-own` (форма B0; npm — 283 пакета/42 с); startup 🟢 —
  маркер setup 14:49:56Z, релог после атомарного write 14:55:55Z, WARN плагина
  нет (аномалия B0-i1 повторилась: первый CLI-прогон завис — сессия
  `ses_f1781a1b6ffe…` `succeeded`, повтор «ок»); разведка пробой: события
  `session.created` c `parentID`/`agent`/`model` (поток серверный — фильтровать),
  `permission.evaluate` (`action`/`resources`/`effect`), `tool.before/after`,
  `session.context` (`agent`/`model`), `command.list`; нативно — `stats --json`,
  `session export` (`info.agent/model/parentID`). Документы — `wave0b-own.md` +
  шпаргалка `wave0b-own-plugin-guide.md`; улики — `target/wave0b-own-i1/`;
  лента — r4; реестр: B0-i8 ✅, `B0-own` →
  `B0-own-i1` ✅ + `B0-own-P1…P5` ⬜. Осталось: BO-i2 (`wave0-guard`, плагин vs
  `experimental.policies`) — по подтверждению; далее BO-i3…BO-i7; коммит
  документов — пакетом (C13).
- 28.09.2026 · **ревью BO-i1** (сервисная сессия): вердикт подтверждён
  (независимая проверка реестра/памяти/ленты/маркера/улик/сессий по API);
  probe-плагин полигона погашен (`_off/`; журнал 87→123 строки, остановлен
  15:10:57Z, контрольный прогон — без роста); приёмы для
  BO-i2 — «прогрев» (аномалия первого прогона, 2/2) и фильтр журнала по
  `project`/`session`; находка-кандидат F/D — в отчёт BO-i7.
- 28.09.2026 · **service-t11-closeout** (закрытие T-11 + хвосты): `migrator` —
  T-16 (stale-тест, H7; Q12: «выигрывает файл»; `src/mcp.rs:235`), реестр
  (H7→T-16, F6 — частично), пометка D38; `docs-writer` — T-11 ✅ (карточка,
  сводка, примечания с уликами r3 + Run 4/T-03 `-r2`), features (`agents-rework`
  🟡 — остаток D42 → T-15 B1-F15; блок «Пилоты состоялись»), T-15 (зависимость
  снята, H7→T-16), CHANGELOG; `validator` — **принято с P3** (отчёт
  `docs/reviews/T-11-closeout-2026-09-28.md`, квитанция `T-11-closeout`,
  критерий T-11 — 4/4, fmt pass, `features_inventory` 4/4); P3 (легенда
  features) закрыт адресной проверкой. Пакет 17 путей (`develop`,
  `chore(process)`, включая правку владельца `rustfmt.toml`) — ждёт
  подтверждения; записи сформированы до `git add` (C13/F43).
- 28.09.2026 · **service-t11-closeout/r2 — права `validator`** (замечание
  владельца): `git branch --contains` отклонялся движком (паттерна нет) при
  приёмке закрытия T-11. Журнал: Q54 → D49 (migrator); канон вносила сервисная
  сессия — `.opencode/agents/validator.md:28` (`git branch --contains *`,
  read-only) + `review.md:81` (список роли). Аудит `auditor` — «Инструкция ↔
  права: расхождений нет» (P3 про имя отчёта в F44 закрыт); приёмка `validator`
  — **принято** (P1/P2/P3 нет), смоук `git branch --contains 0a5832f` прошёл
  (`* develop`, `exp/agent-cycle-rerun`, `exp/agent-update-t15w0`);
  отчёт `docs/reviews/service-permissions-2026-09-28.md`, квитанция
  `service-permissions`. Уточнение владельца учтено: `git diff *` у роли был и
  есть; отказ `git diff -- .opencode/...` — квик dot-пути, не пробел прав.
  Дальше: пакет `git` (develop ahead 1 после `6d4c840`) по подтверждению.
- 28.09.2026 · **service-canon-hygiene**: (1) фикс **F45** — у `auditor` не было
  `deny execute` (Code Mode: `tools.shell` → «Unknown tool 'shell'»); внесено
  `auditor.md:32` + причина в теле; (2) аудит дублей
  `cargo fmt|clippy|check|test` (`auditor`): каноны — R2 `dispatch-loop.md:43`,
  DoD `AGENTS.md:304-306`, матрица — фронтматтеры + `review.md` (`:79-92`);
  решение владельца «по таблице аудитора», права — без изменений («оставить
  как есть»); сокращение внесено в 8 файлов канона (`AGENTS.md` `:45,303`,
  `review.md` `:49-50,93-94`, `dispatch-loop.md` `:43,45-46`, `validator.md`
  `:39-40,79-80`, `coder.md` `:53-55`, `tester.md` `:51-53`, `rust-expert.md`
  `:61`, `auditor.md` `:84-86`); реестр — F45 «закрыт», F46 новая (migrator);
  аудит `auditor` — «Инструкция ↔ права: расхождений нет»; приёмка
  `service-canon-hygiene` — **принято с P3** (эта запись закрывает P3);
  отчёт `docs/reviews/service-canon-hygiene-2026-09-28.md`, квитанция
  `service-canon-hygiene`; `develop` ahead 1 (`e1e90d4`); дальше — пакет
  `git` по подтверждению.
- 28.09.2026 · **service-migration-q2q3**: перенос блока **Q2+Q3** →
  `D16-dsl-canon-regex-mvp` (`migrator`; `Resolves: Q2, Q3`, сверка с кодом ✅
  соответствует, задач не требуется — дрейф `GRAMMAR.md` §2 покрыт T-14);
  сопутствующие — `docs-writer` (нота Q3 в `features/README`, шапки
  `# D16 (Q2, Q3)` в `parser`/`lexer`/`execution`, запись в CHANGELOG);
  гигиена — T-14 `§7`→`§2` (`migrator`); приёмка `validator` — **принято**
  (P1/P2/P3 нет; `cargo fmt --check` pass, `features_inventory` 4/4, 47/278);
  отчёт `docs/reviews/migration-q2q3-2026-09-28.md`, квитанция
  `service-migration-q2q3`; пакет docs-коммита ждёт подтверждения
  (`develop` @ `491e153`, синхронен с origin).
- 29.09.2026 · **service-migration-q4**: перенос **Q4 → `D17-priority-out-of-mvp`**
  (`migrator`; сверка с кодом ✅ соответствует — `parse_rule` без `Приоритет`,
  `GRAMMAR.md` §4/§5 согласованы; задач не требуется); сопутствующие —
  `docs-writer` (нота Q4 + ссылки Q4/D17, список фич без устаревшего
  `explain.feature`, шапки `# D17 (Q4)` в `parser`/`execution`/`editor`/`lsp`
  + `client_explanation` «(D17/Q36)», запись в CHANGELOG); приёмка `validator`
  — **принято** (P1/P2/P3 нет; `cargo fmt --check` pass, `features_inventory`
  4/4, 47/278); отчёт `docs/reviews/migration-q4-2026-09-29.md`, квитанция
  `service-migration-q4`; пакет docs-коммита ждёт подтверждения
  (`develop` @ `f488085`, ahead 1).
- 29.09.2026 · **service-migration-q5q6**: перенос **Q5 → `D19-statuses-priorities-canon`**
  и **Q6 — попутно** (`migrator`; `Resolves: Q5`, Q6 не в Resolves — правило
  BRIEF §7; сверка ✅ соответствует — SPEC §6.1/§6.2 ссылаются на README,
  две оси меток, `features_inventory.rs`; задач не требуется); сопутствующие —
  `docs-writer` (нота Q5/Q40 в `features/README` со ссылками Q5/D19, запись в
  CHANGELOG; шапки фич не менялись — решение уровня требований); приёмка
  `validator` — **принято** (P1/P2/P3 нет; fmt pass, `features_inventory` 4/4);
  отчёт `docs/reviews/migration-q5q6-2026-09-29.md`, квитанция
  `service-migration-q5q6`; пакет docs-коммита ждёт подтверждения
  (`develop` @ `788aa4c`, синхронен с origin).
- 29.09.2026 · **service-dod-scope**: правило «пакет без изменений `src/**`,
  `tests/**`, `Cargo.toml` — cargo-прогоны (`fmt`/`clippy`/`test`) не
  выполняются; исключение — правки счётчиков/состава сценариев
  `docs/features/**` → адресный `cargo test --test features_inventory`»
  (наблюдение владельца → Q55 → `D50-dod-by-package-scope`; журнал — migrator;
  канон — сервисная сессия: `review.md` §«Порог существенности» (2 места) +
  `validator.md` чек-лист «Документы и агенты»; `BRIEF.md` §5.7 — docs-writer;
  аудит — «расхождений нет», P3 закрыты до приёмки: `Cargo.toml` во всех
  перечнях, запись о правках в ленте; приёмка validator — **принято**
  (P1/P2/P3 нет; cargo не запускался — по новому правилу); отчёт
  `docs/reviews/service-dod-scope-2026-09-29.md`, квитанция
  `service-dod-scope`; пакет ждёт подтверждения).
- 29.09.2026 · **service-agent-tools**: пакет инструментов экономии токенов —
  скрипт `.opencode/scripts/agents-perms.mjs` (сводка прав ≈3–4 КБ вместо
  122 КБ сырого `opencode debug agents`; `--role`/`--grep`/`--json`/`--all`);
  сырой `opencode debug agents` **убран** у `validator`/`auditor` (канон:
  `validator.md`, `auditor.md`, `review.md`, `AGENTS.md` §«Служебная зона»,
  сценарии `agents-cycle`/`agents-audit`); каноничная форма подсчёта строк
  `rg -c '^' <файл>` (`workspace.md`); журнал Q56 → `D51-agent-tools-token-hygiene`
  (migrator); аудит `auditor` — «расхождений нет» + P2 (AGENTS.md/сценарии) и
  P3 — закрыты до приёмки; приёмка `validator` — **принято** (P3
  `AGENTS.md` в D51 закрыт адресно; **cargo не запускался — D50**; смоук
  скрипта прошёл); отчёт `docs/reviews/service-agent-tools-2026-09-29.md`,
  квитанция `service-agent-tools`; пакет ждёт подтверждения.
- 29.09.2026 · **service-migration-q7**: перенос **Q7 → `D52-glossary-terms-canon`**
  (терминологический канон §11, 14 новых статей + 2 уточнения; **решения Q7 в
  §10 не было — добавлена новая строка №52**, BRIEF §7; `migrator`); сверка ⚪
  документная (статьи §11 на месте; расхождение `meta.json` — T-07); сопутствующие
  — `docs-writer` (ссылки `[Q7]→[D52]` в шапке `publish.feature`, запись в
  CHANGELOG); приёмка `validator` — **принято с P3** (указатель Q7 в архиве без
  темы/даты) → P3 закрыт адресно; `cargo` не запускался (D50); отчёт
  `docs/reviews/migration-q7-2026-09-29.md`, квитанция `service-migration-q7`;
  пакет ждёт подтверждения; следующий свободный ID — Q57/D53.
- 29.09.2026 · **service-migration-q8q10q42**: перенос связки **Q8, Q9, Q10,
  Q42 → `D21-core-semantics-v01`** (строка §10 №21: строгие ошибки исполнения
  Q8/Q9; словарь решений и пустое «не сработало» Q10; схема объяснения
  `snake_case` + `condition` Q42; `migrator`, 2 вызова); сверка ✅ соответствует
  (код/тесты `*_q8/q9/q10/q42`; нюанс `errors.feature` — парсер, v0.2); сопутствующие
  — `docs-writer` (шапки `# D21 (Q8–Q10, Q42)` в 5 фичах, CHANGELOG); приёмка
  `validator` — **отклонено (P1)**: 6 битых ссылок на не перенесённые Q11/Q36–Q39
  → возврат `migrator` (текстом «ожидает переноса») → **повторная приёмка
  принято (iteration 2, отчёт `-r2`)**; `cargo` не запускался (D50); квитанция
  `service-migration-q8q10q42` (iteration 1 rework + iteration 2 accepted);
  пакет ждёт подтверждения.
- 29.09.2026 · **service-migration-q11**: перенос **Q11 → `D53-error-messages-language`**
  (язык сообщений: human — русский, machine — латиница; канонические тексты
  REST-ошибок; **своей строки в §10 не было — добавлена новая №53**, BRIEF §7);
  сверка ✅ соответствует (`rest.rs`/`mcp.rs`/тесты; `version_deprecated` —
  известный, T-09); сопутствующие — `docs-writer` (шапки `# D53 (Q11)` в
  `evaluate`/`rest_api`/`rest_auth`/`errors`, CHANGELOG); бонус переноса:
  `Q8.md`/`Q42.md` — пометки «Q11 ожидает переноса» → живые ссылки; приёмка
  `validator` — **принято с P3** (TRACEABILITY: +`errors.feature`) → P3 закрыт
  адресно; `cargo` не запускался (D50); отчёт
  `docs/reviews/migration-q11-2026-09-29.md`, квитанция `service-migration-q11`;
  пакет ждёт подтверждения; следующие ID — Q57/D54.
- 29.09.2026 · микроправка **D53** (§«Следствия»: +`errors.feature` в списке
  шапочных пометок Q11; `migrator`, адресная проверка `validator` — «согласовано,
  замечаний нет») и **push** `origin/develop` по решению владельца: публикация
  накопленных `8f0c9d9` (Q7/D52), `c9193da` (Q8–Q10/Q42/D21), `746f985` (Q11/D53)
  и микроправки.
- 29.09.2026 · **service-migration-q12q15**: старт блока — раздел 3 архива
  (Q12→D54 новая строка №54, Q13→D14 существующая строка №14, Q14→D55 новая,
  Q15→D56 новая; вариант владельца «Q12–Q15, 4 записи»; база `711a3c8`,
  develop = origin). Предв. сверка: Q12 🟡 (T-16/T-08), Q13 🟡 (T-06/T-07),
  Q14 ✅ (`lib.rs:397`), Q15 ⬜/🟡 — нет `credo merge`/CAS (`mcp.rs:337` —
  ручная `git update-ref`), кандидат T-17. Лента
  `service-migration-q12q15`; маршрут migrator (2 вызова) → docs-writer →
  validator → git; `cargo` не запускается (D50).
- 29.09.2026 · **service-migration-q12q15 — закрытие**: Q12–Q15 перенесены
  (D54 🟡 — T-16/T-08/T-01; D14 🟡 — T-06/T-07; D55 ✅ — задач нет;
  D56 🟡 → новая **T-17**, **P2 утверждён владельцем**). Приёмка `validator`:
  отклонено (P2 — Q15 в «ожидает переноса»; P3 — метки блока, T-01 в
  TRACEABILITY) → rework `migrator` → **`-r2`: принято, замечаний нет**
  (отчёты `docs/reviews/migration-q12q15-2026-09-29.md`, `…-r2.md`; квитанция
  `iteration 2`). Коммит **`5a7fb01`** (39 файлов, +1537/−134) в `develop`,
  **без push** (сужение владельца; `origin/develop` = `711a3c8`, ahead 1).
  Следующий блок — Q16–Q19 (остаток раздела 3); ID: Q57, §10 №57.
- 29.09.2026 · **service-migration-q16q19**: старт блока — Q16→D32 (строка
  №32, общая с Q34 — «ожидает переноса»; T-02), Q17→D57 (новая №57),
  Q18→D35 (строка №35; ✅), Q19→D58 (новая №58; ⚪); база `5a7fb01` (ahead 1
  от origin), в пакет войдут 2 незакоммиченные записи Q12–Q15; лента
  `service-migration-q16q19`; маршрут migrator (2 вызова) → docs-writer →
  validator → git; `cargo` не запускается (D50).
- 29.09.2026 · **service-migration-q16q19 — закрытие**: Q16–Q19 перенесены
  (D32 🟡 — T-02; D57 🟡 — T-06/T-17; D35 ✅ — задач нет; D58 ⚪ — задач нет);
  приёмка `validator` — **принято с первой итерации, замечаний нет**
  (отчёт `docs/reviews/migration-q16q19-2026-09-29.md`, квитанция
  `service-migration-q16q19` iteration 1). Коммит **`e141476`** (32 файла,
  +1156/−183) в `develop`, **без push** (сужение владельца;
  `origin/develop` = `711a3c8`, ahead 2: `5a7fb01`, `e141476`).
  **Раздел 3 архива закрыт**; следующий — раздел 4 «REST API» (Q20–Q27);
  ID: Q57, §10 №59.
- 29.09.2026 · **service-decisions-readme**: старт по запросу владельца —
  сводка `docs/decisions/README.md` (зеркально `questions/README.md`; не канон;
  29 D-файлов D14…D58; колонки D/Тема/Решает/Дата/Задачи/Статус); маршрут
  migrator → docs-writer (карта `docs/README.md` + CHANGELOG) → validator → git;
  база `e141476` (ahead 2), в пакет войдут 2 закрывающие записи Q16–Q19;
  `cargo` не запускается (D50).
- 29.09.2026 · **service-decisions-readme — завершено**: сводка
  `docs/decisions/README.md` (29 строк D14–D58; не канон; поддержка —
  `migrator` при каждом новом `Dn`); карта `docs/README.md` (:18–19, :48) и
  `CHANGELOG.md` (:131–136); приёмка `validator` — принято, замечаний нет
  (P3 про природу пропусков закрыт одной правкой до отчёта); коммит
  **`48c5b77`** + **push**: `origin/develop` = `48c5b77` (опубликованы
  `5a7fb01`, `e141476`, `48c5b77`); дерево чистое.
- 29.09.2026 · **service-migration-q20q23**: старт — раздел 4 «REST API», часть 1
  (Q20→D22 №22, Q21→D23 №23, Q22→D26 №26, Q23→D25 №25; все строки §10 уже
  есть, новых номеров нет); ожидания — ✅ «задач не требуется» (REST реализован,
  тексты сверял D53); свипы D53 и §10-маркеров; `migrator` ведёт новые строки
  в `decisions/README.md`; база `48c5b77` (синхрон с origin); `cargo` не
  запускается (D50).
- 29.09.2026 · **service-migration-q20q23 — закрытие**: принято `validator`
  (iteration 1, замечаний нет; отчёт `docs/reviews/migration-q20q23-2026-09-29.md`);
  коммит **`87cbe13`** (32 файла, +1232/−218) + **push**: `origin/develop` =
  `87cbe13`; сводка `decisions/README.md` — 33 строки.
- 29.09.2026 · **service-migration-q24q26 (4б-1 «Границы MVP»)**: старт —
  Q24→D36 (№36), Q25→D37 (№37), Q26→D24 (№24); все «отложено/вне MVP»,
  ожидания ⚪, задач не требуется; фичи batch/client_explanation/
  import_export; следующая связка — 4б-2 «Транспорт и запуск» Q27+Q30
  (D27/D29, кросс разделов 4→5); база `87cbe13` (синхрон); `cargo` не
  запускается (D50).
- 29.09.2026 · **service-migration-q24q26 — закрытие**: принято `validator`
  (iteration 1; отчёт `docs/reviews/migration-q24q26-2026-09-29.md`); коммит
  **`2e0edcb`** (27 файлов, +994/−132) + **push**: `origin/develop` =
  `2e0edcb`; сводка `decisions/README.md` — 36 строк.
- 29.09.2026 · **service-migration-q28q29 («MCP-контракты»)**: старт по выбору
  владельца (вместо 4б-2; смысловая связка) — Q28→D31 (№31, ✅, T-03),
  Q29→D34 (№34, 🟡, T-04 ✅/T-05 ⬜); канон схем — SPEC §4.5 (D34 ссылается,
  не копирует); 4б-2 (Q27+Q30) отложен в очередь; база `2e0edcb` (синхрон);
  `cargo` не запускается (D50).
- 29.09.2026 · **service-migration-q28q29 — закрытие**: приёмка `validator`
  iteration 1 — отклонено (P2: Q18 в «ожидают переноса» в `D34:420`) →
  rework `migrator` → **`-r2`: принято, замечаний нет** (отчёты
  `migration-q28q29-2026-09-29(-r2).md`); попутно снят «дрейф канона» (единый
  адрес полных схем — D34); коммит **`bc76060`** (37 файлов, +1336/−374) +
  **push**: `origin/develop` = `bc76060`; сводка `decisions/README.md` —
  38 строк. Следующая связка в очереди — 4б-2 «Транспорт и запуск» (Q27+Q30).
- 29.09.2026 · **service-handoff**: создан
  `.opencode/mail/service-handoff-2026-09-29.md` — стартовый документ новой
  сервисной сессии (короткий промт-ссылка + полный брифинг: состояние
  `bc76060`, остаток Q27/Q30–Q41, карта §10→D, процесс, первые шаги). Файл
  не закоммичен (после push `bc76060`) — войдёт в следующий пакет или
  отдельным коммитом по решению владельца; ссылка на него — в ответе
  владельцу.
- 29.09.2026 · **service-migration-q27q30 (4б-2 «Транспорт и запуск»)**: старт по
  подтверждению владельца (одно `question`: состав блока + судьба handoff) —
  Q27 → D27 (строка §10 №27, слаг `D27-rest-launch-address`), Q30 → D29
  (№29, слаг `D29-notebook-mcp-transport`); обе строки есть, кросс-ссылки
  Q27↔Q30 сохранить; ожидания ✅/✅ (Q27 — реализация `src/main.rs` = решение;
  Q30 — credo2 уже stdio-MCP, интеграция Notebook вне кода прототипа); свип
  «ожидает переноса» затрагивает `decisions/README.md:18`, `D31:150`,
  `Q12.md`, `Q15.md`, `questions/README.md`, `D54:132`, `D56:120`
  (карта — в ленте). Handoff `service-handoff-2026-09-29.md` + этот чекпойнт —
  **в этот пакет** (решение владельца). Лента `service-migration-q27q30`;
  маршрут migrator (2 вызова) → docs-writer → validator → git; база `bc76060`
  (синхрон с origin); `cargo` не запускается (D50).
- 29.09.2026 · **service-migration-q27q30 — приёмка и пакет**: `validator`
  iteration 1 — **отклонено** (P2: сдвиг «§-раздел ↔ строки» в D29; P3: счёт
  указателей в ленте) → rework `migrator` (D29 + лента) → **`-r2`:
  принято, замечаний нет** (отчёты `migration-q27q30-2026-09-29(-r2).md`).
  Пакет подтверждён владельцем: **коммит без push** («Только коммит»);
  сообщение `docs(D27, D29): перенос Q27, Q30 — транспорт и запуск: адрес и
  флаги REST, sidecar Notebook`; состав — **27 путей** (26 в дереве +
  чекпойнт `memory/git.md` до `add`), сверка staged — за ролью `git`; хеш —
  в ответе `git` (F43; запись результата — после коммита).
- 29.09.2026 · **service-migration-q27q30 — коммит**: **`ce9f8d7`**
  `docs(D27, D29): перенос Q27, Q30 — транспорт и запуск: адрес и флаги REST,
  sidecar Notebook` (27 файлов, +1239/−70; staged 27/27 = 19 `M` + 8 `A`);
  **без push** (выбор владельца): `origin/develop` = `bc76060`, `develop`
  ahead 1. Итог: Q27+Q30 закрыты (D27/D29); сводка — 40 строк; остаток
  архива — Q31–Q41; свободные ID — Q57, §10 №59.
- 29.09.2026 · **service-migration-q31q33 («Notebook-функции»)**: старт по
  выбору владельца — Q31 → D12 (строка №12), Q33 → D30 (№30); обе строки §10
  есть, задач новых не ожидается (Q31 — UI вне кода `credo2`; Q33 —
  существующие T-08/T-09); свип-карта — в ленте `service-migration-q31q33`;
  в пакет войдут незакоммиченные записи результата q27q30 (дозапись ленты +
  этот файл); база `ce9f8d7` (ahead 1 от origin); `cargo` не запускается
  (D50).
- 29.09.2026 · **service-migration-q31q33 — приёмка и пакет**: `validator` —
  **принято с замечаниями** (P1/P2 нет; P3 `D30:7-13` — `Affects` был неполон)
  → P3 закрыт адресно `migrator` (`Affects` = `§2.2, §4.1, §4.2, §7 и §10`),
  закрытие проверено `validator`. Пакет подтверждён владельцем: **коммит +
  push** («Коммит + push»; push публикует `ce9f8d7` и новый коммит); сообщение
  `docs(D12, D30): перенос Q31, Q33 — Notebook-функции: панель чата, единый
  механизм исполнения`; состав — **33 пути** (32 в дереве + чекпойнт
  `memory/git.md` до `add`); хеши — в ответе `git` (F43).
- 29.09.2026 · **service-migration-q32 («Git-контур»)**: старт по выбору
  владельца — Q32 → D28 (строка №28); ожидание 🟡 (путь реестра
  `checks/{name}/{X}/{Y}/{Z}` не реализован — T-06; UI-части — целевое
  состояние Notebook); свип-карта — в ленте `service-migration-q32`; база
  `61d6471` (синхрон с origin, дерево чистое); `cargo` не запускается (D50).
- 29.09.2026 · **service-migration-q32 — приёмка и пакет**: `validator` —
  **принято, замечаний нет** (отчёт `migration-q32-2026-09-29.md`). Пакет
  подтверждён владельцем: **коммит + push**; сообщение
  `docs(D28): перенос Q32 — Git-контур: реестр X/Y/Z и мультиверсионность`;
  состав — **44 пути** (43 в дереве: 39 `M` + 4 `??`; + чекпойнт `memory/git.md`
  до `add`). Итог: Q32 закрыт (D28; сверка 🟡 — путь реестра, T-06 открыта);
  остаток архива — Q34–Q41; свободные ID — Q57, §10 №59. Хеши — в ответе
  `git` (F43).
- 29.09.2026 · **service-migration-q34 («Тесты», особый случай)**: старт по
  выбору владельца — Q34 дополняет существующий D32 (`Resolves: Q16, Q34`),
  нового D-файла нет; ожидание 🟡 (метка ✅, гейт публикации ⬜ — T-02
  открыта; кэш/`tests/*.тест` — Notebook/v0.2); свип-карта — в ленте
  `service-migration-q34`; база `4af0b1f` (синхрон); `cargo` не запускается
  (D50).
- 29.09.2026 · **service-migration-q34 — приёмка и пакет**: `validator` —
  **принято, замечаний нет** (отчёт `migration-q34-2026-09-29.md`; особый
  случай проверен: все 5 пунктов Q34 в D32, `Affects` полон). Пакет подтверждён
  владельцем: **коммит + push**; сообщение `docs(D32): перенос Q34 — тесты: кэш
  прогонов, тест-гейт (расширение D32)`; состав — **27 путей** (26 в дереве;
  + чекпойнт `memory/git.md` до `add`). Итог: Q34 закрыт (D32; гейт публикации —
  T-02 открыта); остаток архива — Q35–Q41; свободные ID — Q57, §10 №59. Хеши —
  в ответе `git` (F43).
- 29.09.2026 · **service-handoff-r2**: создан
  `.opencode/mail/service-handoff-2026-09-29-r2.md` (промт + состояние
  `e0e06f7`, остаток Q35–Q41, связки очереди) — для новой сервисной сессии;
  не закоммичен, уйдёт в следующий пакет.
- 29.09.2026 · **service-migration-q35 («Создание workspace и шаблоны»)**:
  старт по выбору владельца — Q35 → **новая строка §10 №59** (решения в §10
  нет), слаг `D59-workspace-templates`; свип-карта — в ленте
  `service-migration-q35`; пакет подтверждён заранее: **«Коммит + push»**
  (29.09.2026); база `e0e06f7` = `origin/develop` (синхрон; поправка
  29.09.2026 — ранее ошибочно «ahead 1 от origin — q34 без push»); `cargo`
  не запускается (D50).
- 29.09.2026 · **service-migration-q35 — приёмка и пакет**: `validator` —
  **принято, замечаний нет** (отчёт `docs/reviews/migration-q35-2026-09-29.md`;
  все 4 пункта Q35 + «Следствие» в D59; §10 №59 diff +1; архив: блок Q35 →
  указатель (41→5 строк); сводка — 44 строки; свип — субъекты только Q36–Q41).
  Состав пакета — **22 пути** (21 в дереве: 16 `M` + 5 `??`; + чекпойнт
  `memory/git.md` до `add`); коммит + push (владелец), хеши — в ответе `git`
  (F43). Результат: коммит **`681c4b1`** (22 файла, +656/−46); push
  `e0e06f7..681c4b1` — один коммит (origin уже был `e0e06f7`); `develop` =
  `origin/develop`. Ошибочная вводная «ahead 1» исправлена (лента, `memory/git.md`
  и здесь) — вне пакета, по указанию владельца «без коммита».
- 29.09.2026 · **service-migration-q36q38 («Конвейеры + LSP-состав»)**: старт по
  выбору владельца — Q36+Q38 → общая строка §10 **№18**, новый `D18-*`
  (`Resolves: [Q36], [Q38]`); ожидание — документное (LSP — Фаза 2, «задач
  не требуется» подтвердить); свип-карта — в ленте; база `681c4b1` (синхрон;
  в дереве — незакоммиченная поправка q35; включение поправки в пакет —
  решение владельца перед `git`); `cargo` не запускается (D50).
- 29.09.2026 · **service-migration-q36q38 — приёмка и пакет**: `validator` —
  **принято с замечаниями** (отчёт `docs/reviews/migration-q36q38-2026-09-29.md`;
  P3: маркер Q37 в `Q25:15` — закрыт адресно `migrator`, проверен `validator`,
  квитанция `service-migration-q36q38` iteration 1 + `p3_closure`). Пакет
  подтверждён владельцем: **коммит + push**; **поправка q35 — включена**
  (решение владельца «Включить в пакет»). Состав — **31 путь** (26 `M` + 5 `??`;
  чекпойнт `memory/git.md` — дозапись в отслеживаемый файл). База `681c4b1`
  (синхрон с origin). Хеши — в ответе `git` (F43).
- 29.09.2026 · **service-migration-q37 («Реестр полей»)** — старт по выбору
  владельца: Q37 → строка §10 **№33**, новый `D33-*` (`Resolves: [Q37]`);
  свип-карта — в ленте (включая `D18:135`); пакет — **«Коммит + push»**
  (подтверждено заранее); база `b536df0` (синхрон, дерево чистое); `cargo`
  не запускается (D50).
- 29.09.2026 · **service-migration-q37 — приёмка и пакет**: `validator` —
  **принято, замечаний нет** (отчёт `docs/reviews/migration-q37-2026-09-29.md`;
  D33-fields-registry-source; §10 №33 diff 1 строка; сводка — 46 строк; свип —
  субъект только Q39). Пакет подтверждён владельцем заранее: **коммит + push**;
  состав — **25 путей** (24 в дереве: 20 `M` + 4 `??`; + чекпойнт
  `memory/git.md` до `add`). База `b536df0` (синхрон). Хеши — в ответе `git`
  (F43).
