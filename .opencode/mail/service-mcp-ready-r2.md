# Служебная лента service-mcp-ready — том 2

- **Назначение:** продолжение ленты
  [`service-mcp-ready.md`](service-mcp-ready.md) (том 1, 546 строк — порог
  ~300+ по правилу `dispatch-loop.md` §«Память, лента, терминология»).
  Формат записей — как в `AGENTS.md` §«Память и почта» (append).
- **Открыт:** 2026-09-27 (T-15, W0-i4, дисциплина C волны 0).
- **Том 1:** W0-i1…W0-i3 и записи до 2026-09-27 включительно.

## lead · 2026-09-27 · W0-i4 — замер, дисциплина C, отчёт волны (готово)

- **Дисциплина C (ленты):** длины — `T-03.md` 589, `T-12.md` 407 (обе ✅
  закрыты), `T-13.md` 234, `service-mcp-ready.md` 546 (активная). По правилу
  (300+ → `T-XX-rN.md` со ссылкой) закрытые ленты не трогаем (история);
  для активной служебной ленты заведён том 2 — этот файл, указатель добавлен
  в том 1.
- **Тяжёлые операции i2–i4 (фиксация для замера):** `opencode api get
  /openapi.json` 51 369 Б → срез 39 365 Б; webfetch доков плагинов 57 371 →
  45 371; skill opencode 12 725 → 725; крупные `rg`/`read`/`glob` — срезы;
  `opencode.log` (18,8 МБ) — только точечные `rg -m`, без полного чтения;
  session export и зонды — мелкие.
- **Штатные счётчики OpenCode** (`opencode stats --project .`): сегодня
  (Sep 27): 15 сессий, 62 подагента, 61 промпт, 1 491 шаг, $3,59; токены
  in 13,15 M / out 576 K / reasoning 1,23 M / cache read 182,2 M; инструменты
  2 319 вызовов, 96,5% успех (read 734, shell 606, edit 439, grep 185,
  glob 118, write 96, subagent 72, webfetch 33). За 3 дня: 30/126 сессий,
  3 147 шагов, $9,23; lifetime: 73 сессии, 5 920 шагов, $14,46. Ограничение:
  суб-дневное окно «до/после» штатно недоступно (`from/to` в 2.0.18 через
  `opencode api` не применились — проверено тремя окнами); per-session
  usage — `opencode session export <id>`.
- **Счётчики плагина B1** (на 20:38:50Z; улика
  `target/wave0-i4-plugin-stats.json`): **11 срезов / 156 831 Б** — shell
  3/51 835, webfetch 1/45 371, grep 2/25 878, read 2/23 453, glob 2/9 569,
  skill 1/725. B2: 0 удалённых ключей (MCP-ключей в конфиге репозитория нет —
  правило-страховка).
- **Сопоставимый сценарий (зонды, один вызов):** композиция запроса — база
  (12 ключей) ≈12,6–13,7 КБ схем, system ≈29,7–31,1 КБ (каталог 29 207 Б);
  docs-writer/git — 8 ключей 7 216 Б; analyst — 7/6 775 Б; deny `execute` —
  system −9 323 Б (каталог исчезает) и −1 105 Б схемы; `codemode:false` —
  9 `credo_*` ≈1 509 Б, deny `credo_check_create` −1 схема (207 Б),
  плагин-префикс `credo*` −9 схем. Токены usage: repo base 12 006 → deny
  11 562 → exec 9 407; iso open 6 667 → deny 6 584 → plugin 4 255 (cache
  read 0/0/1 792 — счётчик провайдера, дельты — ориентир).
- **Отчёт:** `docs/tasks/T-15-mcp-ready-process/wave0-report.md`; статусы
  карточки и README T-15 обновлены; чекбоксы W0-i4 в плане закрыты.
- **Остаток / риски:** решение «канон/пилот/откат» — за владельцем (вопрос
  задан); коммит пакета волны и push — после решения; полная боевая проверка
  среза `cargo test` — на первом тяжёлом прогоне (`validator`); канон не
  тронут.

## lead · 2026-09-27 · W0-i4 — решение владельца (вопросы закрыты)

- Ответы (`question`, 2026-09-27): **«Оставить пилотом, фаза C по кандидатам»**
  и **«Да, коммит без push»**.
- Значит: A/B1/B2 остаются включёнными как есть (B2 — страховка); статические
  deny (`execute`/`subagent`/`question`), сплит `AGENTS.md` и права — отдельным
  канон-пакетом фазы C с аудитом; откат волны не выполняется.
