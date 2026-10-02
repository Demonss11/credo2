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

- **30.09.2026, service-docs-lifecycle №1** (`research`): подтверждение сверено
  по ленте; снимок 16 `M` + 1 `D` + 2 `??`, `HEAD` `9948ebf`; к коммиту — 20
  путей (`./`-префикс, без `--`), режим «коммит + push» в `develop`, сообщение
  `docs(D65): рабочие артефакты research/reviews/analysis — удаление по
  отработке, ссылки канона сняты (service-docs-lifecycle №1)`. Хеши — в ответе
  `lead` (не здесь). Осталось: `add` → сверка staged (17 `M` + 1 `D` + 2 `A`) →
  `commit` → `push`.
- **30.09.2026, service-docs-lifecycle №2** (служебная зона, режим
  «коммит + push» в `develop`): подтверждение сверено по записи ленты;
  снимок 12 `M` + 1 `??` = ровно пакет, плюс вне пакета 68 ` D docs/reviews/**`
  и 5 `M` карточек; `HEAD` `21c3c80` = база `origin/develop`. К коммиту — 13
  путей (`./`-префикс, без `--`), сообщение
  `docs(D49): служебная зона — правило «ссылки на review не ставим», право
  validator git rev-parse (service-docs-lifecycle №2)`. Хеши — в ответе `lead`
  (не здесь). Осталось: `add` → сверка staged (12 `M` + 1 `A`) → `commit` →
  `push`. Удаления `docs/reviews/**` и карточки — не трогать (будущий пакет).
- **30.09.2026, service-docs-lifecycle №2 — дельта allowlist**
  (`ls-files`/`check-ignore`, служебная зона, режим «коммит + push» в `develop`):
  подтверждение сверено по записи ленты («Коммит + push»); снимок 9 `M` + 1 `??`
  = ровно пакет (плюс вне пакета 70 ` D docs/reviews/**`, 5 `M` карточек,
  `findings-registry.md`, `memorandum-W8-run5.md`); `HEAD` `f28c8cb` = база
  `origin/develop`. К коммиту — 11 путей (`./`-префикс, без `--`), сообщение
  `docs(D49): права validator/auditor — git ls-files/check-ignore
  (service-docs-lifecycle №2)`. Хеши — в ответе `lead` (не здесь). Осталось:
  `add` → сверка staged (10 `M` + 1 `A`) → `commit` → `push`.
- **01.10.2026, service-statuses-review** (служебная зона, режим «коммит + push»
  прямо в `master` — санкция владельца, исключение из `git-workflow`):
  подтверждение сверено по записи ленты «01.10.2026 · подтверждение пакета»;
  снимок 10 `M` + 3 `??` = ровно пакет (лента и два отчёта приёмки — новые),
  `HEAD` `9173fc4` = база `origin/master`. К коммиту — 13 путей (`./`-префикс,
  без `--`), сообщение
  `docs(D38/D70): канон ролей — сняты дубли слияния и устаревшие §10-пункты
  (service-statuses-review)`. Хеши — в ответе `lead` (не здесь). Осталось:
  `add` → сверка staged (10 `M` + 3 `A`) → `commit` → `push origin master`.
  Примечание: после `push` в отслеживаемые файлы не писать (F43).
- **01.10.2026, service-branch-align** (служебная зона, режим «коммит + push»
  в `master`, затем merge `master` → `develop` — санкция владельца
  «выравниваем»): подтверждение сверено по записи ленты «открытие и пакет»;
  снимок 1 `M` + 1 `??` = ровно пакет (ленты нет — новая, память — правка),
  `HEAD` `3990700` = база `origin/master`, дерево до записей чистое. К коммиту —
  3 пути (`./`-префикс, без `--`), сообщение
  `chore(process): выравнивание develop по master — записи
  (service-branch-align)`. Хеши — в ответе `lead` (не здесь). Осталось:
  `add` → сверка staged (1 `A` + 2 `M`) → `commit` → `push origin master` →
  `switch develop` → `merge --no-ff master` → `push origin develop`.
