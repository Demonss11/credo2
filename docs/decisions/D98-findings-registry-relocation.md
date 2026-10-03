# D98: Реестр находок — переезд в `docs/registry/` и раскол открытые/закрытые

- **Статус:** accepted
- **Дата:** 2026-10-03
- **Resolves:** [Q95](../questions/Q95.md)
- **Спека:** —
- **Affects:** новые `docs/registry/registry.md` и `docs/registry/archive.md`
  (зона `docs/registry/`; создаются задачей
  [T-27](../tasks/T-27-findings-registry-split/README.md));
  [`tests/docs_journal.rs`](../../tests/docs_journal.rs) (whitelist /
  `journal_files()` — новая зона вместо `docs/analysis/findings-registry.md`);
  [`.opencode/rules/journal.md`](../../.opencode/rules/journal.md) §8 (исключение
  канона — новый адрес); [`AGENTS.md`](../../AGENTS.md) (таблица ролей, «Пишет в»
  `migrator`); [D48](D48-findings-registry-owner.md) (обратная пометка «уточнено
  D98» — зона записи/владелец); [D65](D65-reference-policy.md) (уточнение:
  исключение-реестр переезжает в `docs/registry/`, ссылки на архив разрешены);
  [`T-18`](../tasks/T-18-docs-journal-test/README.md) (whitelist теста);
  [`TRACEABILITY.md`](../TRACEABILITY.md) (строка
  [Q53](../questions/Q53.md)/D48 — новый адрес реестра);
  [`.opencode/agents/migrator.md`](../../.opencode/agents/migrator.md) (право
  `edit docs/registry/**` + тело «Реестр находок»); массовая замена ссылок
  `../analysis/findings-registry.md` → `docs/registry/…`
- **Tasks:** [`T-27`](../tasks/T-27-findings-registry-split/README.md) (⬜, L)

## Контекст

Реестр находок прогонов живёт в
[`docs/analysis/findings-registry.md`](../analysis/findings-registry.md) —
**94 строки, 84 находки** (открытые: `F26`, `F27`, `F74`–`F84` — 14; закрытые —
70). `docs/analysis/` — зона **разборов прогонов** (рабочие артефакты, удаляемые
по отработке; архив — git, [D65](D65-reference-policy.md) «Обновление
30.09.2026»). Реестр же — **живой канон-артефакт**: он держится в этой зоне
особенным исключением сразу в нескольких местах — whitelist теста
`tests/docs_journal.rs` (`journal_files()`, `removable_addresses`, D48), канон
[`.opencode/rules/journal.md`](../../.opencode/rules/journal.md) §8,
[`AGENTS.md`](../../AGENTS.md) (таблица ролей), решения
[D48](D48-findings-registry-owner.md)/[D65](D65-reference-policy.md), карточка
[T-18](../tasks/T-18-docs-journal-test/README.md), строка
[Q53](../questions/Q53.md)/D48 в [`TRACEABILITY.md`](../TRACEABILITY.md), право и
тело [`.opencode/agents/migrator.md`](../../.opencode/agents/migrator.md). Полный
контекст и варианты — [Q95](../questions/Q95.md).

Решение владельца 03.10.2026: реестр переезжает в **`docs/registry/`** и
раскалывается на **открытые** и **закрытые**; ссылки (в т.ч. на архив)
разрешены; переезд+раскол — **одной операцией** в объёме будущей задачи.

## Решение

1. **Место.** Реестр находок переезжает в **`docs/registry/`** — отдельную зону
   живого канон-артефакта (вне удаляемой `docs/analysis/`).
2. **Раскол.** Реестр раскалывается на два файла:
   - **`docs/registry/registry.md`** — **открытые** находки (рабочее чтение
     «что открыто сейчас»);
   - **`docs/registry/archive.md`** — **закрытые** находки (история; записи не
     переписываются).
3. **Оба файла — живые канон-артефакты.** Ссылки канона и журнала на них
   **разрешены**, в том числе на `archive.md` — **уточнение**
   [D65](D65-reference-policy.md): исключение-реестр переезжает в `docs/registry/`,
   запрет на `docs/analysis/**` его больше не касается.
4. **Владелец и правило discharge.** Владелец — `migrator` (зона
   `docs/registry/**`); правило ведения прежнее — **идемпотентность/append**:
   записи не переписываются, статусы обновляются по закрытию, ID не
   переиспользуются ([D48](D48-findings-registry-owner.md)).
5. **`docs/analysis/` не тронут** — остаётся зоной разборов прогонов.
6. **Техническая часть — в задаче [T-27](../tasks/T-27-findings-registry-split/README.md)**
   (класс **L** — правка канона/прав «даже одной строкой»; `AGENTS.md`):
   правка `tests/docs_journal.rs` (новая зона / whitelist вместо
   `docs/analysis/findings-registry.md`), обновление канона
   ([`.opencode/rules/journal.md`](../../.opencode/rules/journal.md),
   [`AGENTS.md`](../../AGENTS.md), обратные пометки D48/D65, карточка
   [T-18](../tasks/T-18-docs-journal-test/README.md),
   [`TRACEABILITY.md`](../TRACEABILITY.md),
   [`.opencode/agents/migrator.md`](../../.opencode/agents/migrator.md) + право
   `edit docs/registry/**`) и **массовая замена ссылок**
   `../analysis/findings-registry.md` → `docs/registry/…`; полный DoD.
   **Канон-часть (`AGENTS.md`, `.opencode/**`) правит сервисная сессия** (не
   `coder`/`docs-writer` — вне их прав); код-часть (тест) — `coder`/`tester`;
   маршрут L — полный + `auditor` до коммита.

