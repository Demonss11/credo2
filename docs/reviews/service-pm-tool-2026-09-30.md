# Приёмка: `service-pm-tool` (Q77 → D81, uv-проект `pm`)

**Проверка:** сервисная волна `service-pm-tool` — записи `Q77`/`D81`, канон-строка
карты `AGENTS.md`, uv-проект `.opencode/scripts/pm/`, свидетельства прогонов и
протокол аудита.

**Версия:** `develop` @ `74a9d4e` (= `origin/develop`) + рабочее дерево
(снимок 2026-09-30).

**Вердикт:** **отклонено (rework)** — один P2 (`pm/README.md:62`), правка одна
строка.

---

**P1:** —

**P2:** `.opencode/scripts/pm/README.md:62` — документированное ожидание прогона
устарело после закрытия P3-2: строка `uv run pytest                      # expect: 6 passed`,
тогда как в `tests/test_pm_agents.py` — **7** функций (`rg -n "^def test_"
.opencode/scripts/pm/tests/test_pm_agents.py` → 7, включая
`test_undated_events_sort_last`), и лента фиксирует `uv run pytest` → **7 passed**
(`service-pm-tool.md:139`). Ожидание: README согласован коду (критерий приёмки,
п. 4 запроса; первый аудит подтверждал «README согласован коду, тесты — 6
функций» при 6 тестах — после добавления 7-го теста README не обновлён, адресный
аудит замечаний не перепроверял README). Последствие: человек, выполняющий
документированную проверку (§«Проверка»), получает `7 passed` против
`expect: 6 passed` — расхождение документа с кодом и с свидетельствами волны;
для CI/приёмки документированный ожидаемый вывод недостоверен.
**Правка (одна строка, сервисная сессия / `docs-writer` — по зоне):**
`pm/README.md:62` → `uv run pytest                      # expect: 7 passed`.

**P3:** —

---

## Проверки

- **Границы.** `git status --porcelain` → 7 `M` + 6 `??`:
  `M` `AGENTS.md`, `docs/TRACEABILITY.md`, `docs/questions/README.md`,
  `docs/decisions/README.md`, `.opencode/state/current/progress.yaml`,
  `.opencode/memory/migrator.md`, `.opencode/memory/auditor.md`;
  `??` `.opencode/scripts/pm/`, `Q77`, `D81`, досье
  `pm-tool-design-2026-09-30.md`, `pm-run-2026-09-30.md`, лента
  `.opencode/mail/service-pm-tool.md`. `git diff --stat 74a9d4e -- src tests
  Cargo.toml` → **пусто** →
  **cargo не запускался (D50)**. `src/**`, `tests/**`, `Cargo.toml`,
  `state/current/next_action|current_state` не тронуты.
- **Записи.** `Q77.md`/`D81-pm-process-mining.md` — по §4 `journal.md`
  (Статус/Дата/Приоритет/Связано; Статус/Дата/Resolves/`Спека: —`/Affects/Tasks
  + Контекст/Решение/Следствия/Сверка с кодом/Альтернативы/Ссылки); вердикт
  «Сверка с кодом» — ⚪ **не применимо** с фактами §5.3; `Tasks: —` (служебная
  зона). «Следствия» `D81` перечисляют фильтры `--source`/`--since`/
  `--exclude-service` (`:55`); `rg -n "format" D81` → пусто. Парность
  `Q77:3` `resolved by [D81]` ↔ `D81:5` `Resolves: [Q77]`.
  `TRACEABILITY.md` — **77** Q-строк (`:81` = `| [Q77] | [D81] | resolved | — | — |`),
  `rg -c "^"` = `rg -c "\r$"` = **89** (CRLF и финальный перевод строки целы),
  diff = ровно +1 строка; каталоги — **77** Q-строк / **81** D-строк (+Q77/+D81);
  `Q77`/`D81` уникальны (ID не переиспользованы).
- **Канон.** `git diff AGENTS.md` → **одна изменённая строка** (`1 1`): в карту
  `.opencode/scripts/` добавлены `session-analysis/` и `pm/` (uv-проект; D81).
  `.opencode/agents/**`, `.opencode/rules/**`, `review.md` диффом не тронуты;
  права ролей не расширены (D81 п.6). `node .opencode/scripts/agents-perms.mjs`
  → «**agents: 11 из 18**» (совпало с аудитом ×2 и с составом).