- **01.10.2026, service-review-links** (служебная зона, режим «коммит + push»
  прямо в `develop`, санкция владельца): подтверждение сверено по записи ленты
  «01.10.2026 · подтверждение пакета (гейт)»; снимок 5 `M` + 2 `??` = ровно
  пакет (лента и отчёт приёмки — новые), `HEAD` `b9fd791` = база
  `origin/develop`. К коммиту — 8 путей (`./`-префикс, без `--`), сообщение
  `docs(T-03): снята запрещённая ссылка на отчёт приёмки из карточки
  (review.md §«Хранение отчётов»)`. Хеши — в ответе `lead` (не здесь). Осталось:
  `add` → сверка staged (6 `M` + 2 `A`) → `commit` → `push origin develop`.
  Примечание: после `push` в отслеживаемые файлы не писать (F43).
- **01.10.2026, service-lifecycle-w2-prep** (служебная волна, режим «коммит + push»
  прямо в `develop`, санкция владельца): подтверждение сверено по записи ленты
  «01.10.2026 · подтверждение пакета (гейт)»; снимок 8 `M` + 5 `??` (лента, Q79,
  D83, отчёт приёмки, папка T-20 — новые) = ровно пакет + мой чекпойнт = 14; `HEAD`
  `b04a77a` = база `origin/develop`. К коммиту — 14 путей (`./`-префикс, без `--`),
  сообщение `docs(Q79/D83): волна 2 TRACEABILITY — разбор open-строк задачей T-20
  (service-lifecycle-w2-prep)`. Хеши — в ответе `lead` (не здесь). Осталось:
  `add` → сверка staged (9 `M` + 5 `A`) → `commit` → `push origin develop`.
  Примечание: после `push` в отслеживаемые файлы не писать (F43).
- **01.10.2026, service-lifecycle-w1** (служебная волна, режим «коммит + push»
  прямо в `develop`, санкция владельца): подтверждение сверено по записи ленты
  «01.10.2026 · подтверждение пакета (гейт)»; снимок 12 `M` + 4 `??` = ровно
  пакет (лента, Q78, D82, отчёт приёмки — новые) + мой чекпойнт = 17; `HEAD`
  `cb7d159` = база `origin/develop`. К коммиту — 17 путей (`./`-префикс, без
  `--`), сообщение
  `docs(Q78/D82): жизненный цикл TRACEABILITY — open · in work · done; волна 1
  разметки (service-lifecycle-w1)`. Хеши — в ответе `lead` (не здесь). Осталось:
  `add` → сверка staged (13 `M` + 4 `A`) → `commit` → `push origin develop`.
  Примечание: после `push` в отслеживаемые файлы не писать (F43).
- **01.10.2026, service-rules-revision** (служебная волна, режим «коммит + push»
  прямо в `develop`, санкция владельца): подтверждение сверено по записи ленты
  «01.10.2026 · подтверждение пакета (гейт)» (волна 1); снимок 14 `M` + 4 `??`
  (лента, Q80, D84, отчёт приёмки — новые) = ровно пакет; `HEAD` `3821811` =
  база `origin/develop` (`## develop...origin/develop`). К коммиту — 18 путей
  (`./`-префикс, без `--`), сообщение
  `docs(Q80/D84): ревизия .opencode/rules — норма без истории, дедупликация,
  якоря (service-rules-revision)`. Хеши — в ответе `lead` (не здесь). Осталось:
  `add` → сверка staged (14 `M` + 4 `A`) → `commit` → `push origin develop`.
  Примечание: после `push` в отслеживаемые файлы не писать (F43).
