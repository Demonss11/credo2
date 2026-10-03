# D91: C2 — `validate-state.mjs` (валидатор схемы состояния)

- **Статус:** accepted
- **Дата:** 2026-10-03
- **Resolves:** [Q88](../questions/Q88.md)
- **Спека:** — (SPEC §10 — указатель [D70](D70-spec-reduction.md); запись в
  `SPECIFICATION.md` не требуется)
- **Affects:** `.opencode/scripts/validate-state.mjs` (новый скрипт),
  [`state-schema.md`](../../.opencode/rules/state-schema.md),
  [`dispatch-loop.md`](../../.opencode/rules/dispatch-loop.md),
  `.opencode/agents/validator.md`, [`review.md`](../../.opencode/rules/review.md),
  [`AGENTS.md`](../../AGENTS.md)
- **Tasks:** [T-15](../tasks/T-15-mcp-ready-process/README.md) (фаза C, строка
  реестра `C2`)

## Контекст

Схема состояния процесса канонизирована отдельным файлом
(`.opencode/rules/state-schema.md`, [D86](D86-state-schema.md)): четыре артефакта
(`next_action`, `current_state`, `progress`, `receipts`), поля, типы, enum'ы и
семь инвариантов. Однако проверка схемы остаётся ручной: дрейф `iteration`
(F15) и несоответствия записей (`owner_response` без `channel`, убывающий
`session_index`, чужой enum) ловятся только вниманием приёмки. Записка
[`mcp-ready-process.md`](../tasks/T-15-mcp-ready-process/mcp-ready-process.md)
§5 и карточка [T-15](../tasks/T-15-mcp-ready-process/README.md) (фаза C, строка
`C2`) предусматривают скрипт `validate-state.mjs` — «pre-flight + приёмка
`validator`; неизвестные поля — предупреждение».

Владелец на открытии сервисной операции 03.10.2026 (лента r13) выбрал
вариант (A). Полный контекст — [Q88](../questions/Q88.md). Правки процесса
(`.opencode/**`) — зона сервисной сессии; решение лишь канонизирует их состав.

## Решение

1. **Место и роль.** `validate-state.mjs` — скрипт служебной зоны
   (`.opencode/scripts/**`, вне канона), читает состояние и проверяет его по
   контракту `.opencode/rules/state-schema.md` (`D86`). Второй источник схемы
   (JSON-схема) не вводится: единственный контракт — `state-schema.md` (Q41).
2. **Точки применения:** `pre-flight` перед прогоном и приёмка `validator`
   (адресный прогон); в `review.md` скрипт добавляется в доступные команды
   `validator`, в `validator.md` — в чек-лист приёмки состояния.
3. **Проверяемый контракт.** Артефакты `next_action.yaml`, `current_state.yaml`,
   `progress.yaml`, `receipts.yaml`: обязательные поля; enum'ы (`status`,
   `phase`, `acceptance`, `action`, `expect_match`, `verdict`, `replan_reason`,
   `kind`); формат дат (`YYYY-MM-DD`); непустые `result`/`next` у записей
   `progress`; условно обязательные (`role`, `expect_match` — при `dispatch`;
   `channel`, `owner_response`, `question` — при `surface_to_user`).
4. **Инварианты:** `iteration ≥ 1`; `rework = iteration − 1`; единое значение
   `iteration` в плане, состоянии, `progress` и `receipts` текущего раунда;
   `session_index` не убывает в `progress`; при `action: surface_to_user`
   обязательны `channel` и `owner_response` (дословно); пустые `result`/`next`
   запрещены.
5. **Строгость:** записи `progress`/`receipts`, созданные до начала машинной
   проверки (порог `--since`, по умолчанию 03.10.2026 — дата C2), проверяются
   **мягко** (недостающие поля — предупреждение, `state-schema.md` §«Расширение
   и применимость»); записи с порога и актуальные `next_action`/`current_state` —
   строго. Неизвестные поля — **предупреждение**, не ошибка (эволюция без
   жёсткости).
