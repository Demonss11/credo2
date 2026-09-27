# Матчинг shell-прав OpenCode v2 — пробы (T-13 / F4)

- Дата: 27.09.2026
- Ветка: `feature/T-13-agent-hardening`
- Метод: read-only пробы по одной команде.
- Термин: «отклонена» — слой прав вернул `permission.rejected` («Permission denied: shell») **до** исполнения команды.
- Участники: `git`-роль (18 проб + 14 уточняющих), `coder` (8 проб).

## 1. Пробы `git`-роли (allow: `git diff *`, `git status *`, `git log *`, `git show *`, `git rev-parse *`, `git remote -v`, `rg *`)

| Команда | Результат |
|---|---|
| `git diff` | прошла |
| `git diff --stat` | прошла |
| `git diff docs/analysis` | прошла |
| `git diff -- docs/analysis` | прошла |
| `git diff .opencode/rules/dispatch-loop.md` | прошла |
| `git diff -- .opencode/rules/dispatch-loop.md` | **отклонена** |
| `git status --short` | прошла |
| `git status -sb` | прошла |
| `git log --oneline -1` | прошла |
| `git rev-parse --show-toplevel` | прошла |
| `git show --stat -1` | прошла |
| `git remote -v` | прошла |
| `git ls-remote origin` | **отклонена** (паттерна нет) |
| `rg --files .opencode/rules` | прошла |
| `rg -n "dispatch-loop" docs/analysis` | прошла |
| `rg -n "dispatch-loop" -- docs/analysis` | прошла |
| `rg dispatch-loop ; echo done` | **отклонена** |
| `rg dispatch-loop \| Select-Object -First 1` | **отклонена** |

## 2. Уточняющая серия (триггер `--`)

| Команда | Результат |
|---|---|
| `git diff -- docs/.zzz-nonexistent` | прошла |
| `git diff -- .gitignore` | **отклонена** |
| `git diff -- src/mcp.rs` | прошла |
| `git diff -- .opencode` | **отклонена** |
| `git diff -- docs/.opencode` | прошла |
| `git diff -- .foo-nonexistent` | **отклонена** |
| `git diff -- ./src/mcp.rs` | прошла |
| `git diff -- ./.opencode/rules/dispatch-loop.md` | прошла |
| `git diff -- .opencode/rules/dispatch-loop.md` | **отклонена** (воспроизведение) |
| `git diff .opencode/rules/dispatch-loop.md` (без `--`) | прошла |
| `git diff .gitignore` (без `--`) | прошла |
| `git diff --stat -- .opencode` | **отклонена** |
| `git status -- .opencode` | **отклонена** |
| `git diff -- src/mcp.rs .opencode` | **отклонена** |
| `git diff -- .opencode src/mcp.rs` | **отклонена** |

## 3. Пробы `coder` (allow: `cargo check *`, `cargo fmt *`, `cargo clippy *`, `rg *`)

| Команда | Результат |
|---|---|
| `cargo check` | прошла |
| `cargo check --all-targets` | прошла |
| `cargo fmt --check` | прошла |
| `cargo clippy --all-targets -- -D warnings` | прошла |
| `cargo check --all-targets --quiet` | прошла |
| `rg zzz_nonexistent_pattern --files` | прошла (exit 1 — результат rg) |
| `rg zzz_nonexistent_pattern -- docs/analysis` | прошла (exit 1) |
| `rg zzz_nonexistent_pattern ; echo done` | **отклонена** |

## 4. Выводы

1. `*` матчит 0+ токенов: флаги (`--stat`, `--oneline -1`, `--show-toplevel`), пути, токен `--`; ограничения на форму аргументов нет.
2. Составные команды (`;`, `|`) отклоняются **целиком, до матчинга паттерна** — правило «команды одиночные» подтверждено машинно.
3. Единственный воспроизводимый отказ по паттерну: аргумент, **начинающийся с точки** (`.opencode`, `.gitignore`, `.foo`), **после `--`**; без `--` проходит; `docs/.opencode` (точка не в начале) проходит; `./…` (точка+слэш) проходит. Триггер — «`.` + имя» после `--`, не зона `.opencode/**` и не «скрытость» сама по себе.
4. Заявленный в Run 3 отказ `git diff --stat` **объяснён** (§6): фактическая команда содержала префикс `git -C <путь>`, который не матчится паттернами от начала команды; сам `--stat` проходит.
5. `git ls-remote` отклонён по отсутствию паттерна; потребность не подтверждена — allow-листы не расширять.
6. `cargo`-формы с `--` (`-- -D warnings`) проходят; ограничение — только составные команды.

## 5. Рекомендации для канона

- Правило путей: аргументы, начинающиеся с точки (`.opencode/**`, `.gitignore`), в git-командах с `--` указывать с префиксом `./` или абсолютно; без `--` голый путь проходит, каноническая форма — `./…`.
- Команды — одиночные (обоснование — движок отклоняет `;`/`|` до матчинга).
- Не использовать `git -C <путь>`: паттерны матчатся **от начала команды**; рабочая директория задаётся полем `workdir` вызова shell.
- Allow-листы ролей не расширять.
- Ссылка на этот документ — из `.opencode/rules/review.md`.

## 6. Разбор Run 3: откуда взялся «отклонённый `git diff --stat`»

Источники: экспорт сессии Run 3 `ses_f1dcbc9daffeoIODQ9kMKsu2Rj` и дочерней сессии `docs-writer` `ses_f1dc478feffeSt6kHAyO8n3t1O` (27.09.2026; `opencode session export`).

Фактическая отклонённая команда (дословно):

    git -C D:\pyTechNotes\dar\dar7\dar\dar\prototypes\credo2 diff --stat

Причина отказа — **префикс `git -C <путь>`**: паттерны ролей матчатся от начала команды (`git diff *`, `git status *`, …), а команда начинается с `git -C`, поэтому ни один паттерн не подходит. Флаг `--stat` и `--`-пути не при чём (пробы §1–2: `git diff --stat` без `-C` проходит).

Прочие отказы Run 3 (для полноты, все ожидаемые):

- `git ls-remote --heads origin feature/T-03-check-create` — у `git`-роли нет паттерна `ls-remote` (расширение не требуется, §4 п. 5);
- `git branch -vv`, `git reflog -15`, `git stash list` — у `lead` нет паттернов (диагностика аномалии после ручного вмешательства владельца; расширение не требуется).
