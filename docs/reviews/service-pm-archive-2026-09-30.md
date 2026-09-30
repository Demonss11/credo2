# Приёмка: `service-pm-archive` (30.09.2026)

**Проверка:** адресная приёмка сервисной волны `service-pm-archive` — доработка
`pm` по трём решениям владельца (артефакты в `pm/output/`; вне git; накопительный
архив `output/events.jsonl` + best-effort снимок в `clean-logs.mjs`); согласованность
кода ↔ «Обновления 30.09.2026» D81 ↔ `pm/README.md` ↔ `AGENTS.md` ↔
`git-workflow.md` §«Подтверждение и очистка логов»; границы, версии и права ролей.

**Версия:** `develop` @ `180a91a` (= `origin/develop`) + рабочее дерево
(16 M + 2 `??`, включая эту запись `memory/validator.md` и `mail/`-ленту), 2026-09-30.

**Вердикт:** принято с замечаниями

**P1:** —

**P2:** —

**P3:** `docs/decisions/D81-pm-process-mining.md:7-10` (`Affects`) не перечисляет
`.opencode/scripts/clean-logs.mjs` и `.opencode/rules/git-workflow.md`, хотя блок
«Обновление 30.09.2026» (`:79-89`) меняет оба (снимок в `clean-logs`, абзац в
`git-workflow.md` §«Подтверждение и очистка логов»). Последствие: неполная
трассировка «решение ↔ артефакты» — при будущих правках `clean-logs.mjs` /
`git-workflow.md` связь с D81 не видна. Правка — один буллет в `Affects`
(`migrator`), не блокер.

**Наблюдения (не находки):**

- `D81:87-89` — «строку `AGENTS.md` … дополняет сервисная сессия» в настоящем
  времени, тогда как строка уже дополнена в этой же волне (ср. «уже дополнено»
  про `git-workflow.md`); блок написан как нарратив волны (`:62` «код вносится
  сервисной сессией параллельно»), на смысл не влияет (совпадает с наблюдением
  аудита r2, лента `:190-192`).
- Ключ архива (`archive.py:28-48`) не содержит времени: лента с тем же именем и
  **дословно** теми же полями заголовков (`роль · дата · статус`) после цикла
  `clean-logs` неотличима от уже заархивированной — соответствующее событие
  добавится как повтор и будет пропущено. Практический риск низок (нужно
  совпадение имени волны, даты и статуса), но это кандидат в новый `Q` — не
  блокер этой волны.

**Проверки (команды → результат):**

- `git status -sb` → `## develop...origin/develop`; 16 M + 2 `??`
  (`.opencode/mail/service-pm-archive.md`, `.opencode/scripts/pm/src/pm_agents/archive.py`);
  `src/**`, `tests/**` (Rust), `Cargo.toml`, `opencode.json`, `.opencode/agents/**`,
  `review.md` в наборе отсутствуют.
- `git diff --numstat` → `AGENTS.md 1/1`; `git-workflow.md 5/0`; `uv.lock 1/1`;
  `tests/test_pm_agents.py 127/1`; `D81 32/0`; `clean-logs.mjs 44/4`;
  `cli.py 79/15`, `events.py 44/15`, `report.py 10/0`, `README.md 36/10`.
- `git diff -- AGENTS.md` → ровно одна изменённая строка карты (`.opencode/scripts/`:
  + снимок `clean-logs`/`--no-snapshot`, + `pm/output/`, «вне git»).
- `git diff -- ./.opencode/rules/git-workflow.md` → `+5`, один новый абзац
  (`:176-180`) внутри §«Подтверждение и очистка логов» (заголовок `:167`, карта
  `rg -n "^#{2,3} "` подтверждает).
- `rg -c "^def test_" .opencode/scripts/pm/tests/test_pm_agents.py` → **15**;
  `rg -n "expect:" .opencode/scripts/pm/README.md` → `:85  # expect: 15 passed`.
- Версии `pm`: `pyproject.toml:3` = `0.2.0`; `__init__.py:3` = `0.2.0`;
  `uv.lock:759-760` (`name = "pm-agents"`, `version = "0.2.0"`).
