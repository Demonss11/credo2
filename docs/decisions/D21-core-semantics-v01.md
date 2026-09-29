# D21: Семантика ядра v0.1 — строгие ошибки исполнения, словарь решений и канон объяснения

- **Статус:** accepted
- **Дата:** 2026-09-24
- **Resolves:** [Q8](../questions/Q8.md), [Q9](../questions/Q9.md),
  [Q10](../questions/Q10.md), [Q42](../questions/Q42.md)
- **Спека:** [`SPECIFICATION.md`](../SPECIFICATION.md) §10, решение №21
- **Affects:** [`../../src/core.rs`](../../src/core.rs) (`EvalError`,
  `evaluate_rule`, `Explanation`, `condition_to_string`, `contract_from_rule`);
  [`../../src/rest.rs`](../../src/rest.rs) (422 `evaluation_failed`);
  [`../../src/mcp.rs`](../../src/mcp.rs) (ошибка инструмента
  `evaluation_failed`);
  [`../features/errors.feature`](../features/errors.feature) (🟡 — сценарий
  типов на парсере, целевое v0.2);
  [`../features/explain.feature`](../features/explain.feature),
  [`../features/explain_full.feature`](../features/explain_full.feature) (✅);
  [`../features/execution.feature`](../features/execution.feature) (🟡 — из-за
  `Приоритет`, [Q4](../questions/Q4.md));
  [`../features/test_draft.feature`](../features/test_draft.feature);
  [`../features/README.md`](../features/README.md) (статусы/ноты Q8–Q10, Q42);
  [`../GRAMMAR.md`](../GRAMMAR.md) (`Решение = <Имя>`, Q10)
- **Tasks:** — (см. «Сверка с кодом»)

## Контекст

Четыре решения одной строки `SPECIFICATION.md` §10 №21 («Семантика ядра v0.1»)
определяют поведение движка на границах: что делать с отсутствующим полем
([Q8](../questions/Q8.md)), с несовместимыми типами ([Q9](../questions/Q9.md)),
как представлять решение и «правило не сработало» ([Q10](../questions/Q10.md)) и
какой контракт имеет объяснение ([Q42](../questions/Q42.md)). До решения код
подставлял отсутствующему полю `0.0` (ложные срабатывания), молча возвращал
`matched = false` при сравнении разных типов, добавлял в контракт решение
`"Pass"` и не отдавал `condition` в объяснении; фичи при этом ожидали строгое
поведение и полную схему. Полный контекст — в файлах вопросов.

## Решение

1. **Отсутствующее поле — строгая ошибка ([Q8](../questions/Q8.md)).**
   `core.rs::evaluate_rule` возвращает `Result<Explanation, EvalError>`;
   отсутствующее поле → `EvalError::UnknownField(поле)` с русским сообщением
   «Неизвестное поле: …». Ошибка исполнения → REST 422
   `{ "error": "Неизвестное поле: …" }`; в MCP — ошибка инструмента.
   Разделение состояний «нет параметра» и «есть параметр, но пустой» —
   плановое (v0.2+, вместе со строгой типизацией), не вопрос MVP.
   Обоснование: ложные срабатывания опаснее падения.

2. **Типы проверяются на исполнении ([Q9](../questions/Q9.md)).**
   Строгая статическая типизация — плановая (v0.2), в MVP — runtime-проверка:
   сравнение значений разных типов → `EvalError::TypeMismatch` («Несовместимые
   типы: поле … имеет тип …, ожидался …»). Строки поддерживают только `==`/`!=`;
   `<`/`>` для строк — та же ошибка типов. Равенство `f64` (`==`/`!=`) остаётся
   точным, эпсилон не вводится (риски ложных срабатываний выше); переход на
   `Decimal` — v0.2 (SPEC §10 п. 13).

3. **Словарь решений задаёт банк; «не сработало» — пустое решение
   ([Q10](../questions/Q10.md)).** Жёсткого перечня решений в ядре нет: решения —
   свободные идентификаторы (`Решение = <Имя>` в
   [`GRAMMAR.md`](../GRAMMAR.md)), форма словаря (per-bank таблица/HashMap) —
   при разработке БД/справочников. `contract_from_rule` **не** добавляет
   `"Pass"` — в контракт попадают только реально используемые в правиле решения.
   «Правило не сработало»: единое представление — пустые `decision`/`reason`
   в ядре, `""` в JSON объяснения.