- Коммит пакета `w0_i3_i4` (8 путей: `token-guard.ts`, ленты, память, план,
  карточка, README, отчёт) в `exp/agent-update-t15w0` — dispatch `git`,
  без push.

## lead · 2026-09-27 · W0-i4 закрыт — коммит `57abbbe` (без push)

- `dispatch git` (пакет `w0_i3_i4`): `expect` совпал — коммит **`57abbbe`**
  «feat(plugin): wave 0 W0-i3+i4 — B2 tool-схемы, замер и отчёт волны»
  (8 файлов, +538/−29) в `exp/agent-update-t15w0`; ветка впереди `origin` на 2;
  push не выполнялся (по решению владельца); проверки — `git log -2`,
  `git status -sb`, `git show --stat HEAD` (сверено).
- Итог волны 0: A/B1/B2 — **пилот** (решение владельца); отчёт —
  `docs/tasks/T-15-mcp-ready-process/wave0-report.md`; фаза C — отдельным
  канон-пакетом по кандидатам отчёта §4 (deny `execute`/`subagent`/`question`,
  сплит `AGENTS.md`, права; аудит обязателен).
- Остаток: push ветки — отдельным решением; рабочая «грязь» R7 (эта запись +
  чекпойнт `memory/git.md`) — следующим пакетом.

## lead · 2026-09-27 · Run 5 — выбран первый тяжёлый прогон: T-04 «MCP-ошибки»

- Решение владельца (`question`, 2026-09-27): первый тяжёлый прогон —
  **T-04** (P1, M-пилот из карточки T-15: конверт `isError` + 10 кодов §4.5).
- Обоснование: без зависимостей; разблокирует T-02 (код `publish_failed`) и
  цепочку T-05; сопоставим с Run 4 (T-03) для метрик волны; `validator`
  прогонит `cargo test` → закрывается боевая проверка B1 (срез ~12 КБ).
- **Развилка перед стартом:** волна 0 (плагин B1/B2, `formatter`) живёт только
  на `exp/agent-update-t15w0`; `develop` (@ `3eb0856`) её не содержит —
  для Run 5 с пилотом нужно решить базу ветки (вопрос владельцу задан).
  `exp` — потомок `develop` (merge возможен без конфликтов).
- **Ответ владельца** (`question`, 2026-09-27): «Локальный merge exp→develop» —
  сначала локальный merge `--no-ff` (без push), затем стандартный старт
  `feature/T-04-…` от `develop`. Пакет `w0_merge` (исполняет `git`): коммит R7
  на `exp` (лента + память) → `git switch develop` → `git merge --no-ff
  exp/agent-update-t15w0`; **push не выполнять**; далее — планирование T-04
  (`analyst`) и стандартный старт ветки задачи отдельным подтверждением.

## lead · 2026-09-28 · Run 5 закрыт: T-04 (L) — коммит `3545afa`, merge `cd845d4`, develop отправлен (волна 0 опубликована)

- Итог: приёмка accepted (`docs/reviews/T-04-2026-09-28.md`, 107/0;
  `features_inventory` 4/4, 47/278), аудит пройден (P1 нет; P2 — право
  `auditor` на запись в ленту — в меморандум Run 5), статусы ✅, CHANGELOG.
  Ветка `feature/T-04-mcp-errors` удалена local+origin; `develop` в синхроне с
  origin (`3eb0856..cd845d4`, включая коммиты волны 0 — публикация разрешена
  владельцем).
- B1 (боевая проверка): вывод `cargo test` ≈7 КБ — ниже порога 12 КБ (не
  срезан); срез подтверждён на diff `src/mcp.rs` ≈21 КБ. Волна 0 — пилот
  (решение владельца 2026-09-27).
- Дальше: меморандум Run 5 (находки: P2; ошибочное требование плана
  `delete_draft`; два обрыва `tester` по лимиту шагов; B1-факт); решение
  владельца по P2 (правка канона); процессные записи — отдельным пакетом.

## lead · 2026-09-28 · меморандум W9 (Run 5) подготовлен; директивы владельца

- Владелец: `.opencode/.gitignore` обновлён (убран `state/` → `.opencode/state/**`
  трекается); подготовить меморандум Run 5; **по P2 — делаем
  `edit .opencode/mail/**`** (право `auditor` на запись в ленту).
