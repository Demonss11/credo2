# service-mcp-ready — подготовка документов «процесс, готовый к MCP»

- **Дата:** 2026-09-27 · **Исполнитель:** сессия `lead` Run 4 по указанию
  владельца («подготовь документы»; целевая картина — MCP, план максимум).
- **Вход:** внешние аналитические записки `ext-an-W8-run4.md` и
  `ext-an-2-W8-run4.md` (в папке загрузок владельца, вне репозитория);
  меморандум W8 (Run 4) §4–§7/§12; канон `dispatch-loop.md` /
  `git-workflow.md`; реестры `docs/features`, `docs/tasks`.
- **Сделано (documents-first, до реализации):**
  1. проектная записка `docs/analysis/mcp-ready-process.md` — целевая модель
     (слои, схема состояния, каталог операций ≤10, re-raise, финализация и
     правило останова, метрики, чек-лист готовности, миграция, фазы A–E);
  2. пять процессных фич (`docs/features/agents-state-schema`, `-session-checkpoint`,
     `-re-raise`, `-metrics`, `-mcp-readiness`; 19 сценариев) + строки, заметка
     и счётчики в `docs/features/README.md` (47 файлов / 278 сценариев;
     процесс — 11/55);
  3. карточка `docs/tasks/T-15-mcp-ready-process/README.md` и строка в
     `docs/tasks/README.md` (P1; зависит от T-11; источник — записка,
     ID и журнал — предварительные, за `migrator`).
- **Проверки:** структура фич — по правилам `features_inventory.rs` (первая
  строка `# language: ru`; ровно одна `Функция:`; счётчики сверены вручную:
  4+4+4+3+4 = 19); полный прогон (`cargo test --all`) — за `validator`; канон
  (`.opencode/rules/**`, `AGENTS.md`) не тронут; код и тесты не менялись.
- **Дальше / риски:** журнальные Q/D и подтверждение ID — `migrator` (до
  старта T-15); merge T-03 — решение владельца; далее по плану записки:
  Run 5 (S-пилот) → T-15 (L) → Run 6 (M) → заморозка → MCP (отдельно).
  Изменения не закоммичены (рабочие данные; пакет — по подтверждению).

## lead · 2026-09-27 · merge T-03 выполнен (открытый вопрос §12.1 снят)

- По директиве владельца: `feature/T-03-check-create` влита в `develop`
  (merge-коммит `89ebd42`, process-коммит `8d9c9dc`), ветка удалена
  (local + origin); детали — лента `.opencode/mail/T-03.md`.
- Записка `docs/analysis/mcp-ready-process.md` §12: вопрос 1 закрыт;
  `develop` содержит T-03 (регресс-кейс доступен).

## lead · 2026-09-27 · выбор по process-commit и git-фасаду (владелец принял)

- Решено (владелец, 2026-09-27): process-commit исполняет роль `git` по
  «постоянному» пакету (подтверждение — один раз на прогон); микро-роль
  `checkpoint` не вводится; фасад — команды OpenCode
  `.opencode/commands/git/…`; в перспективе git-MCP — узкий инструмент
  `checkpoint` у `lead`.
- Сделано: записка `docs/analysis/mcp-ready-process.md` — §5.4 (операция 8),
  §6 (выбор механизма), §12.3 (вопрос закрыт), Приложение A (эскиз git-MCP);
  карточка T-15 — фаза C (исполнитель, команды), «Черновик команды-фасада»,
  примечания; команда `/git/status` создана
  (`.opencode/commands/git/status.md`; read-only, временное решение до
  git-MCP).
- Проверки: в командах — только текстовые шаблоны (без shell-блоков `!`);
  изменяющие git-операции не выполнялись; канон ролей и прав не тронут.
- Дальше: `/git/checkpoint` — в фазе C T-15 (когда появятся `process/runN`
  и «постоянный» пакет); при внедрении команд включить
  `.opencode/commands/**` в область аудита `auditor`.

## lead · 2026-09-27 · wave 0 T-15 — токен-гигиена (документ подготовлен)

