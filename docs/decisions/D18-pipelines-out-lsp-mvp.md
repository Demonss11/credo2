# D18: Конвейеры вне MVP, LSP в MVP — состав сервера и язык v0.1

- **Статус:** accepted
- **Дата:** 2026-09-24
- **Resolves:** [Q36](../questions/Q36.md), [Q38](../questions/Q38.md)
- **Спека:** [`SPECIFICATION.md`](../SPECIFICATION.md) §10, решение №18
- **Affects:** [`SPECIFICATION.md`](../SPECIFICATION.md) §3.3 (состав LSP MVP),
  §5 (модули `dar-core`/`lsp-dar`), §8 (таблица фаз, Фаза 2), §10;
  [`../features/lsp.feature`](../features/lsp.feature) (символы — целевое
  состояние; конвейер — вне v0.1),
  [`../features/graph_view.feature`](../features/graph_view.feature) (⏳,
  отложен); [`../features/README.md`](../features/README.md) (нота Q36/Q38);
  [`GRAMMAR.md`](../GRAMMAR.md) §1/§2 (одно правило в файле), §4 (ключевые
  слова), §5 (v0.2 — конвейеры) — сверено, правок не требуется
- **Tasks:** — (см. «Сверка с кодом»)

## Контекст

Два взаимосвязанных решения одной строки [`SPECIFICATION.md`](../SPECIFICATION.md)
§10 №18. **Q36:** `lsp.feature` (сценарий documentSymbol) ожидал «Конвейер
ПотребКредит» с тремя правилами и одним скорингом в одном файле, тогда как
`SPECIFICATION.md` §10 п. 14 фиксировал «одно правило = один файл», а
`graph_view.feature` (граф конвейера) был помечен ⏳. **Q38:** `lsp.feature`
требовал formatting, semanticTokens, definition, references, documentSymbol,
hover; `SPECIFICATION.md` §3.3/§5 перечислял лишь диагностику, автодополнение и
символы, а модули `dar-core` — `completion.rs`/`explain.rs` без
`format`/`tokens`. Полный контекст — в файлах вопросов.

## Решение

1. **Конвейеры/скоринги/таблицы — вне MVP ([Q36](../questions/Q36.md)).**
   Язык v0.1 — **одно правило в одном `.dar`-файле**
   ([`GRAMMAR.md`](../GRAMMAR.md) §1/§2, ограничение 1; ключевые слова §4).
   `Конвейер`/скоринг/таблицы в MVP не вводятся. Сценарий documentSymbol в
   [`lsp.feature`](../features/lsp.feature) трактуется как «несколько открытых
   файлов» (workspace symbols), а не как дерево внутри одного файла. Граф
   конвейера остаётся отложенным
   ([`graph_view.feature`](../features/graph_view.feature) ⏳). Возврат — v0.2
   вместе с `Приоритет` ([Q4](../questions/Q4.md),
   [D17](D17-priority-out-of-mvp.md)).

2. **LSP входит в MVP ([Q38](../questions/Q38.md)); состав обязателен.**
   initialize/handshake; синхронизация документов (didOpen/didChange);
   diagnostics; completion — ключевые слова + значения решений (поля — вне MVP,
   Q37); hover по правилу;
   documentSymbol/workspaceSymbols (одно правило = один файл, п. 1);
   semanticTokens. **Вне MVP → v0.2:** formatting, definition, references,
   completion полям из реестра (Q37). Реализация —
   крейт `lsp-dar` поверх публичного API `parse_rule` (Фаза 2,
   [`SPECIFICATION.md`](../SPECIFICATION.md) §3.3/§5/§8); стратегия
   «толстый парсер, тонкий интерфейс» — [`GRAMMAR.md`](../GRAMMAR.md) §3.
   Сценарии `lsp.feature` с `Приоритет`, полями и конвейером помечаются как
   целевое состояние.

## Следствия

- **Язык v0.1:** одно правило = один файл; составные конструкции (конвейеры,
  скоринги, таблицы) и `Приоритет` — v0.2 ([`GRAMMAR.md`](../GRAMMAR.md) §2/§4/§5).
- **Требования:** `lsp.feature` — сценарии symbols/definition/references,
  опирающиеся на конвейер/скоринг/`Приоритет`, относятся к целевому состоянию
  (v0.2); состав MVP LSP зафиксирован в `SPECIFICATION.md` §3.3.
  `graph_view.feature` — отложен (⏳); счётчики `features/README.md` не меняются
  (актуальные числа — в `features/README.md`).
- **Nоты:** `features/README.md` несёт ноту «Решения Q36/Q38» (блок пометок,
  `:83`); шапки `lsp.feature`/`graph_view.feature` с обратной ссылкой
  `# Dn (Qn) …` — зона `docs-writer` (§5.6).
- **Отсрочки и возврат:** конвейеры и `Приоритет` возвращаются вместе в v0.2;
  formatting/definition/references и completion полям реестра (Q37) — также
  v0.2. Потребителя/кода в периметре MVP это не меняет.
