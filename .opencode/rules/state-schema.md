# Правило: state-schema (схема состояния процесса)

**Когда читать:** `analyst` — перед записью `next_action.yaml`/`current_state.yaml`
(разделы этих артефактов + §«Инварианты»); `validator` — при приёмке
состояния; `auditor` — при аудите, затрагивающем схему. Остальные роли файл
не читают: `lead` пишет `progress` по краткой форме в своём теле, `git`
работает с пакетом по подтверждению.

**Основание:** `D86` (Q83) — схема как канон; `D42` — модель
`iteration`/участка; `D78` — программа T-15. Проект — записка
[`mcp-ready-process.md`](../../docs/tasks/T-15-mcp-ready-process/mcp-ready-process.md)
§5 (ссылка, не копия).

## Назначение и границы

- Схема описывает **storage-слой** процесса: состав артефактов состояния,
  поля, типы, инварианты, писателя/читателя. Решений процесса (что делать)
  схема не принимает.
- Артефакты — `.opencode/state/current/`: `next_action.yaml`,
  `current_state.yaml`, `progress.yaml`, `receipts.yaml`.
- При расхождении записей источник истины — `receipts.yaml`
  (`dispatch-loop.md` §«Состояние и записи»).
- Неизвестные поля — **предупреждение**, не ошибка; расширение — записью в
  журнал и обновлением этой схемы (`D86`).
- Схема — контракт для `validate-state.mjs` (C2) и будущих MCP-операций
  storage/validation/query.

## Артефакты и роли

| Артефакт | Режим | Писатель | Читатели |
|---|---|---|---|
| `next_action.yaml` | перезапись (план) | `analyst` | `lead`, роли, `validator`, `git`, `auditor` |
| `current_state.yaml` | снимок на re-plan | `analyst` | `lead`, роли, `validator`, `git`, `auditor` |
| `progress.yaml` | append | `lead` | все роли |
| `receipts.yaml` | append | `validator` | все роли |

Типы: `string`, `int`, `bool`, `date` (`YYYY-MM-DD`), `list<T>`, `map`,
`enum(...)`. Пути — относительно корня репозитория. Строки — скаляры YAML
(многострочные — `>`/`|`), при необходимости в кавычках.

## `next_action.yaml` — план

Обязательные поля:

| Поле | Тип | Значения / инвариант |
|---|---|---|
| `task` | string | ID задачи (`T-XX`) или сервисной операции |
| `iteration` | int | ≥ 1; только rework-раунд (`D42`) |
| `status` | enum | `in_progress` · `awaiting_user` (гейт) · `blocked` · `idle` (активной работы нет) |
| `progress_marker` | string | маркер участка (slug); участок адресуется им, не `iteration` |
| `as_of` | date | дата плана |
| `next` | list<map> | непустая очередь действий |
| `resume_hint` | string | краткая сводка для resume/восстановления |

`next[]` (действие): `kind` — enum `dispatch` · `surface_to_user` ·
`wait_for_user` · `complete` (обязательно); `role` — string (обязательно при
`dispatch`); `brief`, `expect`, `reason` — string; `package` — string.

Опционально: `task_kind` (enum `task` · `service`), `class` (enum `S` · `M` ·
`L`), `class_dispute` (bool), `class_note`, `branch` (map: `name`, `base`,
`base_head`, `note`), `scope_constraints` (list<string>), `route_exclusions`
(list<string>), `package` (map: `mode`, `note`, `add_paths`), `re_raise` (map:
`id`, `category`, `origin`, `failed_clause`, `fix`, `blocking`, `resolved`),
`inherited_boundaries` (list<string>).

## `current_state.yaml` — снимок

Обязательные поля:

| Поле | Тип | Значения / инвариант |
|---|---|---|
| `as_of` | date | момент снимка (re-plan) |
| `task` | string | ID задачи/операции |
| `iteration` | int | ≥ 1; совпадает с `next_action` |
| `rework` | int | = `iteration − 1` (`D42`) |
| `session_index` | int | ≥ 1; номер текущей сессии `lead` |
| `phase` | enum | `planning` · `implementation` · `verification` · `closing` · `done` |
| `acceptance` | enum | `pending` · `rework` · `accepted` · `accepted_with_notes` |

Опционально: `operation`, `kind`, `task_note`, `class`, `class_dispute`,
`status`, `progress_marker`, `acceptance_reports`, `artifacts` (map),
`route_pending` (list<string>), `route_exclusions` (list<string>), `package`
(map), `base` (map: `develop`, `master`), `branch` (map: `name`, `base`,
`base_head`, `published`, `note`), `gate_pending`, `worktree_remainder`,
`snapshot_check`, `constraints` (list<string>), `resume` (map),
`mixed_worktree` (bool), `inherited_boundaries` (list<string>).