- Меморандум: [`docs/analysis/memorandum-W9-run5.md`](../../docs/analysis/memorandum-W9-run5.md)
  — F15–F21 (F16=P2, F17 бриф↔канон, F18 steps, F19 `git branch -vv`,
  F20 `rustfmt.toml` edition, F21 синхронизация канона про `state/**`),
  готовые правки §4, драфты Q/D §5, чек-лист Run 5 (§3.1), сравнительная
  таблица (§3).
- Дальше: пакет правок канона по протоколу (migrator → правки → auditor →
  validator → коммит) — объём уточняется у владельца.

## lead · 2026-09-28 · решение владельца: правки канона отложены; push записей Run 5

- Ответ владельца (`question`): **«откладываем реализацию. пушим в origin»** —
  пакет правок F16–F21 (меморандум W9 §4) сейчас не применяется; текущий пакет —
  процессные записи Run 5 (ленты, память — в задаче), **меморандум W9** и
  **первый снапшот `.opencode/state/**`** (`.gitignore` обновлён владельцем) →
  коммит и `push origin develop`.
- Меморандум W9: статус «внедрение отложено» (правка в файле).
- Пакет `run5_records` (исполняет `git`): 8 путей — `.opencode/.gitignore`,
  `mail/T-04.md`, `mail/service-mcp-ready-r2.md`, `state/current/*.yaml` (4),
  `docs/analysis/memorandum-W9-run5.md`; коммит + push, без merge/удаления.

## lead · 2026-09-28 · улика: схема вызовов T-04 (Run 5)

- Создан файл
  [`docs/analysis/T-04-call-graph-2026-09-28.md`](../../docs/analysis/T-04-call-graph-2026-09-28.md) —
  фактическая карта вызовов/переходов Run 5: схема (до прогона → старт →
  реализация → закрытие), таблица вызовов, числа (17 вызовов субагентов в 14
  сессиях; 2 гейта владельца; resume `coder` 1×, `tester` 2×), точки ветвления.
- Остаток: файл + эта запись — рабочая «грязь», следующим пакетом; канон не
  тронут.

## lead · 2026-09-28 · session-analysis: переход корень → субагентские

- В утилиту `.opencode/scripts/session-analysis/` добавлен
  **`find-subagents.mjs`**: по корневой сессии (ID или JSON) находит дочерние —
  читает маркер `<subagent sessionID="ses_…" state="…">` в результатах вызова
  `subagent`; выводит таблицу (агент, статус, индекс сообщения, описание,
  `(×N)` при resume-вызовах) или `--json`; `--export` — выгружает транскрипты
  всех детей, `--fresh` — переэкспорт родителя; предупреждает, если передан
  транскрипт дочерней сессии (показывает `parentID`).
- README утилиты обновлён: шаг 0 «Быстрого старта», новый раздел «Как выйти из
  корневой сессии в субагентские», строка в таблице файлов, «Грабли».
- Проверено на корневой сессии Run 5 `ses_f1b8398d2ffe4VC7LhwgnYggwl`:
  **19 вызовов / 16 дочерних сессий**; `--json` и `--export` (16 JSON в
  `%TEMP%\opencode\sessions`) — ок; режимы `--help`/ошибки — коды 0/2.
- Файл лежит в пока untracked-папке утилиты (пакет другой сессии) — коммит по
  общему порядку; канон не тронут.

## git · 2026-09-27 · готово · пакет w0_i3_i4

- Сделано: коммит **`57abbbe`** `feat(plugin): wave 0 W0-i3+i4 — B2
  tool-схемы, замер и отчёт волны` (8 файлов, +538/−29) в ветке
  `exp/agent-update-t15w0`; push **не** выполнялся (вне пакета).
- Проверки: перед — `git status -sb` + `git log -1 --oneline`: 6 `M` + 2 `??`,
  ровно 8 путей, HEAD `fb8d619`; `git add` восемью точными путями →
  `git diff --cached --name-only` = 8 путей; после — `git log -2 --oneline` →
  `57abbbe`, `fb8d619`; `git status -sb` → `ahead 2`, дерево чистое;
  `git show --stat HEAD` → состав совпал.
- Дальше / риски: push ветки — на отдельный подтверждённый пакет; рабочая
  «грязь» (этот отчёт + чекпойнт в `git.md`) не закоммичена (R7). Заметка:
  включены lead-записи W0-i4 (лента т. 2 + решения владельца) — коммит
  фиксирует волну 0 целиком; предупреждения LF→CRLF — только EOL.

