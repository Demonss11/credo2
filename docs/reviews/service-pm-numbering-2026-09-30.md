# Приёмка: service-pm-numbering (нумерация событий архива pm)

**Проверка:** адресная приёмка сервисной волны `service-pm-numbering`
(30.09.2026): сквозная нумерация `n` событий архива `pm/output/events.jsonl`
(присвоение при первом попадании, продолжение после `clean-logs.mjs`),
выгрузка `output/events.csv`, печать номера продолжения в `clean-logs.mjs`.
Сверка «код ↔ D81 («Обновление 30.09.2026 (нумерация)») ↔ `pm/README.md` ↔
`clean-logs.mjs`», тесты/доки, границы, версия.

**Версия:** `develop` @ `6a0d342` + рабочее дерево (13 `M` + 1 `??`, 2026-09-30)

**Вердикт:** принято с замечаниями

**P1:** — критичных проблем нет (P1/P2 отсутствуют).

**P2:** — нет.

**P3:** `docs/decisions/D81-pm-process-mining.md:110-111` — канон перечисляет
формат выгрузки как «(№; дата; источник; кейс; активность; роль; действие;
результат)» (8 полей; так же в решении владельца, лента `:11-12`), тогда как
`cli.py:258-287` отдаёт **10** колонок — добавлены `iteration`,
`expect_match`; тест фиксирует именно 10 (`tests/test_pm_agents.py:286-288`).
Последствие: документированная схема `events.csv` неполна — сверка артефакта с
каноном/решением владельца даёт «лишние колонки». Правка — одна строка в D81
(дополнить перечисление `; итерация; expect_match`), адресат `migrator`;
альтернатива — урезать код. Не блокер.

**Проверки:**

- `git status --porcelain` → 13 `M` + 1 `??`:
  `M` — `clean-logs.mjs`, `pm/README.md`, `pm/pyproject.toml`,
  `pm/src/pm_agents/{__init__,archive,cli,events,report}.py`,
  `pm/tests/test_pm_agents.py`, `pm/uv.lock`, `docs/decisions/D81-…`,
  `memory/migrator.md`, `state/current/progress.yaml`; `??` —
  `mail/service-pm-numbering.md`. `AGENTS.md`, `.opencode/agents/**`,
  `.opencode/rules/**`, `opencode.json`, `src/**`, `tests/**` (Rust),
  `Cargo.toml` — в диффе отсутствуют.
- `git log -3 --oneline develop` → HEAD `6a0d342` (`chore(pm): … архив событий,
  pm/output и снимок лент в clean-logs`).
- `git diff -U0` по `archive.py` / `clean-logs.mjs` / `cli.py` / `events.py` /
  `report.py` / `D81` → только ожидаемые хунки (см. ниже).
- `git diff --numstat` → `clean-logs.mjs` 38/6, `archive.py` 27/3, `cli.py` 36/2,
  `events.py` 1/0, `report.py` 3/1, `README.md` 16/6, `test_pm_agents.py` 55/0,
  `pyproject.toml`/`__init__.py`/`uv.lock` 1/1 каждый, `D81` 21/0.
- `git diff --numstat -- ./AGENTS.md ./.opencode/agents ./.opencode/rules
  ./opencode.json ./src ./tests ./Cargo.toml` → пусто.

**Что проверено и ок:**

- **`archive.py`** — `_row` пишет `"n": event.n` (`:71`), `_from_row` читает
  `n` толерантно (`:94,:108`, `n>0` иначе 0); `merge_archive` (`:157-182`):
  `legacy = any(n <= 0)`, пропущенные номера нумеруются по порядку,
  `counter = max(n)+1` по существующим, новым — `event.n = counter`; запись
  (`_write`) при `added or legacy` — разовый бэкфилл; `next_number = max+1`
  (`:185-187`). Присвоение при первом попадании и неизменность — по ключу
  `_key` (стабильные поля + номер повтора).
- **`events.py`** — поле `n: int = 0` (0 — вне архива) (`:38`).
- **`cli.py`** — `archive_next = next_number(merged_events)` (`:195`), лог
  «…следующее — №%d…» (`:196-202`); `events.csv` из `logbook.events`
  (`:258-287`), колонка `n`: `event.n or ""` → **пуста при `--no-archive`**
  (n=0) — совпадает с решением; в `--no-archive` архив не читается/пишется.
- **`report.py`** — шапка «(+N новых, всего M; следующее событие — №K)»
  (`:46-51`) при включённом архиве; ветка `--no-archive` — «отключён».
- **`clean-logs.mjs`** — `lastArchiveNumber()` (`:151-165`, последняя непустая
  строка архива, tolerant, `n>0`) и `printArchiveContinuation()`
  (`:167-175`, «…пронумерованы до №N; …продолжится с №N+1»); в `--dry-run` —
  внутри блока `toDelete>0`, префикс `[dry-run] ` (`:210-218`); в реальном
  прогоне — после `snapshotPm()` (`:228-229`), без зависимости от
  `--no-snapshot`; сбой снимка — `console.warn`, очистка продолжается
  (`:140-148`). «Снимок-ветка» (dry-run/`--no-snapshot`) не ломается.
