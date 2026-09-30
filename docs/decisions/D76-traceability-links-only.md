# D76: TRACEABILITY — колонки Q и D: только ссылки на записи журнала

- **Статус:** accepted
- **Дата:** 2026-09-30
- **Resolves:** [Q72](../questions/Q72.md)
- **Спека:** —
- **Affects:** [`TRACEABILITY.md`](../TRACEABILITY.md) (колонки Q и D — только
  ссылки; легенда); согласовано с [D63](D63-journal-index-lifecycle.md),
  [D65](D65-reference-policy.md), [D60](D60-docs-ownership-sync.md); источник —
  [F46](../analysis/findings-registry.md)
- **Tasks:** — (см. «Сверка с кодом»)

## Контекст

[D63](D63-journal-index-lifecycle.md) развёл роли документов: каталоги
[`questions/README.md`](../questions/README.md) и
[`decisions/README.md`](../decisions/README.md) несут темы,
[`TRACEABILITY.md`](../TRACEABILITY.md) — связи и жизненный цикл. На практике в
клетках колонок Q и D нарос описательный текст, дублирующий каталоги и D-файлы:
правка одной записи тянула 3–4 файла (кейс F46 —
[`../analysis/findings-registry.md`](../analysis/findings-registry.md)). Полный
контекст — [Q72](../questions/Q72.md).

## Решение

1. **Клетки колонок Q и D содержат только ID-ссылки:**
   `[Qn](questions/Qn.md)` и `[Dn](decisions/Dn-<слаг>.md)`; описательный текст в
   них не ведётся. Это уточнение к D63 и применение принципов D60/D65 («один
   факт — один канон», «ссылка, не копия»).
2. **Темы — в каталогах** ([`questions/README.md`](../questions/README.md):
   `Q | Тема | D`; [`decisions/README.md`](../decisions/README.md):
   `D | Краткая тема | …`), **формулировки — в самих Q/D-файлах**.
3. **Формат применён ко всем существующим строкам** (Q1–Q71) и обязателен для
   новых; строка Q72/D76 — уже в нём.
4. **Легенда `TRACEABILITY.md` дополнена** пояснением: колонки Q и D — только
   ссылки на записи журнала; где искать темы.
5. **Остальные колонки** («Жизненный цикл» · «Задачи» · «Реализация») — без
   изменений (D63).

## Следствия

- Дубли каталогов в `TRACEABILITY.md` сняты; правка темы Q/D больше не тянет
  таблицу (механизм F46 по этому источнику закрыт).
- Тест целостности [D64](D64-journal-integrity-test.md) (`tests/docs_journal.rs`,
  задача [T-18](../tasks/T-18-docs-journal-test/README.md), ещё не написан)
  проверяет наличие строк и парность Q↔D, а не описания, — решение совместимо.
- Правила журнала
  ([`../../.opencode/rules/journal.md`](../../.opencode/rules/journal.md) §6) не
  меняются: таблица остаётся **представлением** связей.

## Сверка с кодом

Вердикт: ⚪ **не применимо** (документы/процесс) — решение правит структуру
документа журнала, файлы `src/**`/`tests/**` не затрагиваются.

Что проверено (чтением, 30.09.2026), чем подтверждено:

- Формат [`TRACEABILITY.md`](../TRACEABILITY.md) до правки (клетки Q/D с
  описаниями) и после (только ссылки; 72 строки Q).
- Каталоги [`questions/README.md`](../questions/README.md) (несёт темы) и
  [`decisions/README.md`](../decisions/README.md).
- Решения [D63](D63-journal-index-lifecycle.md) (роли каталогов и таблицы),
  [D64](D64-journal-integrity-test.md) (состав проверки таблицы),
  [D65](D65-reference-policy.md) («ссылка, не копия»).
- `tests/docs_journal.rs` в репозитории отсутствует (задача T-18 открыта).

`cargo` не запускался (§5.3, [D50](D50-dod-by-package-scope.md)): решение
документное, кода прототипа не меняет.

**Задач не требуется:** правка зоны `migrator` выполнена этим изменением;
машинная проверка целостности — за
[T-18](../tasks/T-18-docs-journal-test/README.md)
([D64](D64-journal-integrity-test.md)).

## Альтернативы

- **Сохранить описания в колонках** — отклонено: дубль каталогов и D-файлов,
  кейс F46, конфликт с D65.
- **Описания только у новых строк** — отклонено: несогласованность таблицы
  (два формата одновременно).

## Ссылки

- Вопрос: [Q72](../questions/Q72.md)
- Связанные: [D63](D63-journal-index-lifecycle.md) (единая таблица связей);
  [D65](D65-reference-policy.md) («ссылка, не копия»);
  [D60](D60-docs-ownership-sync.md) (один факт — один канон)
- Источник: [`../analysis/findings-registry.md`](../analysis/findings-registry.md) (F46)
- Связи: [`TRACEABILITY.md`](../TRACEABILITY.md);
  [`../../.opencode/rules/journal.md`](../../.opencode/rules/journal.md)
  (процесс журнала)
