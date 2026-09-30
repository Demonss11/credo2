# pm — process mining агентского процесса

Инструмент постфактум-анализа процесса рабочей группы CREDO2: за один проход из
состояния цикла (`.opencode/state/current/*`) и лент (`.opencode/mail/*.md`)
строит event log и считает DFG переходов между ролями, варианты маршрутов и
метрики приёмок. Решение — [D81](../../../docs/decisions/D81-pm-process-mining.md);
дизайн — [досье](../../../docs/analysis/pm-tool-design-2026-09-30.md).

**Кто запускает.** Владелец или сервисная сессия (`uv run`). У ролей рабочей
группы нет `uv`/`node` в allowlist — это не их зона; роли читают готовые отчёты.

**Артефакты — вне репозитория** (`%TEMP%\opencode\pm-<дата>\`, конвенция
`session-analysis`); в git попадает только итоговый отчёт в `docs/analysis/`.

## Установка и запуск

```powershell
cd .opencode/scripts/pm
uv sync                 # core: pyyaml; dev-группа: pytest
uv sync --extra viz     # + networkx/matplotlib/plotly — графики
uv run pm-agents        # артефакты → %TEMP%\opencode\pm-<дата>\
```

Примеры:

```powershell
# только состояние, без графиков, с фильтрами
uv run pm-agents --source state --no-viz --exclude-service --since 2026-09-27

# свои каталоги и порог «популярных» переходов
uv run pm-agents --state-dir .opencode/state/current --mail-dir .opencode/mail `
  --output-dir "$env:TEMP\opencode\pm-manual" --threshold 3
```

## Что в отчёте

- DFG: все переходы и «популярные» (порог `--threshold`);
- варианты маршрутов (топ-10) и частоты активностей;
- метрики приёмки (`accepted` / `accepted_with_notes` / `rework`), rework-rate,
  несовпадения `expect`, итерации;
- позиции активностей и длительность кейсов в днях (даты без времени — секунды
  не измеряются);
- текущий план `next_action` (задача, класс, длина очереди).

## Файлы

| Файл | Назначение |
|---|---|
| `pyproject.toml` | uv-проект: зависимости слоями (core / `viz` / `pm4py`), CLI `pm-agents` |
| `src/pm_agents/io_state.py` | загрузка `progress`/`receipts`/`next_action` (строгая структура) |
| `src/pm_agents/io_mail.py` | tolerant-парсер лент (`## <роль> · <дата> · <статус>`) |
| `src/pm_agents/events.py` | модель event log: слияние `progress` и `mail`, кейс = задача |
| `src/pm_agents/mining.py` | DFG, варианты, метрики |
| `src/pm_agents/viz.py` | PNG/HTML-визуализации (ленивые импорты, extra `viz`) |
| `src/pm_agents/report.py` | markdown-отчёт (`summary.md` и stdout) |
| `src/pm_agents/cli.py` | CLI и пайплайн |
| `tests/` | pytest-тесты (core, без viz) |

## Проверка

```powershell
uv run pytest                      # expect: 7 passed
uv run pm-agents --help
uv run pm-agents --no-viz --log-level ERROR
```

## Грабли

- **Даты без времени.** В `progress.yaml` только дата (`at`), поэтому порядок
  событий внутри дня — по позиции в файле; «секундные» задержки недостоверны.
- **`mail` — свободный markdown.** Парсятся только заголовки
  `## <роль> · <дата> · <статус>`; `## iteration N` и прочие пропускаются.
- **Кейсы.** `T-XX` — задачи, `service-*` — служебные волны, без `task` —
  `unknown-<idx>` (текущая задача из `next_action` не подставляется).
- **Extra `viz`.** Без него графики пропускаются с предупреждением; core
  (метрики/CSV/JSON/markdown) работает всегда.
- **Артефакты — вне репозитория.** Не коммитить `%TEMP%`-выгрузки; в git —
  только итоговый отчёт.
- **Файлы — сервисная зона.** Правки — через сервисную сессию владельца.