4. **Канон объяснения — латиница, тексты русские
   ([Q42](../questions/Q42.md)).** Ключи `snake_case`: `rule_name`, `condition`,
   `actual_value`, `matched`, `decision`, `reason`; тексты `decision`/`reason` —
   русские (машина/человек: ключи латиницей ради стабильного JSON-контракта,
   содержательные значения — по-русски). `Explanation` дополнен полем
   `condition` (каноническая форма `<поле> <оператор> <значение>`,
   `condition_to_string`). `priority` в схему не входит — вне v0.1
   ([Q4](../questions/Q4.md)), вернётся в v0.2 с конвейерами. Ошибки исполнения
   (п. 1–2) — вне `Explanation` (возвращаются через `Result`), отдельного
   варианта «объяснение с ошибкой» нет.

## Следствия

- `evaluate_rule` — единственная точка исполнения: строгие ошибки (пп. 1–2)
  возвращаются через `Result`, REST/MCP транслируют их в `evaluation_failed`
  (REST — 422, MCP — ошибка инструмента).
- Контракт объяснения стабилен для `check.test` и REST; `explain.feature`
  переписана под канон (ключи латиницей, сценарий с приоритетом удалён), в
  `explain_full.feature` убраны `priority`, добавлен шапочный комментарий.
- Требования: `explain.feature` / `explain_full.feature` — ✅; `errors.feature` и
  `execution.feature` остаются 🟡 (первая — из-за сценария типов на парсере,
  целевое v0.2; вторая — из-за `Приоритет`, [Q4](../questions/Q4.md));
  `features/README.md` помечает расхождения.
- `SPECIFICATION.md` §10 пополнен решением №21; `GRAMMAR.md` уже описывает
  решения как свободные идентификаторы — правок не потребовалось.
- Словарь решений банка (справочник/HashMap) — вне MVP-ядра, при разработке БД.

## Сверка с кодом

Вердикт: ✅ **соответствует** — ядро реализует все четыре части решения
(строгие ошибки Q8/Q9, словарь решений и пустое решение Q10, схема объяснения
Q42).

Что проверено (чтение кода, фич и тестов, 29.09.2026), чем подтверждено:

- [`src/core.rs`](../../src/core.rs):
  - `enum EvalError` — `UnknownField(String)` (комментарий Q8) и
    `TypeMismatch { field, actual, expected }` (комментарий Q9); `Display` даёт
    «Неизвестное поле: {field}» и «Несовместимые типы: поле {field} имеет тип
    {actual}, ожидался {expected} в сравнении».
  - `evaluate_rule -> Result<Explanation, EvalError>`: отсутствующее поле →
    `UnknownField` (`Ok_or_else`, не `0.0`); число×число — `<`/`>`/`==`/`!=`;
    строки — только `==`/`!=`; иначе → `TypeMismatch`.
  - `struct Explanation` — `rule_name`, `condition`, `decision`, `reason`,
    `actual_value`, `matched`; поля `priority` нет. `condition` заполняется
    `condition_to_string` (`"{field} {op} {value}"`).
  - `contract_from_rule` — `decisions: vec![rule.action.decision.clone()]`;
    `"Pass"` не добавляется.
  - Юнит-тесты: `missing_field_is_strict_error_q8`,
    `missing_field_does_not_trigger_false_positive_q8`,
    `type_mismatch_is_error_q9`,
    `unmatched_rule_has_empty_decision_q10`,
    `contract_decisions_follow_rule_vocabulary_q10`,
    `explanation_has_latin_keys_and_condition_q42` (проверяет и отсутствие
    `priority`).
- [`src/rest.rs`](../../src/rest.rs): `evaluate_rule(...).map_err(...)` →
  `evaluation_failed`, `StatusCode::UNPROCESSABLE_ENTITY` (422), текст — из
  `EvalError` (Q8/Q9).
- [`src/mcp.rs`](../../src/mcp.rs): `evaluate_rule(...).map_err(|e|
  ToolError::evaluation(e.to_string()))` — ошибка инструмента MCP с кодом
  `evaluation_failed`; ответ `check.test` содержит `rule_name`, `condition`,
  `actual_value` (верхний уровень, Q29).
