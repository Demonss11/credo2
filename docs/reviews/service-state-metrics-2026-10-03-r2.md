# Приёмка: service-state-metrics — повторная (`-r2`)

**Проверка:** служебная операция r18 (T-15, `C6` — метрики из состояния):
закрытие P2-1 (права на `state-metrics.mjs`) + фактический прогон скрипта
(R2) + адресный `docs_journal` + `features_inventory` + `agents-perms`.
Полный DoD не запускался (`D50` — `src/**`, `tests/**`, `Cargo.toml` не менялись).

**Версия:** ветка `develop`, HEAD `2b467f5` + рабочее дерево (пакет не закоммичен).

**Вердикт:** **принято** (P1/P2/P3 нет).

**P1:** — критичных проблем нет.

**P2:** —

**P3:** —

## Проверки

| Команда | Результат |
|---|---|
| `node .opencode/scripts/state-metrics.mjs` | полный отчёт: 190 записей progress, 24 задачи, 4 сессии; dispatch git 25 / validator 23 / migrator 19 / tester 11 / docs-writer 10 / coder 5 / auditor 4; re-plan 34, convergence 34, категория состояния `expect_mismatch`; очереди 0/0; квитанции accepted 1/2/3 = 53/17/3, accepted_with_notes 1 = 13, rework 1/2 = 17/5; упоры s1=0, s2=0, s3=2, s4=3 |
| `node .opencode/scripts/state-metrics.mjs --json` | валидный JSON: `filter`/`open`/`run` (в т.ч. `limit_hits_by_session`)/`dispatch_by_role`/`replan`/`queues`/`receipts.by_verdict_iteration` |
| `node .opencode/scripts/state-metrics.mjs --task T-16` | progress 40 (1 задача), сессии 3/4; квитанции **3** (accepted 2 = 1, rework 2 = 2) — фильтр по `task` |
| `node .opencode/scripts/state-metrics.mjs --session 3` | progress 29 (1 сессия), упоры s3=2; квитанции **108** (не обнуляются — у них нет `session_index`) — P2-1 закрыт |
| `node .opencode/scripts/state-metrics.mjs --task T-16 --out <tmp>` | файл записан (`[written] …`) |
| `cargo test --test docs_journal` | **14 passed / 0 failed** (в т.ч. `no_addresses_to_removable_or_session_data`, `no_line_number_addresses_in_live_journal`) |
| `cargo test --test features_inventory` | **4 passed / 0 failed** |
| `node .opencode/scripts/agents-perms.mjs` | **agents: 11 из 18**; `validator` и `lead` — оба `allow:node .opencode/scripts/state-metrics.mjs (+ *)` |
| `git diff --numstat -- src tests Cargo.toml` | пусто |
| `git diff --check` | пусто |

## Что проверено и ок

- **P2-1 (права) — закрыт:** право на запуск скрипта есть у двух ролей —
  `.opencode/agents/validator.md:25-26` и `.opencode/agents/lead.md:22-23`;
  канон-список `.opencode/rules/review.md:88-101` синхронизирован
  (validator `:92`, lead `:100`). `agents-perms` подтверждает машинно —
  расхождений «инструкция ↔ права» нет.
- **Фактический прогон скрипта (R2) воспроизведён** всеми заявленными
  режимами; вывод совпадает с отчётом сервисной сессии (190/24/4, dispatch,
  re-plan 34, квитанции 73/21/13 по группам); фильтры корректны
  (`keepProgress` — task+session, `keepReceipt` — только task, `:136-141`).
- **Аудиторские фиксы P2/P3 закрыты по факту:**
  - `--out` — присутствует (`D96:54` = фактический флаг, `--file` отсутствует);
  - категории re-plan из `replan_reason`, иначе «(не указана)» (`:184`),
    категория текущего состояния — отдельной строкой (`:188`);
  - `iteration` в таблице и JSON (`:277`, `:316-320`);
  - упоры лимита по сессиям — маркер `result`/`note` (`:168-171`), столбец
    `:238-242`, JSON `limit_hits_by_session`;
  - мёртвая `verdicts` удалена (`rg "verdicts"` по файлу — 0 совпадений);
  - `indexOf(raw)` → индексный цикл (`:59-95`).
- **D96 ↔ факт:** состав секций (п.3 `:44-50`) совпадает с выводом; источник —
  только состояние (п.4); `--task/--session/--out/--dir` (п.5); канон — ссылка
  без дублирования таблиц (п.6).
- **Журнал:** Q93↔D96 парны (`questions/README.md:115`, `decisions/README.md:125`);
  `TRACEABILITY.md:97` — Q93/D96 `in work`, T-15 🚧; «Сверка с кодом» D96:79-109 —
  ⚪ не применимо (процесс/служебная зона), обосновано; задача-носитель `C6`
  создана (отдельная не требуется).
- **Канон-ссылки:** `dispatch-loop.md:139` («метрики — скриптом», без таблиц) и
  `AGENTS.md:26` (карта скриптов) — без дублей.
- **Карточка/фича:** T-15 `C6` 🚧 (`README.md:222`); фича
  `agents-metrics.feature:2` — `# D96 (Q93)`, `docs/features/README.md:301` — 🟡.
  Диффы минимальны (1 строка на файл).
- **Границы:** изменены ровно объявленные файлы — `.opencode/scripts/state-metrics.mjs`
  (новый), `.opencode/rules/{dispatch-loop,review}.md`,
  `.opencode/agents/{validator,lead}.md`, `AGENTS.md`, Q93/D96/каталоги/TRACEABILITY,
  карточка T-15, `docs/features/**`, лента/память; `src/**`, `tests/**`,
  `Cargo.toml` не тронуты.

## Наблюдения (не находки)

- **Хвост данных:** `replan_reason` в `progress.yaml` не заполняется (0
  совпадений), поэтому все 34 re-plan попадают в категорию «(не указана)», а
  метрика конвергенции равна `total`. Это дрейф **данных** (`lead` не пишет
  условно-обязательное поле, `state-schema.md`), не дефект скрипта; поведение
  допущено P2-2. Не блокирует `C6` — отдельное решение (зеркало/джойн из
  `next_action.re_raise`). Зафиксировано пометкой.