- По запросу владельца (перерасход токенов): изучены V2-доки `instructions` /
  `formatters` / `plugins` (+ `agents`) и конфигурация репо; подготовлен
  отдельный документ задачи T-15 —
  `docs/tasks/T-15-mcp-ready-process/wave0-token-hygiene.md`
  («Wave 0 — токен-гигиена, вне канона»): диагноз расхода (главный множитель —
  фикс-оверхед на сессию × число сессий), состав (A: `formatter` + чистка
  мёртвого `instructions`; B: плагин-пилот `token-guard.ts` — обрезка выводов
  и tool-схем; C: дисциплина чтения/логов), замер до/после, риски, порядок.
- Ссылки: карточка T-15 («Wave 0»), записка `mcp-ready-process.md` §10
  (строка W0). Канон не тронут; исполнение — по слову владельца.

## lead · 2026-09-27 · wave 0 — изучены permissions/policies (уточнения внесены)

- Изучены V2-доки `permissions` и `policies`; уточнения внесены в
  `docs/tasks/T-15-mcp-ready-process/wave0-token-hygiene.md`: §3.5 —
  статические permissions (`<server>_<tool>`, `execute`/Code Mode), policies
  как страховки (`*--force*`, `*reset --hard*`, `.ssh`), оговорка
  «запрет ≠ обрезка схемы без проверки»; шаг 3 порядка — сравнение статики с
  плагином.
- Значимо для волны: `execute` (Code Mode) и MCP-схемы — крупная фикс-статья;
  статические запреты — канон (фаза C, аудит), в wave 0 — только проверка
  гипотезы. Канон не тронут.

## lead · 2026-09-27 · wave 0 — изучены references (вне волны)

- Изучен V2-док `references`: доступ к каталогам вне проекта (алиас + путь +
  `description` в инструкциях; содержимое — по требованию; права — обычные
  `external_directory`/`read`; ссылки прав не выдают). Для токен-экономии
  wave 0 не применяем; зафиксировано в `wave0-token-hygiene.md` §3.6 как
  кандидат «после демо» (DAR-доки workspace, архив внешних обзоров).
  Канон не тронут.

## lead · 2026-09-27 · wave 0 — изучены skills (контекст по требованию)

- Изучен V2-док `skills`: тело скилла не в промпте — в запрос идут только
  ID/имя/`description`, загрузка по требованию (`skill`), поддерживающие файлы
  (`scripts/`, `references/`, шаблоны) читаются по ссылке; право `skill` по ID
  (allow/ask/deny) скрывает скиллы от модели и настраивается по агентам.
  Зафиксировано в `wave0-token-hygiene.md` §3.7: для wave 0 изменений нет;
  кандидат фазы C — сузить `rust-skills` профильными ролями; на будущее —
  шаблоны/чек-листы как скиллы и HTTP-каталог плейбуков. Канон не тронут.

## validator · 2026-09-27 · сервисная приёмка

- **Сделано:** независимая приёмка пакета документов «процесс, готовый к MCP»
  (T-15/MCP-ready + финализация меморандума W8, Run 4) перед выкладкой. Кода нет —
  изменения только в `docs/**`, `.opencode/commands/git/status.md`, почте/памяти.
  Прогон — полный `cargo test --all`; `features_inventory` и счётчики сверены
  машинно (тест) и вручную (перечень файлов, число сценариев).
