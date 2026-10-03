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

- **03.10.2026, сервисный пакет r20 (T-27 — реестр находок: переезд в
  `docs/registry/` и раскол, Q95/D98)** — лента `service-mcp-ready-r20`: база
  `develop` = `origin/develop` = `HEAD` = `0e5a158` (`git log -1 --oneline`,
  r19-коммит); ветки нет (сервисная операция, коммит в `develop`).
  Подтверждение владельца — лента, секция «пакет r20 — гейт владельца»
  (`:84-87`): «**„Подтверждаю пакет“** (03.10.2026)», запись владельца, до
  вызова `git`; первичная проверка не нашла строку — при resume дозаписана перед
  «Следующее действие», сверена. Снимок `git status --porcelain` = 8 `M` + 5 `??`
  = 13 + запись роли `git` (F43) → ожидание staged: 9 M + 5 A. В пакете указан
  `.opencode/memory/service.md`, но рабочее дерево по нему чистое (как в r19) —
  `add` no-op, в staged не появится. Вне пакета — `.credo/sandbox.json`
  (git-ignored, не добавлять). Чекпойнт и отчёт в ленту — до `add` (F43).
  Осталось: `add` точными путями (15) → `diff --cached --check` (пусто) →
  `diff --cached --name-status` (ожидание 9 M + 5 A; service.md не появится) →
  коммит `docs(process): T-27 — реестр находок: переезд в docs/registry/ и раскол
  (Q95/D98); приёмка` → `push origin develop` (таймаут ≥ 5 мин). Ветки не
  создаются/не удаляются, `master` не трогается; после `push` в отслеживаемые
  файлы не писать (хеши — в ответе `lead`).
- **03.10.2026, сервисный пакет r19 (T-15, C8 — развязка кандидатов B0, Q94/D97)** —
  лента `service-mcp-ready-r19`: база `develop` = `origin/develop` = `HEAD` =
  `8b21865` (`git log --oneline -3`, r18-коммит); ветки нет (сервисная операция,
  коммит в `develop`). Подтверждение владельца — лента, секция «закрытие — готово;
  пакет r19 — гейт владельца» (`:90-94`, «Подтверждаю пакет» (03.10.2026), запись
  владельца, до вызова `git`). **Снимок `git status --porcelain` = 10 `M` + 4 `??`
  = 14** (не 12 M): `.opencode/memory/service.md` — без изменений (в пакете
  указан, но рабочее дерево чистое; `add` — no-op), запись роли `git` (F43)
  допиcана → ожидание staged: 11 M + 4 A. Вне пакета — `.credo/sandbox.json`
  (чист; не добавлять). Чекпойнт и отчёт в ленту — до `add` (F43). Осталось:
  `add` точными путями (16) → `diff --cached --check` (пусто) →
  `diff --cached --name-status` (ожидание 11 M + 4 A; service.md не появится) →
  коммит `docs(process): T-15 r19 — C8: развязка кандидатов B0 (Q94/D97); приёмка`
  → `push origin develop` (таймаут ≥ 5 мин). Ветки не создаются/не удаляются,
  `master` не трогается; после `push` в отслеживаемые файлы не писать (хеши — в
  ответе `lead`).
- **03.10.2026, сервисный пакет r18 (T-15, C6 — метрики из состояния, D96/Q93)** —
  лента `service-mcp-ready-r18`: база `develop` = `origin/develop` = `HEAD` =
  `2b467f5` (`git log -1 --oneline`); ветки нет (сервисная операция, коммит в
  `develop`). Подтверждение владельца — лента, секция «закрытие — готово; пакет
  r18 — гейт владельца», `:137-140` («Подтверждаю пакет» (03.10.2026), запись
  владельца, до вызова `git`). Снимок совпал с пакетом — 17 `M` + 6 `??` = 23 +
  запись роли `git` (F43) = 24 (ожидание staged: 18 M + 6 A). Вне пакета —
  `.credo/sandbox.json` (продуктовые данные; не добавлять). Чекпойнт и отчёт в
  ленту — до `add` (F43). Осталось: `add` точными путями (24) → `diff --cached
  --check` (пусто) → `diff --cached --name-status` (ожидание 18 M + 6 A) → коммит
  `chore(process): T-15 r18 — C6: метрики из состояния (state-metrics.mjs, D96);
  приёмка` → `push origin develop` (таймаут ≥ 5 мин). Ветки не создаются/не
  удаляются, `master` не трогается; после `push` в отслеживаемые файлы не писать
  (хеши — в ответе `lead`).
