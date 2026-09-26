# Сервисная лента: git-flow и модели агентов (2026-09-26)

- **Тема:** переход на git-flow (ветка задачи создаётся до работы), модели всех
  ролей — `opencode-go/deepseek-v4.1-flash`.
- **Основание:** решение владельца (сессия 2026-09-26); эталонный процесс —
  `mcp_doc_server/.opencode/rules/git-workflow.md` (уровень рабочего пространства).
- **Формат отчётов:** `AGENTS.md` §Рабочая группа агентов (один писатель, append).

## lead · 2026-09-26 · пакет подтверждён (git-flow + flash)

- Подтверждение (решение владельца, 2026-09-26): коммит сервисного изменения,
  push и bootstrap ветки `develop` от нового HEAD.
- Пакет: `chore: git-flow с ветками задач и модели ролей на flash` — 10 ролей
  (модели flash; правки `git.md`/`lead.md`/`auditor.md`), `git-workflow.md`,
  `AGENTS.md`, фичи + README (42/255), эта лента, отчёты приёмки r1–r3;
  затем `develop` и `git push -u origin develop`. Отчёт `git` пишется **до**
  коммита и входит в него (дерево чистое).
- Дальше / риски: пилот T-03 — отдельной сессией с веткой
  `feature/T-03-check-create`; ветка `exp/agent-cycle-rerun` остаётся архивной.

## validator · 2026-09-26 · в работе (чекпойнт до прогона)

- Сделано (только чтение/анализ, версия `5b6f978` + рабочее дерево): состав
  изменений = периметр; `src/tests/Cargo` чист; модели 10/10 flash; права
  (`git branch -d *` → `ask`, `cargo test *` — только validator,
  `opencode debug agents` — validator+auditor); README 42/255, процесс 32
  (6+3+6+5+6+6).
- Проверки (запускаю сейчас): `cargo fmt --check`,
  `cargo test --test features_inventory`, `opencode debug agents`.
- Дальше / риски: вердикт после прогона; полный DoD не требуется.

## validator · 2026-09-26 · отклонено (P2, правка в одну строку)

- Сделано: отчёт — `docs/reviews/service-git-flow-2026-09-26.md`. Проверено:
  периметр изменения, права (`git branch -d *` → `ask`; `cargo test *` — только
  `validator`; `opencode debug agents` — `validator`+`auditor`), модели 10/10
  flash, согласованность канона и фич, счётчики 42/255 (процесс 32).
- Проверки: `cargo fmt --check` → ok; `cargo test --test features_inventory` →
  4 passed; `opencode debug agents` ×2 → 17 агентов, 10 flash;
  `git status --porcelain -- src tests Cargo.toml Cargo.lock` → пусто.
- **Находка P2:** `.opencode/agents/auditor.md:140` — headless-команда задаёт
  старую модель `opencode-go/deepseek-v4-pro`, тогда как фронтматтер —
  `deepseek-v4.1-flash` (стр. 4). Правка: заменить на flash (или убрать
  `--model`). Остальное — без замечаний.
- Дальше / риски: одна правка `auditor.md` → повторная адресная приёмка
  (`-rN`) и только потом коммит; содержательная часть git-flow верна.

## validator · 2026-09-26 · принято (r2)

- Сделано: адресная повторная приёмка после r1. P2 закрыта — `auditor.md:142`
  headless-команда на `opencode-go/deepseek-v4.1-flash` (совпадает с
  фронтматтером). Маски веток у `lead` (18–20) и `auditor` (21–23) только
  read-only (`-l`/`-a`/`--show-current`); `git.md:32` `git branch -d *` = `ask`,
  `git.md:41` `question` = `deny`; `lead.md:87–89` — DoD ссылкой на `AGENTS.md`;
  лента и `AGENTS.md:127–129` — порядок «аудит → приёмка». 10/10 ролей flash;
  `cargo test *` — только `validator.md:19`. Отчёт:
  `docs/reviews/service-git-flow-2026-09-26-r2.md`.
