# D86: Схема состояния процесса — отдельный канон, модель D42

- **Статус:** accepted
- **Дата:** 2026-10-02
- **Resolves:** [Q83](../questions/Q83.md)
- **Спека:** — (SPEC §10 — указатель [D70](D70-spec-reduction.md); запись в
  `SPECIFICATION.md` не требуется)
- **Affects:** `.opencode/rules/state-schema.md` (новый канон),
  [`dispatch-loop.md`](../../.opencode/rules/dispatch-loop.md),
  [`AGENTS.md`](../../AGENTS.md) (карта репозитория),
  `.opencode/agents/analyst.md`, `.opencode/agents/lead.md`,
  [`T-15`](../tasks/T-15-mcp-ready-process/README.md)
- **Tasks:** [T-15](../tasks/T-15-mcp-ready-process/README.md) (см. «Сверка с
  кодом»)

## Контекст

F15 — дрейф `iteration`: в `next_action`/`current_state` номер участка плана
использовался как значение `iteration` при `rework: 0`. Данные прогона
02.10.2026 подтвердили дрейф (`iteration` = номер участка `1→2→3→4` при
`rework=0`; раунды `-r1/-r2/-r3` при `iteration 1`).
[D42](D42-expect-iteration.md) закрепил модель: `iteration` — только
rework-раунд (`rework = iteration − 1`), участок адресуется
`progress_marker`/`resume_hint`. Проектная записка
[`mcp-ready-process.md`](../tasks/T-15-mcp-ready-process/mcp-ready-process.md) §5
содержит схему состояния (поля артефактов, инварианты, каталог операций) как
проект к утверждению. Развилка «место схемы» и «модель `iteration`/участка»
решена владельцем на гейте открытия 02.10.2026 (лента r8). Полный контекст —
[Q83](../questions/Q83.md).

## Решение

1. **Место схемы — отдельный канон** `.opencode/rules/state-schema.md`: поля,
   типы, инварианты, писатель/читатель по каждому артефакту. Ссылка на схему
   добавляется в [`dispatch-loop.md`](../../.opencode/rules/dispatch-loop.md);
   карта репозитория — в [`AGENTS.md`](../../AGENTS.md). Содержание полей
   схемы в решении **не дублируется** — ссылка на файл схемы (Q41).
2. **Модель участка — по [D42](D42-expect-iteration.md).** `iteration` — только
   rework-раунд (`rework = iteration − 1`); адрес участка —
   `progress_marker`/`resume_hint`; новое поле участка (`section`) **не
   вводится**. Дрейф `iteration` ловят схема и `validate-state.mjs` (C2).
3. **Четыре артефакта состояния:** `next_action`, `current_state`, `progress`,
   `receipts`; при расхождении записей источник истины — `receipts`.
4. **Инварианты:** `iteration ≥ 1`; `rework = iteration − 1`; единое значение
   `iteration` в плане, состоянии, `progress` и квитанциях; `session_index` —
   сквозная нумерация сессий прогона; при `action: surface_to_user` обязательны
   `channel` и `owner_response` (дословно, вместе с `channel`).
5. **Расширение схемы** — записью в журнал и обновлением схемы; неизвестные
   поля до обновления — предупреждение, не ошибка (эволюция без жёсткости).

## Следствия

- Класс ошибок F15 закрывается: значение `iteration` едино и ограничено
  rework-циклом, участок явно отделён (`progress_marker`/`resume_hint`).
  Закрытие находки — после приёмки C1, отдельно.
- Схема, писатели/читатели и инварианты собраны в одном каноне; процесс
  ссылается на него, не дублируя формулировки (Q41).
- На схему опираются `validate-state.mjs` (C2), session-commit, метрики и
  последующие MCP-операции над состоянием.
- Правки процесса (`.opencode/rules/**`, `AGENTS.md`) — зона сервисной сессии;
  продуктовый код `src/**` и контракты не затрагиваются.

## Сверка с кодом

Вердикт: ⚪ **не применимо** (процесс/документы) — решение канонизирует схему
состояния агентского процесса; продуктовый код и контракты не меняет.

Что проверено (чтением, 02.10.2026), чем подтверждено:

- **Модель `iteration`/участок:** [D42](D42-expect-iteration.md) — `iteration`
  только rework-раунд, `rework = iteration − 1`, участок —
  `progress_marker`/`resume_hint`, поле участка отклонено в альтернативах D42.
- **Проект схемы:** [`mcp-ready-process.md`](../tasks/T-15-mcp-ready-process/mcp-ready-process.md)
  §5 — поля, инварианты, каталог операций (ссылка, не копия).
- **Фактический формат состояния:** `state/current/*.yaml` и канон
  [`dispatch-loop.md`](../../.opencode/rules/dispatch-loop.md) — чтением;
  отдельного файла схемы нет.
- **Свобода номера:** `docs/questions/Q83.md` / `docs/decisions/D86-*`
  отсутствовали (поиск по дереву); номер сквозной
  (§2 [`journal.md`](../../.opencode/rules/journal.md)).

`cargo` не запускался (§5.3, [D50](D50-dod-by-package-scope.md)): решение
документное, кода не касается; адресный прогон — `validator` на приёмке
(R2).

**Задача —** [`T-15`](../tasks/T-15-mcp-ready-process/README.md): решение
легитимирует схему в рамках программы (фаза C, задача C1); закрытие F15 — после
приёмки C1.

## Альтернативы

- **Поле `section`** (адрес участка в состоянии) — отклонено по
  [D42](D42-expect-iteration.md): участок адресуется
  `progress_marker`/`resume_hint`; новое поле дублирует их и расширяет формат
  без нужды.
- **Раздел в `dispatch-loop.md`** — отклонено в пользу отдельного файла: схема
  — самостоятельный канон, на который ссылаются операции над состоянием;
  размещение разделом перегружает канон цикла.

## Ссылки

- Вопрос: [Q83](../questions/Q83.md)
- Основания: [D42](D42-expect-iteration.md) (модель `iteration`/участок);
  [D78](D78-t15-mcp-ready-program.md) (программа T-15)
- Артефакты: карточка [T-15](../tasks/T-15-mcp-ready-process/README.md);
  записка [`mcp-ready-process.md`](../tasks/T-15-mcp-ready-process/mcp-ready-process.md)
  §5 (проект схемы)
- Находка: F15 в [`findings-registry.md`](../analysis/findings-registry.md)
- Сервисная операция 02.10.2026 (лента r8) — решения владельца
- Связи: [`TRACEABILITY.md`](../TRACEABILITY.md)