- **03.10.2026, сервисный пакет r17 (T-15 → T-26, фаза E — дизайн process-MCP,
  Q92/D95)** — лента `service-mcp-ready-r17`: база `develop` = `origin/develop` =
  `HEAD` = `67ca43d` (`git log --oneline -3`); ветки нет (сервисная операция,
  коммит в `develop`). Подтверждение владельца — лента, секция «пакет r17 — гейт
  владельца» (:86, запись владельца/`lead`, до вызова `git`). Снимок совпал с
  пакетом — 10 `M` + 5 `??` = 15 + запись роли `git` (F43) = 16 (ожидание staged:
  11 M + 5 A). Вне пакета — нет (`.credo/sandbox.json` чист). Чекпойнт и отчёт в
  ленту — до `add` (F43). Осталось: `add` точными путями (16) → `diff --cached
  --check` (пусто) → `diff --cached --name-status` (ожидание 11 M + 5 A) → коммит
  `docs(process): T-26 — фаза E: дизайн process-MCP (Q92/D95); приёмка` →
  `push origin develop` (таймаут ≥ 5 мин). Ветки не создаются/не удаляются,
  `master` не трогается; после `push` в отслеживаемые файлы не писать (хеши — в
  ответе `lead`).
- **03.10.2026, сервисный пакет r16 (T-15, C11 — профиль-флаг B2, D94/Q91)** —
  лента `service-mcp-ready-r16`: база `develop` = `origin/develop` = `HEAD` =
  `92bf91c` (`git log --oneline -3`); ветки нет (сервисная операция, коммит в
  `develop`). Подтверждение владельца — лента, секция «закрытие — готово; пакет
  r16 — гейт владельца» (:85, запись владельца/`lead`, до вызова `git`). Снимок
  совпал с пакетом — 12 `M` + 5 `??` = 17 + запись роли `git` (F43) = 18
  (ожидание staged: 13 M + 5 A). Вне пакета — `.credo/sandbox.json` (продуктовые
  данные; не добавлять). Чекпойнт и отчёт в ленту — до `add` (F43). Осталось:
  `add` точными путями (18) → `diff --cached --check` (пусто) →
  `diff --cached --name-status` (ожидание 13 M + 5 A) → коммит `chore(process):
  T-15 r16 — C11: профиль-флаг B2 (off/codemode/mcp), тест профилей (D94);
  приёмка` → `push origin develop` (таймаут ≥ 5 мин). Ветки не создаются/не
  удаляются, `master` не трогается; после `push` в отслеживаемые файлы не
  писать (хеши — в ответе `lead`).
- **03.10.2026, сервисный пакет r15 (T-15, C3/C4 — session-commit, D93)** —
  лента `service-mcp-ready-r15`: база `develop` = `origin/develop` = `HEAD` =
  `f58e27c` (`git log -1 --oneline`); ветки нет (сервисная операция, коммит в
  `develop`). Подтверждение владельца — лента, секция «закрытие — готово; пакет
  r15 — гейт владельца» (:115, запись `lead`, до вызова `git`). Снимок совпал с
  пакетом — 20 `M` + 5 `??` = 25 + запись роли `git` (F43) = 26 (ожидание staged:
  21 M + 5 A). Вне пакета — `.credo/sandbox.json` (чист). Чекпойнт и отчёт в
  ленту — до `add` (F43). Осталось: `add` точными путями (26) → `diff --cached
  --check` (пусто) → `diff --cached --name-status` (ожидание 21 M + 5 A) →
  коммит `chore(process): T-15 r15 — session-commit: ветка сессии, теги,
  снапшот, фасад /git/checkpoint (C3/C4; D93); приёмка` → `push origin develop`
  (таймаут ≥ 5 мин). Ветки не создаются/не удаляются, `master` не трогается;
  после `push` в отслеживаемые файлы не писать (хеши — в ответе `lead`).