## git · 2026-09-28 · готово · пакет w0_merge

- Сделано: коммит R7 **`ae2b62d`** `chore: служебные записи W0-i4 (лента,
  память)` (2 файла, +67) в `exp/agent-update-t15w0`; `git switch develop`
  (HEAD `3eb0856`); локальный `git merge --no-ff exp/agent-update-t15w0` →
  merge-коммит **`0437799`** «Слияние exp/agent-update-t15w0 (волна 0 T-15)
  в develop» (10 файлов, +1139/−5). Push **не** выполнялся (вне пакета).
- Проверки: до — `git status -sb` + `git log -1 --oneline`: ровно 2 `M`
  (лента r2, память), ahead 2, HEAD `57abbbe`; `git add` двумя путями →
  `git diff --cached --name-only` = 2 пути; после merge — `git log -3
  --oneline` → `0437799`, `ae2b62d`, `57abbbe`; `git status -sb` →
  `## develop...origin/develop [ahead 7]`, дерево чистое; `git show --stat
  HEAD` → состав: `plugins/token-guard.ts`, `wave0-plan/report/smoke/
  token-hygiene`, README T-15, `opencode.json`, ленты т. 1/т. 2, память.
- Дальше / риски: push `develop`/`exp` — отдельным решением владельца; эта
  запись + чекпойнт `memory/git.md` — рабочая «грязь» (R7-r2, ровно 2 пути)
  на `develop`, не закоммичены (служебный коммит — по решению `lead`); далее
  `analyst` (T-04) и старт `feature/T-04-…` от `develop` — отдельным
  подтверждением. `--force`/rebase/`add -A`/`branch -d`/`tag` не применялись.

## git · 2026-09-28 · готово · пакет run5_records

- Сделано: `git add` 8 точными путями (`./` для скрытых; предупреждения
  LF→CRLF — EOL, не отклонение) → индекс сверен (`git diff --cached --name-only`
  = 8 путей) → коммит **`a7eac82`** `chore: записи прогона Run 5 (T-04) +
  меморандум W9; state-снапшот` (8 файлов, +884/−2) → `git push origin develop`
  (`cd845d4..a7eac82`, timeout 360000 мс).
- Проверки: до — `git status -sb` (3 `M` + 2 `??`, `state/` — ровно 4 файла) +
  `git log -1 --oneline` → HEAD `cd845d4` (снимок совпал); после —
  `git log -2 --oneline` → `a7eac82`, `cd845d4`; `git status -sb` →
  `## develop...origin/develop` (синхрон, дерево чистое); `git show --stat HEAD`
  → состав совпал (`.gitignore` 3, `T-04.md` 19, лента r2 40, 4 `state/*.yaml`
  34/117/255/65, меморандум 353).
- Дальше / риски: `--force`/rebase/`add -A`/merge/`branch -d`/`tag`/иные ветки
  не применялись; `target/`, `.credo/`, `node_modules/` не коммитились.
  Эта запись + чекпойнт `memory/git.md` — рабочая «грязь» на `develop`, не
  закоммичены (вне пакета `run5_records`). Осталось: `complete` (lead).

## migrator · 2026-09-28 · готово · журнал Q47–Q53/D42–D48 + реестр F15–F39

- Сделано: новые записи журнала по W8 т. 2 — Q47–Q53/D42–D48
  (`docs/questions/Q47..Q53.md`; `docs/decisions/D42-expect-iteration.md`,
  `D43-auditor-mail.md`, `D44-run5-refinements.md`, `D45-wave0-quality-config.md`,
  `D46-product-process-commits.md`, `D47-git-refinements-run5.md`,
  `D48-findings-registry-owner.md`); реестр `docs/analysis/findings-registry.md`
  дополнен F15–F39; `docs/TRACEABILITY.md` — строки Q47–Q53;
  `docs/SPECIFICATION.md` §10 — решения №42–48.
- Проверки: все D — сверка ⚪ (процесс/права/конфиг; продукт не менялся);
  подтверждения — чтение канона (`dispatch-loop.md`, `AGENTS.md`,
  `git-workflow.md`, роли) и `rights-probe-2026-09-28.md`; право на реестр видно
  в фронтматтере `migrator.md` (F39/D48), запись в реестр после правки успешна.
  `cargo` не запускался.