- **01.10.2026, service-rules-revision-w2** (служебная волна, режим «коммит + push»
  прямо в `develop`, санкция владельца): подтверждение сверено по записи ленты
  «сервисная сессия · 01.10.2026 · подтверждение пакета (гейт)»; снимок
  15 `M` + 2 `??` (лента, отчёт приёмки — новые) = ровно пакет + мой чекпойнт = 17;
  `HEAD` `52d8989` = база `origin/develop`. К коммиту — 17 путей (`./`-префикс,
  без `--`), сообщение
  `docs(Q80/D84): волна 2 канона агентов — якорь R5, чистка agents/**
  (service-rules-revision-w2)`. Хеши — в ответе `lead` (не здесь). Осталось:
  `add` → сверка staged (15 `M` + 2 `A`) → `commit` → `push origin develop`.
  Примечание: после `push` в отслеживаемые файлы не писать (F43).
- **01.10.2026, service-rules-revision-w2 (P3, закрытие)** (служебная волна,
  режим «коммит + push» в `develop`, затем merge `develop` → `master` + push —
  санкция владельца): подтверждение сверено по записи ленты «сервисная сессия ·
  01.10.2026 · подтверждение пакета (гейт) — коммит + push + merge в master»;
  снимок 9 `M` + 1 `??` (отчёт приёмки — новый) = ровно пакет + мой чекпойнт = 10;
  `HEAD` `c8ffb0f` = база `origin/develop`; `master` @ `9173fc4` — предок, конфликтов
  нет. К коммиту — 10 путей (`./`-префикс, без `--`), сообщение
  `docs(Q80/D84): канон ролей — «см.» в ссылках на R2 (P3,
  service-rules-revision-w2)`. Хеши — в ответе `lead` (не здесь). Осталось:
  `add` → сверка staged (9 `M` + 1 `A`) → `commit` → `push origin develop` →
  `switch master` → `merge --no-ff develop` → `push origin master`.
  Примечание: после `push` в отслеживаемые файлы не писать (F43).
- **02.10.2026, service-question-kodaskills** (служебная волна, режим «коммит + push»
  прямо в `develop`, санкция владельца): подтверждение сверено по записи ленты
  «lead · 2026-10-02 · гейт пройден (surface_to_user)» («Коммит + push develop
  (Recommended)»); снимок до записей 8 `M` + 3 `??` = ровно пакет (лента, Q81,
  отчёт приёмки — новые) + мой чекпойнт = 12 (9 `M` + 3 `A`); `HEAD` `c2f905f` =
  база `origin/develop` (`## develop...origin/develop`). К коммиту — 12 путей
  (`./`-префикс, без `--`), сообщение
  `docs(Q81): вопрос об изучении внешнего материала KodaSkills
  (service-question-kodaskills)`. Хеши — в ответе `lead` (не здесь). Осталось:
  `add` → сверка staged (9 `M` + 3 `A`) → `commit` → `push origin develop`.
  Примечание: после `push` в отслеживаемые файлы не писать (F43).
- **02.10.2026, T-18 branch_start** (`feature/T-18-docs-journal-test`, режим
  «создать ветку + push», коммитов нет): подтверждение сверено по ленте
  `.opencode/mail/T-18.md` — «lead · 2026-10-02 · гейт ветки пройден
  (surface_to_user)», ответ владельца «Создать ветку + push (Recommended)»;
  база `HEAD` `5786875` = `origin/develop` (`## develop...origin/develop`).
  Снимок дерева (не трогается): 4 `M` (mail/service-question-kodaskills,
  state/current_state, state/next_action, state/progress) + 2 `??`
  (mail/T-18.md, docs/analysis/T-18-2026-10-02.md) — остаются в дереве, их
  подхватит пакет branch_end. Осталось: `switch -c
  feature/T-18-docs-journal-test develop` → `push -u origin
  feature/T-18-docs-journal-test`. `--force`/`reset --hard`/`rebase`/удаление
  веток запрещены; база не совпала бы — стоп и возврат `lead` (re-plan).
