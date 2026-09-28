# Приёмка `service-agent-tools` (Q56/D51) — 2026-09-29

**Проверка:** пакет инструментов экономии токенов ролей: новый
`.opencode/scripts/agents-perms.mjs` вместо сырого `opencode debug agents` у
`validator`/`auditor`; каноничная форма подсчёта строк `rg -c '^' <файл>`;
журнал Q56/D51; фичи и `AGENTS.md`.

**Версия:** `develop`, HEAD `01a70fd` + рабочее дерево (2026-09-29); 13 M +
4 `??` (`git status --porcelain`).

**Вердикт:** принято с замечаниями (один P3, не блокер).

**P1:** — критичных проблем нет.
**P2:** — нет.
**P3:** `docs/decisions/D51-agent-tools-token-hygiene.md:7-12` (`Affects`) и
`:47-49` («Решение» п. 4) перечисляют канон-правки (`validator.md`,
`auditor.md`, `review.md`, `workspace.md` + скрипт), но не `AGENTS.md`, хотя
строки `AGENTS.md:149-151` («машинная сверка прав … дважды … `agents: 11 из
18`») — прямое следствие D51 (устранение P2-1 аудита). Последствие: трассировка
«решение ↔ артефакты» неполна — при следующем аудите канона происхождение
строки `AGENTS.md` не связано с D51/Q56; правка — добавить `AGENTS.md` в
`Affects` (одна строка, `migrator`).

**Проверки:**

- `node .opencode/scripts/agents-perms.mjs` → `agents: 11 из 18 (--all — все)`;
  `validator` — `node .opencode/scripts/agents-perms.mjs` (+ `*`), сырой
  `opencode debug agents` отсутствует; `cargo test *` присутствует только у
  `validator`; у `auditor` — скрипт + `opencode reload`.
- `node .opencode/scripts/agents-perms.mjs --role validator` → права совпадают с
  фронтматтером `validator.md:7-34`, `steps=36`,
  `model=opencode-go/deepseek-v4.1-flash`.
- `node .opencode/scripts/agents-perms.mjs --grep cargo` → `hits: 22` (включая
  `deny read Cargo.lock` у 9 ролей, `allow edit Cargo.toml` у `coder`,
  `cargo check|fmt|clippy` у `coder`/`rust-expert`/`tester`, `cargo
  fmt|clippy|test` у `validator`); `cargo test *` — одно вхождение, `validator`.
- `node .opencode/scripts/agents-perms.mjs --help` → справка (режимы
  `--role/--grep/--json/--all/--help`).
- `node .opencode/scripts/agents-perms.mjs --json` → JSON-сводка 11 ролей
  (`mode`, `steps`, `model`, `edit`, `shell`).
- `node .opencode/scripts/agents-perms.mjs --all --grep "opencode debug agents"`
  → `hits: 0` (сырой команды нет ни у одной роли, включая встроенные).
- `node .opencode/scripts/agents-perms.mjs --all --grep "opencode reload"` →
  `hits: 1` (`auditor`).
- `node .opencode/scripts/agents-perms.mjs --role nosuchrole` → ошибка
  «роль не найдена» + список ролей (по исходнику — код 2; см. «Технические
  проблемы»).
- `rg -n "debug agents" ./.opencode/rules AGENTS.md opencode.json` → пусто;
  `rg -n "debug agents" docs/features` → пусто; `rg … .opencode/agents` — только
  скрипт (`auditor.md:25-26,152,179`, `validator.md:21-22`).
- Инструкция ↔ права: `review.md:86-97` ↔ фронтматтеры 11 ролей — сходится
  (`rg`, git-наборы, `cargo`-наборы, скрипт, `opencode reload` только
  `auditor`).
