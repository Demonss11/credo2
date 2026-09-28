# Сервисная лента: service-migration-q4 (перенос Q4 → D17)

**Назначение:** миграция журнала — **Q4** «`Приоритет` входит в MVP?»
(`SPECIFICATION.md` §10 №17: `Приоритет` вне MVP; синтаксис не входит в v0.1,
вернётся в v0.2 с конвейерами).
**Основание:** решение владельца 29.09.2026 («давай Q4 преносить»); порядок —
`docs/BRIEF.md` §7 (раздел 0, по возрастанию Q). Пилот — Q1 → D15; предыдущий
блок — Q2+Q3 → D16 (коммит `f488085`, `develop` ahead 1, без push).
**Маршрут:** `migrator` → `docs-writer` (сопутствующие) → `validator` → `git`
(docs-коммит по подтверждению).

Формат записей — `AGENTS.md` §«Память и почта».

---

## сервисная сессия · 29.09.2026 · открытие операции

- Сделано: сверка исходников до вызова ролей — архивный блок Q4
  (`OPEN_QUESTIONS.md:46-64`, решено 2026-09-24); `SPECIFICATION.md` §10 №17;
  `GRAMMAR.md` §4 (таблица: `Приоритет` — ❌, вернётся с конвейерами) и §5;
  нота Q4 в `features/README.md:75-78`; `SPECIFICATION.md:760` (инлайн-нота
  §7).
- Проверки: фактический список фич с «приоритетом» (`rg -li`) —
  `parser`, `execution`, `editor`, `lsp`, `client_explanation` (синтаксис
  `Приоритет: 100;` и сортировка причин) + `semver` (иное: приоритет MAJOR).
  **Расхождение с архивом:** `explain.feature` в блоке Q4 указан, но
  `Приоритет` там уже нет — зафиксировать при переносе. В `src/**` вхождений
  `Приоритет` нет (парсер его не поддерживает).
- Дальше / риски: `migrator` — `Q4.md`, `D17-<слаг>.md`, указатель в архиве,
  `TRACEABILITY.md`, `questions/README.md`, ссылка в §10 №17; сверка с кодом
  (ожидание: ✅ соответствует; задач не требуется); устаревший список фич в
  ноте — на `docs-writer` (сопутствующие).

## migrator · 29.09.2026 · готово

- Сделано: перенос **Q4 → D17**. Созданы `docs/questions/Q4.md` (resolved by
  D17; дата 2026-09-24; приоритет 🔴 блокер) и
  `docs/decisions/D17-priority-out-of-mvp.md` (accepted; `Resolves: Q4`,
  `Спека: §10 №17`, `Tasks: —`). Архив: блок Q4 заменён указателем
  («Мигрирован 2026-09-29 (блок Q4)»). Обновлены `docs/TRACEABILITY.md` (строка
  Q4), `docs/questions/README.md` (строка Q4), `docs/SPECIFICATION.md` §10 №17
  (ссылка на D17; текст строки не переписывался).
- Сверка (§5.3): ✅ **соответствует**. Чтением — `src/core.rs` `parse_rule`
  (regex `Правило`/`Если`/`Решение`/`Причина`, ветки `Приоритет` нет; в `src/`
  ключевого слова нет, только несвязанные комментарии core/main), `GRAMMAR.md`
  §4/§5, фичи. Зафиксировано: архивный список фич устарел —
  `explain.feature` больше не содержит `Приоритет`; фактически `parser`,
  `execution`, `editor`, `lsp` + требование сортировки в `client_explanation`;
  `semver.feature` — иной (MAJOR), не относится. Расхождение = устаревший текст
  архива и шапки фич (зона `docs-writer`), задачи по коду не требует.
- Дальше / риски: `docs-writer` — актуализировать список фич в ноте Q4
  (`features/README.md`), при необходимости обратные ссылки `# D17 (Q4)` в шапках
  фич; затем `validator`. `cargo`/git не запускались. Коммит — за `git`.

## docs-writer · 29.09.2026 · готово

- Сделано: сопутствующие документы к переносу Q4 → D17.
  `features/README.md:75-79` — в ноте Q4 добавлены ссылки `[Q4](../questions/Q4.md)`
  и `[D17](../decisions/D17-priority-out-of-mvp.md)`; список фич приведён к факту:
  убран `explain.feature` (его сценарии — `rule_name`/`condition`/`actual_value`),
  остались `parser`, `execution`, `editor`, `lsp`, `client_explanation` (целевое
  v0.2). Обратные ссылки `# D17 (Q4): `Приоритет` — вне MVP (v0.2, с конвейерами)`
  в шапках `parser.feature` (после строки D16), `execution.feature` (после D16),
  `editor.feature`, `lsp.feature` (после `# language: ru`); в
  `client_explanation.feature:8` `(Q4/Q36)` → `(D17/Q36)` (дублирования Q4 нет).
  `CHANGELOG.md:25-28` — запись «Перенос Q4 → D17» в «### Документация» после
  записи Q2/Q3.