- **Проверки:**
  - `git status --porcelain` (HEAD `89ebd42`, merge T-03): изменены
    `docs/features/README.md`, `docs/tasks/README.md`, `.opencode/mail/T-03.md`,
    `.opencode/memory/git.md`; untracked — `.opencode/commands/`,
    `.opencode/mail/service-mcp-ready.md`, `.opencode/mail/service-w8-memorandum.md`,
    `docs/analysis/mcp-ready-process.md`, `docs/analysis/memorandum-W8-run4.md`,
    5×`docs/features/agents-*.feature`, `docs/tasks/T-15-mcp-ready-process/`.
    **`src/**` и `tests/**` не тронуты** (после merge `89ebd42`) — совпадает
    с заявленным периметром. Кода нет → `fmt`/`clippy` не требуются.
  - `cargo test --all` → **90 passed / 0 failed** (lib 49 + `features_inventory`
    4/4 + `mcp_draft` 14 + `publish` 12 + `rest` 11; doc-tests 0); compile
    `Finished` без предупреждений.
  - `features_inventory` → **4/4 ok** (`feature_files_are_valid_documents`,
    `feature_files_match_readme_inventory`, `scenario_counts_match_readme`,
    `readme_totals_match_files`). Счётчики `docs/features/README.md`:
    **47 файлов / 278 сценариев** (backend 27/148, frontend 9/75, процесс 11/55).
    Перечень файлов — ровно 47 `.feature`; новые `agents-*`: `state-schema` 4,
    `session-checkpoint` 4, `re-raise` 4, `metrics` 3, `mcp-readiness` 4 = **19**.
- **Вердикт: accepted** (P1/P2/P3 нет). Изменённые мною файлы (в пакет выкладки):
  `.opencode/mail/service-mcp-ready.md` (эта запись), `.opencode/memory/validator.md`.
  Ничего не коммитил, канон/код не правил.

## lead · 2026-09-27 · выкладка документов — пакет подтверждён владельцем

- Директива владельца: **«отправим в удалённый репозиторий все изменения»**
  (2026-09-27). Приёмка — `validator` (раздел выше): **accepted**
  (`cargo test --all` 90/0; `features_inventory` 4/4; счётчики 47/278;
  `src/**`/`tests/**` не тронуты).
- Пакет `push_docs_t15` (исполняет `git`, идемпотентно):
  1. сверка: ветка `develop`, HEAD `89ebd42`; изменённые tracked — ровно
     `docs/features/README.md`, `docs/tasks/README.md`,
     `.opencode/mail/T-03.md`, `.opencode/memory/git.md`,
     `.opencode/memory/validator.md`; untracked — ровно:
     `docs/analysis/memorandum-W8-run4.md`,
     `docs/analysis/mcp-ready-process.md`, 5×`docs/features/agents-*.feature`
     (`state-schema`, `session-checkpoint`, `re-raise`, `metrics`,
     `mcp-readiness`), `docs/tasks/T-15-mcp-ready-process/` (README + wave0),
     `.opencode/mail/service-w8-memorandum.md`,
     `.opencode/mail/service-mcp-ready.md`,
     `.opencode/commands/git/status.md`; расхождение — стоп без изменяющих
     команд;
  2. `git pull origin develop`; `git switch -c feature/T-15-mcp-ready-process develop`;
  3. коммит A — `docs(T-15): записка MCP-ready процесса, фичи, карточка, wave 0, /git/status`;
     пути: `docs/analysis/mcp-ready-process.md`, `docs/features/README.md`,
     5×`docs/features/agents-*.feature`,
     `docs/tasks/README.md`, `docs/tasks/T-15-mcp-ready-process/README.md`,
     `docs/tasks/T-15-mcp-ready-process/wave0-token-hygiene.md`,
     `.opencode/commands/git/status.md`, `.opencode/mail/service-mcp-ready.md`;
  4. коммит B — `docs: финализация меморандума W8 (Run 4) и служебные записи`;
     пути: `docs/analysis/memorandum-W8-run4.md`,
     `.opencode/mail/service-w8-memorandum.md`, `.opencode/mail/T-03.md`,
     `.opencode/memory/git.md`, `.opencode/memory/validator.md`;
  5. `git push -u origin feature/T-15-mcp-ready-process` (таймаут ≥5 мин);
  6. `git switch develop`; `git merge --no-ff feature/T-15-mcp-ready-process
     -m "Слияние feature/T-15-mcp-ready-process в develop"`;
  7. `git push origin develop` (таймаут ≥5 мин);
  8. `git branch -d feature/T-15-mcp-ready-process`;
     `git push origin --delete feature/T-15-mcp-ready-process`.
- Не входит: правки канона и кода; исполнение wave 0 (это выкладка
  документов); прочие ветки и теги.