- Счётчики фич: `rg -c "^  Сценарий:" docs/features` → 276 (47 файлов),
  `rg -c "Структура сценария:" docs/features` → 2 → **278** =
  `docs/features/README.md:303` («47 файлов, 278 сценариев»); правки фич —
  только текст шагов (`agents-cycle.feature:51`, `agents-audit.feature:17`),
  состав/счётчики не менялись.
- `rg -c "^" .opencode/mail/service-dod-scope.md` → `272` (каноничная форма
  `rg -c '^'` из `workspace.md:24-25` рабочая; совпадает с замером D51:80).
- Журнал: Q56 (`resolved by D51`) ↔ D51 (`Resolves: Q56`, `Спека` §10 №51,
  `Affects`, `Tasks: —`) ↔ `TRACEABILITY.md:27` ↔ `questions/README.md:44` ↔
  `SPECIFICATION.md:872` (§10 №51, одна строка); `OPEN_QUESTIONS.md` не тронут
  (новый Q — верно); ID Q56/D51 уникальны.
- «Сверка с кодом» D51 — ⚪ с фактами-замерами; «Задач не требуется» (D51:84-86).
- `rg -n "1\.9 КБ" docs` → пусто; `rg "3–4 КБ"` → 6 вхождений (`D51:35,53`,
  `Q56:51,63`, `SPEC:872`, `agents-perms.mjs:7`).
- Снимок номеров строк в D51:75-76 помечен «на момент ревизии 29.09.2026, до
  правок D51» — P3 аудитора закрыт.
- Ссылки: `Q56`/`D51`/`TRACEABILITY`/`questions/README` — относительные пути
  живые (`../SPECIFICATION.md`, `../../.opencode/agents/*`, `../../.opencode/
  rules/*`, `../decisions/D44|D49|D50`, `../../.opencode/mail/service-agent-tools.md`).
- Границы: `git diff --stat -- src tests Cargo.toml` → пусто; изменены только
  `docs/**`, `.opencode/**`-текст/память, `AGENTS.md`.

**DoD (cargo не запускался — D50):** правило `review.md:36-39`
(`docs/features/**` — исключение только при правках счётчиков/состава
сценариев). Состав пакета: `src/**`, `tests/**`, `Cargo.toml` не менялись
(`git diff --stat -- src tests Cargo.toml` пусто), счётчики и состав фич не
менялись (278 подтверждено независимым `rg`-подсчётом) — применимо базовое
правило D50: cargo-прогоны (`fmt`/`clippy`/`test`/`features_inventory`) не
выполняются. Эквивалентность инвариантов `features_inventory` подтверждена
адресно: 47 файлов, 276 + 2 = 278, README:303; правки фич — один шаг в строке,
не заголовок `Сценарий:`.

**Технические проблемы:** код выхода скрипта инструментально не подтверждён:
харнесс на неуспешную команду отдаёт «Exited with code 1» и для заведомо иных
кодов (`rg --nosuchflag` → 1 вместо документированного `rg` 2), поэтому
различение 1/2 по выводу невозможно. Коды 0/1/2 проверены по исходнику
(`agents-perms.mjs:48,69,78,121,130,146,159`); смоук-пути `exit 0` видны
(`--help`, сводка, `--role`, `--grep`, `--json`). Сырую `opencode debug agents`
не запускал — права нет по замыслу D51 (проверка «её нет ни у кого» выполнена
скриптом).

**Что проверено и ок:** `agents-perms.mjs` (исходник + все режимы), фронтматтеры
`validator.md`/`auditor.md`, `review.md`, `workspace.md`, `AGENTS.md:149-151`,
`agents-cycle.feature:51`, `agents-audit.feature:17`, счётчики фич, Q56/D51,
`TRACEABILITY.md:27`, `questions/README.md:44`, §10 №51, ссылки, границы
пакета. Аудит `auditor` (машинная сверка, P2-1/P2-2/P3-1/P3-2) подтверждён
независимо; закрытие P2/P3 зафиксировано в ленте `service-agent-tools.md`.
