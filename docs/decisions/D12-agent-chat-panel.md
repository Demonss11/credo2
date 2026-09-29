# D12: Чат агента — правая панель основного окна Notebook

- **Статус:** accepted
- **Дата:** 2026-09-26
- **Resolves:** [Q31](../questions/Q31.md)
- **Спека:** [`SPECIFICATION.md`](../SPECIFICATION.md) §10, решение №12
- **Affects:** [`SPECIFICATION.md`](../SPECIFICATION.md) §10;
  [`features/notebook_ui.feature`](../features/notebook_ui.feature),
  [`features/agent_minimal.feature`](../features/agent_minimal.feature),
  [`features/inline_execution.feature`](../features/inline_execution.feature)
  (целевое состояние; шапки/комментарии `# Dn (Qn) …` — зона `docs-writer`);
  [`features/README.md`](../features/README.md) (заметка Q31)
- **Tasks:** — (см. «Сверка с кодом»)

## Контекст

SPEC §10 (п. 12) оставлял чат-интерфейс «отдельно от основного UI | Обсуждается
отдельно», при этом `notebook_ui.feature` описывал правую панель «содержит чат с
агентом», а `agent_minimal.feature` — панель в приложении. Место чата и судьба
результатов исполнения (`Explanation`) не были зафиксированы. Полный контекст —
[Q31](../questions/Q31.md).

## Решение

1. **Чат с агентом — правая панель основного окна Notebook**, третья колонка
   трёхколоночного layout (файловое дерево | редактор | чат). По умолчанию
   видима; можно стянуть через resize или скрыть (toggle).
2. **Чат общий для всего workspace**, но агент получает контекст активного
   правила (открытый таб) как часть MCP-запроса.
3. **Результаты исполнения** (`Explanation` из `check.test`):
   - не показываются отдельным сообщением в чате;
   - отображаются инлайн в редакторе: подсветка строки условия, которое
     сработало, плюс блок результата под правилом (человекочитаемый формат из
     `Explanation`);
   - это реализует принцип «результат рядом с кодом».
   Механизм исполнения и источник `Explanation` — [Q33](../questions/Q33.md)
   ([D30](D30-execution-mechanism.md)).
4. **Рассуждения агента** (chain-of-thought, вызовы инструментов):
   - отображаются в чате как живой диалог;
   - вызовы инструментов — сворачиваемые блоки («▶ Вызов `check.create`»,
     «▶ Вызов `check.test`»);
   - JSON-ответы инструментов в чате не показываются, только краткое
     человекочитаемое резюме.
5. **Отдельное окно чата** (внешний Slack) — вне MVP.

## Следствия

- `SPECIFICATION.md` §10 (решение №12) — формулировка чата-панели, инлайн-
  результатов и границы MVP; п. 12 переформулирован (был «отдельно от основного
  UI»).
- Требования (целевое состояние; шапки — зона `docs-writer`):
  [`features/notebook_ui.feature`](../features/notebook_ui.feature) (панель чата:
  видима по умолчанию, resize/toggle),
  [`features/agent_minimal.feature`](../features/agent_minimal.feature) (контекст
  активного правила, инлайн-результаты, свёрнутые вызовы без JSON),
  [`features/inline_execution.feature`](../features/inline_execution.feature)
  (подсветка сработавшего условия),
  [`features/README.md`](../features/README.md) (счётчики, заметка).
- Инлайн-результаты опираются на `Explanation`, который возвращает MCP
  `check.test`; механизм — [Q33](../questions/Q33.md)
  ([D30](D30-execution-mechanism.md)), транспорт — [Q30](../questions/Q30.md)
  ([D29](D29-notebook-mcp-transport.md)). Сам чат-UI — часть Notebook.
- Интеграция (layout, панель, рендер инлайн-результатов) — **задача Notebook**,
  вне кода прототипа `credo2`; правок в `src/**` не требуется.

## Сверка с кодом

Вердикт: ⚪ **не применимо** — решение описывает UI-слой DAR Notebook (место и
поведение чата), которого в коде прототипа `credo2` нет; сверка документная.

Что проверено (чтением, 29.09.2026), чем подтверждено:

- [`src/`](../../src/) — только `main.rs` (CLI/запуск), `mcp.rs` (MCP-сервер),
  `rest.rs` (REST/OpenAPI), `core.rs` (домен и `Explanation`), `lib.rs`
  (хранилище/манифест); чата, панели и UI-слоя в `credo2` нет — отсюда ⚪.
- Карта зон — [`features/README.md`](../features/README.md) «Соответствие коду»
  (`:322–329`): домен/DSL → `src/core.rs`, хранилище → `src/lib.rs`,
  MCP-инструменты → `src/mcp.rs`, REST → `src/rest.rs`, CLI → `src/main.rs`;
  UI-зоны в карте нет — Notebook вне периметра прототипа.
- MCP `check.test` и структура `Explanation` в прототипе уже есть
  ([`src/mcp.rs`](../../src/mcp.rs), [`src/core.rs`](../../src/core.rs)), но это
  предмет [Q33](../questions/Q33.md) ([D30](D30-execution-mechanism.md)), а не
  места чата.
- Требования `features/notebook_ui.feature`, `features/agent_minimal.feature`,
  `features/inline_execution.feature` — **целевое состояние** (Notebook), не код
  `credo2`; счётчики [`features/README.md`](../features/README.md) не меняются.

`cargo` не запускался (§5.3, [D50](D50-dod-by-package-scope.md)): решение не
меняет код прототипа; адресный прогон, если понадобится, — за `validator`.

**Задач не требуется:** правок в коде прототипа решение не требует — чат и
инлайн-результаты живут в UI Notebook, вне периметра `credo2`. Требования
`features/*.feature` не переписываются (обратные комментарии `# Dn (Qn) …` в
шапках — зона `docs-writer`, §5.6). Механизм исполнения (`check.test`/`check.run`)
покрыт [Q33](../questions/Q33.md) ([D30](D30-execution-mechanism.md)) и задачами
T-08/T-09.

## Альтернативы

- **Отдельное окно чата (внешний Slack)** — отклонено для MVP: разрывает
  контекст «результат рядом с кодом» и требует внешнего сервиса; вернётся
  пост-MVP.
- **Показывать результаты `check.test` сообщением в чате** — отклонено:
  инлайн-подсветка в редакторе ближе к принципу «результат рядом с кодом»; чат
  остаётся для рассуждений агента.
- **JSON-ответы инструментов в чате как есть** — отклонено: шум для
  риск-технолога; показывается краткое человекочитаемое резюме.

## Ссылки

- Вопрос: [Q31](../questions/Q31.md)
- Связанные: [Q33](../questions/Q33.md)
  ([D30](D30-execution-mechanism.md) — единый механизм исполнения и
  `Explanation`); [Q30](../questions/Q30.md)
  ([D29](D29-notebook-mcp-transport.md) — транспорт MCP в Notebook);
  [Q28](../questions/Q28.md)
  ([D31](D31-check-create-contract.md) — контракт `check.create`)
- Краткий канон: [`SPECIFICATION.md`](../SPECIFICATION.md) §10, решение №12
- Связи: [`TRACEABILITY.md`](../TRACEABILITY.md)