- Ожидание: `develop` содержит merge-коммит с A и B; ветка удалена локально и
  на origin; дописи ленты/памяти после коммитов — рабочие (R7).

## git · 2026-09-27 · готово (пакет `push_docs_t15`)

- **Сделано:** сверка совпала (ветка `develop`, HEAD `89ebd42`; 5 `M` — ровно
  `features/README.md`, `tasks/README.md`, `mail/T-03.md`, `memory/git.md`,
  `memory/validator.md`; 13 `??` — ровно по списку пакета, лишнего нет).
  `pull origin develop` → `Already up to date` → `switch -c
  feature/T-15-mcp-ready-process develop`. Коммит A **`a054a46`** (12 файлов,
  +896/−3), коммит B **`30febd6`** (5 файлов, +701). `push -u origin
  feature/T-15-mcp-ready-process` → `* [new branch]` (timeout 360000).
  Merge `--no-ff` → **`afe6a7a`** (17 файлов, +1597/−3);
  `push origin develop` → `89ebd42..afe6a7a`. Ветка удалена: `branch -d`
  (`was 30febd6`) + `push origin --delete` (`- [deleted]`).
- **Проверки:** `git log -3 --oneline develop` → `afe6a7a`, `30febd6`,
  `a054a46`; `git branch -a` → `feature/T-15-mcp-ready-process` нет (local и
  origin); `git status -sb` → `## develop...origin/develop` (чисто, в синхроне).
  Перед каждым `add` состав индекса сверялся `git diff --cached --name-only`
  (A — 12 путей, B — 5). Пропущено как уже выполненное: нет.
- **Дальше / риски:** дописи ленты/памяти после коммитов (эта запись и чекпойнт
  `memory/git.md`) — рабочая «грязь», не закоммичены (R7); служебный коммит —
  по решению `lead`. `target/`, `.credo/`, `.opencode/state/**` не трогались;
  `--force`/`-D`/rebase не применялись.

## lead · 2026-09-27 · выкладка завершена — `develop` @ `afe6a7a` в origin

- `dispatch git` (пакет `push_docs_t15`): `expect` совпал — коммиты `a054a46`
  (A: документы T-15, 12 файлов, +896/−3) и `30febd6` (B: меморандум W8 и
  записи, 5 файлов, +701); merge `--no-ff` → `afe6a7a` (17 файлов, +1597/−3);
  push `89ebd42..afe6a7a`; ветка `feature/T-15-mcp-ready-process` удалена
  (local + origin); `develop` в синхроне.
- Документы T-15 (записка MCP-ready, фичи, карточка, wave 0, `/git/status`) и
  финализация W8 — в `origin/develop`. Остаток: дописи этой ленты и
  `.opencode/memory/git.md` после коммитов — рабочие (R7); wave 0
  (A/B1/B2 + опционально policies) — по слову владельца.

## lead · 2026-09-27 · системный коммит записей (по директиве владельца)

- Директива владельца: **«у develop остались не закоммиченные изменения —
  нужен системный коммит»** (2026-09-27). Состав: эта лента
  (`.opencode/mail/service-mcp-ready.md`) и `.opencode/memory/git.md`
  (дописи после выкладки `push_docs_t15`).
- Пакет `service_records` (исполняет `git`, идемпотентно): **служебный
  коммит прямо в `develop`** по директиве владельца (прецедент — `bf56a4a`,
  «Create service-run4-preflight.md»); канон допускает прямые коммиты лишь
  для служебной очистки логов — расширение практики зафиксировать в фазе C
  T-15.
  1. сверка: `git status -sb` — изменённые tracked ровно эти два файла;
     `git log -1 --oneline` — `afe6a7a`; иных изменений/untracked нет;
  2. отчёт роли и чекпойнт в память — **до** коммита (входят в него);
  3. `git add .opencode/mail/service-mcp-ready.md` →
     `git add .opencode/memory/git.md` →
     `git commit -m "chore: служебные записи после выкладки документов (лента, память)"`;
  4. `git push origin develop` (таймаут ≥5 мин);
  5. после push — никаких файловых записей (итог вернуть `lead`).