- Дальше / риски: Tasks у всех записей — «—» (канон внесён сервисной сессией;
  карточка T-15 (F26) — `docs-writer`). Пакет журнала — на аудит `auditor` и
  приёмку `validator`, затем коммит `git` (одна запись — один коммит).
  F15/F26/F27 остаются открытыми по реестру.

## validator · 2026-09-28 · чекпойнт (до прогона) · приёмка W8-config

- Сделано: разобран конфиг-пакет качества (меморандум W8 т. 2 §4.6). Прочитаны
  `rustfmt.toml` (`edition="2024"`, `max_width=80`, `tab_spaces=4`,
  `match_block_trailing_comma`, `merge_derives`), `rust-toolchain.toml`
  (`channel="1.96.0"`, components rustfmt/clippy/rust-src/rust-analyzer),
  `.cargo/config.toml` (`[target.'cfg(all())'] rustflags=["-D","warnings"]`),
  `.gitattributes` (`* text=auto eol=lf`), `opencode.json`.
- Проверки: `rg --files -g clippy.toml` — пусто (файла нет); `rg whitelist
  opencode.json` — нет совпадений; `git diff opencode.json` — удалён только
  legacy-блок `provider.opencode-go.whitelist` (−4 строки). `git log --oneline -3`
  → HEAD `a7eac82`; `git status -sb`/`git diff --stat`: ровно 10 файлов
  `src/**`+`tests/**` (реформат: core 93, lib 134, main 4, mcp 97, rest 23,
  common/mod 52, features_inventory 28, mcp_draft 6, mcp_errors 14, rest 41) —
  выборочно просмотрены диффы main/mcp_draft/mcp_errors/features_inventory:
  только переносы строк/переупорядочивание импортов; смоук-файл
  `target/fmt-smoke-2026-09-28.rs` прочитан (`rg`): `use serde_json::{Value,
  json};` — порядок style edition 2024. Рабочее дерево содержит также
  канон/журнал/меморандум — вне скоупа пакета.
- Дальше / риски: запускаю полный DoD (`cargo fmt --check`, `cargo clippy
  --all-targets -- -D warnings`, `cargo test --all`), ожидание 107/0,
  `features_inventory` 4/4 (47/278). Это тяжёлая операция — отчёт с вердиктом
  будет дописан сюда после прогона.

## validator · 2026-09-28 · принято · W8-config

- Сделано: полный DoD конфиг-пакета качества (меморандум §4.6, шаг 5 §6) на
  снимке `develop` HEAD `a7eac82` + рабочее дерево. Вердикт **принято**
  (P1/P2/P3 нет); отчёт `docs/reviews/W8-config-2026-09-28.md`; квитанция
  append в `.opencode/state/current/receipts.yaml` (`task: W8-config`,
  `iteration: 1`, `verdict: accepted`).
- Проверки: `cargo fmt --check` — pass; `cargo clippy --all-targets --
  -D warnings` — pass (`Finished dev` 1m14s); `cargo test --all` — **107/0**
  (lib 54 + features_inventory 4 + mcp_draft 18 + mcp_errors 8 + publish 12 +
  rest 11). `features_inventory` 4/4; `docs/features/README.md` — 47 файлов /
  278 сценариев. `clippy.toml` отсутствует (`rg --files -g clippy.toml` пусто;
  `git log -- clippy.toml` пусто — в истории не было); `rg whitelist
  opencode.json` — нет, `git diff opencode.json` — удалён только
  `provider.opencode-go.whitelist` (−4). Реформат `src/**`+`tests/**` — ровно
  10 файлов (+335/−157), выборочные диффы — только переносы строк/импорты/
  trailing-comma в `match`. Вывод `cargo test` (~7 КБ) не срезан token-guard.
- Дальше / риски: смоук-файл `target/fmt-smoke-2026-09-28.rs` прочитан
  точечным `rg` — `use serde_json::{Value, json};` (style edition 2024),
  согласуется с §4.6 (полные права на `target/**` — у сервисной сессии).
  `git rev-parse`/`git ls-files` — вне allowlist, заменены `git log`. Пакет
  принят, коммита не делал; рабочее дерево содержит канон/журнал вне скоупа —
  их аудит за `auditor`. F15/F26/F27 открыты.

## auditor · 2026-09-28 · готово · аудит канон-пакета W8-т2 (Run 5)