- **02.10.2026, T-21 branch_start** (`feature/T-21-mcp-test-struct-api`, режим
  «создать ветку + push», коммитов нет): подтверждение сверено по ленте
  `.opencode/mail/T-18.md` — «lead · 2026-10-02 · гейт T-21 пройден
  (surface_to_user)», ответ владельца «Ветка + порядок (Recommended)»; база
  `HEAD` `5786875` = `origin/develop` (`## develop...origin/develop`).
  Рабочее дерево — незакоммиченные пути T-18 в ветке
  `feature/T-18-docs-journal-test`; не коммитить, не сбрасывать (пакет
  branch_end T-18). Осталось: `switch -c
  feature/T-21-mcp-test-struct-api develop` → `push -u origin
  feature/T-21-mcp-test-struct-api`. `--force`/`reset --hard`/`rebase`/удаление
  веток запрещены; база не совпала бы — стоп и возврат `lead` (re-plan).
- **02.10.2026, T-22 branch_start** (`feature/T-22-mcp-draft-test-fix`, режим
  «создать ветку + push», коммитов нет): подтверждение сверено по ленте
  `.opencode/mail/T-18.md` (строки 171–177) — «lead · 2026-10-02 · гейт T-22
  пройден (surface_to_user)», ответ владельца «Ветка T-22 + порядок
  (Recommended)»; база `HEAD` `5786875` = `origin/develop` (обе сверены
  `rev-parse`). Текущая ветка — `feature/T-21-mcp-test-struct-api` на `5786875`;
  дерево смешанное (23 `M` + 10 `??`; незакоммиченные пути T-18/T-21) — рабочие
  пути не трогать, не коммитить, не сбрасывать. Осталось: `switch -c
  feature/T-22-mcp-draft-test-fix develop` → `push -u origin
  feature/T-22-mcp-draft-test-fix` → отчёт в ленту T-22 (открыть) + дописать
  чекпойнт. `--force`/`reset --hard`/`rebase`/удаление веток запрещены;
  `develop` не трогать; база не совпала бы — стоп и возврат `lead` (re-plan).
  **ГОТОВО:** ветка создана от `5786875` и опубликована
  (`push -u origin feature/T-22-mcp-draft-test-fix`, `* [new branch]`, upstream
  установлен); коммитов нет; `develop` не тронут; рабочее дерево сохранено
  (23 `M` + 10 `??`); отчёт в ленте `T-22.md`.
- **02.10.2026, T-21/T-18/T-22 closeout + master** (три feature-ветки на
  `5786875` = `develop` = `origin/develop`, коммитов нет; порядок T-21 → T-18 →
  T-22; режим «commit + merge --no-ff + push + delete branch», финал — merge
  develop → master). Подтверждение сверено по ленте `.opencode/mail/T-21.md` —
  «lead · 2026-10-02 · гейт пройден (surface_to_user) — 3 пакета + master»,
  ответ владельца «Подтверждаю: 3 пакета + master (Recommended)» (и одноимённая
  запись в `.opencode/mail/T-18.md`). Составы пакетов (add_paths) — в записи
  гейта: T-21 = 16 путей, T-18 = 27 путей, T-22 = 6 путей; общие файлы
  (`docs/tasks/README.md`, `docs/CHANGELOG.md`, `state/current/progress.yaml`,
  `receipts.yaml`, `memory/tester.md`) фиксируются первым коммитом (T-21), при
  поздних switch откатываются к базе и в staged не появляются — штатно (F43).
  Снимок 42 пути (27 `M` + 15 `??`) покрыт union'ом. К коммитам — точные пути
  пакетов (`./`-префикс, без `--`), сообщения:
  `code(T-21): struct-API ToolError в unit-тестах src/mcp.rs`,
  `code(T-18): тест целостности журнала tests/docs_journal.rs`,
  `code(T-22): снятие ошибок компиляции tests/mcp_draft.rs`. Записи роли git
  (этот чекпойнт + отчёт в ленту T-21) — один раз до первого `add` (F43).
  Осталось: T-21 add→staged-сверка→commit→switch develop→pull→merge --no-ff→push
  develop→delete branch (local+origin); то же T-18, T-22; затем `switch master` →
  `pull origin master` → `merge --no-ff develop` → `push origin master` → `switch
  develop`. Хеши — в ответе `lead` (не здесь). После финального push в
  отслеживаемые файлы не писать.
