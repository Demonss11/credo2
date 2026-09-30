# D2: Файл = источник истины (file-first)

- **Статус:** accepted
- **Дата:** 2026-09-29 (до-журнальное решение; оформлено ретроспективно 29.09.2026, [D69](D69-retro-decisions.md))
- **Resolves:** — (до-журнальное решение, вопроса в журнале нет — допустимо для ретро-D)
- **Спека:** [`SPECIFICATION.md`](../SPECIFICATION.md) §10, №2 (историч.; §10 упраздняется решением [D70](D70-spec-reduction.md))
- **Affects:** [`draft.feature`](../features/draft.feature), [`test_draft.feature`](../features/test_draft.feature), [`publish.feature`](../features/publish.feature); [`lib.rs`](../../src/lib.rs), [`mcp.rs`](../../src/mcp.rs)
- **Tasks:** — (реализация потока — [D54](D54-source-of-truth-flow.md)/T-01/T-08; здесь не дублируются)

## Контекст

До-журнальное решение (строка №2 [`SPECIFICATION.md`](../SPECIFICATION.md) §10).
Формулировка: «Файл = источник истины»; обоснование: «Принцип file-first. Нет
привязки к БД». Ретроспективное оформление — [D69](D69-retro-decisions.md).
Принцип конкретизирован позже в [D54](D54-source-of-truth-flow.md) (поток
`файл → черновик → публикация`).

## Решение

Источник истины правила — `.dar`-файл в workspace; песочница черновиков и
реестр публикаций — производные слои, а не первичное хранилище. База данных для
правил не требуется.

## Следствия

- Черновик хранит `source` и `source_hash`; расхождение файла и черновика —
  метка `stale` ([D54](D54-source-of-truth-flow.md)/[T-16](../tasks/T-16-stale-check-test/README.md)).
- Публикация — из проверенного состояния черновика; материализация `.dar` при
  публикации — [T-08](../tasks/T-08-materialize-source-file/README.md).

## Сверка с кодом

Вердикт: 🟡 **расхождение** — принцип реализован частично и целиком покрыт
каноном [D54](D54-source-of-truth-flow.md) с задачами [T-01](../tasks/T-01-draft-source-hash/README.md)
(✅), [T-08](../tasks/T-08-materialize-source-file/README.md) (⬜) и
[T-16](../tasks/T-16-stale-check-test/README.md) (⬜).

- **Черновик хранит источник и хэш:** [`mcp.rs`](../../src/mcp.rs) `:239–258`
  (`source_hash`, `tested_at`), `:440` (`stale` при расхождении `checksum`);
  юнит-тесты `:618+` (T-01).
- **Публикация — из производного слоя:** `src/lib.rs`/`src/mcp.rs` (реестр
  версий), без записи `.dar`-файла на публикации.
- Периметр MVP: расхождение закрыто задачами [D54](D54-source-of-truth-flow.md)
  (T-01/T-08/T-16), отдельной задачи у №2 нет — не дублировать.

`cargo` не запускался (§5.3, [D50](D50-dod-by-package-scope.md)): ретро-оформление,
кода не меняет.

**Задач не требуется:** канон потока — [D54](D54-source-of-truth-flow.md);
его открытые задачи уже заведены и не дублируются.

## Альтернативы

Историей не зафиксированы: сжатый перечень §10 до-журнального периода;
альтернатива «БД как источник истины» отвергнута самим принципом file-first.

## Ссылки

- Ретро-оформление: [D69](D69-retro-decisions.md) (Q65)
- Связанные: [D54](D54-source-of-truth-flow.md) (поток источника истины — канон
  конкретизации); [D70](D70-spec-reduction.md)
- Строка канона: [`SPECIFICATION.md`](../SPECIFICATION.md) §10, №2 (историч.)
- Связи: [`TRACEABILITY.md`](../TRACEABILITY.md)
