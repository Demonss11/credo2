# Приёмка: сервисная операция r18 (T-15, C6 — метрики из состояния)

- **Тип:** служебная операция (процесс/служебная зона, D50)
- **Предмет:** T-15, `C6` — метрики/очереди из состояния; Q93 → D96;
  скрипт `.opencode/scripts/state-metrics.mjs`
- **Дата:** 2026-10-03
- **Версия:** `develop`, HEAD `2b467f5` + рабочее дерево (пакет не закоммичен)

## Вердикт

**Отклонено (rework) — P2 ×1.** Скрипт и канон-ссылки по существу верны, все
ранее отмеченные аудитом P2/P3 закрыты; блокирует **невыполнимость заявленной
R2-проверки** (прогон скрипта) из-за отсутствия права у ролей.

## P2

**P2-1. Права ролей не расширены под новый канон-механизм метрик → прогон
`state-metrics.mjs` невозможен ни одной ролью.**

- Факт: `node .opencode/scripts/state-metrics.mjs` (в т.ч. `--json`,
  `--task T-16`, `--session 3`) → `Permission denied: shell` (shell
  deny-by-default). Проверено тремя формами.
- `docs/decisions/D96-state-metrics.md:108` — «прогон самого
  `state-metrics.mjs` — адресно на приёмке» (т.е. за `validator`).
- `.opencode/rules/dispatch-loop.md:138-141` — «**Метрики и очереди**
  выводятся **из состояния** скриптом `node .opencode/scripts/state-metrics.mjs`».
- `.opencode/agents/validator.md:17-34` — allowlist: `agents-perms.mjs`,
  `validate-state.mjs`; `state-metrics.mjs` **отсутствует**.
- `.opencode/rules/review.md:88-92` — список команд `validator` без
  `state-metrics.mjs`.
- `node .opencode/scripts/agents-perms.mjs` — `lead` также без права
  (только `validate-state.mjs`); роль-потребитель канона запустить скрипт не
  может.
- Последствие: R2-проверка C6 (прогон скрипта), назначенная решением и
  заданием на приёмку, **невыполнима** ни `validator`, ни `lead`;
  `auditor` отказался ранее по той же причине и делегировал прогон `validator`
  (лента r18). До расширения прав канон обещает механизм, который роли не в
  силах исполнить.
- Правка: добавить `node .opencode/scripts/state-metrics.mjs` (+ `*`) в
  фронтматтер `validator` (и, по решению владельца, `lead` — как потребителя
  метрик) и синхронно в `.opencode/rules/review.md` §«Доступные команды»;
  изменение прав/канона — класс L, свежий аудит `auditor` до коммита.

## P3

Нет (по существу находок стиля, требующих правки, не осталось).

## Хвост данных (аудит; не дефект скрипта, приёмку C6 не блокирует)

`replan_reason` в `.opencode/state/current/progress.yaml` — **0 совпадений**
(подтверждено `rg`), `owner_override` — **0 совпадений**. Поэтому все re-plan
(скрипт `:184`) попадают в категорию «(не указана)», а `convergence`
(`:186`) = `replanTotal`. Это дрейф **данных** (`lead` не заполняет
опциональное поле, `state-schema.md`), не дефект скрипта. Кандидат на отдельное
решение: зеркало `replan_reason` в записи `lead`/`analyst` либо джойн из
`next_action.re_raise`. Зафиксировано отдельно, не блокирует C6.

## Что проверено и ок

- **D96 ↔ факт:** формат CLI+`--json` (D96:41-43 ↔ скрипт `:31,221-327`);
  секции D96 п.3 (`:44-50`) ↔ скрипт (прогон `:232-242`, dispatch `:244-248`,
  re-plan/конвергенция `:251-260`, очереди `:263-269`, квитанции `:272-285`);
  источник — только состояние (`:130-134`); `--out`/`--dir` (D96:54-55 ↔
  `:28,32,323-325`); «канон — ссылка, не копия» (D96:57-60 ↔
  `dispatch-loop.md:138-141`, `AGENTS.md:26` — без таблиц).
- **Секции/поля:** dispatch по ролям (`:161,244-247`); шаги по сессиям
  (`:162-166,240-241`); упоры лимита — маркер `result`/`note` (`:167-172,241`);
  категория состояния отдельной строкой (`:188,255`); `iteration` в квитанциях
  (таблица `:277,282`, JSON `:316-318`).