## `progress.yaml` — запись (append)

Обязательные поля записи:

| Поле | Тип | Значения / инвариант |
|---|---|---|
| `at` | date | дата действия |
| `task` | string | ID задачи/операции |
| `iteration` | int | ≥ 1; значение раунда |
| `session_index` | int | ≥ 1; номер сессии, записавшей действие |
| `action` | enum | `dispatch` · `surface_to_user` · `wait_for_user` · `re-plan` · `complete` |
| `result` | string | краткий результат действия |
| `next` | string | следующее действие / итог |

Условно обязательные: `role` (string) — при `dispatch` и `re-plan`;
`expect_match` (enum `true` · `partial` · `false`) — при `dispatch`;
`channel` (enum `question` · `text`) и `owner_response` (string, дословно) —
при `surface_to_user`; `question` (string) — при `surface_to_user`.

Опционально: `package`, `pre_gate`, `post_package`, `replan_reason`, `round`,
`resume` (bool), `note`.

## `receipts.yaml` — квитанция приёмки (append)

| Поле | Тип | Обяз. | Значения / инвариант |
|---|---|---|---|
| `task` | string | да | ID задачи/операции |
| `iteration` | int | да | ≥ 1; раунд приёмки; `-rN` номера отчёта = N |
| `verdict` | enum | да | `accepted` · `accepted_with_notes` · `rework` |
| `report` | string | да | путь отчёта `docs/reviews/…` |
| `at` | date | да | дата приёмки |
| `dod` | map | нет | результаты команд DoD (по составу пакета, `D50`) |
| `snapshot` | string | нет | версия, на которой выполнена приёмка |

## Инварианты

1. `iteration ≥ 1`; `rework = iteration − 1`; одно значение `iteration` в
   плане, состоянии, `progress` и квитанциях текущего раунда (`D42`).
2. Участок адресуется `progress_marker`/`resume_hint`; `iteration` номер
   участка не несёт (`D42`; F15).
3. `progress` и `receipts` — только append: записи не переписываются и не
   удаляются; исправление — новой записью; восстановленная запись помечается
   `note`.
4. Один писатель на файл (таблица артефактов); `result`/`next` не пустые.
5. При `action: surface_to_user` запись содержит `channel` и `owner_response`
   дословно; `channel: question` — норма, `text` — отклонение от нормы
   (`dispatch-loop.md` §«Владелец: канал и фиксация»).
6. `expect_match` — только из перечисленных значений; сверяется с `expect`
   плана.
7. Даты — `YYYY-MM-DD`; значения enum — из списков выше; иное —
   предупреждение.

## `session_index`

- Сквозной номер сессии `lead` в рамках прогона (цикл работ по задаче):
  начинается с 1, растёт на каждый новый запуск сессии; не переиспользуется.
- Resume той же сессии (`sessionID`) номер не меняет.
- Заполняет: `lead` — в каждой записи `progress` своей сессии; `analyst` —
  в `current_state.session_index` (номер текущей сессии) на re-plan.
- Проверка: значения в `progress` не убывают; `current_state.session_index` =
  номер последней записи `progress` (если состояние актуально).
- Назначение: единый адрес сессии для записей, метрик (C6) и атрибуции
  (F60); текстовые «сессия №X» — вне схемы.

## `owner_response`

- Фиксируется только при `surface_to_user` — дословная цитата ответа
  владельца (выбор варианта — точная формулировка варианта; сужение/отмена —
  точная формулировка).
- `result` — краткое следствие для цикла (что делаем дальше); цитату не
  заменяет.
- Пересказ вместо цитаты — нарушение (`dispatch-loop.md` §«Владелец: канал и
  фиксация»).

## Расширение и применимость

- Новое поле — сначала запись в журнал и обновление этой схемы, затем
  использование; до обновления — предупреждение валидатора.
- Актуальные `next_action`/`current_state` и новые записи проверяются строго;
  записи `progress`/`receipts`, созданные до принятия схемы, — мягко
  (недостающие поля — предупреждения).

## Связи

- Модель `iteration`/участка — `D42`; программа и фазы — `D78`; схема как
  канон — `D86` (Q83).
- Нормы цикла (действия, каденция, гейты) — `dispatch-loop.md`; журнал —
  `journal.md`; приёмка — `review.md`; git-пакет — `git-workflow.md`.
- Валидатор схемы — `validate-state.mjs` (C2, карточка T-15); каталог
  операций (≤ 10) — записка §5.4, канонизация — отдельными задачами фазы C.
