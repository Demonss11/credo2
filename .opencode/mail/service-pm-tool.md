# Сервисная лента: service-pm-tool (process-mining инструмент `pm`)

**Назначение:** инструмент постфактум-анализа агентского процесса CREDO2 —
DFG переходов между ролями, варианты маршрутов, метрики приёмки/rework поверх
`.opencode/state/current/*` и `.opencode/mail/`. Черновик владельца —
`.opencode/scripts/pm/mine_agents.py` (557 стр., вне git); предложение команды —
uv-проект в `.opencode/scripts/pm`.

**Решение владельца:** делегировано команде («изучи, обсуди с другими агентами и
примите решение»).

**Маршрут службы:** `migrator` (Q77→D81) → сервисная сессия (код uv-проекта) →
`auditor` (аудит канона `AGENTS.md`) → `validator` (приёмка) → `git` (пакет).

**Границы:** `.opencode/scripts/**`, `docs/analysis/**`, лента и памяти ролей,
`AGENTS.md` (карта `.opencode/scripts/`). `src/**`, `tests/**`, `Cargo.toml`,
`state/current/*` не трогаются; `cargo` не запускается (D50).

Формат записей — `AGENTS.md` §«Память и почта».

---

## analyst · 30.09.2026 · план обновлён

- Сделано: досье `docs/analysis/pm-tool-design-2026-09-30.md` — источники/модель
  событий, состав uv-проекта (core `pyyaml`; extra `viz`; extra `pm4py`),
  место запуска (владелец/сервисная сессия) и выводов (вне репо + отчёт в
  `docs/analysis/`), список улучшений черновика, Q77/D81. Класс — служебная
  операция (объём L-подобный: канон + новая подсистема).
- Проверки: чтение среза (`progress`/`receipts`/`next_action`/`mail`), сверка
  свободных ID (Q76/D80 заняты → Q77/D81 свободны); `cargo` не запускался.
- Дальше / риски: `migrator` — перенос Q77/D81; затем код; риск колёс Python 3.14
  (снят слоением extras); `mail` — tolerant-парсер.

## migrator · 30.09.2026 · Q77→D81

- Сделано: заведены `docs/questions/Q77.md` (resolved by D81) и
  `docs/decisions/D81-pm-process-mining.md` (accepted; 7 пунктов решения, ⚪
  «Сверка с кодом», «задач не требуется»); строки в `docs/TRACEABILITY.md`
  (CRLF) и каталогах `questions/README.md` / `decisions/README.md`.
- Проверки: `^`=89 и `\r$`=89 (`docs/TRACEABILITY.md` — CRLF цел); Q-строк 77
  (`TRACEABILITY.md`, `questions/README.md`); D-строк 81 (`decisions/README.md`);
  ссылки новых записей резолвятся (glob); `git status --porcelain` — только
  ожидаемое. `cargo` не запускался (D50).
- Дальше / риски: сервисная сессия — код uv-проекта `.opencode/scripts/pm/` по
  D81 (7 пунктов + улучшения досье пп. 1–15); затем `auditor` → `validator` →
  `git`. Артефакты — вне репо; открытый пункт — MCP-интеграция `pm` (T-15).

## сервисная сессия · 30.09.2026 · uv-проект `pm` собран и прогнан

- Сделано: в `.opencode/scripts/pm` собран uv-проект по D81 (core `pyyaml`;
  extra `viz`; extra `pm4py`): `pyproject.toml`, `uv.lock`, `src/pm_agents/`
  (`io_state`, `io_mail`, `events`, `mining`, `viz`, `report`, `cli`), `tests/`
  (6 тестов + фикстуры), `README.md`, `.gitignore`; `AGENTS.md` — карта
  `.opencode/scripts/` дополнена строкой про `pm/` (D81 п.7). Черновик
  `mine_agents.py` — **на решение владельца на гейте** (удалить/legacy).