- `rg -n "TEMP|tempfile|gettempdir" .opencode/scripts/pm/src` → пусто (вывод
  TEMP-дефолта нет; exit 1).
- `read .opencode/scripts/pm/output` → 5 артефактов (`summary.md`, `metrics.json`,
  `dfg_edges.csv`, `variants.csv`, `events.jsonl`); в `git status` их нет —
  каталог игнорируется (`pm/.gitignore:5 output/`).
- `node .opencode/scripts/agents-perms.mjs` → `agents: 11 из 18`; у роли `git` —
  прежний `node .opencode/scripts/clean-logs.mjs`, `uv` ни одной роли не добавлен;
  права `validator` совпадают с `review.md:86-87`.
- Свидетельства прогонов (`uv`/`cargo` ролью не исполняются): лента
  `:61-71` — `uv run pytest` → 15 passed; прогон №1 `+412`, №2 `+0`; срез
  `--source state --exclude-service` → `+0`, архив 412 строк; `--dry-run`/
  `--no-snapshot`/сбой снимка — на скретч-копии (живое удаление лент не выполнялось).

**Что проверено и ок:**

- **Решение (а)** — дефолт артефактов: `default_output_dir()` (`cli.py:28-36`) =
  `<root>/.opencode/scripts/pm/output` (маркер `find_repo_root` по
  `.opencode/state/current`), фолбэк без репозитория — `parents[2]/output`, т.е.
  `pm/output/`; `main` использует его при `output_dir=None` (`:164`).
- **Решение (б)** — вне git: `pm/.gitignore:5` `output/`, артефакты в дереве
  присутствуют и в `git status` не появляются.
- **Решение (в)** — архив: `merge_archive` (`archive.py:142-163`) сливает по
  стабильному ключу (`_identity :28-48` + номер повтора `_key :51-53`),
  `unknown-<idx>` обнуляется только для `source == "progress"` (`:34-36`);
  запись только при `added` → идемпотентность; атомарная перезапись
  (tmp + `os.replace`, `:128-139`); tolerant-чтение битых строк (`:105-125`);
  `clean-logs.mjs` снимок — `snapshotPm` (`:126-147`) при
  `!noSnapshot && toDelete > 0` (`:197`), вне `--dry-run` (`:182-190`),
  сбой → `console.warn` + продолжение (`:138-143`), `--no-snapshot` (`:44,67`).
- **Фильтры — только срез:** `parse_events` вызывается без `source` (`cli.py:173`,
  «оба источника всегда»), фильтры `--source`/`--since`/`--exclude-service` — в
  `finalize_event_log` (`events.py:141-148`); `--no-archive` (`cli.py:186`),
  `--archive` (`:187`); строка архива в шапке отчёта (`report.py:45-51`).
- **Тесты проверяют заявленное:** идемпотентность (`test_archive_is_idempotent`),
  живучесть после удаления лент (`test_archive_survives_mail_removal`),
  одинаковые заголовки (`test_archive_keeps_identical_headings`), накопление
  (`test_archive_accumulates_new_events`), `--since` не режет архив
  (`test_since_filters_view_not_archive`), `--no-archive`, свой `--archive`.
- **Канон:** `AGENTS.md` — одна строка; `git-workflow.md` — один абзац; тексты
  согласованы с кодом (снимок только при непустом списке лент, opt-out, сбой —
  предупреждение); права ролей не расширены (`agents-perms`).
- **Границы:** продуктовый крейт (`src/**`, `tests/**` Rust, `Cargo.toml`) диффом
  не тронут; скачок `docs/features/**`/счётчиков отсутствует →
  `cargo` не запускался — **D50**.

**Технические заметки:** `git check-ignore` не входит в allow-list `validator`
(`review.md:86-87`) — проверено чтением `pm/.gitignore:5` и отсутствием `output/`
в `git status`; `uv`/`cargo` ролью не исполняются — свидетельства прогонов взяты
из ленты (уровень «лента»).
