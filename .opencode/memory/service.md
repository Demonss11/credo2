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