- Проверки (сервисная сессия; роль `git` в коде не участвовала): `uv sync`
  (CPython 3.14, pyyaml 6.0.3) и `uv sync --extra viz` (networkx 3.7,
  matplotlib 3.11.2, plotly 7.1.0) — ok; `uv run pytest` → **6 passed**;
  прогон `uv run --project .opencode/scripts/pm pm-agents` на реальных данных →
  **394 события, 38 кейсов**, DFG «service-session → migrator → docs-writer →
  validator → git», приёмки **82.1%**, rework 17.9%; артефакты (9 файлов:
  PNG/HTML/CSV/JSON/`summary.md`) — `%TEMP%\opencode\pm-2026-09-30\`; итоговый
  отчёт — [`docs/analysis/pm-run-2026-09-30.md`](../../docs/analysis/pm-run-2026-09-30.md).
- Дальше / риски: аудит `auditor` (канон `AGENTS.md` + инструмент) → приёмка
  `validator` → пакет `git` по гейту; открытые пункты — судьба черновика,
  MCP-интеграция `pm` (T-15).

---

## auditor · 30.09.2026 · аудит service-pm-tool

- Сделано: независимый аудит канон-правки `AGENTS.md` + uv-проекта
  `.opencode/scripts/pm/` против [D81](../../docs/decisions/D81-pm-process-mining.md)/
  [Q77](../../docs/questions/Q77.md); прочитано 17 файлов (код `pm`, тесты,
  Q77/D81, лента, отчёт прогона, память), 10 `rg`.
- Проверки: `git diff AGENTS.md` — ровно одна строка (в карту `.opencode/scripts/`
  добавлен `pm/` — D81 п.7); `agents-perms.mjs` ×2 → «agents: 11 из 18»,
  идентично; `.opencode/agents/**`, `.opencode/rules/**`, `review.md` диффом не
  тронуты, права ролей не расширены (D81 п.6). Состав `pm/` и слои зависимостей
  (core `pyyaml`; extra `viz` networkx/matplotlib/plotly; extra `pm4py`;
  `seaborn` нет) совпадают D81; дефолт вывода — `tempfile` (`cli.py:29`).
  Код прочитан: `io_state` (`StateError`, `allow_single` только `next_action`),
  `io_mail` (tolerant-парсер, алиас «сервисная сессия»), `events`
  (`unknown-<idx>`, слияние `(дата, источник, индекс)`, синтетический
  timestamp), `mining` (`accepted_with_notes`=приёмка, очередь из `next`,
  позиции без «секунд»), `viz` (ленивые импорты + `VizUnavailable`), `cli`
  (флаги `--source/--since/--exclude-service/--no-viz/--threshold`, UTF-8
  stdout), `README` согласован коду, тесты — 6 функций + фикстуры.
  Записи: Q77/D81 по §4 (⚪, `Спека: —`, `Tasks: —`); `TRACEABILITY` 77 Q-строк
  (Q77/D81 `:81`), `questions/README` 77, `decisions/README` 81; ссылки живые
  (в т.ч. `session-analysis/README.md`, досье); ID не переиспользованы.
  Границы: `git status --porcelain` — `M` AGENTS.md, TRACEABILITY, оба README
  каталогов, `progress.yaml`, память `migrator`; `??` `.opencode/scripts/pm/`,
  Q77, D81, досье, `pm-run-2026-09-30.md`, лента; `src/**`, `tests/**`,
  `Cargo.toml`, `next_action|current_state|receipts` — чисты.
- Свидетельства прогонов (уровень: лента + отчёт; `uv` ролью не исполнялся —
  прав нет): `uv sync`/`--extra viz` ok; `uv run pytest` → 6 passed; прогон на
  данных репо 394 события/38 кейсов, приёмки 82.1% — отчёт
  [`pm-run-2026-09-30.md`](../../docs/analysis/pm-run-2026-09-30.md) существует и
  согласован ленте; `uv.lock` на месте; `.venv/`/`.pytest_cache/`/артефакты —
  вне git (`.gitignore`).
- Вердикт: **P1 нет**; инструкция ↔ права — расхождений нет.
- P2: черновик `mine_agents.py` (557 стр.) лежит в корне `pm/` и не покрыт
  `.gitignore` → `git add .opencode/scripts/pm/` затянет его в пакет (против
  «черновик вне git» — Q77/D81). Фикс: строка в `.gitignore` `pm/.gitignore`
  либо удаление/перенос по гейту владельца.
- P3: (1) правленая строка карты `AGENTS.md:25` не называет существующую
  `.opencode/scripts/session-analysis/` (сосед, на который ссылается D81);
  (2) события с неразобранной датой сортируются в 1970 (`events.py:143/152`),
  а не в конец — на реальных данных не проявляется.
- Дальше / риски: судьба `mine_agents.py` — на гейте владельца; MCP-интеграция
  `pm` (T-15) — вне периметра. Открытый вопрос: D81 (Следствия) упоминает
  фильтр `--format`, в CLI его нет — подтвердить у владельца (вне периметра
  аудита). Правок не вносил.

---

## migrator · 30.09.2026 · аудит P3 — D81

- Сделано: в `docs/decisions/D81-pm-process-mining.md` §«Следствия» список
  улучшений черновика приведён к реализованному CLI: `--source`/`--since`/
  `--exclude-service` (заменено `--exclude-service`/`--since`/`--format`;
  `--format` не вводился). Остальной текст D81 не тронут.
- Проверки: `rg -n "format" docs/decisions/D81-pm-process-mining.md` → пусто;
  замена одна (строка 55); `git status --porcelain` — только ожидаемое.
  `cargo` не запускался (D50).
- Дальше / риски: `validator` → `git` (пакет). Коммит — за `git`.

---

## сервисная сессия · 30.09.2026 · аудит P2/P3 — закрыто

- **P2:** `pm/.gitignore` — добавлен `mine_agents.py` (черновик не попадёт в
  коммит; удаление/сохранение — на гейте владельца).
- **P3-1:** карта `AGENTS.md` дополнена `session-analysis/` (та же строка карты).
- **P3-2:** `events.py` — события без разобранной даты сортируются **в конец**
  (было — в начало); добавлен тест `test_undated_events_sort_last`;
  `uv run pytest` → **7 passed**.
- **P3-3 (D81):** перечень фильтров в «Следствиях» приведён к реализованному
  (`--source`/`--since`/`--exclude-service`); запись `migrator · аудит P3 — D81`.

---

## auditor · 30.09.2026 · service-pm-tool — замечания закрыты

- Сделано: адресная проверка закрытия P2/P3 волны `service-pm-tool` (прочие
  результаты аудита в силе); ~8 файлов, 6 `rg`.
- Проверки: **P2** — `pm/.gitignore:6–7` содержит `mine_agents.py`;
  `rg --files .opencode/scripts/pm` (с учётом игнора) файл не показывает,
  `rg --files -u …` показывает (файл на диске цел) → в наборе untracked его нет
  (прокси `git ls-files -o --exclude-standard`; сама команда ролью не разрешена).
  **P3-1** — `git diff --numstat AGENTS.md` → `1 1` (одна изменённая строка),
  строка карты содержит `session-analysis/`; `.opencode/agents/**`, `review.md`,
  `.opencode/rules/**` диффом не тронуты. **P3-2** — `events.py:152–159`, ключ
  сортировки `(e.date is None, e.date or _EMPTY_DATE, источник, seq)` — события
  без даты идут **в конец**; тестов 7, включая `test_undated_events_sort_last`.
  **P3-3** — `rg -n "format" docs/decisions/D81-…` → пусто, §«Следствия» `:55`
  = `--source`/`--since`/`--exclude-service` (D81 — `??`, `git diff` неприменим;
  правка — одна строка по записи `migrator · аудит P3 — D81`).
- Свидетельства прогонов: `uv run pytest` → 7 passed — по ленте (ролью `uv` не
  исполнялся: прав нет), уровень «свидетельство».
- Вердикт: **P2, P3-1, P3-2, P3-3 — закрыто**; P1/P2 в силе нет; инструкция ↔
  права — расхождений нет.
- Остаток (не находка): запись сервисной сессии `:84` вставлена **перед**
  адресуемым ею аудитом `:96` (не append в хвост) — следствие для читаемости,
  правку не вносил (чужие записи не переписываю). Открытый пункт владельца —
  судьба черновика `mine_agents.py` (гейт). Правок не вносил.

---

## сервисная сессия · 30.09.2026 · rework приёмки (P2) — README

- **P2 (validator):** `pm/README.md` — ожидание прогона приведено к числу
  тестов: «expect: **7 passed**» (было 6; после P3-2 тестов стало 7). Одна
  строка; код не менялся.
- Дальше: повторная приёмка `-r2` — адресно: README ↔ число тестов ↔ лента.

---

## validator · 30.09.2026 · service-pm-tool — отклонено (rework, P2)

- Сделано: приёмка волны `service-pm-tool` (`Q77`→`D81`, канон-строка карты
  `AGENTS.md`, uv-проект `.opencode/scripts/pm/`, свидетельства прогонов,
  протокол аудита). Вердикт — **отклонено (rework)**: **P2**
  [`pm/README.md:62`](../../.opencode/scripts/pm/README.md) — «`uv run pytest`
  `# expect: 6 passed`», тогда как тестов **7** (`test_undated_events_sort_last`
  добавлен закрытием P3-2) и лента фиксирует **7 passed** (`:139`); README не
  обновлён после добора теста. **Правка (одна строка):**
  `pm/README.md:62` → `# expect: 7 passed`. Отчёт —
  [`docs/reviews/service-pm-tool-2026-09-30.md`](../../docs/reviews/service-pm-tool-2026-09-30.md);
  квитанция `service-pm-tool` iteration 1 (`rework`) — append.
- Проверки: `git diff --stat 74a9d4e -- src tests Cargo.toml` пусто → cargo не
  запускался (D50); `git status` — 7 M + 6 ?? только ожидаемых путей;
  `Q77`/`D81` по §4 (⚪, `Спека: —`, `Tasks: —`, фильтры
  `--source/--since/--exclude-service`; `rg "format"` в D81 пусто);
  TRACEABILITY 77 Q-строк и `^`=`\r$`=89, `:81` `resolved | — | —`, каталоги
  77/81, ID уникальны; `git diff AGENTS.md` — одна изменённая строка (`1 1`:
  +`pm/` и +`session-analysis/`), `.opencode/agents|rules/**` и `review.md` не
  тронуты, `agents-perms` = 11 из 18; состав и слои `pm/` (core `pyyaml`;
  extras `viz`/`pm4py`; README/ uv.lock/ tests), `.gitignore` с `mine_agents.py`
  (в untracked-набор не входит; `git status -uall` — 19 файлов),
  `events.py:154-155` — даты без разбора в конец, тестов 7; свидетельства
  прогонов (уровень «свидетельство»): `uv sync`×2, `uv run pytest`,
  прогон 394 события/38 кейсов/82.1% — отчёт `pm-run-2026-09-30.md`
  согласован; P1 нет, P2/P3-1/P3-2/P3-3 аудита закрыты адресно; §5.7 чист.
- Дальше / риски: **rework** — правку `pm/README.md:62` (одна строка) вносит
  сервисная сессия; повторная приёмка — отчёт `-r2` (адресно: README ↔ число
  тестов ↔ лента). Наблюдение (не находка): блок ленты
  `migrator · аудит P3 — D81` (`:49`) стоит до аудита (`:84`), на который
  отвечает; основная цепочка протокола (analyst → migrator → сервисная сессия →
  auditor → закрытие → auditor) в порядке; прежний остаток аудита (запись
  сервисной сессии перед аудитом) исправлен. Уровень посылок: `uv` ролям
  недоступен — прогоны приняты как свидетельство (лента + отчёт).

---

## validator · 30.09.2026 · service-pm-tool — принято (r2)

- Сделано: повторная приёмка (`-r2`) после rework — адресная перепроверка
  закрытия P2 (README ↔ число тестов). Вердикт — **принято**, остатка нет.
  Отчёт — [`docs/reviews/service-pm-tool-2026-09-30-r2.md`](../../docs/reviews/service-pm-tool-2026-09-30-r2.md);
  квитанция `service-pm-tool` iteration 2 (`accepted`) — append.
- Проверки: `rg -n "expect:" .opencode/scripts/pm/README.md` → одна строка
  (`:62`) «`# expect: 7 passed`»; `rg -c "^def test_"
  .opencode/scripts/pm/tests/test_pm_agents.py` → **7**; запись ленты
  «сервисная сессия · rework приёмки (P2) — README» (`:172-177`) на месте и
  соответствует правке; `git status` — 9 M + 7 ?? (тот же состав, что в конце
  r1; мои `memory/validator.md` и `receipts.yaml` — зона приёмки);
  `git diff --numstat 74a9d4e` по `AGENTS.md`/`TRACEABILITY`/каталогам = как в
  r1 (новых правок нет); `git diff --stat 74a9d4e -- src tests Cargo.toml`
  пусто → cargo не запускался (D50); лента — порядок блоков по заявленной
  последовательности (аудит → `migrator` P3 → закрытие → `auditor` закрыто →
  rework README), наблюдение r1 о позиции блока `migrator · аудит P3 — D81`
  **снято** (блок перенесён после аудита).
- Дальше / риски: замечаний нет — волна `service-pm-tool` готова к пакету `git`
  по гейту (открытые пункты вне приёмки: судьба черновика `mine_agents.py`,
  MCP-интеграция `pm` → T-15). Наблюдение (не находка): после переноса блоков
  исторические ссылки на строки внутри ленты устарели (`auditor` `:165-168`,
  `validator` r1 `:209-213`), и запись rework (`:172`) стоит перед записью
  `validator` r1 (`:181`) — файл оперативный, блоки самоописательны; чужие
  записи не переписываю, свою историческую (r1) тоже — это улика отчёта.

---

## сервисная сессия · 30.09.2026 · подтверждение пакета

- Гейт пройден: владелец подтвердил **коммит + push** и судьбу черновика
  (`question`, ответы: «Коммит + push (Recommended)», «Оставить локально вне
  git (Recommended)») — `mine_agents.py` остаётся на диске в корне `pm/` и не
  коммитится (`pm/.gitignore`).
- База: `develop` @ `74a9d4e` (= `origin/develop`); режим — коммит прямо в
  `develop` (как предыдущие волны), затем `git push origin develop`.
- Пакет (17 путей операции + запись роли `git` по F43 = 18):
  - `.opencode/mail/service-pm-tool.md`
  - `.opencode/memory/auditor.md`
  - `.opencode/memory/git.md` (чекпойнт роли `git`, новый до `add`)
  - `.opencode/memory/migrator.md`
  - `.opencode/memory/validator.md`
  - `.opencode/state/current/progress.yaml`
  - `.opencode/state/current/receipts.yaml`
  - `.opencode/scripts/pm/` (uv-проект; `mine_agents.py` — вне пакета)
  - `AGENTS.md`
  - `docs/TRACEABILITY.md`
  - `docs/decisions/D81-pm-process-mining.md`
  - `docs/decisions/README.md`
  - `docs/questions/Q77.md`
  - `docs/questions/README.md`
  - `docs/analysis/pm-tool-design-2026-09-30.md`
  - `docs/analysis/pm-run-2026-09-30.md`
  - `docs/reviews/service-pm-tool-2026-09-30.md`
  - `docs/reviews/service-pm-tool-2026-09-30-r2.md`
- Сообщение коммита: `chore(pm): uv-проект process mining агентов (D81)`.
- Замечание: запись роли `git` (лента + `memory/git.md`) формируется **до**
  `add` и входит в коммит (F43); после `push` в отслеживаемые файлы не пишем
  (хеши — ответом `lead`).

## git · 30.09.2026 · пакет выполняется (коммит + push)

- Сделано: сверены подтверждение (запись выше «подтверждение пакета») и
  состояние: база `develop` = `origin/develop` = `74a9d4e` (синхрон), дерево —
  9 `M` + 8 `??`, состав = пакету (посторонних нет); `pm/` = 19 файлов,
  `mine_agents.py`/`.venv/` — вне git по `pm/.gitignore`.
- Проверки: `git status -sb`, `git log -1 --oneline` (74a9d4e), `git status
  --porcelain -uall ./.opencode/scripts/pm` → 19 `??`; записи F43 (лента +
  память) сделаны до `add`.
- Дальше / риски: `add` точными путями из ленты → staged-сверка
  (`git-check.mjs --staged`) → `commit chore(pm): uv-проект process mining
  агентов (D81)` → `push origin develop` (≥ 5 мин); post-push git-команд нет,
  хеши — в ответе `lead` (F43).
