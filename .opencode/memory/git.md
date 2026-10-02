# Память: git (git-операции)

- **Канон:** `.opencode/rules/git-workflow.md` (цикл пакета — §«Минимальный
  цикл `git`», решение D75).
- **Правило:** чекпойнт — пакет, выполненные шаги, что осталось (кратко);
  хеши — в ответе `lead` (F43), здесь не хранятся.

## Знание роли (уроки)

- **Цикл, запреты, дисциплина путей — в правиле:** `.opencode/rules/git-workflow.md`
  §«Минимальный цикл `git`» (единый дом, D75/P2); сверка состояния —
  `node .opencode/scripts/git-check.mjs [--staged] [--expect=N]`.
- **Урок: чтения ленты и памяти — хвостами** (`offset`/`limit`): полные чтения
  дают срезы и повторные вызовы.
- **Урок: quoting** — многословные шаблоны `git log --grep` — в кавычках
  (без кавычек падают).
- **Отказы прав:** команды одиночные (без `;`, пайпов, перенаправлений);
  `git -C` не использовать (паттерны матчатся от начала команды) — рабочий
  каталог в `workdir`; пути с точкой — с `./`.
- **Порядок записей:** чекпойнт памяти и отчёт в ленту — **до** `add`, входят в
  пакет; после `push` в отслеживаемые файлы не писать.
- **Идемпотентность:** одна проверка перед шагом; уже сделанное пропусти;
  `push` — таймаут ≥ 5 мин, при обрыве — повтор статуса и продолжение.
- **Дисциплина путей:** `add` — только точные пути пакета; имя ветки содержит
  идентификатор артефакта; `--force`/`reset --hard`/`rebase` запрещены;
  `target/`, `.credo/`, `node_modules/` не коммитятся.

## Чекпойнты

- **02.10.2026, branch_end T-24** (лента `T-24.md`, пакет 20 путей): ветка
  `feature/T-24-rest-cli-contour-test` = origin (коммитов нет); база develop =
  origin/develop = `caac10d`. Подтверждение владельца — лента §«гейт пакета —
  подтверждён» (дословно «Подтверждаю пакет (Recommended)»; `progress.yaml`).
  Снимок совпал с `package.add_paths` — 15 `M` + 5 `??` = 20 (5 ??:
  `mail/T-24.md`, `docs/analysis/T-24-2026-10-02.md`,
  `docs/reviews/T-24-2026-10-02.md`, `docs/reviews/T-24-2026-10-02-r2.md`,
  `tests/rest_cli.rs`). Чекпойнт и отчёт в ленту — до `add` (F43). Осталось:
  `add` точными путями (20) → staged-сверка (15 M + 5 A) → коммит «code(T-24):
  интеграционный тест CLI-контура REST» → `switch develop` → `pull origin
  develop` → merge `--no-ff` «Слияние feature/T-24-rest-cli-contour-test в
  develop» → `push origin develop` → удаление ветки (local + origin). `master`
  не трогается. Риск: `push origin --delete` под гардом CCSN — не обходить,
  эскалировать (прецедент T-20).
- **02.10.2026, branch_start T-24** (лента `T-24.md`, действие плана
  `branch_start`; класс S): база `develop` = `origin/develop` = `HEAD` = `caac10d`
  (проверено `git rev-parse develop origin/develop HEAD`). Подтверждение
  владельца — лента `T-24.md`, секция «гейт ветки — подтверждён» (дословно
  «Подтверждаю ветку и порядок (Recommended)»); продублировано в `progress.yaml`
  (02.10, `owner_response`). Дерево: 3 M F43-подхвата T-20 (`mail/T-20.md`,
  `memory/service.md`, `state/current/progress.yaml`) + 2 M записей плана
  (`current_state.yaml`, `next_action.yaml`) + 2 ?? (`mail/T-24.md`,
  `docs/analysis/T-24-2026-10-02.md`) — переносятся в ветку как есть, не
  коммитятся. Чекпойнт и отчёт в ленту — до операции (F43-паттерн).
  Осталось: `switch develop` → `pull origin develop` → `switch -c
  feature/T-24-rest-cli-contour-test develop` → `push -u origin
  feature/T-24-rest-cli-contour-test`. `master` не трогается; коммитов нет.