- **02.10.2026, T-23 closeout** (вариант Б — коммит прямо в `develop`, ветки
  нет; затем merge develop → master). Подтверждение сверено по записи ленты
  `.opencode/mail/service-traceability-closeout.md` — «lead · 2026-10-02 · гейт
  пакета пройден (surface_to_user)», ответ владельца «Подтверждаю: коммит +
  develop + master (Recommended)». База: `develop` = `origin/develop` = `71ece40`;
  `master` = `origin/master` = `c6ebc41`; дерево — снимок 13 путей (10 `M` + 3
  `??`). К коммиту — 14 путей (13 пакета + этот чекпойнт, F43):
  `docs/TRACEABILITY.md`, `docs/analysis/findings-registry.md`,
  `docs/analysis/T-23-2026-10-02.md`, `docs/reviews/T-23-2026-10-02.md`,
  `.opencode/mail/service-traceability-closeout.md`, `.opencode/mail/T-18.md`,
  `.opencode/state/current/{progress,next_action,current_state,receipts}.yaml`,
  `.opencode/memory/{migrator,validator,auditor}.md`, `.opencode/memory/git.md`.
  Сообщение: `docs(T-23): синхронизация TRACEABILITY (статусы T-18/T-21/T-22,
  жизненный цикл Q60/Q73/Q76) + F57`. Осталось: `add` 14 путей (`./`-префикс,
  без `--`) → `diff --cached --name-status` (11 `M` + 3 `A`) → `commit` →
  `push origin develop` → `switch master` → `pull origin master` → `merge --no-ff
  develop` → `push origin master` → `switch develop`. Хеши — в ответе `lead`
  (не здесь). После push в отслеживаемые файлы не писать (F43).
- **02.10.2026, service-t15-run-review** (служебная волна, режим «коммит + push»
  прямо в `develop`, затем merge develop → master). Подтверждение сверено по
  ленте `.opencode/mail/service-t15-run-review.md` — §«гейт пакета пройден»
  (:85–88), ответ владельца «Коммит + develop + master (Recommended)» (дубль —
  `progress.yaml` :879). База: `develop` = `origin/develop` = `98f7225`;
  `master` = `origin/master` = `e696e77`; дерево — снимок 11 `M` + 8 `??` = 19
  путей, ровно пакет. К коммиту — 20 путей (19 пакета + этот чекпойнт, F43):
  `docs/TRACEABILITY.md`, `docs/analysis/findings-registry.md`,
  `docs/tasks/README.md`, `docs/tasks/T-15-mcp-ready-process/README.md`,
  `.opencode/memory/{service,migrator,auditor,validator,git}.md`,
  `.opencode/state/current/{progress,receipts}.yaml`,
  `.opencode/mail/{service-traceability-closeout,service-t15-run-review}.md`,
  `docs/analysis/T-15-run-2026-10-02-{build,analyst,tester,migrator,validator,git}-session.md`,
  `docs/reviews/service-t15-run-review-2026-10-02.md`. Сообщение:
  `docs(T-15): разбор прогона 02.10 (B1: F26/F27/F15), F58–F61, статусы T-15
  (service-t15-run-review)`. Осталось: `add` 20 путей (`./`-префикс, без `--`) →
  `diff --cached --name-status` (12 `M` + 8 `A`) → `commit` → `push origin develop`
  → `switch master` → `pull origin master` → `merge --no-ff develop` →
  `push origin master` → `switch develop`. Хеши — в ответе `lead` (не здесь).
  После push в отслеживаемые файлы не писать (F43).