- **03.10.2026, сервисный пакет r14 (T-15, C17 — предикат зачётного прогона,
  D92)** — лента `service-mcp-ready-r14`: база `develop` = `origin/develop` =
  `HEAD` = `eeac481` (`git log -1 --oneline`); ветки нет. Подтверждение
  владельца — лента, секция «сервисная сессия · пакет r14 — гейт владельца»
  (запись `lead`/владельца, до вызова `git`). Снимок совпал с пакетом —
  12 `M` + 4 `??` = 16 + запись роли `git` (F43) = 17 (ожидание staged:
  13 M + 4 A). Вне пакета — `.credo/sandbox.json` (F83). Чекпойнт и отчёт в
  ленту — до `add` (F43). Осталось: `add` точными путями (17) → `diff --cached
  --check` (пусто) → `diff --cached --name-status` (ожидание 13 M + 4 A) →
  коммит `chore(process): T-15 r14 — предикат «зачётного прогона»
  (state-schema; D92); приёмка` → `push origin develop` (таймаут ≥ 5 мин).
  Ветки не создаются/не удаляются, `master` не трогается; после `push` в
  отслеживаемые файлы не писать (хеши — в ответе `lead`).
- **03.10.2026, сервисный пакет r12 (T-15, C5+C7, D90)** — лента
  `service-mcp-ready-r12`: база `develop` = `origin/develop` = `HEAD` = `6cd75cf`
  (`git log -1 --oneline`); ветки нет. Подтверждение владельца — лента, секция
  «гейт пакета — подтверждён» (`owner_response` дословно «Коммит + push develop
  (Recommended)»). Снимок совпал с пакетом — 17 `M` + 5 `??` = 22 + запись роли
  `git` (F43) = 23 (ожидание staged: 18 M + 5 A). Чекпойнт и отчёт в ленту — до
  `add` (F43). Осталось: `add` точными путями (23) → `diff --cached
  --name-status` (ожидание 18 M + 5 A) → коммит `chore(process): T-15 r12 —
  C5/C7 (re-raise, самоотчёт/фичи; D90); приёмка` → `push origin develop`. Ветки
  не создаются/не удаляются, `master` не трогается; после `push` в отслеживаемые
  файлы не писать (хеш — в ответе `lead`).
- **03.10.2026, сервисный пакет r11 (T-15, P1+P2+P3, D89)** — лента
  `service-mcp-ready-r11`: база `develop` = `origin/develop` = `HEAD` = `e826157`
  (`git log -1 --oneline`); ветки нет. Подтверждение владельца — лента, секция
  «гейт пакета — подтверждён» (`owner_response` дословно «Коммит + push develop
  (Recommended)»). Снимок совпал с пакетом — 18 `M` + 5 `??` = 23 + запись роли
  `git` (F43) = 24 (ожидание staged: 19 M + 5 A). Чекпойнт и отчёт в ленту — до
  `add` (F43). Осталось: `add` точными путями (24) → `diff --cached
  --name-status` (ожидание 19 M + 5 A) → коммит `chore(process): T-15 r11 —
  P1/P2/P3 (ветки дочерних задач, заморозка, session-commit; D89); приёмка` →
  `push origin develop`. Ветки не создаются/не удаляются, `master` не трогается;
  после `push` в отслеживаемые файлы не писать (хеш — в ответе `lead`).
- **02.10.2026, branch_end совмещённого пакета T-16+T-25** (лента `T-25.md`,
  пакет 32 пути): ветка `feature/T-25-d65-analysis-addresses` = origin = `HEAD`
  = `develop` = `origin/develop` = `2958bf5` (0 коммитов); `master` =
  `origin/master` = `46b98c2`. Подтверждение владельца — `progress.yaml`
  (02.10, T-16, iteration 2, `session_index: 4`, `surface_to_user`), дословно
  «Совмещённый пакет, 2 коммита (Recommended)». Снимок `git status --porcelain`
  = 34 пути (25 M + 9 ??), `add_paths` 32 = 34 − 2 чужих сервисных
  (`mail/service-mcp-ready-r10.md`, `memory/service.md`). Чекпойнт и отчёт в
  ленту — до `add` (F43). Осталось: продуктовый `add` (19) → коммит `code(T-16,
  T-25): stale-тест по тексту файла + снятие D65-адресов (Q12/D54, Q61/D65)` →
  процессный `add` (14) → коммит `chore(process): записи прогона T-16/T-25` →
  `switch develop` → `pull origin develop` → `merge --no-ff` → `push origin
  develop` → удаление обеих веток (local + origin; CCSN на remote — владелец
  вручную, D87). `master` не трогается; после `push` в отслеживаемые файлы не
  писать.
