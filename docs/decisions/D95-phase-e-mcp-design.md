# D95: Фаза E — открытие: дизайн process-MCP (задача T-26)

- **Статус:** accepted
- **Дата:** 2026-10-03
- **Resolves:** [Q92](../questions/Q92.md)
- **Спека:** —
- **Affects:** новая задача [`T-26`](../tasks/T-26-mcp-server-design/README.md),
  сводка [`tasks/README.md`](../tasks/README.md),
  карточка [`T-15`](../tasks/T-15-mcp-ready-process/README.md) (фаза E),
  [`agents-mcp-readiness.feature`](../features/agents-mcp-readiness.feature),
  [`TRACEABILITY.md`](../TRACEABILITY.md) (строка [Q92](../questions/Q92.md)/D95)
- **Tasks:** [`T-26`](../tasks/T-26-mcp-server-design/README.md) (⬜; зависит от
  заморозки процесса — фаза D [`T-15`](../tasks/T-15-mcp-ready-process/README.md))

## Контекст

Программа [T-15](../tasks/T-15-mcp-ready-process/README.md) (D78) ведёт процесс
агентного контура к «сериализации» в MCP-сервер: **storage + validation + query**
над состоянием прогона, **без принятия решений** (записка
[`mcp-ready-process.md`](../tasks/T-15-mcp-ready-process/mcp-ready-process.md)
§0, §3). Владелец задал последовательность «сначала процесс — потом MCP»; фаза E
карточки [`T-15`](../tasks/T-15-mcp-ready-process/README.md) `:161-164` —
«дизайн MCP-сервера — 6–10 инструментов по списку операций; отдельная задача
(после заморозки)». Приложение A записки §241-257 описывает **git-MCP** —
другой слой, следующий горизонт. Полный контекст и развилки —
[Q92](../questions/Q92.md).

Ответы владельца 03.10.2026 (сервисная операция r17): **(1)** предмет фазы E —
**только process-MCP**; **(2)** задача заводится ⬜ с зависимостью «после
заморозки процесса (фаза D)».

## Решение

1. **Фаза E открывается как отдельная задача**
   [`T-26`](../tasks/T-26-mcp-server-design/README.md) — **дизайн
   process-MCP**: 6–10 инструментов по списку операций записки §5.4
   (storage + validation + query); **решений не принимает** — исполняет уже
   принятое (границы §3 записки).
2. **Предмет — только process-MCP.** git-MCP (Приложение A) в предмет `T-26` не
   входит: это отдельный горизонт (другой слой), позже — после отладки
   файлового MCP; его дизайн/scope — отдельной задачей.
3. **Старт — после заморозки процесса** (фаза D [`T-15`](../tasks/T-15-mcp-ready-process/README.md)):
   `T-26` заводится со статусом ⬜ и зависимостью «заморозка процесса (фаза D
   T-15)». Вход задачи — также **подготовленный B2** ([D94](D94-b2-profile-flag.md),
   `B2_PROFILE`).
4. **Класс/маршрут.** Априори **L** (контракты/новая подсистема); финально
   фиксирует `analyst` при взятии.
5. **Референсы дизайна** — kibi (traceability), Semantic Anchors (контракты),
   BRHP (validation signals), Telemetry DB (query-слой) — записка §10.
6. **Связь с T-15:** карточка [`T-15`](../tasks/T-15-mcp-ready-process/README.md)
   (фаза E) ссылается на [`T-26`](../tasks/T-26-mcp-server-design/README.md);
   прогресс фазы E ведётся по `T-26`, не строкой внутри T-15.
7. **Приоритет `T-26`** — P1 (продолжение программы [`T-15`](../tasks/T-15-mcp-ready-process/README.md),
   категория «контракты»); старт гейтится заморозкой (фаза D), но работа —
   в рамках доведения процесса до готовности к MCP.

## Следствия

- [`T-26`](../tasks/T-26-mcp-server-design/README.md) видна в
  [`TRACEABILITY.md`](../TRACEABILITY.md) через пару
  [Q92](../questions/Q92.md)/D95 (правило
  [D77](D77-tasks-visibility-completeness.md)); статус ⬜ — в реестре и карточке.
