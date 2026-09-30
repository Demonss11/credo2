# D80: Полнота фич — каждая `*.feature` поимённо видна в TRACEABILITY

- **Статус:** accepted
- **Дата:** 2026-09-30
- **Resolves:** [Q76](../questions/Q76.md)
- **Спека:** —
- **Affects:** [`TRACEABILITY.md`](../TRACEABILITY.md) (колонка «Реализация» —
  поимённо), [`features/README.md`](../features/README.md) (пометка полноты —
  `docs-writer`), заголовки
  [`wasm.feature`](../features/wasm.feature)/[`manifest_sync.feature`](../features/manifest_sync.feature)
  (`docs-writer`), карточка [`T-18`](../tasks/T-18-docs-journal-test/README.md)
  (добор проверок); согласовано с [D77](D77-tasks-visibility-completeness.md)
  (аналогичный контур задач)
- **Tasks:** [`T-18`](../tasks/T-18-docs-journal-test/README.md) (добор проверок
  полноты фич; см. «Сверка с кодом»)

## Контекст

Анализ 30.09.2026: **47** фич `*.feature`; поимённо в
[`TRACEABILITY.md`](../TRACEABILITY.md) — **35**; без имени — **12**:
`manifest_sync`, `wasm` и 10 агентских (кроме `agents-cycle`); wildcard
«`agents-*.feature` (6 файлов)» у Q44 устарел (файлов — 11). Полнота задач уже
закреплена правилом [D77](D77-tasks-visibility-completeness.md); для фич — тот же
контур, но без правила. Полный контекст — [Q76](../questions/Q76.md).

## Решение

1. **Правило:** каждая фича из [`features/README.md`](../features/README.md)
   **поимённо** представлена в [`TRACEABILITY.md`](../TRACEABILITY.md) (колонка
   «Реализация») хотя бы одной строкой; wildcard-обобщения
   (`agents-*.feature`, «и др.») как представление **не используются**.
2. **Источник связи** — заголовок фичи (`# Dn (Qx)`) либо журнальное основание
   (`T-15` → [D78](D78-t15-mcp-ready-program.md)/[Q74](../questions/Q74.md)); у
   «сирот» заголовки восполняются: `wasm` →
   [D11](D11-wasm-native-first.md); `manifest_sync` →
   [D34](D34-mcp-tool-contracts.md)/[D28](D28-two-git-contours.md) (`docs-writer`).
3. **Применение:** Q43+Q44 — шесть агентских D38/D39 поимённо; Q74/D78 — пять фич
   `T-15`; Q29/D34 и Q32/D28 — `manifest_sync`; Q65/D69 — `wasm` (ретро-линия
   D11).
4. **Машинная проверка** — добор в `tests/docs_journal.rs`
   ([`T-18`](../tasks/T-18-docs-journal-test/README.md), v0.1): полнота фич и
   обратная живость.
5. **Обратная полнота:** каждая фича, упомянутая в
   [`TRACEABILITY.md`](../TRACEABILITY.md), существует в
   [`features/`](../features/README.md).

## Следствия

- Wildcard-счётчики и «и др.» уходят из [`TRACEABILITY.md`](../TRACEABILITY.md);
  устаревшие числа («6 файлов») не дрейфуют (один факт — один канон).
- Сироты (`wasm`, `manifest_sync`) получили журнальную линию (D11; D34/D28).
- Машинная проверка полноты фич — [`T-18`](../tasks/T-18-docs-journal-test/README.md).

## Сверка с кодом

Вердикт: ⚪ **не применимо** (документы/процесс) — решение фиксирует правило
представления фич в индексе журнала; продуктовый код прототипа не меняет. Факты по
§5.3 (чтением, 30.09.2026):

- [`features/`](../features/README.md) — **47** `*.feature`;
- [`TRACEABILITY.md`](../TRACEABILITY.md) — **12** фич без имени
  (`manifest_sync`, `wasm`, 10 агентских), у Q44 wildcard «(6 файлов)» устарел
  (файлов 11);
- заголовки [`wasm.feature`](../features/wasm.feature)/
  [`manifest_sync.feature`](../features/manifest_sync.feature) — без `# D`
  (основания: [D11](D11-wasm-native-first.md);
  [D34](D34-mcp-tool-contracts.md)/[D28](D28-two-git-contours.md)).

`cargo` не запускался (§5.3, [D50](D50-dod-by-package-scope.md)): запись
документная, кода прототипа не меняет.

**Задача —** [`T-18`](../tasks/T-18-docs-journal-test/README.md): добор проверок
полноты фич.

## Альтернативы

- **(б) Wildcard-обобщения** — отклонено: не поимённо; счётчик «6 файлов» уже
  устарел.
- **(в) Разовый фикс без правила** — отклонено: повторит кейс `T-15` (дрейф без
  правила).

## Ссылки

- Вопрос: [Q76](../questions/Q76.md)
- Связанные: [D77](D77-tasks-visibility-completeness.md) (полнота задач —
  аналогичный контур), [D63](D63-journal-index-lifecycle.md) (единая таблица),
  [D64](D64-journal-integrity-test.md) (тест целостности, T-18),
  [D20](D20-features-docs-dod.md) (фичи — документация)
- Артефакты: [`features/README.md`](../features/README.md);
  [`TRACEABILITY.md`](../TRACEABILITY.md)