- **02.10.2026, service-mcp-ready-r5 BO-i2 — wave0-guard** (служебная волна,
  режим «коммит + push» прямо в `develop`; ветки нет, master не трогаем).
  Подтверждение сверено по ленте `.opencode/mail/service-mcp-ready-r5.md` —
  §«гейт пакета BO-i2 пройден» (:62–76), ответ владельца «Коммит BO-i2 +
  BO-i3». База: `develop` = `origin/develop` = `fed94d6`; дерево — снимок
  6 `M` + 1 `??` (лента r5 — новая) = ровно пакет 7 путей. К коммиту — 8 путей
  (7 пакета + этот чекпойнт, F43): `.opencode/mail/service-t15-run-review.md`,
  `.opencode/memory/migrator.md`, `.opencode/memory/service.md`,
  `.opencode/memory/git.md`, `.opencode/state/current/progress.yaml`,
  `docs/tasks/T-15-mcp-ready-process/README.md`,
  `docs/tasks/T-15-mcp-ready-process/wave0b-own.md`,
  `.opencode/mail/service-mcp-ready-r5.md`. Сообщение:
  `chore(process): T-15 B0-own BO-i2 — wave0-guard: пробы, вердикт, карточка P2`.
  Ожидаемая staged-сверка: 7 `M` + 1 `A`. Осталось: `add` 8 путей (`./`-префикс,
  без `--`) → `diff --cached --name-status` → `commit` → `push origin develop`.
  Хеши — в ответе `lead` (не здесь). После push в отслеживаемые файлы не писать
  (F43); BO-i3 — отдельным действием.
- **02.10.2026, service-mcp-ready-r5 BO-i3 — wave0-observe** (служебная волна,
  режим «коммит + push» прямо в `develop`; ветки нет, master не трогаем).
  Подтверждение сверено по ленте `.opencode/mail/service-mcp-ready-r5.md` —
  §«гейт пакета BO-i3 пройден» (:115–125), ответ владельца «Коммит BO-i3 +
  BO-i4». База: `develop` = `origin/develop` = `85658fd`; дерево — снимок
  5 `M` = ровно пакет 5 путей (+ этот чекпойнт). К коммиту — 6 путей (5 пакета +
  этот чекпойнт, F43): `.opencode/mail/service-mcp-ready-r5.md`,
  `.opencode/memory/migrator.md`, `.opencode/memory/service.md`,
  `.opencode/memory/git.md`, `docs/tasks/T-15-mcp-ready-process/README.md`,
  `docs/tasks/T-15-mcp-ready-process/wave0b-own.md`. Сообщение:
  `chore(process): T-15 B0-own BO-i3 — wave0-observe: пробы, вердикт, карточка P1`.
  Ожидаемая staged-сверка: 5 `M` + 1 `A`. Осталось: `add` 6 путей (`./`-префикс,
  без `--`) → `diff --cached --name-status` → `commit` → `push origin develop`.
  Хеши — в ответе `lead` (не здесь). После push в отслеживаемые файлы не писать
  (F43).
- **02.10.2026, service-mcp-ready-r5 BO-i4 — metrics-report** (служебная волна,
  режим «коммит + push» прямо в `develop`; ветки нет, master не трогаем).
  Подтверждение сверено по ленте `.opencode/mail/service-mcp-ready-r5.md` —
  §«гейт пакета BO-i4 пройден» (:164–174), ответ владельца «Коммит BO-i4 +
  BO-i5». База: `develop` = `origin/develop` = `9a811b4`; дерево — снимок
  5 `M` = ровно пакет 5 путей (+ этот чекпойнт). К коммиту — 6 путей (5 пакета +
  этот чекпойнт, F43): `.opencode/mail/service-mcp-ready-r5.md`,
  `.opencode/memory/migrator.md`, `.opencode/memory/service.md`,
  `.opencode/memory/git.md`, `docs/tasks/T-15-mcp-ready-process/README.md`,
  `docs/tasks/T-15-mcp-ready-process/wave0b-own.md`. Сообщение:
  `chore(process): T-15 B0-own BO-i4 — metrics-report: пробы, вердикт, карточка P4`.
  Ожидаемая staged-сверка: 5 `M` + 1 `A`. Осталось: `add` 6 путей (`./`-префикс,
  без `--`) → `diff --cached --name-status` → `commit` → `push origin develop`.
  Хеши — в ответе `lead` (не здесь). После push в отслеживаемые файлы не писать
  (F43).
