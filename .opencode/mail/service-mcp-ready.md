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