- Ожидание: `develop` содержит служебный коммит (в `origin`), рабочее дерево
  чистое и в синхроне.

## git · 2026-09-27 · шаг 2 готов (пакет `service_records`)

- **Сделано:** сверка шага 1 совпала — ветка `develop` в синхроне с
  `origin/develop`, изменённых tracked ровно два
  (`.opencode/mail/service-mcp-ready.md`, `.opencode/memory/git.md`), иных
  изменений/untracked нет; `git log -1 --oneline` → `afe6a7a`. Подтверждение
  владельца — запись `lead` выше (директива «нужен системный коммит»).
- **Что делает коммит:** только эти две служебные записи (лента + память);
  канон, код и тесты не трогаются; **служебный коммит прямо в `develop`**
  (прецедент `bf56a4a`). Отчёт роли и чекпойнт `memory/git.md` дописаны **до**
  коммита и входят в него.
- **Проверки:** `git status -sb` (до) — `## develop...origin/develop` + 2 `M`;
  `git log -1 --oneline` → `afe6a7a`.
- **Дальше / риски:** `git add` двумя путями → `git commit -m "chore: служебные
  записи после выкладки документов (лента, память)"` → `git push origin develop`
  (таймаут ≥5 мин); после push файловых записей не делается.

## lead · 2026-09-28 · wave 0 — исполнено: разметка + A (конфиг)

- Разметка волны: `docs/tasks/T-15-mcp-ready-process/wave0-plan.md`
  (итерации W0-i1…W0-i4, границы, проверки, открытые вопросы). Первый этап —
  W0-i1 (A: конфиг) — размечен подробно.
- Исполнено W0-i1, правки только в `opencode.json`:
  - **A1:** удалено мёртвое поле `"instructions": ["AGENTS.md"]` (V2 не
    резолвит; AGENTS.md подхватывается автоматически) вместе с прежним
    комментарием;
  - **A2:** добавлено `"formatter": true` (без `package.json` активируется
    только `rustfmt` для `.rs`; markdown/JSON не затрагиваются).
- Проверки: JSONC распарсен без ошибок; ключи конфига — `$schema`, `shell`,
  `default_agent`, `formatter`, `provider`, `mcp`; `instructions` отсутствует;
  `git diff` по волне — ровно `opencode.json` + план + эта лента. Канон
  (`AGENTS.md`, `.opencode/rules/**`, агенты) не тронут.
- Оговорка для аудита: форматтер запускает харнесс, не роль — расширение прав
  роли отсутствует.
- Отложить: smoke автоформата на 1–2 `.rs` требует реального прогона OpenCode
  (в текущей среде харнесс недоступен) — проверить на ближайшем прогоне и
  отметить в плане.
- Дальше: W0-i2 (плагин `token-guard.ts`, B1) — по слову владельца; коммит
  пакета — по подтверждению (вопрос 1 в плане: служебный в `develop` или
  ветка).

## lead · 2026-09-28 · wave 0 — W0-i2 исполнена (B1: плагин token-guard)

- Создан `.opencode/plugins/token-guard.ts` (автозагрузка из
  `.opencode/plugins/**`; для вступления в силу — рестарт сервиса OpenCode).
  Хук `tool.execute.after`: лимит `OUTPUT_LIMIT = 12_000` симв. (голова 60% +
  хвост 40%, константы в начале файла), сохранение строк
  `error|warning|FAILED|failed|passed` из опущенной середины (до 200, без
  дублей), ANSI-чистка, для `read` — пометка «остаток опущен (offset/limit)»,
  инструмент `task` исключён из среза. Ошибки хука не «глушат» роль: при любом
  исключении вывод остаётся как есть.
- **Отступление от §3.2 карточки:** персистентного `ctx.storage` у плагина в
  текущем SDK нет — счётчики (срезы, байты, по инструментам) пишутся в
  локальный снимок `.opencode/plugins/token-guard.stats.json` (атомарно, вне
  коммита); вывод — событие `session.idle` в stdout сервера. Выбор зафиксирован
  в плане (W0-i2).