- Сделано: независимый аудит канон-пакета 28.09.2026 (меморандум §4.1–4.5,
  4.7–4.16, §6 шаг 6) на снимке `develop` HEAD `a7eac82` + рабочее дерево.
  Проверено: фронтматтеры ↔ `review.md` §«Доступные команды», зоны записи
  (YAML ↔ `AGENTS.md` ↔ тело), микропроверка «память ↔ канон» (F35), новые
  правила `dispatch-loop`/`git-workflow`/`AGENTS.md`/`review.md`, дубли и
  ссылки; улики F38 и смоук deny `execute` приняты как внешние (не
  переисполнялись). P1 нет; P2 нет; P3 — три (см. ответ сессии).
- Проверки: `opencode reload` → `opencode debug agents` ×2 — состав свежий и
  стабилен: `coder.steps 52`, `tester.steps 36`, `auditor.steps 32`,
  `git.steps 28`, `lead.steps 16`, `analyst.steps 20`; `edit .opencode/mail/**`
  — у всех 11 ролей, `edit docs/analysis/findings-registry.md` — у `migrator`,
  `execute: deny` — ровно у `docs-writer` и `git`, `git branch -vv` — allow у
  `git`; `B2_PREFIXES = {}` в `token-guard.ts`. Вывод `debug agents` ~51 КБ
  срезан token-guard; полный текст прочитан из сохранённого файла вывода.
- Дальше / риски: канон-пакет к коммиту по существу готов; закрыть три P3
  (правки — сервисная сессия) перед приёмкой `validator`. Вердикт: инструкция ↔
  права — одно расхождение (`.opencode/agents/auditor.md:158`); остальное
  совпадает. F35-дрейфа памяти не найдено.

## auditor · 2026-09-28 · готово · аудит r2 конфиг-пакета (три P3 закрыты)

- Сделано: повторный аудит после закрытия трёх P3 r1 (меморандум §4.1–4.16).
  Проверено ровно: (1) `auditor.md` п.6 — headless как процедура
  владельца/сервисной сессии, без предписания роли `opencode run` и без
  дублирования модели; (2) снятие дублей эскалации в `coder.md` (п.3),
  `tester.md` (п.5), `AGENTS.md` §«Лимиты шагов» — короткие ссылки на
  `dispatch-loop.md` §«Hard rules», канон правила — только там; (3) `analyst.md`
  — точечный `deny` на `docs/analysis/findings-registry.md` после
  `allow docs/analysis/**`. P1/P2 нет; P3 нет.
- Проверки: `opencode reload` → `opencode debug agents` ×2 — состав стабилен,
  11 ролей (`coder 52`, `tester 36`, `auditor 32`, `git/migrator 28`, `lead 16`,
  `analyst/docs-writer/researcher 20`, `rust-expert 24`, `validator 36`);
  `analyst` → `docs/analysis/findings-registry.md` `deny`, `migrator` → тот же
  путь `allow` (обе прогонки); порядок last-match-wins подтверждён. Вывод
  `debug agents` ~51 КБ срезан token-guard — улика в сохранённом файле вывода.
  Якорь `dispatch-loop.md` §«Hard rules» существует.
- Дальше / риски: правок нет (канон не тронут); вердикт — расхождений нет;
  пакет к приёмке `validator` готов; коммит — за сервисной сессией.

## validator · 2026-09-28 · чекпойнт (W8-canon, до прогона)

- Взято в приёмку: канон-пакет 28.09.2026 (шаг 6 §6 меморандума W8 т. 2) на
  снимке `develop` HEAD `a7eac82` + рабочее дерево. Прочитаны канон/журнал/
  процессные доки (состав — в памяти роли). **Машинно:** `opencode reload` —
  отказ (вне прав `validator`); `opencode debug agents` ×2 — идентичны,
  11 ролей, `coder 52`, `tester 36`, права (`auditor` mail, `migrator` реестр,
  `analyst` deny реестра, `docs-writer`/`git` deny `execute`, `git branch -vv`)
  на месте. `git diff --stat -- src tests` — те же 10 файлов (+335/−157), что
  в принятом W8-config: новых правок кода нет.
- Тяжёлая операция: запускаю `cargo test --all` (страховочный DoD) — результат
  допишу сюда; затем — отчёт `docs/reviews/W8-canon-2026-09-28.md` и квитанция
  `W8-canon` в `receipts.yaml`.

## validator · 2026-09-28 · принято · W8-canon

