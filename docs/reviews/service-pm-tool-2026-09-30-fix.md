# Приёмка: `service-pm-tool` — пост-правка «пути от корня репозитория»

**Проверка:** адресная приёмка правки после волны (после коммита `c137b63`) —
дефолтные пути состояния/лент определяются **от корня репозитория**, запуск
`uv run pm-agents` из `.opencode/scripts/pm` больше не падает.

**Версия:** `develop` @ `c137b63` (= `origin/develop`) + рабочее дерево
(снимок 2026-09-30).

**Вердикт:** **принято** — P1/P2/P3 нет.

---

## Границы и DoD

- `git status --porcelain` → **5 M, `??` нет**:
  `.opencode/scripts/pm/README.md`, `.opencode/scripts/pm/src/pm_agents/cli.py`,
  `.opencode/scripts/pm/tests/test_pm_agents.py`, лента
  `.opencode/mail/service-pm-tool.md`, `.opencode/state/current/progress.yaml`
  (запись `lead` о закрытии волны, `iteration: 1`, коммит `c137b63` — вне
  правки). Посторонних путей нет.
- `git diff --stat c137b63 -- src tests Cargo.toml` → **пусто** →
  **cargo не запускался (D50)**.
- `git diff --stat c137b63` → ровно **5 файлов**: лента (+17), `README.md`
  (+5/−1), `cli.py` (+44/−6), тесты (+16), `progress.yaml` (+7). Иные файлы
  `pm/` не тронуты.

## Соответствие правки

- `cli.py`:
  - `find_repo_root(start)` — идёт вверх по `(current, *current.parents)` и
    возвращает каталог, где есть `.opencode/state/current` (`is_dir()`), иначе
    `None` (`:32-38`);
  - `resolve_state_dir(explicit)` / `resolve_mail_dir(explicit)` — при явном
    пути возвращают его, иначе `root / DEFAULT_STATE_DIR` (`DEFAULT_STATE_DIR =
    Path(".opencode/state/current")`, `DEFAULT_MAIL_DIR = Path(".opencode/mail")`),
    а без найденного корня — прежний относительный дефолт (`:41-54`);
  - `parse_args`: `--state-dir`/`--mail-dir` `default=None` (`:68`, `:74`) с
    пояснением «по умолчанию — от корня репозитория»;
  - `main`: `state_dir = resolve_state_dir(args.state_dir)` /
    `mail_dir = resolve_mail_dir(args.mail_dir)` (`:147-148`) и далее **только**
    resolved-значения — в `build_event_log` (`:151`) и `render_report_md`
    (`:223`, `mail_dir=mail_dir if args.source != "state" else None`); сырых
    `args.state_dir`/`args.mail_dir` в коде **не осталось**
    (`rg "args\.state_dir|args\.mail_dir"` → пусто);
  - в сообщении об ошибке добавлена подсказка про явные пути (`:160`).
- `tests/test_pm_agents.py`: `test_repo_root_resolution` (`:77`) — создаёт
  `repo/.opencode/state/current` + вложенный `repo/.opencode/scripts/pm`,
  `monkeypatch.chdir(nested)`, проверяет `find_repo_root` → корень,
  `resolve_state_dir(None)`/`resolve_mail_dir(None)` → от корня и возврат
  явного пути. Всего `rg -c "^def test_"` → **8**.
- `README.md`: `:62` — `uv run pytest  # expect: 8 passed`; в «Грабли» добавлен
  пункт «**Запуск из любого каталога**» (маркер `.opencode/state/current`, вверх
  по дереву; вне репозитория — явные `--state-dir`/`--mail-dir`).

## Свидетельство (уровень «свидетельство»)

`uv` ролям недоступен — прогоны исполнены сервисной сессией и зафиксированы
лентой (`.opencode/mail/service-pm-tool.md:280-293`): `uv run pytest` →
**8 passed** (добавлен `test_repo_root_resolution`); контрольный прогон **из
`pm/`** — найдены `…\credo2\.opencode\state\current` и `…\mail`, отчёт построен
(**405 событий, 38 кейсов**; приёмки 81.0%, rework 19.0%). Заявленное
согласуется с диффом (тестов 8, дефолты от корня). Рост числа событий против
первого прогона (394 → 405) объясним: в event log вошли записи, добавленные
после первого прогона (эта же лента: подтверждение пакета, fix; `receipts`/
`progress`), — расхождения с правкой нет.

## §5.7 и чистота

README согласован коду (ожидание тестов = 8, инструкции запуска соответствуют
новому поведению); ссылки `README.md` (`../../../docs/decisions/D81…`,
`../../../docs/analysis/…`) живы; правка не расширяет права ролей (файлы
служебной зоны); миграционных маркеров нет. Дифф ровно по трём файлам `pm/` —
иные не тронуты.

## Наблюдение (не находка, для читаемости)

Лента: запись `сервисная сессия · fix после приёмки` вставлена **перед**
закоммиченной (`c137b63`) записью `git · пакет выполняется (коммит + push)`,
которая описывает пакет волны (база `74a9d4e`) и потому хронологически
предшествует правке; `git diff c137b63` показывает вставку `+17` в середину
(перед `## git`), а не append в конец. Блоки самоописательны (у `git`-записи
своя база и сообщение коммита), поэтому влияние — только на порядок чтения; при
желании блок fix переносится после `git`. На решение не влияет.

**Что проверено и ок:** границы (5 M, без посторонних; `src`/`tests`/`Cargo.toml`
чисты → cargo не запускался); `find_repo_root`/`resolve_*`, `default=None`,
использование resolved-путей в обоих местах `main`, подсказка в ошибке; тест
`test_repo_root_resolution` и число тестов 8; `expect: 8 passed` и пункт «Запуск
из любого каталога» в README; лента-свидетельство (8 passed; прогон из `pm/` —
405/38/81.0%); отсутствие иных правок `pm/`; §5.7.
Правка готова к пакету `git` по гейту.