- **Проект `pm/`.** Состав: `pyproject.toml`, `uv.lock`, `README.md`,
  `.gitignore`, `src/pm_agents/` (8 модулей: `__init__`, `__main__`, `cli`,
  `io_state`, `io_mail`, `events`, `mining`, `report`, `viz`), `tests/`
  (`test_pm_agents.py` + фикстуры `state/*`, `mail/*`). Слои зависимостей:
  core `pyyaml>=6.0`; extra `viz` = networkx/matplotlib/plotly; extra `pm4py`;
  `seaborn` нет; CLI `pm-agents = pm_agents.cli:main`; `requires-python >=3.12`;
  dev-группа `pytest`.
  `.gitignore` покрывает `.venv/`, `__pycache__/`, `*.egg-info/`,
  `.pytest_cache/`, `output/` и `mine_agents.py` (черновик — судьба на гейте).
  `git status -uall .opencode/scripts/pm` → 19 файлов к коммиту, черновика среди
  них нет (`mine_agents.py` виден только `rg --files -u` — на диске цел, вне
  git). `events.py:154-155` — ключ сортировки `e.date is None` → неразобранные
  даты идут в конец (P3-2 закрыт).
- **Свидетельства прогонов (уровень «свидетельство»).** `uv` ролям недоступен:
  прогоны исполнены сервисной сессией и зафиксированы лентой —
  `uv sync` + `uv sync --extra viz` (CPython 3.14, pyyaml 6.0.3; networkx 3.7,
  matplotlib 3.11.2, plotly 7.1.0) — ok; `uv run pytest` → **7 passed** (`:139`);
  прогон на реальных данных → **394 события, 38 кейсов**, приёмки **82.1%**,
  rework 17.9%; отчёт `docs/analysis/pm-run-2026-09-30.md` существует и
  согласован (`:23` — 394/38; `:82` — 82.1%/17.9%); артефакты вне репозитория
  (`%TEMP%\opencode\pm-2026-09-30\`). Прямых прогонов `uv` в этой приёмке нет —
  прав нет (это и есть уровень «свидетельство»).
- **Аудит.** Лента: `analyst` → `migrator` Q77→D81 → `migrator` аудит P3 — D81 →
  сервисная сессия «проект собран и прогнан» → `auditor` аудит → сервисная
  сессия «аудит P2/P3 закрыто» → `auditor` «замечания закрыты». Первый аудит —
  **P1 нет**, P2 (черновик не покрыт `.gitignore`) и P3-1 (карта без
  `session-analysis/`), P3-2 (неразобранные даты в начало), P3-3 (`--format` в
  D81); закрытие подтверждено адресно (`pm/.gitignore:6-7`;
  `AGENTS.md` `1 1`; `events.py:152-159`; `rg "format" D81` → пусто).
- **Чек-лист §5.7.** Ссылки живые (`Q77`/`D81`, `TRACEABILITY`, досье,
  `pm-run-2026-09-30.md`, `.opencode/scripts/session-analysis/README.md`,
  `T-15`/`D78`); статусы синхронны (`Q77` `resolved by [D81]` ↔
  `TRACEABILITY` `resolved` ↔ каталоги; `Tasks: —`); миграционных маркеров в
  изменённых файлах нет.

## Наблюдение (не находка)

Лента: блок `migrator · аудит P3 — D81` (`:49`) стоит **до** аудита (`:84`),
на P3 которого он отвечает, и до записи «проект собран» (`:62`); текст блока
говорит «приведён к **реализованному** CLI». Основная цепочка протокола
(analyst → migrator → сервисная сессия → auditor → закрытие → auditor) —
в порядке; прежний остаток аудита (запись сервисной сессии перед аудитом)
исправлен. При желании — перенести блок `:49` после аудита; на решение не
влияет.

**Что проверено и ок:** границы (кроме README-числа — см. P2) — `src`/`tests`/
`Cargo.toml`/канон ролей не тронуты; записи `Q77`/`D81` по §4; `TRACEABILITY`
77/89 CRLF и каталоги 77/81; канон-строка `AGENTS.md` (1 строка, `pm/` +
`session-analysis/`); состав и слои `pm/`, `.gitignore` с `mine_agents.py`,
отсутствие черновика в наборе untracked; README согласован коду **кроме
ожидаемого числа тестов** (P2); свидетельства прогонов (лента + отчёт,
394/38/82.1%); закрытие P2/P3-1/P3-2/P3-3 аудита; чек-лист §5.7.