- Сделано: процессная приёмка канон-пакета 28.09.2026 (шаг 6 §6 меморандума
  W8 т. 2) на снимке `develop` HEAD `a7eac82` + рабочее дерево. Вердикт
  **принято** (P1/P2/P3 нет); отчёт `docs/reviews/W8-canon-2026-09-28.md`;
  квитанция append в `.opencode/state/current/receipts.yaml` (`task: W8-canon`,
  `iteration: 1`, `verdict: accepted`).
- Проверки: `opencode debug agents` ×2 — выводы идентичны (51378 Б), 11 ролей;
  `coder.steps 52`, `tester.steps 36`, `auditor.steps 32`, `git/migrator 28`,
  `lead 16`; `edit .opencode/mail/**` ×11; `findings-registry.md` deny у
  `analyst` / allow у `migrator`; `execute: deny` — только `docs-writer`/`git`;
  `git branch -vv` у `git`. `cargo test --all` — **107/0**, `features_inventory`
  4/4 (47/278); `git diff --stat -- src tests` — те же 10 файлов (+335/−157),
  что в W8-config. Журнал Q47–Q53/D42–D48 (поля, ⚪, SPEC §10 №42–48,
  TRACEABILITY), реестр F15–F42, ссылки и Q41 (эскалация — только
  `dispatch-loop` §Hard rules; `state/**` — 4 согласованных текста) — ок.
- Дальше / риски: пакет к коммиту — за `git` (после подтверждения владельцем);
  F15/F26/F27 открыты. Техническое: `opencode reload` — вне прав `validator`
  (отказ), заменено `debug agents` ×2. Файлы вне зон записи не правились;
  коммитов нет.

## сервисная сессия · 2026-09-28 · подтверждение владельца: пакет коммитов W8 (4 пакета, в develop)

- Владелец подтвердил (2026-09-28, сессия сервисного контура): **4 пакета** —
  A `chore(style)`, B `chore(agents)`, C `docs(Q47–Q53)`, D `chore(process)`;
  дамп `docs/analysis/opencode_debug_agents.txt` — **удалить** (выполнено);
  коммиты — **прямо в `develop`** (исключение из git-flow, решение владельца);
  `push origin develop`; шаг 7 (`docs-writer`) выполнен до коммита — его правки
  входят в пакет D.
- Основание: приёмки `W8-config`/`W8-canon` (accepted), аудит `auditor`
  (расхождений нет), меморандум `memorandum-W8-run5.md` §6 (шаги 5–7).
- Точные списки путей — в брифе роли `git` (перечень по `git status --porcelain`
  на 2026-09-28).

## сервисная сессия · 2026-09-28 · подтверждение владельца: пакет w8_records (в develop)

- Владелец подтвердил (2026-09-28): процессный коммит трёх файлов —
  `.opencode/mail/service-mcp-ready-r2.md`, `.opencode/memory/git.md`,
  `docs/analysis/memorandum-W8-run5.md`; прямо в `develop`; `push origin develop`.
  Шаг 9 (следующий прогон S/M) — отложен; после пакета — доработки.

## git · 2026-09-28 · готово · пакет W8 — 4 коммита в develop + push

- Сделано: 4 коммита **прямо в `develop`** (исключение из git-flow, решение
  владельца; подтверждение — запись «сервисная сессия · 2026-09-28» выше):
  A **`22f7683`** `chore(style): rustfmt 80 + edition 2024; конфиги качества;
  реформат src/tests` (15 файлов, +369/−161); B **`c0acfeb`** `chore(agents):
  канон Run 5 — права, эскалация, R2, state, коммит-политика (D42–D48)`
  (15, +3947/−55); C **`a2424b6`** `docs(Q47–Q53): решения Run 5 (D42–D48)`
  (16, +884); D **`b1e1bbb`** `chore(process): записи W8/Run 5 + state (шаг 7,
  аудит, приёмки)` (33, +4452/−378). `git push origin develop`:
  **`a7eac82..b1e1bbb`**.
- Проверки: снимок `git status --porcelain` до работы = 79 путей ровно по
  пакету (лишнего/недостающего нет); пред-стейджнутый `git mv`-перенос
  `memorandum-W9-run5.md → memorandum-W8-run5.md` (`R100`) снят из индекса
  (`git restore --staged`, дерево не тронуто) и добавлен в D;
  `git diff --cached --name-only` перед каждым коммитом = 15/15/16/33;
  `git log --oneline -6` — A–D на месте; `git status -sb` →
  `## develop...origin/develop` (синхрон, дерево чистое); `git show --stat HEAD`
  = 33 файла (перенос записан как create+delete).