## Следствия

- Реестр перестаёт быть исключением внутри удаляемой зоны: исключение-whitelist
  заменяется отдельной зоной; `docs/analysis/` возвращается к чистому назначению
  — разборы прогонов.
- Открытое отделено от истории: рабочее чтение сужается до `registry.md`;
  `archive.md` сохраняет 70 закрытых записей как живую историю (ссылки
  разрешены — не «удаляемая»).
- Владелец (`migrator`) и правило discharge не меняются — правятся только зона и
  право; «реестр — улика, не канон» ([D41](D41-dispatch-refinements.md))
  сохраняется.
- Массовая замена ссылок — разовая техническая операция задачи `T-27`;
  продуктовый код и контракты CREDO не затрагиваются (правка теста
  `docs_journal.rs` — процессная проверка журнала).

## Сверка с кодом

Вердикт: ⚪ **не применимо** (процесс/документы) — решение о месте и расколе
живого канон-артефакта; продуктового кода CREDO не меняет. Техническая правка
теста `tests/docs_journal.rs` кодифицируется в задаче
[T-27](../tasks/T-27-findings-registry-split/README.md), прогон — за `validator`.

Что проверено (чтением, 03.10.2026), чем подтверждено:

- **Факт реестра:** [`docs/analysis/findings-registry.md`](../analysis/findings-registry.md) —
  94 строки, 84 записи `F*`; открытые: `F26`, `F27`, `F74`–`F84` (14); закрытые —
  70 (rg по `^\| F\d+ \|`).
- **«Зашитость» реестра (факт):**
  `tests/docs_journal.rs` — `journal_files()` (`:677`), whitelist зоны
  `docs/analysis/` и `name == "findings-registry.md"` в `removable_addresses`
  (`:819`–`:878`); [`.opencode/rules/journal.md`](../../.opencode/rules/journal.md)
  §8 (исключение-реестр, основание D65/D48); [`AGENTS.md`](../../AGENTS.md)
  (таблица ролей, «Пишет в» `migrator`);
  [`.opencode/agents/migrator.md`](../../.opencode/agents/migrator.md) — право
  `edit docs/analysis/findings-registry.md` (`:14`) и тело (`:89`).
- **Основания-решения:** [D48](D48-findings-registry-owner.md) (владелец/зона),
  [D65](D65-reference-policy.md) (политика ссылок, исключение-реестр),
  [T-18](../tasks/T-18-docs-journal-test/README.md) (whitelist теста) — файлы
  существуют (glob).
- **Свобода номеров:** созданные [Q95](../questions/Q95.md) и `D98`; последние
  занятые — [Q94](../questions/Q94.md)/[D97](D97-c8-b0-roster.md); задача `T-27`
  (последняя занятая — [T-26](../tasks/T-26-mcp-server-design/README.md));
  нумерация сквозная (§2 [`journal.md`](../../.opencode/rules/journal.md)).

`cargo` не запускался (§5.3, [D50](D50-dod-by-package-scope.md)): решение
процессное/документное, кода CREDO не касается; адресный прогон
(`docs_journal`) — за `validator` на приёмке (R2).

**Задача —** [`T-27`](../tasks/T-27-findings-registry-split/README.md) (⬜, L):
создание `docs/registry/{registry,archive}.md`, раскол 84 записей, правка
`tests/docs_journal.rs`/канона/ссылок, полный DoD; канон-часть — сервисная
сессия, код-часть — `coder`/`tester`. Работу в этой операции **не выполняем** —
только оформление.

## Альтернативы

- **(B) Только раскол на месте** — отклонено: живой канон-артефакт остаётся в
  удаляемой зоне `docs/analysis/`, исключение-whitelist не снимается.
- **(C) Оставить как есть** — отклонено: рассогласование «зона разборов ↔ живой
  канон» сохраняется, читаемость падает с ростом файла.

## Ссылки

- Вопрос: [Q95](../questions/Q95.md)
- Основания-решения: [D48](D48-findings-registry-owner.md) (владелец/зона),
  [D65](D65-reference-policy.md) (политика ссылок)
- Артефакты: [`docs/analysis/findings-registry.md`](../analysis/findings-registry.md)
  (текущий реестр), [T-18](../tasks/T-18-docs-journal-test/README.md) (тест),
  [`tests/docs_journal.rs`](../../tests/docs_journal.rs) (whitelist)
- Задача: [T-27](../tasks/T-27-findings-registry-split/README.md) (⬜, L)
- Сервисная операция 03.10.2026 (r20) — решение владельца
- Связи: [`TRACEABILITY.md`](../TRACEABILITY.md)