- **P2-1 (receipts-фильтр):** закрыт — `keepProgress` (task+session) и
  `keepReceipt` (только task) разделены (`:136-141`); в схеме `receipts`
  `session_index` нет (`state-schema.md`), фильтр по сессии к ним неприменим.
- **P2-2 (категория):** закрыт — читается из `replan_reason`, иначе
  «(не указана)» (`:184`); `stateCategory` — отдельно (`:188`).
- **P2-3 (`iteration`):** закрыт (см. выше).
- **P2-4 (`--out`):** закрыт — D96:54 называет `--out`; `--file` в скрипте
  отсутствует (`rg` — 0).
- **P2-5 (упоры по сессиям):** закрыт — столбец «упоров лимита» (`:238-241`) +
  JSON `limit_hits_by_session` (`:300`); фича `:12` согласована.
- **P3 `indexOf(raw)`:** закрыт — индексный цикл `for (let i…)` (`:59-95`);
  `verdicts` удалена (в файле отсутствует).
- **Журнал/каталоги/связи:** Q93↔D96 парны (`Q93:3 resolved by D96`,
  `D96:5 Resolves Q93`); `questions/README.md:115`, `decisions/README.md:125`
  (D96, Q93, accepted, 2026-10-03); `TRACEABILITY.md:97` Q93/D96 `in work`,
  T-15 🚧; карточка `T-15:222` `C6` 🚧 (D96); фича `agents-metrics.feature:2`
  `# D96 (Q93)`; `features/README.md:301` 🟡 (3 сценария, счётчик не тронут);
  номера строк в каноне отсутствуют (`docs_journal` 14/0).
- **`metrics-report.mjs` не смешан:** в `state-metrics.mjs` — только
  комментарий-разграничение (`:5`); нативные метрики вынесены (`D96:51-53`).
- **Границы:** `git diff --numstat -- src tests Cargo.toml` пусто;
  `git diff --check` пусто; `git status --short` — ровно объявленный состав
  (11 M + 4 ??), включая новый `state-metrics.mjs`, Q93/D96, ленту r18.
- **Ссылки:** относительные ссылки Q93/D96/фичи резолвятся
  (`../decisions/`, `../questions/`, `../features/`, `../tasks/`,
  `../../.opencode/scripts/`).
- **Инструкция ↔ права:** `agents-perms.mjs` — «11 из 18»; расхождений состава
  команд `review.md`/фронтматтеров, кроме самой находки P2-1, нет.

## Проверки (команды → результат)

| Команда | Результат |
|---|---|
| `node .opencode/scripts/state-metrics.mjs` | **permission denied** (права роли; P2-1) |
| `node .opencode/scripts/state-metrics.mjs --json` | **permission denied** (P2-1) |
| `node .opencode/scripts/state-metrics.mjs --task T-16` | **permission denied** (P2-1) |
| `node .opencode/scripts/state-metrics.mjs --session 3` | **permission denied** (P2-1) |
| `cargo test --test docs_journal` | **14 passed / 0 failed** |
| `cargo test --test features_inventory` | **4 passed / 0 failed** |
| `node .opencode/scripts/agents-perms.mjs` | ok — `agents: 11 из 18` |
| `git diff --numstat -- src tests Cargo.toml` | пусто |
| `git diff --check` | пусто |

**Полный DoD не запускался** (D50 — процесс/служебная зона; `src/**`,
`tests/**`, `Cargo.toml` не тронуты).

**Технические проблемы:** прямой прогон `state-metrics.mjs` отказан
permission-гардом (доступные команды роли — `review.md:88-92`). Одиночные формы
испробованы; замена — чтение скрипта + сверка D96. Осталось непроверенным:
фактический вывод секций/JSON/фильтров на живых данных — не воспроизведён
`validator` (заявлен сервисной сессией и аудитом чтением). Это и есть
предмет P2-1.

## Рекомендация

Вернуть на доработку: расширить права (`validator`, при решении — `lead`) на
`node .opencode/scripts/state-metrics.mjs` + синхронизировать `review.md`;
пройти свежий аудит (L) и повторить адресную приёмку (`-r2`) с фактическим
прогоном скрипта. Хвост `replan_reason` — отдельным решением после C6.