- **02.10.2026, branch_start T-25** (лента `T-25.md`, действие плана
  `branch_start`; класс S, цикл T-25-first — разблокировка чистого пакета T-16,
  F43): база `develop` = `origin/develop` = `HEAD` = `2958bf5` (проверено
  `git rev-parse develop origin/develop HEAD`); `master` = `origin/master` =
  `46b98c2`. Ветки `feature/T-25-d65-analysis-addresses` нет локально и на
  origin (`git ls-remote --heads` пусто). Подтверждение владельца — лента
  `T-25.md` + `progress.yaml` (02.10, T-25, iteration 1, `session_index: 4`,
  `surface_to_user`, `owner_response` дословно «Подтверждаю ветку и порядок
  (Recommended)»). Дерево (24 M + 6 ?? = 30 путей): M git-независимые — пакет
  T-16 (`src/lib.rs`, `src/mcp.rs`, `tests/mcp_draft.rs`, `docs/CHANGELOG.md`,
  `docs/TRACEABILITY.md`, `docs/tasks/README.md`,
  `docs/tasks/T-16-stale-check-test/README.md`, `docs/analysis/findings-registry.md`,
  `docs/decisions/D87…`, `docs/decisions/D88…`, `docs/questions/Q84.md`,
  `docs/questions/Q85.md`, `docs/reviews/T-16-*` (untracked), `docs/analysis/T-16-*`,
  `mail/T-16.md`), state/memory сервисные M + `mail/service-mcp-ready-r10.md`,
  S-файлы T-25 (`docs/tasks/T-25-d65-analysis-addresses/` untracked) — переносятся
  в ветку как есть, не коммитятся. Чекпойнт и отчёт в ленту — до операции
  (F43-паттерн). Осталось: `switch develop` → `pull origin develop` → `switch -c
  feature/T-25-d65-analysis-addresses develop` → `push -u origin
  feature/T-25-d65-analysis-addresses`. `master` не трогается; коммитов нет.
- **02.10.2026, сервисный пакет r10 (T-15, C9/C12, D88)** — лента
  `service-mcp-ready-r10`: база `develop` = `origin/develop` = `HEAD` = `af53e23`
  (проверено `git log -1 --oneline`); ветки нет. Подтверждение владельца —
  лента, секция «гейт пакета — подтверждён» (`owner_response` дословно «Коммит +
  push develop (Recommended)»). Снимок совпал с пакетом — 15 `M` + 4 `??` = 19
  + запись роли `git` (F43) = 20 (ожидание staged: 16 M + 4 A). Чекпойнт и
  отчёт в ленту — до `add` (F43). Осталось: `add` точными путями (20) →
  `diff --cached --name-status` (ожидание 16 M + 4 A) → коммит `chore(process):
  T-15 r10 — C9/C12 (лимит lead 24, записи, re-plan; D88); приёмка` →
  `push origin develop`. Ветки не создаются, `master` не трогается; после
  `push` в отслеживаемые файлы не писать (хеш — в ответе `lead`).
- **02.10.2026, сервисный пакет r9 (T-15, правки S-пилота T-24)** — лента
  `service-mcp-ready-r9`: база `develop` = `origin/develop` = `HEAD` = `6c28e51`
  (проверено `git log -1 --oneline`); ветки нет. Подтверждение владельца —
  лента, секция «гейт пакета — подтверждён» (`owner_response` дословно «Коммит +
  push develop (Recommended)»). Снимок совпал с пакетом — 23 `M` + 5 `??` = 28
  + запись роли `git` (F43) = 29 (ожидание staged: 24 M + 5 A). Чекпойнт и
  отчёт в ленту — до `add` (F43). Осталось: `add` точными путями (29) →
  `diff --cached --name-status` (ожидание 24 M + 5 A) → коммит `chore(process):
  T-15 r9 — правки по S-пилоту T-24 (права/CCSN/state/модель, D87); приёмка` →
  `push origin develop`. Ветки не создаются, `master` не трогается; после
  `push` в отслеживаемые файлы не писать (хеш — в ответе `lead`).
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
