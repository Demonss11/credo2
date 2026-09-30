# D8: MCP для агента

- **Статус:** accepted
- **Дата:** 2026-09-29 (до-журнальное решение; оформлено ретроспективно 29.09.2026, [D69](D69-retro-decisions.md))
- **Resolves:** — (до-журнальное решение, вопроса в журнале нет — допустимо для ретро-D)
- **Спека:** [`SPECIFICATION.md`](../SPECIFICATION.md) §10, №8 (историч.; §10 упраздняется решением [D70](D70-spec-reduction.md))
- **Affects:** [`mcp.rs`](../../src/mcp.rs); [`mcp_tools.feature`](../features/mcp_tools.feature), [`agent_minimal.feature`](../features/agent_minimal.feature)
- **Tasks:** — (реализовано; см. «Сверка с кодом»)

## Контекст

До-журнальное решение (строка №8 [`SPECIFICATION.md`](../SPECIFICATION.md) §10).
Формулировка: «MCP для агента»; обоснование: «Стандарт для AI-инструментов».
Ретроспективное оформление — [D69](D69-retro-decisions.md). Контракты и
транспорт уточнены позже: [D34](D34-mcp-tool-contracts.md),
[D29](D29-notebook-mcp-transport.md), [D30](D30-execution-mechanism.md).

## Решение

Интеграция агента в CREDO — через **MCP-инструменты** (`check.*`), по стандарту
протокола, а не через произвольный внутренний API.

## Следствия

- Единый MCP-слой обслуживает и агента, и Notebook-редактор
  ([D29](D29-notebook-mcp-transport.md)/[D30](D30-execution-mechanism.md)).
- Контракты инструментов и коды ошибок — [D34](D34-mcp-tool-contracts.md).

## Сверка с кодом

Вердикт: ✅ **соответствует**.

- **MCP-сервер реализован:** [`src/mcp.rs`](../../src/mcp.rs) — RMCP-сервер
  (`run_stdio`), спецификации инструментов `check.*` (`tool_specs`),
  draft-first; `src/main.rs` поднимает stdio.
- **Инструменты:** `check.create/get/test/delete/publish/deprecate/list_published/
  rebuild_manifest` (канон — `AGENTS.md`, контракты — [D34](D34-mcp-tool-contracts.md)).
- Подтверждено чтением кода и картой зон [`features/README.md`](../features/README.md)
  (`mcp_tools.feature`); адресный прогон не требуется.

`cargo` не запускался (§5.3, [D50](D50-dod-by-package-scope.md)): ретро-оформление,
кода не меняет.

**Задач не требуется:** MCP-слой реализован; открытые уточнения контрактов —
в задачах [D34](D34-mcp-tool-contracts.md) (T-04/T-05), здесь не дублируются.

## Альтернативы

Историей не зафиксированы; MCP выбран как стандарт для AI-инструментов.

## Ссылки

- Ретро-оформление: [D69](D69-retro-decisions.md) (Q65)
- Связанные: [D34](D34-mcp-tool-contracts.md) (контракты MCP);
  [D29](D29-notebook-mcp-transport.md) (транспорт sidecar/stdio);
  [D30](D30-execution-mechanism.md) (единый механизм исполнения); [D70](D70-spec-reduction.md)
- Строка канона: [`SPECIFICATION.md`](../SPECIFICATION.md) §10, №8 (историч.)
- Связи: [`TRACEABILITY.md`](../TRACEABILITY.md)
