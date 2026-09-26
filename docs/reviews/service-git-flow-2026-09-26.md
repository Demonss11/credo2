# Приёмка: git-flow и модели агентов (service-git-flow)

**Проверка:** сервисное изменение канона — переход на git-flow с ветками задач
(`feature/T-XX-<слаг>` до работы, merge `--no-ff` и удаление — за `git`) и
модель всех 10 ролей `opencode-go/deepseek-v4.1-flash`.
**Версия:** HEAD `5b6f978` + рабочее дерево (изменения не закоммичены);
снимок 2026-09-26.
**Вердикт:** отклонено (одна находка P2 — правка в одну строку; остальное
проверено и в порядке).

## Находки

**P1:** — критичных проблем нет.

**P2:** `.opencode/agents/auditor.md:140` — в теле роли команда headless-прогона
жёстко задаёт старую модель: `opencode run --agent auditor --model
opencode-go/deepseek-v4-pro --auto`, тогда как фронтматтер теперь
`model: opencode-go/deepseek-v4.1-flash` (строка 4) и текст той же строки
говорит «с моделью роли». Ожидание: команда использует модель роли (flash) либо
не задаёт `--model` вовсе. Факт: при копировании документированной команды
аудит запустится на `deepseek-v4-pro`, то есть не на модели роли — миграция на
flash для `auditor` остаётся неполной в его же инструкции. Правка: заменить
`deepseek-v4-pro` → `deepseek-v4.1-flash` (или убрать явный `--model`).
Проверено `rg -n "deepseek-v4-pro" AGENTS.md .opencode docs` — других вхождений
в каноне нет (два совпадения в `docs/reviews/*` — исторические улики, не канон).

**P3:** —

## Проверки

- `git status --porcelain -- src tests Cargo.toml Cargo.lock` → пусто.
- `git status --porcelain` → ровно периметр: 10 изменённых файлов
  (`auditor.md`, `git.md`, `lead.md`, `rust-expert.md`, `validator.md`,
  `git-workflow.md`, `AGENTS.md`, `features/README.md`, `agents-cycle.feature`,
  `agents-git-approval.feature`) + новая лента `?? .opencode/mail/service-git-flow.md`.
- `cargo fmt --check` → без вывода (ok).
- `cargo test --test features_inventory` → 4 passed, 0 failed.
- `opencode debug agents` ×2 (после `reload`) → 17 агентов; у всех 10 ролей
  `providerID: opencode-go`, `model.id: deepseek-v4.1-flash` (10 вхождений в
  обоих прогонах, размер вывода идентичен — 2355 строк).
- Права: `git branch -d *` → `effect: ask` (только `git.md`); `cargo test *` →
  `allow` только у `validator` (одно вхождение); `opencode debug agents` —
  `allow` только у `validator` и `auditor`.
- Счётчики: `rg -c "Сценарий:" docs/features` + 2 «Структура сценария» =
  253 + 2 = 255; процесс агентов 6+3+6+5+6+6 = 32; файлов 42 — совпадает с
  `docs/features/README.md` (строки 284–286) и с прогоном теста.

## Что проверено и ок

- `.opencode/rules/git-workflow.md`: таблица git-flow
  (`master`/`develop`/`feature/<идент>`/`release/*`/`hotfix/*`), «Старт задачи»
  (ветка создаётся **до** работы), «Завершение задачи» (merge `--no-ff`, push
  `develop`, удаление локально и на `origin`), «Синхронизация»,
  «Подтверждение и очистка логов» (подтверждение до удаления, сверка из
  `HEAD` через `git show HEAD:<лента>`, `clean-logs.mjs`).
- `AGENTS.md`: «Цикл задачи» (ветка — шаг 2, до исполнителей), раздел «Git»,
  строка про сервисные ленты `.opencode/mail/service-<тема>.md` и
  `clean-logs.mjs`.
- `.opencode/agents/lead.md`: шаг 4 «Ветка до работы», «Полезные вызовы»
  (`@git` — ветка, коммиты, завершение).
- `.opencode/agents/git.md`: раздел «Ветки задачи» (старт/завершение), YAML
  `git branch -d *` под `ask`; `release/*`, `hotfix/*` и запрет мержа
  `publish/{name}-{version}` — согласованы с правилом.
- Согласованность маршрута: `AGENTS.md` ↔ `git-workflow.md` ↔ `lead.md` ↔
  `git.md` ↔ `agents-cycle.feature` (ветка до работы; merge в `develop`) ↔
  `agents-git-approval.feature` (5 сценариев: ветка, пакет, завершение,
  идемпотентность, push ≥ 5 мин).
- `.opencode/scripts/clean-logs.mjs` существует; путь бэкапа в скрипте
  (`<temp>/opencode/logs-backup-<ts>/`) совпадает с формулировкой правила.
- Битых относительных ссылок в изменённых файлах не найдено; дублей канона не
  добавлено.