- Проверки: `git status --porcelain -- src tests Cargo.toml Cargo.lock` → пусто;
  `git log -1 --oneline` → `5b6f978` (тот же HEAD, новых коммитов нет);
  `rg` по моделям/веткам/`question`/`cargo test` → ожидаемо;
  `opencode debug agents` ×2 → права совпадают с YAML, flash; счётчики 42/255
  (процесс 32) — как в r1.
- **Вердикт: принято.** P1/P2/P3 — нет. Дальше: коммит пакета (`git`) после
  подтверждения `lead`; перед коммитом изменений системы агентов канон требует
  свежий аудит `auditor`.

## validator · 2026-09-26 · принято (r3, завершающая)

- Сделано: завершающая приёмка после P3 аудита r2. Исправление ровно одно —
  `.opencode/agents/git.md` §«Ветки задачи» свёрнут до ссылки на
  `.opencode/rules/git-workflow.md` («Старт задачи», «Завершение задачи»);
  цепочки команд из тела убраны (Q41). Якоря правила существуют
  (`git-workflow.md:45,66`); сводка совпадает по смыслу; `AGENTS.md` команд не
  дублирует. Права `git` не изменились (`git.md:32` `git branch -d *` ask,
  `git.md:41` `question` deny). Отчёт:
  `docs/reviews/service-git-flow-2026-09-26-r3.md`.
- Проверки: `git diff .opencode/agents/git.md` → правка только в разделе
  (команд в теле нет); `git status --porcelain` → периметр прежний;
  `opencode debug agents` → права/модели/состав без изменений; `rg` по
  `git switch|merge|push|branch` → только YAML-права и политика таймаута.
  Полный DoD и `features_inventory` не гонял: правка — текст роли, код/тесты/
  фичи не менялись.
- **Вердикт: принято.** P1/P2/P3 — нет. Дальше: коммит пакета (`git`) после
  подтверждения `lead` (перед коммитом системы агентов — свежий аудит
  `auditor`).

## git · 2026-09-26 · в работе (отчёт до коммита)

- Сделано (отчёт пишется **до** коммита и входит в него): пакет сверен по ленте
  (`lead · 2026-09-26 · пакет подтверждён`) и с `git status` — HEAD `5b6f978`,
  ветки `develop` нет, периметр = 4 untracked + 11 modified ровно по списку.
- Проверки: `git status --porcelain`; `git log -1 --oneline`; `git branch -a`.
- Дальше / риски: `git add` точными путями → `git commit` (`chore: git-flow …`)
  → `git push` → `git switch -c develop` → `git push -u origin develop`;
  отчёт с хешем — после push в ленту не пишу (по условию `lead`), верну в ответе.

## git · 2026-09-26 · готово (коммит, push, develop)

- Сделано: коммит `chore: git-flow с ветками задач и модели ролей на flash`
  (хеш и результат push — в ответе `lead`); ветка `develop` создана от нового
  HEAD и запушена в `origin`; дерево чистое, `develop` в синхроне с `origin`.
- Проверки: `git branch --show-current` → `develop`; `git status` → чисто;
  `git status -sb` → в синхроне; `git log -3 --oneline`.
- Дальше / риски: ветка `exp/agent-cycle-rerun` — архив; пилот T-03 отдельной
  сессией с `feature/T-03-check-create`.

## lead · 2026-09-26 · пакет подтверждён (очистка логов)

- Подтверждение (решение владельца, 2026-09-26): служебный коммит очистки логов
  прямо в `develop`; правило-исключение — в `git-workflow.md`.
- Пакет (два коммита, одна операция):
  1. `chore: git-workflow — служебный коммит очистки логов` — строка-исключение
     в `.opencode/rules/git-workflow.md` + эта запись подтверждения;
  2. `chore: очистка логов после сервисной операции` — сброшенные памяти
     (`git.md`, `validator.md`) и удаление этой ленты; `git` сверяет
     подтверждение из истории (`git show HEAD:<лента>`).
- Дальше / риски: почта пуста, дерево чистое; пилот T-03 — отдельной сессией
  с веткой `feature/T-03-check-create`.