- Проверки: изолированный прогон хуков на синтетических выводах (Node 20) —
  7 сценариев зелёные (нетронутый короткий вывод; ANSI без среза; срез с
  сохранением ERROR из середины и хвоста; пометка read; исключение task;
  не-string output; рост счётчиков + печать итога). Боевая проверка на реальных
  `cargo test`/`rg` — за ней в следующий прогон (флаг в плане открыт).
- Случайно изменённый `.gitignore` откачен (вне рамок B1). Коммит — в текущую
  ветку, без push (директива владельца от 2026-09-28).
- Дальше: W0-i3 (B2 — обрезка tool-схем; сначала сверка черновых правил с
  брифами ролей и запись согласия в ленту **до** включения кода).

## lead · 2026-09-28 · wave 0 — разбор ветки, фикс плагина (V1→V2), smoke закрыт

- Ветка `exp/agent-update-t15w0` (local + origin; до этой записи дерево
  чистое): коммиты `9affa8d` (A: конфиг + план + smoke-протокол), `1bd0f0c`
  (B1: плагин), `25c5979` (merge PR #3 `work-stage-layout-956e9` — учесть при
  слиянии в `develop`).
- **Диагностика ошибки плагина** («Plugin must export a default definition
  with an id and an effect or setup function»): рабочее приложение —
  `@opencode/cli` **2.0.18** (V2); первая версия `token-guard.ts` написана под
  V1-хуки (`@opencode-ai/plugin@1.18.x`, функции-хуки). Типы
  `@opencode-ai/plugin@1.18.30` в `~/.config/opencode/node_modules` —
  устаревший V1-остаток; актуальные типы — `@opencode/plugin@2.0.18`
  (`Plugin.define`, `ctx.tool.hook`, `ctx.storage`).
- **Исправление** (`.opencode/plugins/token-guard.ts`): переписан под V2 —
  `import { Plugin } from "@opencode/plugin"`; `Plugin.define({ id, setup })`;
  `ctx.tool.hook("execute.after")` — подмена `event.result`
  (`content: string | Content[]`, `output?`); счётчики — `ctx.storage` (ключ
  `stats`), вывод — `console.log` сервера на каждый срез. Логика B1 сохранена
  (лимит ~12 КБ, голова+хвост, важные строки, ANSI, `read`-пометка;
  исключения — `subagent`/`task`). Отступление по счётчикам снято.
- **Smoke форматтера (W0-i1) закрыт** в этом же прогоне:
  `target/wave0-formatter-smoke.rs` (намеренно плохой формат) после записи
  инструментом перечитан уже отформатированным — rustfmt работает;
  `target/wave0-formatter-smoke.md` не изменён — prettier/biome не активны.
  Временные файлы — в `target/` (вне git).
- Осталось: (1) `opencode service restart` → убедиться, что ошибка загрузки
  исчезла (плагин активен); (2) боевая проверка B1 (счётчики растут, роли не
  «глушатся»); (3) W0-i3 (B2) и W0-i4 (замер/отчёт); (4) коммит исправления в
  ветку — после подтверждения загрузки. Канон не тронут.

## lead · 2026-09-28 · wave 0 — фикс #2 плагина: убран импорт (резолв)

- После рестарта ошибка изменилась на «Plugin failed to load»; причина из лога
  сервера (`~/.local/share/opencode/log/opencode.log`, ref `err_07d08b9f`):
  `ResolveMessage: Cannot find package '@opencode/plugin' imported from
  .../token-guard.ts` — локальный плагин без установленного рядом пакета не
  резолвит импорт.
- Исправление: `import { Plugin } from "@opencode/plugin"` убран; default —
  простой объект `{ id: "token-guard", async setup(ctx) {…} }` (загрузчик
  проверяет только форму: `id` + `setup`/`effect`). Логика и счётчики — без
  изменений (`ctx.tool.hook("execute.after")`, `ctx.storage`). Импорт вернётся
  только ради типов — при появлении `package.json` в проекте.
