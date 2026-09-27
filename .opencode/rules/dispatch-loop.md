# Правило: dispatch-loop (цикл задач CREDO)

**Когда читать:** `lead` — перед каждым исполнением плана; `analyst` — перед
планированием; роли — при вопросах о маршруте. Решение — D39
(`docs/decisions/D39-loop-dispatcher.md`).

## Контракт loop-а

- `lead` — **loop-диспетчер**: исполняет `next_action.yaml` буквально
  («do not embellish, do not improvise»), решений не принимает.
- `analyst` — **решатель**: одно решение за вызов на свежем срезе (лента,
  память, карточка, `Dn`, фичи) → досье `docs/analysis/<T-XX>-<дата>.md`,
  план `next_action.yaml`, сводка `current_state.yaml`; контекст не копит.
- **Состояние** — `.opencode/state/current/` (вне git, не канон):
  `next_action.yaml` и `current_state.yaml` (пишет `analyst`), `progress.yaml`
  (append, пишет `lead`), `receipts.yaml` (append, пишет `validator`).
  Resume, статус и закрытие — по состоянию.

## Действия очереди

| Действие | Что делает `lead` |
|---|---|
| `dispatch` | вызывает роль с брифом из плана; `expect` — ожидаемый статус отчёта |
| `surface_to_user` | задаёт вопрос владельцу (`question`) и фиксирует ответ |
| `wait_for_user` | останавливается до ответа владельца |
| `complete` | закрывает задачу; короткий итог пользователю |

## Каденция

- План пишется **до точки ветвления** (ветвление: отчёт `validator`,
  несовпадение `expect` с отчётом, scope-вопрос, resume).
- После каждого действия `lead` — запись результата в `progress.yaml`
  (append) и в ленту задачи (append).
- Re-plan (`analyst`) — при исчерпании очереди, несовпадении `expect` и
  всегда при resume.
- Natural checkpoint каждые 6 действий; в headless — запись без паузы.

## Hard rules

- `cargo test` — только `validator` (R2, `AGENTS.md`); полный прогон — один.
- Scope-решения (границы задач, контракты, форматы) — запись Q/D через
  `migrator` **до** исполнения.
- ≥ 3 итерации без прогресса (`progress_marker` не меняется) →
  `surface_to_user`, `status: blocked`.
- Git-пакет (D38 R7): `surface_to_user` несёт точный пакет; подтверждение
  фиксирует `lead` (лента), `git` сверяет его из ленты и состояния.
- Fast path для S: `git` (ветка) → `coder` → `validator` →
  `surface_to_user` (пакет) → `git` → `complete`; `validator` обязателен.
- Приёмка закрывается квитанцией `validator` в `receipts.yaml`; без неё
  `complete` не исполняется.
- Канон агентов: сервисная сессия → аудит `auditor` → приёмка `validator` →
  коммит `git` (не меняется).