- **02.10.2026, service-mcp-ready-r5 BO-i5 — wave0-checkpoint** (служебная волна,
  режим «коммит + push» прямо в `develop`; ветки нет, master не трогаем).
  Подтверждение сверено по ленте `.opencode/mail/service-mcp-ready-r5.md` —
  §«гейт пакета BO-i5 пройден (коммит; пауза)» (:217–227), ответ владельца
  «Коммит BO-i5, пауза». База: `develop` = `origin/develop` = `11110bd`; дерево —
  снимок 5 `M` = ровно пакет 5 путей (+ этот чекпойнт, F43). К коммиту — 6 путей:
  `.opencode/mail/service-mcp-ready-r5.md`, `.opencode/memory/migrator.md`,
  `.opencode/memory/service.md`, `.opencode/memory/git.md`,
  `docs/tasks/T-15-mcp-ready-process/README.md`,
  `docs/tasks/T-15-mcp-ready-process/wave0b-own.md`. Сообщение:
  `chore(process): T-15 B0-own BO-i5 — wave0-checkpoint: пробы, вердикт, карточка P3`.
  Ожидаемая staged-сверка: 6 `M`. Осталось: `add` 6 путей (`./`-префикс, без `--`)
  → `diff --cached --name-status` → `commit` → `push origin develop`. Хеши — в
  ответе `lead` (не здесь). После push в отслеживаемые файлы не писать (F43).
- **02.10.2026, service-mcp-ready-r5 BO-i6/i7 — wave0-attribution (свёрнут), отчёт,
  аудит** (служебная волна, режим «коммит + push» прямо в `develop`; ветки нет,
  master не трогаем). Подтверждение сверено по ленте
  `.opencode/mail/service-mcp-ready-r5.md` — §«гейт BO-i6/i7 пройден (коммит +
  перенос)» (:306–319), ответ владельца «Без P2 до C10». База: `develop` =
  `origin/develop` = `e740603`; дерево — снимок 6 `M` + 1 `??` = ровно пакет
  7 путей (+ этот чекпойнт, F43). К коммиту — 8 путей: `.opencode/mail/service-mcp-ready-r5.md`,
  `.opencode/memory/auditor.md`, `.opencode/memory/migrator.md`,
  `.opencode/memory/service.md`, `.opencode/memory/git.md`,
  `docs/tasks/T-15-mcp-ready-process/README.md`,
  `docs/tasks/T-15-mcp-ready-process/wave0b-own.md`,
  `docs/tasks/T-15-mcp-ready-process/wave0b-own-report.md`. Сообщение:
  `chore(process): T-15 B0-own BO-i6/i7 — wave0-attribution (свёрнут), отчёт, аудит`.
  Ожидаемая staged-сверка: 7 `M` + 1 `A`. Осталось: `add` 8 путей (`./`-префикс,
  без `--`) → `diff --cached --name-status` → `commit` → `push origin develop`.
  Хеши — в ответе `lead` (не здесь). После push в отслеживаемые файлы не писать
  (F43); перенос P1/P3/P4 — отдельным пакетом (сервисная сессия).