- Фичи: [`errors.feature`](../features/errors.feature) — «Неизвестное поле» и
  «Несовместимые типы» согласованы по текстам;
  [`explain.feature`](../features/explain.feature) (`rule_name`, `condition`,
  `actual_value`, комментарий Q42) и
  [`explain_full.feature`](../features/explain_full.feature) (шесть полей,
  пустое `decision` у несработавшего правила) — ✅;
  [`test_draft.feature`](../features/test_draft.feature) — `decision` «пусто» и
  ошибка `evaluation_failed` при отсутствии поля.
- [`features/README.md`](../features/README.md): `explain.feature` ✅,
  `explain_full.feature` ✅, `errors.feature` 🟡 («Неизвестное поле» и
  «Несовместимые типы» на исполнении (Q8/Q9); «Файл пуст» и ошибки типов на
  парсере — v0.2), `execution.feature` 🟡 (из-за `Приоритет`, [Q4](../questions/Q4.md)).

**Зафиксированное требование-нюанс (вне периметра MVP):** сценарий
[`errors.feature`](../features/errors.feature) «Ошибка при сравнении разных
типов» описан на этапе **парсера** («Когда парсер обрабатывает текст»), тогда как
regex-минимум ловит ошибку типов **на исполнении** ([Q3](../questions/Q3.md) /
[D16](D16-dsl-canon-regex-mvp.md)). Это целевое поведение лексера/AST (v0.2);
статус 🟡 уже отражён в [`features/README.md`](../features/README.md), задача по
нему не заводится (вне периметра MVP). Правка шапок фич — зона `docs-writer`.

Адресный прогон не требовался: вердикт основан на чтении кода, фич и юнит-тестов
(`src/core.rs`; сценарии исполнения REST — ручной прогон, полный `cargo test
--all` выполняет `validator`).

**Задач не требуется:** код уже реализует решение целиком (строгие ошибки,
словарь решений, схема объяснения), расхождений в периметре MVP нет; нюанс
`errors.feature` (ошибка типов на парсере) — целевое состояние v0.2, уже
помечено в `features/README.md`. Словарь решений банка — вне MVP-ядра.

## Альтернативы

- **Мягкая подстановка `0` для отсутствующего поля** (вариант (б) [Q8](../questions/Q8.md))
  — отклонено: условие `< 21` для отсутствующего поля срабатывало ложно; ложные
  срабатывания опаснее падения.
- **Явный `null`/`unknown`** (вариант (в) [Q8](../questions/Q8.md)) — отклонено в
  MVP: расширяет контракт; разделение «нет параметра»/«пустой параметр» —
  плановое v0.2+.
- **Проверка типов на парсинге / строгая статическая типизация уже в MVP**
  ([Q9](../questions/Q9.md)) — отклонено: требует лексера/AST (v0.2); MVP-парсер
  regex-минимум ([D16](D16-dsl-canon-regex-mvp.md)).
- **Эпсилон для `==` `f64`** ([Q9](../questions/Q9.md)) — отклонено: риски
  ложных срабатываний выше; вопрос снимается переходом на `Decimal` в v0.2.
- **Фиксированный enum решений в грамматике** ([Q10](../questions/Q10.md)) —
  отклонено: у каждого банка свой словарь; ядро оперирует решениями как
  свободными идентификаторами.
- **`"Pass"` как решение по умолчанию в контракте** ([Q10](../questions/Q10.md))
  — отклонено: в контракт попадают только реально используемые решения.
- **Русские имена полей объяснения** ([Q42](../questions/Q42.md)) — отклонено
  ради стабильного машиночитаемого JSON-контракта; русскими остаются
  содержательные тексты `decision`/`reason`.

## Ссылки

- Вопросы: [Q8](../questions/Q8.md), [Q9](../questions/Q9.md),
  [Q10](../questions/Q10.md), [Q42](../questions/Q42.md)
- Связанные решения: [D16](D16-dsl-canon-regex-mvp.md) (канон языка v0.1),
  [D17](D17-priority-out-of-mvp.md) (`priority` вне MVP)
- Краткий канон: [`SPECIFICATION.md`](../SPECIFICATION.md) §10, решение №21
- Канон языка: [`GRAMMAR.md`](../GRAMMAR.md)
- Связи: [`TRACEABILITY.md`](../TRACEABILITY.md)