- Проверки: пути ссылок существуют — `docs/questions/Q4.md`,
  `docs/decisions/D17-priority-out-of-mvp.md` (glob); счётчики 47/278 не менялись;
  правка шапок не задевает блоки сценариев. Прогон `features_inventory.rs` — за
  `validator`.
- Дальше / риски: приёмка `validator` (docs) и пакет `git`. Остаток: `cargo`/git
  не запускались (запрет вызова); `docs/questions/**`, `decisions/**`,
  `OPEN_QUESTIONS.md`, `TRACEABILITY.md`, `SPECIFICATION.md`, `tasks/**` не
  трогались.

## validator · 29.09.2026 · чекпойнт (до прогона)

- Сделано: адресная проверка переноса Q4 → D17 (факты собраны, осталось
  `cargo`). Снимок: `develop`, HEAD `f488085` + рабочее дерево (13 M + 3 `??`).
  `git diff -- src tests AGENTS.md opencode.json` пусто; `rg -i приоритет src` —
  только `core.rs:414` (semver-комментарий) и `main.rs:56` (флаг); `parse_rule` —
  regex `Правило`/`Если`/`Решение`/`Причина`. `GRAMMAR.md:91` §4 (`Приоритет` ❌),
  `:97` §5; `SPECIFICATION.md:838` (§10 №17 → D17); `TRACEABILITY.md:11`,
  `questions/README.md:28`; указатель `OPEN_QUESTIONS.md:46-49`; 4 шапки
  `# D17 (Q4)` + `client_explanation:8` `(D17/Q36)`; слаг уникален.
- Проверки: `git status --porcelain`, `git log -1 --oneline`, `git diff`,
  `rg` — ок. Далее: `cargo fmt --check` + `cargo test --test features_inventory`
  (ожидание 4/4, 47/278).
- Дальше / риски: полный `cargo test --all` не перезапускаю — `src/**`/`tests/**`
  неизменны с W8-config.

## validator · 29.09.2026 · принято

- Сделано: приёмка переноса **Q4 → D17**. Вердикт **принято**, P1/P2/P3 нет.
  Отчёт — [`docs/reviews/migration-q4-2026-09-29.md`](../../docs/reviews/migration-q4-2026-09-29.md).
  Перенос (§7): полный текст Q4 — в `questions/Q4.md`; в архиве только указатель
  (`OPEN_QUESTIONS.md:46-49`); ID/дата 2026-09-24 сохранены; `D17` = §10 №17
  (`SPECIFICATION.md:838`); `Resolves/Спека/Affects/Tasks` заполнены; сверка ✅ +
  «Задач не требуется». Сверка §5.3 подтверждена фактами: `parse_rule` без
  `Приоритет`, `GRAMMAR.md` §4 (`:91`)/§5 (`:97`), фактический список фич
  (`parser/execution/editor/lsp` + сортировка в `client_explanation`), в
  `explain.feature` ключевого слова больше нет.
- Проверки: `cargo fmt --check` → pass; `cargo test --test features_inventory` →
  **4/4 ok** (47/278); `git diff -- src tests AGENTS.md opencode.json` пусто;
  `git diff 22f7683..HEAD -- src tests` пусто (полный `--all` не гонялся —
  обосновано); `rg "D17 \(Q4\)" docs` → ровно 4 шапки; слаг уникален.
- Дальше / риски: `git` — docs-коммит по подтверждению (одна запись: перенос
  Q4 → D17). Квитанция `service-migration-q4` iteration 1 — append в
  `receipts.yaml`. Правки принятых файлов вне порога существенности — повторная
  приёмка (`review.md`).

---

## сервисная сессия · 29.09.2026 · пакет подтверждён (коммит + push)

- Подтверждение (одно, `question`, владелец): **«Коммит + push»** — публикация
  `origin/develop` выложит накопленный `f488085` и новый коммит (ahead 2).
- Следствие для `git`: идемпотентно — записи до `add` (эта лента + чекпойнт
  `memory/git.md`), `add` ровно 21 путь по списку, сверка staged, коммит
  `docs(D17): перенос Q4 — Приоритет вне MVP` (BRIEF §5.7), затем
  `git push origin develop` (таймаут ≥ 5 мин); после push в отслеживаемые
  файлы не писать (F43); хеши — в ответе сервисной сессии.

## git · 29.09.2026 · в работе (записи до add)

- Сделано: подтверждение «Коммит + push» сверено по этой ленте (строка 115);
  снимок до совпал — `develop`, ahead 1, база HEAD `f488085`, 16 `M` + 4 `??`
  = 20 путей; коммита `docs(D17): перенос Q4 — Приоритет вне MVP` в `git log`
  нет; чекпойнт `memory/git.md` — 21-й путь пакета (запись сделана).
- Проверки: `git status --porcelain` → 20 (порядок и состав = списку пакета);
  `git log --all --oneline` → целевого сообщения нет.
- Дальше / риски: `add` ровно 21 путь → сверка `git diff --cached --name-only`
  → коммит → проверки → `push origin develop`; после push в отслеживаемые файлы
  не пишу (F43), хеши — в ответе. Риск: обрыв push — повтор статуса без
  перезаписи истории.