- **`D81`** — блок «Обновление 30.09.2026 (нумерация)» (`:99-119`) добавлен
  одним куском (+21/0), §5 «Решение» и прежний блок «Обновление 30.09.2026»
  целы; ссылки `../analysis/pm-tool-design-2026-09-30.md`, `../TRACEABILITY.md`,
  `../questions/Q77.md` резолвятся; новых `Q`/`D` нет, `Affects` не менялся
  (пути `pm/**` + `clean-logs.mjs` там есть).
- **Тесты/доки** — `rg -c "^def test_"` = **18**; `pm/README.md:95`
  «expect: 18 passed»; +3 новых теста по существу: `test_archive_numbers_stable_and_continue`
  (1..k, повтор неизменен, новое = k+1), `test_legacy_archive_backfilled`
  (снятие `n` → бэкфилл 1..k, повтор +0), `test_events_csv_export`
  (заголовок, число строк = `total_events`, номера 1..N); README — раздел
  «Сквозная нумерация», `events.csv` в артефактах и в файловой таблице.
- **Артефакты-улики** — `output/events.jsonl`: 426 строк, все содержат `"n"`,
  первая `n=1`, последняя (`:426`) `n=426`; `output/events.csv`: 427 строк
  (заголовок + 426), шапка из 10 колонок; `output/summary.md:7` — «+0 новых,
  всего 426; следующее событие — №427» (подтверждает идемпотентность повтора и
  печать «продолжится с №427» из письма-ленты).
- **Версия** — 0.3.0: `pyproject.toml:3`, `__init__.py:3`, `uv.lock:760`;
  `rg "0\.2\.0" pm` пусто.
- **Границы** — `src/**`, `tests/**` (Rust), `Cargo.toml` не тронуты; канон
  агентов (`AGENTS.md`, `.opencode/agents/**`, `.opencode/rules/**`,
  `opencode.json`) — диффом чист → аудит «инструкция ↔ права» не требуется;
  `docs/analysis/**` не тронут; живого удаления лент нет (в `git status` нет
  удалений `mail/**`).

**Технические ограничения (что не проверено и почему):**

- `uv`/`node` не в allow-list роли `validator` (`review.md:86-87`) — прогоны
  `uv run pytest` (18 passed), бэкфилл реального архива (№1–426, повтор +0),
  `clean-logs --dry-run` (до №426 / с №427), скретч-тест (сбой снимка →
  предупреждение + «до №41; с №42»; `--no-snapshot` → печать без снимка) и
  `node --check` приняты **на уровне свидетельств ленты** (`mail/service-pm-numbering.md:52-60`);
  поведение `clean-logs.mjs` подтверждено чтением кода (см. «ок»).
- **`cargo` не запускался — D50** (`src/**`/`tests/**`/`Cargo.toml` в диффе
  отсутствуют, счётчики/состав `docs/features/**` не менялись); исключение
  «правки счётчиков/состава фич» не сработало.

---

## Обновление 2026-09-30 — P3 закрыт (дельта-приёмка)

**Версия дельты:** `develop` @ `6a0d342` + рабочее дерево (13 `M` + 2 `??`,
2026-09-30); код `pm/**` и `clean-logs.mjs` с r1 не менялся.

**Вердикт дельты:** принято, замечаний нет (P1/P2/P3 нет).

- **P3 закрыт** (`migrator`, микроправка): `D81:110-112` — подпункт «выгрузка»
  перечисляет **10** полей: «№; дата; источник; кейс; активность; роль;
  действие; результат; итерация; `expect_match`» — совпадает с заголовком
  `cli.py:259-271` (`n;date;source;case_id;activity;role;action;result;iteration;expect_match`)
  и тестом `tests/test_pm_agents.py:286-288`.
- **Изоляция дельты:** `git diff -U0 -- ./docs/decisions/D81-pm-process-mining.md`
  → ровно один хунк `@@ -98,0 +99,22 @@` (вставка блока «Обновление
  30.09.2026 (нумерация)»); `git diff --numstat` D81 = **22/0** (в r1 было
  `21/0` — прирост ровно на одну строку перечисления); иных хунков в файле нет
  (строки `:1-98` не тронуты), §5 «Решение» и структура целы.
- **Регрессий нет:** `git diff --numstat` всех файлов кода/доков волны идентичен
  r1 (`clean-logs.mjs` 38/6, `archive.py` 27/3, `cli.py` 36/2, `events.py` 1/0,
  `report.py` 3/1, `README.md` 16/6, `test_pm_agents.py` 55/0,
  `pyproject.toml`/`__init__.py`/`uv.lock` 1/1, `progress.yaml` 7/0); выросли
  только память `migrator` (20→28) и D81 (21→22).
- Границы прежние: `src/**`, `tests/**` (Rust), `Cargo.toml`, `AGENTS.md`,
  `.opencode/agents/**`, `.opencode/rules/**`, `opencode.json` — чисты;
  **`cargo` не запускался — D50**; `uv`/`node` ролью не исполняются (allow-list
  `review.md:86-87`), прогоны — на уровне свидетельств ленты.