- Дальше / риски: `--force`/rebase/`add -A`/merge/ветки/теги не применялись;
  `target/`, `.credo/`, `node_modules/` не коммитились. Эта запись и чекпойнт
  `memory/git.md` — рабочая «грязь» после пакета (не закоммичены).
- Заметка: перенос сделан `git mv` (стейджится сразу) — в статусе `RM`; пакет
  `git` с переносом снимает пред-стейдж `git restore --staged` до первого
  коммита.

## git · 2026-09-28 · готово · пакет w8_records — коммит в develop + push

- Сделано (шаг 8 §6 меморандума W8, пакет `w8_records`): подтверждение владельца
  — лента, запись «сервисная сессия · 2026-09-28 · подтверждение владельца: пакет
  w8_records (в develop)». Снимок `git status --porcelain` = ровно 3 пути,
  совпал с пакетом; ветка `develop` (`## develop...origin/develop`, синхрон).
  `git add` точными путями (LF→CRLF — EOL, не отклонение) → `git diff --cached
  --name-only` = 3 → коммит **`779018c6bc149d06087f5832dc883576ba889737`**
  `chore(process): записи пакета w8_commits` (`779018c`, 3 файла, +57/−5) →
  `git push origin develop` (**`b1e1bbb..779018c`**, timeout 360000 мс).
- Проверки: `git log --oneline -3` → `779018c`, `b1e1bbb`, `a2424b6`;
  `git status -sb` → `## develop...origin/develop` (синхрон, дерево чистое);
  `git show --stat HEAD` = 3 файла (`mail/service-mcp-ready-r2.md`,
  `memory/git.md`, `docs/analysis/memorandum-W8-run5.md`).
- Дальше / риски: `--force`/rebase/`add -A`/merge/ветки/теги не применялись;
  `target/`, `.credo/`, `node_modules/` не коммитились. Эта запись и чекпойнт
  `memory/git.md` — рабочая «грязь» после пакета (не закоммичены; подберёт
  следующий процессный коммит).

## сервисная сессия · 2026-09-28 · T-15 B0 — карточка и документы волны подготовлены (без проб)

- Сделано: карточка T-15 дополнена фазой **B0** (инструментальная обвязка, вне
  канона), фаза B разделена на B0/B1, расширены C–E, добавлена фаза **F**
  (инструменты владельца); созданы `wave0b-plugins.md` (карточка волны),
  `wave0b-plan.md` (итерации B0-i1…i8) и шаблоны `wave0b-probes.md`,
  `wave0b-report.md` — `docs/tasks/T-15-mcp-ready-process/`; §10 записки
  `mcp-ready-process.md` синхронизирован (wave 0, фазы A–F, B0/F).
- Основание: разбор каталога awesome-opencode (136 плагинов, 28.09.2026) и
  решения владельца 28.09.2026: раскладка B0/C/D/E/F; пробы «temp-проект →
  служебная зона»; сервисная сессия вне канона.
- Проверки: канон не тронут (правки — только `docs/**` и эта лента); пробы не
  запускались; F26 — пересмотр фаз B–D выполнен карточкой, fast path S (D39)
  остаётся до S/M-прогона; статус F26 в реестре находок — за `migrator`.
- Дальше / риски: старт B0-i1 (temp-проект) — по указанию владельца; перенос
  зелёного в служебную зону — после `auditor` и подтверждения владельца; коммит
  документов — пакетом по подтверждению (`git`). Рабочая «грязь» до пакета:
  эта запись, `memory/git.md`, карточка T-15, §10 записки, 4 файла wave0b
  (untracked).

## git · 2026-09-28 · готово · пакет wave0b_docs

- Сделано: сверка `git status --porcelain` — ровно 8 путей по пакету (лишнего/
  недостающего нет); `git add` 8 точными путями → `git diff --cached --name-only`
  = 8 → коммит `docs(T-15): фаза B0 — карточка, wave0b-документы, синхронизация
  записки` (коммит содержит 8 путей; хеш — в отчёте `lead`).
- Проверки: предупреждения LF→CRLF — EOL, не отклонение; `--force`/rebase/
  `add -A`/merge/`branch`/`tag` не применялись; `target/`, `.credo/`,
  `node_modules/` не коммитились; **push НЕ выполнялся** (не подтверждён).
- Дальше / риски: push — отдельным подтверждением; ветка `develop` ahead 1.