- Фаза E карточки [`T-15`](../tasks/T-15-mcp-ready-process/README.md) получает
  ссылку на задачу; отдельной строки `E1`/`E2` реестр T-15 не отменяет — они
  остаются в сводной таблице карточки.
- git-MCP (Приложение A) остаётся вне предмета: его оформление — при отдельном
  решении владельца.
- Продуктовый код `src/**` и контракты CREDO не затрагиваются; `T-26` —
  дизайн-документ.

## Сверка с кодом

Вердикт: ⚪ **не применимо** (процесс/план) — решение открывает фазу E как
дизайн-задачу и канонизирует её границы; продуктовый код и контракты CREDO не
меняет.

Что проверено (чтением, 03.10.2026), чем подтверждено:

- **Свобода номеров:** созданный [`Q92`](../questions/Q92.md) и `D95`;
  последние занятые — [Q91](../questions/Q91.md)/[D94](D94-b2-profile-flag.md)
  ([`questions/README.md`](../questions/README.md),
  [`decisions/README.md`](../decisions/README.md)); нумерация сквозная (§2
  [`journal.md`](../../.opencode/rules/journal.md)).
- **Свобода задачи:** `T-26` — первая свободная после
  [`T-25`](../tasks/T-25-d65-analysis-addresses/README.md)
  ([`tasks/README.md`](../tasks/README.md)).
- **Фаза E карточки T-15:** `:161-164` — «дизайн MCP-сервера — 6–10
  инструментов по списку операций; storage + validation + query; решений не
  принимает; отдельная задача (после заморозки)».
- **Каталог операций:** записка §5.4 (`≤10` операций; `append_progress`,
  `set_next_action`, `set_current_state`, `register_receipt`, `register_re_raise`/
  `resolve_re_raise`, `record_dispatch_metrics`, `add_deferred`/`resolve_deferred`,
  `checkpoint_session`); §7 — query-слой (метрики из состояния).
- **Приложение A (git-MCP):** записка §241-257 — операции `git.*`, права по
  инструментам, «другой слой»; в предмет `T-26` не входит.
- **Вход B2:** [D94](D94-b2-profile-flag.md) п.6 — фаза E отдельной задачей,
  вход — подготовленный B2.

`cargo` не запускался (§5.3, [D50](D50-dod-by-package-scope.md)): решение
процессное, кода CREDO не касается; адресный прогон (`docs_journal`) — за
`validator` на приёмке (R2).

**Задача —** [`T-26`](../tasks/T-26-mcp-server-design/README.md) (⬜): дизайн
process-MCP; зависит от заморозки процесса (фаза D
[`T-15`](../tasks/T-15-mcp-ready-process/README.md)).

## Альтернативы

- **(B) Process-MCP + git-MCP сразу** — отклонено: git-MCP — другой слой
  (записка §Приложение A), его дизайн преждевременен до отладки файлового MCP;
  шире предмет и выше риск.
- **(C) E — продолжение T-15 без отдельной задачи** — отклонено: фаза E
  достаточно обособлена (класс априори L — контракты/новая подсистема); ведётся
  своей задачей, прогресс — по `T-26`.

## Ссылки

- Вопрос: [Q92](../questions/Q92.md)
- Основания: [D78](D78-t15-mcp-ready-program.md) (программа T-15),
  [D94](D94-b2-profile-flag.md) (B2 — вход E)
- Артефакты: карточка [`T-15`](../tasks/T-15-mcp-ready-process/README.md)
  (фаза E); записка
  [`mcp-ready-process.md`](../tasks/T-15-mcp-ready-process/mcp-ready-process.md)
  (§5–§10, Приложение A); карточка
  [`T-26`](../tasks/T-26-mcp-server-design/README.md)
- Сервисная операция 03.10.2026 (r17) — решение владельца (лента)
- Связи: [`TRACEABILITY.md`](../TRACEABILITY.md)
