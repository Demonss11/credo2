# D77: Полнота задач — каждая T-XX видна в TRACEABILITY через пару Q/D

- **Статус:** accepted
- **Дата:** 2026-09-30
- **Resolves:** [Q73](../questions/Q73.md)
- **Спека:** —
- **Affects:** [`tasks/README.md`](../tasks/README.md) (преамбула — правило
  полноты), [`T-18-docs-journal-test/README.md`](../tasks/T-18-docs-journal-test/README.md)
  (добор проверок v0.1), [`TRACEABILITY.md`](../TRACEABILITY.md) (полнота
  представления); уточнение к [D64](D64-journal-integrity-test.md); ретро-фикс —
  [Q74](../questions/Q74.md)/[D78](D78-t15-mcp-ready-program.md)
- **Tasks:** [`T-18`](../tasks/T-18-docs-journal-test/README.md) (добор проверок;
  см. «Сверка с кодом»)

## Контекст

Сверка 30.09.2026: реестр [`tasks/README.md`](../tasks/README.md) — **19 задач**
(T-01…T-19); [`TRACEABILITY.md`](../TRACEABILITY.md) показывает **18** (колонка
«Задачи»), статусы совпадают. Единственная задача без пары Q/D и невидимая в
таблице — [`T-15`](../tasks/T-15-mcp-ready-process/README.md) (реестр: «Q/D —
`migrator`»; карточка: «Журнальные `Qn`/`Dn` заводит `migrator` до старта»).
Преамбула реестра обещает «задач «из воздуха» здесь нет», но требования **полноты
представления** (все задачи видны через [`TRACEABILITY.md`](../TRACEABILITY.md)) канон
не содержит — поэтому `T-15` и выпала. Согласовано с
[D63](D63-journal-index-lifecycle.md) (единая таблица связей) и
[D64](D64-journal-integrity-test.md) (тест целостности). Полный контекст —
[Q73](../questions/Q73.md).

## Решение

1. **Полнота вперёд:** каждая задача из [`tasks/README.md`](../tasks/README.md) имеет
   источник Q/D и присутствует **хотя бы в одной** строке
   [`TRACEABILITY.md`](../TRACEABILITY.md) (в колонке «Задачи»).
2. **Задача без пары Q/D — черновик:** в работу не берётся; пара заводится
   `migrator` до старта (прецедент — [`T-15`](../tasks/T-15-mcp-ready-process/README.md)).
3. **Обратная полнота сохраняется:** каждая `T-XX`, упомянутая в
   [`TRACEABILITY.md`](../TRACEABILITY.md), существует в [`tasks/`](../tasks/README.md).
4. **Статусные пометки** ⬜/🚧/✅ в [`TRACEABILITY.md`](../TRACEABILITY.md) совпадают
   со статусом в реестре [`tasks/README.md`](../tasks/README.md) и в карточке задачи.
5. **Машинная проверка** — дополнение `tests/docs_journal.rs`
   ([`T-18`](../tasks/T-18-docs-journal-test/README.md), v0.1): полнота задач и
   синхронность статусов.
6. **Вопрос строки в** [`.opencode/rules/journal.md`](../../.opencode/rules/journal.md)
   **§7** — отдельно (служебная зона + аудит; вне этой записи).

## Следствия

- Дыра `T-15` закрывается парой [Q74](../questions/Q74.md)/[D78](D78-t15-mcp-ready-program.md):
  реестр становится полным (**19/19**).
- Тест `tests/docs_journal.rs` (T-18) ловит «задачу из воздуха» и дрейф статусов
  между реестром, карточками и [`TRACEABILITY.md`](../TRACEABILITY.md).
- Карточка [`T-18`](../tasks/T-18-docs-journal-test/README.md) дополняется в этой же
  операции (часть 2).
- Строки Q47–Q53 (D42–D48) не трогаются: проверка их связи с
  [`T-15`](../tasks/T-15-mcp-ready-process/README.md) по фактам — за `validator`.
- **Обновление 30.09.2026:** полнота задач внесена в канон журнала (§7
  [`journal.md`](../../.opencode/rules/journal.md)) —
  [D79](D79-journal-canon-completeness.md).

## Сверка с кодом

Вердикт: ⚪ **не применимо** (документы/процесс) — решение фиксирует правило ведения
реестра задач и журнала; продуктовый код прототипа не меняет.

Что проверено (чтением, 30.09.2026), чем подтверждено:

- [`tasks/README.md`](../tasks/README.md) — сводка **19 задач** (T-01…T-19),
  источник — решения журнала `Dn`/`Qn`.
- [`TRACEABILITY.md`](../TRACEABILITY.md) — **[`T-15`](../tasks/T-15-mcp-ready-process/README.md)
  отсутствует** в колонке «Задачи»; остальные 18 видны.
- `tests/docs_journal.rs` ещё не написан
  ([`T-18`](../tasks/T-18-docs-journal-test/README.md) открыта) — машинной проверки
  полноты нет.

`cargo` не запускался (§5.3, [D50](D50-dod-by-package-scope.md)): запись
документная, кода прототипа не меняет.

**Задач не требуется сверх** [`T-18`](../tasks/T-18-docs-journal-test/README.md):
правка правила — зона `migrator` (реестр/журнал); машинная проверка — дополнение
T-18.

## Альтернативы

- **(б) Строки/секция задач без Q/D в таблице** — отклонено: ломает D63 («строка =
  пара Q→D») и возвращает прецедент задач без источника.
- **(в) Точечный фикс без правил** — отклонено: текущая дыра `T-15` возникла именно
  из-за отсутствия правила полноты; фикс без правила не защищает от повторения.

## Ссылки

- Вопрос: [Q73](../questions/Q73.md)
- Связанные: [D63](D63-journal-index-lifecycle.md) (единая таблица),
  [D64](D64-journal-integrity-test.md) (тест целостности; уточняется),
  [D60](D60-docs-ownership-sync.md) (один факт — один канон);
  [Q74](../questions/Q74.md)/[D78](D78-t15-mcp-ready-program.md) (ретро-фикс `T-15`)
- Связи: [`TRACEABILITY.md`](../TRACEABILITY.md);
  [`T-18`](../tasks/T-18-docs-journal-test/README.md) (карточка)
