# Досье: process-mining инструмент `pm` агентского процесса CREDO2

- **Дата:** 2026-09-30 · **Операция:** сервисная волна `service-pm-tool` (не T-XX)
- **Источник:** черновик владельца `.opencode/scripts/pm/mine_agents.py` (557 стр., `??` — вне git);
  предложение команды по uv-проекту; прецеденты [D75](../../docs/decisions/D75-git-lean-workflow.md)
  (хелпер+allowlist) и [D51](../../docs/decisions/D51-agent-tools-token-hygiene.md).
- **Класс:** сервисная операция, объём L-подобный (новая подсистема + правка канона `AGENTS.md`)
  → маршрут служебной волны: `migrator`(Q77→D81) → сервисная сессия (код) → `auditor` → `validator` → `git`.
  Роли/T-XX не затронуты; `state/current/*` не трогаем.

## Данные (факты среза)
- `progress.yaml` — append-only: `at` (**дата без времени!**), `task`, `iteration`, `action` ∈
  {dispatch, re-plan, surface_to_user, complete}, `role` (у dispatch/re-plan), `result`,
  `expect_match` (bool | строка «partial…»), `next`, иногда `package`/`round`/`resume`/`question`/`pre_gate`.
- `receipts.yaml` — `task`, `iteration`, `verdict` ∈ {accepted, **accepted_with_notes**, rework},
  `report`, `dod{fmt,clippy,test,features_inventory,…}`, `snapshot`, `at`.
- `next_action.yaml` — `task`, `iteration`, `status`, `class`, `acceptance`, `resume_hint`;
  очередь — ключ **`next`** (список `{kind, role, brief, expect, reason, package}`).
- Почта `.opencode/mail/`: на срез только `service-*.md` (36), `T-*.md` нет; события — заголовки
  `## <роль> · <дата> · <статус>` (свободный markdown).

## Варианты
- **(а) Python uv-проект** `.opencode/scripts/pm` — **принят**: pandas/networkx/matplotlib/plotly
  дают DFG, варианты и графы; `pm4py` — отдельный extra.
- **(б) Расширить `session-analysis` (Node)** — отвергнут: другой домен (транскрипты сессий, не
  state-лог), дублирование, Node-стек беден для DFG/вариантов. Переиспользуем его конвенции
  (README, «кто запускает», артефакты вне репо).

## Решение
1. **Проект:** uv; `pyproject.toml` + `uv.lock` (в git), пакет `src/pm_agents/`, CLI `pm-agents`,
   `tests/` (pytest), `README.md`, локальный `.gitignore` (`.venv/`, `__pycache__/`, `*.egg-info`).
2. **Зависимости слоями:** core = `pyyaml` (+stdlib csv/json) → метрики/CSV/JSON/markdown;
   extra `viz` = `networkx`+`matplotlib`+`plotly`(, `pandas`) → PNG/HTML; extra `pm4py` —
   опциональный. `seaborn` убрать.
3. **Источники:** `progress`+`receipts`+`next_action` (обязательные) + `mail` (enrichment,
   tolerant-парсер). Кейс = `task`; неизвестное → `unknown-<idx>` (не подставлять текущую задачу).
4. **Модель событий:** активность = `role` для dispatch/re-plan, иначе `action`; время =
   (`at`-дата, глобальный порядковый номер в секундах); дельты — по порядку, не по «секундам».
5. **Выходы:** DFG (все/популярные), варианты, heatmap, Sankey HTML, частоты, `metrics.json`,
   `dfg_edges.csv`, `variants.csv`, `summary.md`. Артефакты — **вне репозитория**
   (`%TEMP%\opencode\pm-<дата>\`, как session-analysis); в git — только итоговый отчёт `docs/analysis/`.
6. **Запуск:** владелец/сервисная сессия (`uv run`). Права ролей **не меняются** (у ролей нет
   `uv`/`node`); роли только читают готовые отчёты.
7. **Канон:** строка про `pm/` — в карту `AGENTS.md` §`.opencode/scripts/`; протокол:
   журнал (Q77/D81) → правки → `auditor` → `validator` → `git`.

## Улучшения черновика (баги/несоответствия)
1. `NON_AGENT_ACTIONS` объявлен, но не используется — применить (маппинг служебных действий) или удалить.
2. `next_action`: очередь в ключе **`next`**, а не `queue`/`actions` → `queue_length` всегда пуст. Парсить
   `next`; вывести `class`/`status`/`acceptance`.
3. `receipts`: `accepted_with_notes` не учтён → `acceptance_rate` занижен. Приёмки = {accepted,
   accepted_with_notes}, отдельно `with_notes`/`rework`; использовать `dod`/`snapshot`.
4. `mail` не читается вовсе (реальный event log вызовов ролей, кейс — лента) — добавить, tolerant.
5. `at` без времени → `avg_delta_seconds`/`bottlenecks` недостоверны (внутри дня ≈0).
6. `case_id`: fallback на `next_action.task` сваливает неизвестные в текущую задачу → `unknown-<idx>`.
7. `_as_list` для словаря без контейнера возвращает `[data]` — маскирует структурную ошибку.
8. `expect_match` (bool|"partial…") не агрегируется — добавить счётчик несовпадений (качество цикла).
9. Тяжёлые импорты на уровне модуля → CLI падает без viz-зависимостей: lazy-импорты + `--no-viz`.
10. PM4Py API: новые версии ждут `case_id_key`/`activity_key`/`timestamp_key` — проверить/обернуть.
11. Heatmap на `seaborn` → заменить на `matplotlib` (убрать зависимость).
12. `to_csv` без BOM ломает Excel-RU → `encoding="utf-8-sig"`.
13. `summary.md` не формируется (только stdout) — добавить markdown-сводку.
14. `variants`: `groupby(...).apply(tuple)` без `sort=False` может терять порядок — сортировать явно.
15. Нет фильтров CLI (`--exclude-service`, `--since`, `--format`) — добавить.

## Риски
- Python 3.14: колёса pandas/matplotlib/pm4py могут отсутствовать → слоение extras (core без viz).
- Бриткость парсинга `mail` (свободный markdown) → tolerant + `--lenient`, mail опционален.
- `at` без времени → запрет выводов о реальных задержках.
- Инструмент внешний (`uv`): цепочка постановки/аудита не расширяет права ролей.

## Открытые пункты
- Интеграция `pm` как MCP-инструмента (связь с T-15) — отдельной задачей.
- Дефолт вывода: temp (предложен) vs `pm/output/` с локальным `.gitignore`.
- Порог «популярных» переходов; трактовка `accepted_with_notes` в метриках приёмки.

`inherited_boundaries: [D75, D51]`
`route_exclusions: []` (маршрут служебной волны без исключений: аудит и приёмка обязательны — правка канона)