- **Разграничение:** LSP-сервер — крейт `lsp-dar` (Фаза 2), вне кода прототипа
  `credo2`; `credo-server` (MCP/REST) и `dar-core` (парсер/исполнение) решения
  не затрагивают.

## Сверка с кодом

Вердикт: ⚪ **не применимо** — решения описывают границу MVP языка v0.1 и состав
будущего LSP-сервера (`lsp-dar`, Фаза 2), которого в коде прототипа `credo2`
нет; сверка документная.

Что проверено (чтением, 29.09.2026), чем подтверждено:

- **Кода LSP нет (⬜ вне прототипа):** поиск по `src/**`
  (`rg -n "lsp|graph|pipeline|Конвейер|скоринг" src`) совпадений не дал;
  [`src/core.rs`](../../src/core.rs) — regex-парсер `parse_rule` и `evaluate_rule`
  для **одного** правила; LSP-сервер — `lsp-dar` (Фаза 2,
  [`SPECIFICATION.md`](../SPECIFICATION.md) §3.3/§5/§8).
- **Конвейеров нет (язык v0.1):** [`GRAMMAR.md`](../GRAMMAR.md) — §2
  ограничение 1 «Одно правило в файле: конвейеры — вне v0.1 (Q36)» (`:50`),
  §4 `Приоритет` ❌ (вернётся с конвейерами), §5 — конвейеры в v0.2 (`:97`).
  Правок не требуется — канон уже отражает решение.
- **Карта зон** — [`features/README.md`](../features/README.md) «Соответствие
  коду»: `lsp.feature` и `graph_view.feature` — **целевое/отложенное
  состояние** (в карте зон кода нет), не код `credo2`.
- **SPEC — ✅ (решение отражено полностью):**
  [`SPECIFICATION.md`](../SPECIFICATION.md) §3.3 (`:232–236`: состав MVP LSP —
  Q38, «одно правило = один файл, Q36»), §8 (Фаза 2 — `lsp-dar`, `:777`),
  §10 №18 (`:841`).
- **Фичи-адресаты:** [`lsp.feature`](../features/lsp.feature) — сценарий
  «Символы документа (outline)» (`:128–133`, «Конвейер ПотребКредит»),
  definition/references (`:116–126`) — целевое состояние; шапка несёт пометки
  D17 (Q4) и Q37. [`graph_view.feature`](../features/graph_view.feature) —
  4 сценария про конвейер, отложен (⏳). Правки шапок — зона `docs-writer`.

`cargo` не запускался (§5.3, [D50](D50-dod-by-package-scope.md)): решение
документное (граница MVP и состав Фазы 2), адресный прогон не требуется.

**Задач не требуется:** LSP-сервер — крейт `lsp-dar` (Фаза 2), вне кода
прототипа `credo2`; конвейеров в языке v0.1 нет, кода прототипа решение не
меняет. [`tasks/README.md`](../tasks/README.md) к Q36/Q38 не привязан
([`T-14`](../tasks/T-14-grammar-message-sync/README.md) — синхронизация цитаты
сообщения парсера из `GRAMMAR.md`, не об этом). Расхождений в периметре MVP нет.

## Альтернативы

- **Ввести конвейеры/скоринги/таблицы в MVP** ([Q36](../questions/Q36.md)) —
  отклонено: требует расширения языка (несколько правил в файле) и парсера,
  выходит за regex-минимум v0.1; демо показывает полный цикл «правило →
  проверка» без составных конструкций.
- **Убрать сценарии конвейера/скоринга из фич совсем** — отклонено: конвейеры
  возвращаются в v0.2 вместе с `Приоритет`; сценарии остаются целевым
  состоянием.
- **Полный набор LSP в MVP** ([Q38](../questions/Q38.md)) — отклонено:
  formatting/definition/references (и completion полям реестра, Q37) требуют
  лексера/AST и справочников (v0.2); MVP ограничен `parse_rule`-совместимым
  набором.
- **Вынести LSP за пределы MVP** — отклонено: LSP — ключевая вау-функция
  редактора (Фаза 2 обязательна для демо), входит в MVP по подтверждению
  владельца.

## Ссылки

- Вопросы: [Q36](../questions/Q36.md), [Q38](../questions/Q38.md)
- Связанные: [Q4](../questions/Q4.md)
  ([D17](D17-priority-out-of-mvp.md) — `Приоритет` вне MVP, возврат в v0.2);
  [Q37](../questions/Q37.md) (закрыт D33), [Q39](../questions/Q39.md); [D22](D22-rest-paths-canon.md) (конвейеры —
  отдельный ресурс v0.2), [D36](D36-batch-deferred.md) (batch),
  [D37](D37-client-explanation-deferred.md) (объяснение клиента) —
  перечисляют конвейеры как v0.2-зависимость
- Краткий канон: [`SPECIFICATION.md`](../SPECIFICATION.md) §10, решение №18;
  §3.3, §5, §8 (Фаза 2)
- Канон языка: [`GRAMMAR.md`](../GRAMMAR.md)
- Связи: [`TRACEABILITY.md`](../TRACEABILITY.md)
