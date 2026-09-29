# D29: Транспорт MCP в Notebook — локальный sidecar по stdio

- **Статус:** accepted
- **Дата:** 2026-09-26
- **Resolves:** [Q30](../questions/Q30.md)
- **Спека:** [`SPECIFICATION.md`](../SPECIFICATION.md) §10, решение №29
- **Affects:** [`SPECIFICATION.md`](../SPECIFICATION.md) §2.2 (`:134`), §2.3
  (`:178`, `:182–187`), §4.1 (`:267–270`), §4.2 (`:303–317`) — stdio — транспорт
  MVP, credo-server как sidecar Notebook, режим «только MCP», внешний сервер —
  v0.2+; §10;
  [`features/agent_minimal.feature`](../features/agent_minimal.feature)
  (шапка/комментарий `# D29 (Q30) …` — зона `docs-writer`);
  [`features/README.md`](../features/README.md) (заметка про транспорт MCP)
- **Tasks:** — (см. «Сверка с кодом»)

## Контекст

SPEC §2.3/§4.1 описывал «MCP-клиент (подключение к credo-server)», транспорт MCP
— «stdio / HTTP+SSE», а `agent_minimal.feature` — «панель подключена к
MCP-серверу через stdio». Не было сказано, кто запускает сервер: Notebook сам
спавнит процесс или подключается к развёрнутому. Полный контекст —
[Q30](../questions/Q30.md).

## Решение

1. **MVP: Notebook спавнит `credo-server` как локальный sidecar-процесс через
   stdio (JSON-RPC 2.0).** Это штатный режим демо.
2. **credo-server работает в режиме «только MCP» (без REST)** — согласно
   [Q27](../questions/Q27.md) ([D27](D27-rest-launch-address.md)): без флагов
   сервер поднимает только stdio-транспорт.
3. **Опубликованный bare-репозиторий** (`.credo/published-repo/`) — локальный, на
   машине пользователя. Совместная работа — вне MVP (согласуется с Q32:
   workspace-репозиторий и published-repo локальны).
4. **Notebook автоматически запускает `credo-server` при старте** (скрыто от
   пользователя). Пользователь видит только UI.
5. **HTTP+SSE транспорт и внешний (общий) сервер — вне MVP**, вернутся для
   multi-user сценариев (v0.2+).

## Следствия

- `SPECIFICATION.md` §2.2 (`:134`), §2.3 (`:178`, `:182–187`), §4.1
  (`:267–270`), §4.2 (`:303–317`) — stdio — транспорт MVP, credo-server как
  sidecar Notebook, режим «только MCP», внешний сервер — v0.2+; §10 — решение
  №29.
- [`features/agent_minimal.feature`](../features/agent_minimal.feature) —
  автозапуск sidecar, режим «только MCP» (шапка/комментарий `# D29 (Q30) …` —
  зона `docs-writer`); [`features/README.md`](../features/README.md) — заметка
  про транспорт MCP.
- Интеграция (спавн и автозапуск sidecar) — **задача Notebook**, вне кода
  прототипа `credo2`; правок в `src/**` не требуется.
- Согласовано с [D27](D27-rest-launch-address.md) (режим «только MCP» без
  флагов — штатный) и [D54](D54-source-of-truth-flow.md) (локальный
  `published-repo`); совместная работа и общий сервер — v0.2+.

## Сверка с кодом

Вердикт: ✅ **соответствует** — `credo2` уже работает как MCP-сервер по stdio
(штатный режим без флагов); интеграция Notebook (спавн и автозапуск sidecar) —
задача Notebook, вне кода прототипа.

Что проверено (чтением кода и SPEC, 29.09.2026), чем подтверждено:

- [`src/main.rs`](../../src/main.rs) — без флагов `rest_addr = None` (`:56–65`),
  REST не поднимается (`:85–90`), запускается `mcp::run_stdio` (`:92`) — это и
  есть режим «только MCP (stdio)» из решения (п. 2), согласованный с
  [D27](D27-rest-launch-address.md).
- [`src/mcp.rs`](../../src/mcp.rs) — `pub async fn run_stdio(...)` (`:18`),
  транспорт — `rmcp::transport::stdio()` (`:23`); инструменты `check.*`
  объявлены в `tool_specs()` (реализация MCP-сервера по stdio).
- [`SPECIFICATION.md`](../SPECIFICATION.md) §2.2 (`:134`) — строка ролей:
  `credo-server` — «Локально (sidecar Notebook, MVP) или на сервере (v0.2+)»;
  §2.3 (`:178`, `:182–187`) — таблица транспортов (MCP: stdio в MVP; HTTP+SSE —
  v0.2+) и пометка Q30 о sidecar Notebook; §4.1 (`:267–270`) — «MCP-транспорт в
  MVP (Q30): Notebook спавнит `credo-server` как локальный sidecar через stdio»;
  §4.2 (`:303–317`; ссылка на Q30 — `:307`, схема сервера — `:286`) — режим
  «только MCP (stdio)» и запуск REST (правок в коде не требуется, `:318–319`);
  §10 — решение №29 (`:852`).
- Карта зон — [`features/README.md`](../features/README.md) «Соответствие коду»:
  MCP-инструменты → [`../../src/mcp.rs`](../../src/mcp.rs) (`:326`), CLI →
  [`../../src/main.rs`](../../src/main.rs) (`:328`).

`cargo` не запускался (§5.3, [D50](D50-dod-by-package-scope.md)): решение не
меняет код прототипа; сверено чтением (адресный прогон, если понадобится, — за
`validator`). Расхождений не выявлено.

**Задач не требуется:** правок в коде прототипа решение не требует — `credo2`
уже MCP-сервер по stdio; целевое поведение Notebook (спавн sidecar, автозапуск
при старте, скрытый от пользователя) — вне периметра `credo2`, это задача
Notebook. Требования `features/*.feature` не переписываются (обратный комментарий
`# D29 (Q30) …` в шапке `agent_minimal.feature` — зона `docs-writer`, §5.6).

## Альтернативы

- **Внешний (общий) `credo-server`** — требует развёртывания, общего состояния и
  настройки секретов; противоречит локальности MVP; отклонено (вернётся в v0.2+
  для multi-user).
- **HTTP+SSE транспорт в MVP** — избыточен для локального демо; stdio (JSON-RPC
  2.0) проще и не открывает порт; отклонено.
- **Ручной запуск сервера пользователем** — противоречит цели «пользователь
  видит только UI»; Notebook запускает sidecar автоматически (скрыто); отклонено.

## Ссылки

- Вопрос: [Q30](../questions/Q30.md)
- Связанные: [Q27](../questions/Q27.md) (запуск и адрес REST; режим «только
  MCP» — [D27](D27-rest-launch-address.md)); [Q12](../questions/Q12.md)
  ([D54](D54-source-of-truth-flow.md)); Q32 (локальность workspace/published-repo)
  — ожидает переноса
- Краткий канон: [`SPECIFICATION.md`](../SPECIFICATION.md) §10, решение №29;
  §2.2 (`:134`), §2.3 (`:178`, `:182–187`), §4.1 (`:267–270`), §4.2 (`:303–317`)
- Связи: [`TRACEABILITY.md`](../TRACEABILITY.md)
