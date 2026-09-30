# Приёмка: `service-doc-refactor` — операция №6 (полнота фич, Q76/D80)

**Проверка:** правило полноты фич (`Q76`/`D80`) и его применение: поимённая
трассировка всех `*.feature` в `TRACEABILITY.md` (колонка «Реализация»), снятие
wildcard-обобщений, заголовки `wasm`/`manifest_sync`, пометка «Полнота» в
`features/README.md`, добор проверок в `T-18`. Единственный прогон DoD —
адресный `cargo test --test features_inventory`.

**Версия:** `develop` @ `c0199d3` (= `origin/develop`) + рабочее дерево
(снимок 2026-09-30).

**Вердикт:** **принято** — P1/P2/P3 нет.

---

## Границы и DoD

- `git diff --stat c0199d3 -- src tests Cargo.toml` → **пусто**; полный
  `cargo test` не требуется (кода нет).
- `git status --porcelain` → **11 M + 2 ??**, только ожидаемые пути:
  `M` `docs/TRACEABILITY.md`, `docs/questions/README.md`,
  `docs/decisions/README.md`, `docs/tasks/T-18-docs-journal-test/README.md`,
  `docs/features/README.md`, `docs/features/wasm.feature`,
  `docs/features/manifest_sync.feature`, лента, `.opencode/memory/migrator.md`,
  `.opencode/memory/docs-writer.md`, `.opencode/state/current/progress.yaml`
  (запись №5 — до операции); `??` `docs/questions/Q76.md`,
  `docs/decisions/D80-features-visibility-completeness.md`. `src/**`, `tests/**`,
  `.opencode/rules|agents/**` не тронуты.

## `TRACEABILITY.md`

- `rg -c "^\| \[Q"` → **76** Q-строк; `rg -c "^"` = `rg -c "\r$"` = **88**
  (CRLF и финальный перевод строки целы).
- `rg "agents-\*|и др\."` → **пусто**; Q43/Q44 вместо wildcard перечисляют
  шесть агентских фич поимённо.
- Строка `:80` — `| [Q76] | [D80] | in work | [T-18] ⬜ | — |`.
- Правки прежних строк — только заявленные клетки:
  - `Q29`/`D34` и `Q32`/`D28` — +`manifest_sync.feature`;
  - `Q65`/`D69` — `wasm.feature` (ретро-линия D11, v0.2);
  - `Q74`/`D78` — пять фич `T-15` (agents-state-schema, agents-session-checkpoint,
    agents-re-raise, agents-metrics, agents-mcp-readiness).
- **Полнота 47/47:** сверка списка `rg --files docs/features -g "*.feature"`
  (47) со всеми именами из `TRACEABILITY` — совпадают **все 47** имён, включая
  12 ранее отсутствовавших (`manifest_sync`, `wasm`, 5 агентских, ранее скрытых
  wildcard, и 5 `T-15`). Обратная живость: каждое упомянутое имя существует.
- Маппинг = основания заголовков: шесть агентских фич имеют заголовки
  `# D38 (Q43)` и `# D39 (Q44)` (все шесть — обе строки Q43/Q44 оправданы);
  пять `T-15`-фич — основание `T-15 (проект)` → `Q74`/`D78`; `manifest_sync` —
  `# D34 (Q29)`/`# D28 (Q32)`; `wasm` — `# D11: … (ретро-D, оформлен D69)`,
  а `D69` действительно перечисляет `D11` (`D69:12`).

## Записи

- `Q76.md` — по §4 (⚪; Статус/Дата/Приоритет/Связано + Контекст/Вопрос/
  Варианты/Рекомендация); контекст сходится: 47 фич, 35 поимённо, 12 без имени.
- `D80-features-visibility-completeness.md` — по §4 (Статус/Дата/Resolves
  (`[Q76]`)/Спека (`—`)/Affects/Tasks (`T-18`)), вердикт «Сверка с кодом» —
  ⚪ **не применимо** с фактами §5.3.
- Каталоги: `questions/README.md` — **76** Q-строк (+Q76),
  `decisions/README.md` — **80** D-строк (+D80); по одному вхождению `Q76`/`D80`
  (ID не переиспользованы).

## `features/`

- `wasm.feature:2` — `# D11: … (ретро-D, оформлен D69)`; `manifest_sync.feature:2-3`
  — `# D34 (Q29): …` и `# D28 (Q32): …`; первая строка обоих — ровно
  `# language: ru`, ровно одна `Функция:` (`rg -c "^Функция:"` = 1 в каждом).
- `features/README.md` — +1 строка: пункт «Полнота» в §«Как читать» (ссылки
  `../TRACEABILITY.md`, `D80`, `tests/docs_journal.rs` с меткой плана T-18 ⬜);
  таблицы и счётчики не тронуты (`:315` «Итого: 47 файлов, 278 сценариев» —
  как было; `git diff` по файлу = ровно +1).

## `T-18`

- «Источник» (`:16`) дополнен `[D80] (Q76) — полнота фич`; в список v0.1
  (`:56-60`) добавлен подпункт: каждая `*.feature` из `features/README.md`
  встречается поимённо в `TRACEABILITY` (колонка «Реализация»);
  wildcard-обобщения (`agents-*`, «и др.») не допускаются; обратно — каждый
  `*.feature` из `TRACEABILITY` существует (`D80`).

## Адресный прогон DoD

- `cargo test --test features_inventory` → **`test result: ok. 4 passed; 0
  failed`** (`feature_files_are_valid_documents`,
  `feature_files_match_readme_inventory`, `readme_totals_match_files`,
  `scenario_counts_match_readme`); счётчики README `:315` — **47 файлов /
  278 сценариев** — подтверждены тестом. Полный `cargo test --all` не
  запускался (кода нет, D50).

## Согласованность и чек-лист §5.7

- Правило `D80` ↔ правки: «поимённо, без wildcard» ↔ снятые обобщения и
  поимённые списки; «источник связи — заголовок/T-15» ↔ выставленные заголовки
  `wasm`/`manifest_sync` и уже существующие заголовки агентских/T-15 фич.
- Ссылки новых фрагментов живые (`Q76`/`D80`, `TRACEABILITY`, `features/README`,
  карточка `T-18`, `D11`, `D34`, `D28`, `D69`); ID не переиспользованы
  (`Q76`/`D80` уникальны); миграционных маркеров в изменённых файлах нет.
- Лента: «операция №6 — открытие» (`:800`) → `migrator` — записи и таблица
  (`:820`) → `docs-writer` — пометка полноты и заголовки (`:845`) — порядок
  маршруту соответствует.

**Что проверено и ок:** границы (только ожидаемые пути; `src`/`tests`/канон не
тронуты); 76 Q-строк и CRLF `TRACEABILITY`; отсутствие wildcard/«и др.»;
полнота 47/47 и соответствие маппинга заголовкам; записи `Q76`/`D80` по §4;
каталоги 76/80; заголовки и структура двух фич; пометка «Полнота» в
`features/README` без изменения таблиц/счётчиков; добор в `T-18`;
адресный `features_inventory` 4/4 ok; чек-лист §5.7; порядок записей ленты.
Операция №6 готова к пакету `git`.