- Проверка: watcher уже следит за файлом (hot-reload); если не подхватилось —
  `opencode service restart`. В логе не должно быть `failed to load plugin`;
  затем — боевая проверка B1 (счётчики, срезы, роли не «глушатся»).

## lead · 2026-09-28 · wave 0 — B1 проверен в бою; плагин загружен (watcher)

- Плагин загрузился без ошибок: watcher сделал hot-reload после правок (лог
  сервера: `msg="loading plugin"` без последующего `failed to load plugin`;
  ошибки `err_07d08b9f` больше нет).
- **Боевые срабатывания B1** (реальный прогон):
  - `read` большого файла (`docs/features/README.md`): 35 777 → ~16,7 КБ,
    маркер «срез: опущено ~19 069 байт», пометка «остаток опущен
    (offset/limit)»;
  - `glob` каталога: 13 915 → ~12,3 КБ, маркер «опущено ~1 559 байт».
- Замечания: `console.log` плагина в `opencode.log` не попадает (stdout
  сервиса); `ctx.storage` — бинарное хранилище (текстовый поиск
  `bytesTrimmed` не находит); визуальная сверка счётчиков — на W0-i4.
- Окружение: в `.opencode/` установлен `@opencode-ai/plugin@1.18.31` — это
  **V1-линия** (не даёт `@opencode/plugin`); для V2-типов нужен
  `@opencode/plugin@2.0.18`. Текущая import-free версия работает независимо;
  `.opencode/.gitignore` покрывает `node_modules/`, `package.json`,
  `package-lock.json` (в git не попадают).
- Осталось: тяжёлые выводы `cargo test`/`rg` — на ближайших прогонах;
  счётчики/экономия — на W0-i4; коммит правок (плагин, план, лента) в ветку.

## lead · 2026-09-28 · wave 0 — фикс #3: пакет 2.0.18, импорт возвращён, рестарт чистый

- Установка (владелец): в `.opencode/` — `@opencode/plugin@^2.0.18`;
  V1-пакет `@opencode-ai/plugin` удалён; `package.json`/`node_modules`/
  `package-lock.json` — под `.opencode/.gitignore` (git чистый).
- Плагин вернулся к канонической форме: `import { Plugin } from
  "@opencode/plugin"` + `Plugin.define({ id, setup })`.
- Диагностика промежуточных сбоев (19:54–19:56): (1) синтаксическая ошибка —
  артефакт ступенчатой правки (watcher перезагружает файл на каждое
  изменение); (2) `Cannot find package` уже после установки — залипший кэш
  резолва в текущем процессе (Bun-путь reload без cache-busting); **чистый
  рестарт** решил.
- После рестарта (20:00:50, новый run `d42551d6`): `msg="loading plugin"`
  **без** `failed to load plugin`; боевой срез подтверждён (grep-вывод
  33 060 → ~12 КБ). Правило гигиены — в шапке плагина: правки одним атомарным
  write.
- Осталось: W0-i3 (B2 — обрезка tool-схем), W0-i4 (счётчики/замер/отчёт),
  коммит правок в ветку.

## lead · 2026-09-28 · wave 0 — коммит правок B1 (пакет подтверждён владельцем)

- Директива владельца: **«делай коммит»** (2026-09-28). Ветка —
  `exp/agent-update-t15w0`; **без push**.
- Пакет `w0_b1_fix` (исполняет `git`, идемпотентно): сверка `git status -sb`
  (ожидаемые изменённые — ровно 4: `.opencode/plugins/token-guard.ts`,
  `docs/tasks/T-15-mcp-ready-process/wave0-plan.md`,
  `docs/tasks/T-15-mcp-ready-process/wave0-smoke-formatter.md`,
  `.opencode/mail/service-mcp-ready.md`; иное — стоп); `git add` этими путями;
  `git commit -m "fix(plugin): wave 0 B1 — token-guard на V2 API
  (@opencode/plugin@2.0.18); smoke форматтера закрыт"`; push НЕ выполнять.
- Ожидание: коммит на ветке; рабочая «грязь» после отчёта роли (лента/память)
  — R7, поедет со следующим пакетом.