6. **Формат запуска:** `node .opencode/scripts/validate-state.mjs [--dir <path>]
   [--file <artifact>] [--json] [--strict] [--since YYYY-MM-DD]`; коды выхода:
   `0` — ошибок нет, `1` — есть ошибки схемы, `2` — ошибка запуска/чтения.

## Следствия

- Дрейф `iteration`, несоответствия enum'ов и записей ловятся машинно в
  `pre-flight` и на приёмке; схема (`D86`) становится исполняемым контрактом.
- Второй источник схемы не появляется: правка `state-schema.md` — единственное
  место изменения контракта.
- Скрипт используется адресно (`--file`), вписывается в бюджет шагов приёмки;
  продуктовый код `src/**` и контракты CREDO не затрагиваются.
- Правки процесса (`.opencode/**`) — зона сервисной сессии; правки фич —
  `docs-writer`.

## Сверка с кодом

Вердикт: ⚪ **не применимо** (процесс/документы) — решение вводит валидатор
канона агентского процесса; продуктовый код CREDO и контракты не меняет.

Что проверено (чтением, 03.10.2026), чем подтверждено:

- **Свобода номеров:** `docs/questions/Q88.md` (создан этим пакетом) и
  `docs/decisions/D91-*`; последние занятые — Q87/D90
  ([`questions/README.md`](../questions/README.md),
  [`decisions/README.md`](../decisions/README.md)); нумерация сквозная (§2
  [`journal.md`](../../.opencode/rules/journal.md)).
- **Основания:** F15 в
  [`findings-registry.md`](../analysis/findings-registry.md); схема —
  `.opencode/rules/state-schema.md`; записка §5 — чтением.
- **Периметр правок:** `.opencode/scripts/validate-state.mjs` (служебная зона),
  `.opencode/rules/{state-schema,dispatch-loop}.md`,
  `.opencode/agents/validator.md`, [`review.md`](../../.opencode/rules/review.md),
  [`AGENTS.md`](../../AGENTS.md) — зона сервисной сессии; правки вносятся
  отдельно.

`cargo` не запускался (§5.3, [D50](D50-dod-by-package-scope.md)): решение
процессное, кода CREDO не касается; адресный прогон — `validator` на приёмке
(R2).

**Задача —** [T-15](../tasks/T-15-mcp-ready-process/README.md): решение
легитимирует валидатор схемы в рамках программы (фаза C, строка реестра `C2`).

## Альтернативы

- **JSON-схема отдельным файлом** — отклонено: второй источник рядом с
  `state-schema.md` рискует разойтись с каноном (Q41); схема — уже канон.
- **Только инварианты, без структуры** — отклонено: опечатки в именах полей и
  чужие значения enum остаются незамеченными; ценность валидатора падает.
- **Проверка всего строго без «мягких» записей** — отклонено: исторические
  записи `progress`/`receipts` (до принятия схемы) дали бы ложные ошибки;
  `state-schema.md` предписывает для них мягкий режим.

## Ссылки

- Вопрос: [Q88](../questions/Q88.md)
- Основания: схема — [D86](D86-state-schema.md) и
  [`state-schema.md`](../../.opencode/rules/state-schema.md); программа —
  [D78](D78-t15-mcp-ready-program.md); записка
  [`mcp-ready-process.md`](../tasks/T-15-mcp-ready-process/mcp-ready-process.md)
  §5
- Смежное: [D89](D89-branch-topology-freeze-session-commit.md) (session-commit),
  [D90](D90-c5-c7-re-raise-selfreport.md) (re-raise, самоотчёт)
- Артефакты: карточка [T-15](../tasks/T-15-mcp-ready-process/README.md)
  (фаза C, строка реестра `C2`)
- Сервисная операция 03.10.2026 — решение владельца (лента r13)
- Связи: [`TRACEABILITY.md`](../TRACEABILITY.md)
