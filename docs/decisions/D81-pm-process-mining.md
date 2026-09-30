# D81: Инструмент `pm` — uv-проект process mining агентского процесса

- **Статус:** accepted
- **Дата:** 2026-09-30
- **Resolves:** [Q77](../questions/Q77.md)
- **Спека:** —
- **Affects:** `.opencode/scripts/pm/**` (новый uv-проект; служебная зона),
  [`AGENTS.md`](../../AGENTS.md) (карта `.opencode/scripts/` — вносит сервисная
  сессия), [`pm-tool-design-2026-09-30.md`](../analysis/pm-tool-design-2026-09-30.md)
  (досье решения)
- **Tasks:** — (служебная зона; вносит сервисная сессия; см. «Сверка с кодом»)

## Контекст

Черновик владельца `.opencode/scripts/pm/mine_agents.py` (557 строк, вне git)
читает `state/current/*`, но не читает `mail/` (реальный event log вызовов ролей),
путает очередь `next_action` (ключ `next`), не учитывает `accepted_with_notes`;
зависимости (pandas/networkx/matplotlib/plotly/pm4py) под Python 3.14 — риск колёс;
у ролей нет прав `uv` — инструмент служебной зоны. Полный контекст, варианты и
список улучшений — досье [`pm-tool-design-2026-09-30.md`](../analysis/pm-tool-design-2026-09-30.md)
и вопрос [Q77](../questions/Q77.md).

## Решение

1. **Проект:** uv-проект `.opencode/scripts/pm`: `pyproject.toml` + `uv.lock`
   (в git), пакет `src/pm_agents/`, CLI `pm-agents`, `tests/` (pytest),
   `README.md`, локальный `.gitignore` (`.venv/`, `__pycache__/`, `*.egg-info`).
2. **Зависимости слоями:** core — `pyyaml` (+stdlib csv/json) → метрики/CSV/JSON/
   markdown; extra `viz` — `networkx` + `matplotlib` + `plotly` → PNG/HTML; extra
   `pm4py` — опциональный. `seaborn` не используется.
3. **Источники:** `progress` + `receipts` + `next_action` (обязательные) + `mail/`
   (tolerant-enrichment). Кейс = `task`/имя ленты; неизвестные → `unknown-<idx>`
   (не подставлять текущую задачу); активность = `role` (dispatch/re-plan) либо
   `action`; порядок = (`at`-дата, глобальный счётчик); выводы о секундных
   задержках не делаются.
4. **Метрики и артефакты:** DFG (все/популярные), варианты, heatmap, Sankey HTML,
   частоты, `metrics.json`, `dfg_edges.csv`, `variants.csv`, `summary.md`; приёмка
   учитывает `accepted_with_notes`; несовпадения `expect_match` — счётчик.
5. **Место:** запуск — владелец/сервисная сессия (`uv run`); артефакты — вне
   репозитория (`%TEMP%\opencode\pm-<дата>\`, как `session-analysis`); в git —
   только итоговый отчёт `docs/analysis/`.
6. **Права ролей не меняются** (у ролей нет `uv`/`node`; роли читают готовые
   отчёты).
7. **Строка `pm/` — в карту `AGENTS.md`** (`.opencode/scripts/`), вносит сервисная
   сессия; протокол волны: `migrator` (Q77/D81) → сервисная сессия (код) →
   `auditor` → `validator` → `git`. Улучшения черновика — по списку досье
   [`pm-tool-design-2026-09-30.md`](../analysis/pm-tool-design-2026-09-30.md),
   выполняются этой волной.

## Следствия

- Черновик `mine_agents.py` заменяется проектом `pm/`; дефекты (see досье,
  пп. 1–15: `next`-очередь, `accepted_with_notes`, `unknown-<idx>`, `at` без
  времени, lazy-импорты/`--no-viz`, BOM, `summary.md`, `variants`-порядок,
  `--source`/`--since`/`--exclude-service`) устраняются волной.
- Инструмент процесса отделён от продукта (`src/**`, `tests/**`, `Cargo.toml` не
  затронуты) и от канона ролей (права не расширяются).
- Артефакты анализа не засоряют репозиторий (вне git); в git — сводный отчёт.
- Открытый пункт: MCP-интеграция `pm` (связка с [T-15](../tasks/T-15-mcp-ready-process/README.md)) —
  отдельной задачей, вне периметра этого решения.

## Сверка с кодом

Вердикт: ⚪ **не применимо** (служебная зона/инструмент) — решение вводит новый
внешний инструмент процесса; продуктовый код прототипа (`src/**`) не меняет.
Факты по §5.3 (чтением, 30.09.2026):

- черновик `.opencode/scripts/pm/mine_agents.py` — 557 строк, вне git (`??`);
- формы `state/current/*` и `mail/` — по досье
  [`pm-tool-design-2026-09-30.md`](../analysis/pm-tool-design-2026-09-30.md);
- окружение инструмента — `uv 0.11.17` / Python 3.14.

`cargo` не запускался (§5.3, [D50](D50-dod-by-package-scope.md)): запись
служебная, кода прототипа не меняет.

**Задач не требуется:** служебная зона — исполняет сервисная сессия (карточка
`T-XX` не заводится); открытый пункт — MCP-интеграция `pm` (связка с T-15) —
отдельно.

## Альтернативы

- **(б) Расширение `session-analysis` (Node)** — отклонено: другой домен
  (транскрипты сессий, не state-лог), дублирование; Node-стек беден для
  DFG/вариантов; конвенции `session-analysis` только переиспользуются (README,
  «кто запускает», артефакты вне репо).
- **(в) Черновик как есть** — отклонено: не читает `mail`, не парсит ключ `next`,
  занижает приёмку (`accepted_with_notes`), риск колёс под Python 3.14.

## Ссылки

- Вопрос: [Q77](../questions/Q77.md)
- Связанные: [D75](D75-git-lean-workflow.md) (прецедент: хелпер служебной зоны +
  allowlist), [D51](D51-agent-tools-token-hygiene.md) (гигиена инструментов
  процесса), [D50](D50-dod-by-package-scope.md) (`cargo` по периметру)
- Артефакты: [`pm-tool-design-2026-09-30.md`](../analysis/pm-tool-design-2026-09-30.md)
  (досье), `.opencode/scripts/session-analysis/README.md` (конвенции артефактов),
  [T-15](../tasks/T-15-mcp-ready-process/README.md)/[D78](D78-t15-mcp-ready-program.md)
  (метрики процесса, связка MCP)
- Связи: [`TRACEABILITY.md`](../TRACEABILITY.md)
