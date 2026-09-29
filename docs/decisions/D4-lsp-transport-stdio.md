# D4: Транспорт LSP — stdio

- **Статус:** accepted
- **Дата:** 2026-09-29 (до-журнальное решение; оформлено ретроспективно 29.09.2026, [D69](D69-retro-decisions.md))
- **Resolves:** — (до-журнальное решение, вопроса в журнале нет — допустимо для ретро-D)
- **Спека:** [`SPECIFICATION.md`](../SPECIFICATION.md) §10, №4 (историч.; §10 упраздняется решением [D70](D70-spec-reduction.md))
- **Affects:** [`lsp.feature`](../features/lsp.feature), [`lsp_notebook.feature`](../features/lsp_notebook.feature)
- **Tasks:** — (вне периметра MVP; см. «Сверка с кодом»)

## Контекст

До-журнальное решение (строка №4 [`SPECIFICATION.md`](../SPECIFICATION.md) §10).
Формулировка: «Транспорт LSP: stdio»; обоснование: «Стандарт протокола, простота».
Ретроспективное оформление — [D69](D69-retro-decisions.md).

## Решение

LSP sidecar ([D3](D3-lsp-sidecar-process.md)) подключается по **stdio** —
стандартному транспорту протокола, без сетевых портов.

## Следствия

- Клиент (Notebook/IDE) запускает sidecar и общается по stdio.
- MCP-транспорт прототипа тоже stdio, но это другой контур
  ([D29](D29-notebook-mcp-transport.md)); LSP-канал — целевое v0.2.

## Сверка с кодом

Вердикт: ⬜ **не реализовано** — LSP-транспорта в прототипе нет (целевое v0.2).

- **LSP-канала нет:** [`src/`](../../src) без LSP-сервера/клиента; stdio
  используется только контуром MCP ([`mcp.rs`](../../src/mcp.rs), `run_stdio`).
- **Требование — целевое:** [`features/lsp.feature`](../features/lsp.feature) ⬜,
  [`features/lsp_notebook.feature`](../features/lsp_notebook.feature) ⬜.

`cargo` не запускался (§5.3, [D50](D50-dod-by-package-scope.md)): ретро-оформление,
кода не меняет.

**Задач не требуется:** решение — архитектурный ориентир вне периметра MVP
(v0.2, фича [`lsp.feature`](../features/lsp.feature)).

## Альтернативы

Историей не зафиксированы; stdio выбран как стандарт и простейший транспорт.

## Ссылки

- Ретро-оформление: [D69](D69-retro-decisions.md) (Q65)
- Связанные: [D3](D3-lsp-sidecar-process.md) (sidecar-процесс);
  [D5](D5-lsp-client-codemirror.md) (клиент); [D29](D29-notebook-mcp-transport.md)
  (stdio-транспорт MCP — отдельный контур); [D70](D70-spec-reduction.md)
- Строка канона: [`SPECIFICATION.md`](../SPECIFICATION.md) §10, №4 (историч.)
- Связи: [`TRACEABILITY.md`](../TRACEABILITY.md)