- **02.10.2026, service-mcp-ready-r5 — перенос P1/P3/P4 (B0-own)** (служебная
  волна, режим «коммит + push» прямо в `develop`; ветки нет, master не трогаем).
  Подтверждение сверено по ленте `.opencode/mail/service-mcp-ready-r5.md` —
  §«гейт пакета переноса пройден» (:346–359), ответ владельца «Коммит + push
  develop». База: `develop` = `origin/develop` = `ac5d382`; дерево — снимок
  8 `M` + 4 `??` = ровно пакет 12 путей (+ этот чекпойнт, F43). К коммиту —
  13 путей: `.opencode/plugins/wave0-observe.ts`,
  `.opencode/scripts/session-checkpoint.mjs`,
  `.opencode/scripts/metrics-report.mjs`,
  `docs/reviews/service-b0-own-transfer-2026-10-02.md`,
  `.opencode/mail/service-mcp-ready-r5.md`,
  `.opencode/memory/{migrator,service,validator,git}.md`,
  `.opencode/state/current/receipts.yaml`,
  `docs/tasks/T-15-mcp-ready-process/{README.md,wave0b-own.md,wave0b-own-report.md}`.
  Сообщение: `chore(process): T-15 B0-own — перенос P1/P3/P4 в служебную зону
  (wave0-observe, session-checkpoint, metrics-report)`. Ожидаемая staged-сверка:
  9 `M` + 4 `A`. Осталось: `add` 13 путей (`./`-префикс, без `--`) →
  `diff --cached --name-status` → `commit` → `push origin develop`. Хеши — в
  ответе `lead` (не здесь). После push в отслеживаемые файлы не писать (F43).
- **02.10.2026, service-mcp-ready-r6 C10 — policies + плагин wave0-guard**
  (служебная волна, режим «коммит + push» прямо в `develop`; ветки нет, master
  не трогаем). Подтверждение сверено по ленте `.opencode/mail/service-mcp-ready-r6.md`
  — §«гейт пакета C10 пройден» (:74–88), ответ владельца «Коммит + push develop».
  База: `develop` = `origin/develop` = `dace3eb`; дерево — снимок 8 `M` + 3 `??`
  = ровно пакет 12 путей (+ этот чекпойнт, F43). К коммиту — 13 путей:
  `.opencode/plugins/wave0-guard.ts`, `.opencode/mail/service-mcp-ready-r6.md`,
  `docs/reviews/service-c10-policies-2026-10-02.md`,
  `.opencode/mail/service-mcp-ready-r5.md`,
  `.opencode/memory/{migrator,service,validator,git}.md`,
  `.opencode/state/current/receipts.yaml`,
  `docs/tasks/T-15-mcp-ready-process/{README.md,wave0b-own.md,wave0b-own-report.md}`.
  Сообщение: `chore(process): T-15 C10 — policies (4 точных) + плагин
  wave0-guard (P2); приёмка`. Ожидаемая staged-сверка: 9 `M` + 4 `A`. Осталось:
  `add` 13 путей (`./`-префикс, без `--`) → `diff --cached --name-status` →
  `commit` → `push origin develop`. Хеши — в ответе `lead` (не здесь). После push
  в отслеживаемые файлы не писать (F43).
- **02.10.2026, service-mcp-ready-r6 — хвосты F62–F66 (C10/B0-own)** (служебная
  волна, режим «коммит + push» прямо в `develop`; ветки нет, master не трогаем).
  Подтверждение сверено по ленте `.opencode/mail/service-mcp-ready-r6.md`
  — §«пакет F62–F66 — гейт пройден (коммит)» (:125–135), ответ владельца
  «Внести F62–F66 сейчас … и коммит/пуш develop». База: `develop` = `origin/develop`
  = `HEAD` = `142458d`; дерево — снимок 5 `M` = ровно пакет 5 путей (+ этот
  чекпойнт, F43). К коммиту — 6 путей: `docs/analysis/findings-registry.md`,
  `docs/tasks/T-15-mcp-ready-process/README.md`,
  `.opencode/mail/service-mcp-ready-r6.md`,
  `.opencode/memory/migrator.md`, `.opencode/memory/service.md`,
  `.opencode/memory/git.md`. Сообщение:
  `docs(T-15): findings F62–F66 (C10/B0-own) + B0-own-P2 закрыт (C10)`.
  Ожидаемая staged-сверка: 6 `M`. Осталось: `add` 6 путей (`./`-префикс, без `--`)
  → `diff --cached --name-status` → `commit` → `push origin develop`. Хеши — в
  ответе `lead` (не здесь). После push в отслеживаемые файлы не писать (F43).