- **02.10.2026, branch_end T-20** (лента `T-20.md`, пакет 20 путей): база
  `develop` = `origin/develop` = `HEAD` = `d414998`; ветка
  `feature/T-20-traceability-wave2`. Подтверждение владельца — лента, секция
  «гейт пакета — подтверждён» (дословно «Коммит + merge в develop
  (Recommended)»); порядок там же. Снимок совпал с пакетом — 14 `M` + 6 `??` =
  20. Чекпойнт и отчёт в ленту — до `add` (F43). Осталось: `add` точными путями
  (20) → `diff --cached --name-status` (ожидание 14 M + 6 A) → `commit` →
  `switch develop` → `merge --no-ff` → `push origin develop` → удаление ветки
  (origin + local). `master` не трогается; сообщение — `docs(T-20): волна 2
  TRACEABILITY — разбор 26 open-строк (9 done, T-24, 16 open v0.2); приёмка`.
- **02.10.2026, branch_start T-20** (лента `T-20.md`, действие плана
  `branch_start`): база `develop` = `origin/develop` = `d414998` (C1); проверено
  `rev-parse develop` = `rev-parse origin/develop`. Подтверждение владельца —
  лента `T-20.md`, секция «гейт предложений — подтверждён» (дословно
  «Подтверждаю расклад (Recommended)»), далее в той же секции — «dispatch git
  (branch_start feature/T-20-traceability-wave2)». Дерево: незакоммиченные
  рабочие пути T-20 (M service-mcp-ready-r8.md, M memory/analyst.md,
  M memory/service.md, M state/current/{current_state,next_action,progress}.yaml,
  ?? mail/T-20.md, ?? docs/analysis/T-20-2026-10-02.md) — переносятся в ветку
  как есть, не коммитятся. Чекпойнт и отчёт в ленту — до операции (F43-паттерн).
  Осталось: `switch -c feature/T-20-traceability-wave2 develop` →
  `push -u origin feature/T-20-traceability-wave2`. `master` не трогается;
  коммитов нет (`package.mode` — `branch_end`).
- **02.10.2026, пакет C1 (T-15, фаза C, схема состояния)** — сервисная лента
  `service-mcp-ready-r8`: база `develop` = `origin/develop` = `2f5544c`;
  подтверждение владельца — лента, секция «гейт возобновлён — „коммить"»
  (`owner_response` дословно «коммить»); снимок совпал — 14 `M` + 6 `??` +
  запись роли `git` (F43) = 21. Чекпойнт и отчёт в ленту — до `add` (F43).
  Осталось: `add` точными путями → `diff --cached --name-status` (ожидание
  15 M + 6 A) → `commit` → `push origin develop`. Ветки не создаются, `master`
  не трогается; сообщение — `chore(process): T-15 C1 — схема состояния
  (state-schema.md, D86); приёмка`.
- **02.10.2026, процессный пакет очистки логов** (сервисная операция T-15,
  лента `service-mcp-ready-r8`): база `develop` = `origin/develop` = `c754c97`;
  подтверждение владельца — лента, секция «гейт открытия» («отдельным
  коммитом»); снимок `git status --porcelain` совпал с пакетом — 17 `D`
  `.opencode/mail/` + 9 `M` `.opencode/memory/` (+ untracked лента r8, в пакет
  не входит). Чекпойнт и отчёт в ленту — до `add` (F43). Осталось: `add`
  точными путями → `diff --cached --name-status` (ожидание 17 D + 9 M) →
  `commit` → `push origin develop`. Ветки не создаются, `master` не трогается;
  сообщение — `chore(process): очистка логов (mail + memory)`.
