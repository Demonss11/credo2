# CREDO2 — прототип DAR

Минимальный язык банковских правил (`.dar`) и MCP-сервер: черновики в
песочнице, публикация версий в git, опциональный REST.

## Что это

**CREDO** — прототип DAR (декларативный DSL кредитного конвейера): один
`.dar`-файл — одно правило «условие → решение (+ причина)». Спецификация —
[`docs/SPECIFICATION.md`](docs/SPECIFICATION.md); язык v0.1 —
[`docs/GRAMMAR.md`](docs/GRAMMAR.md).

## Быстрый старт

```sh
cargo build --release          # → target/release/credo2.exe
opencode mcp list              # ожидаем: ✓ credo connected
```

MCP-сервер `credo` настроен в `opencode.json`; рабочая директория — корень
репозитория, данные — `.credo/` (вне git). Полная сборка, тесты, DoD,
переподключение MCP и запуск REST — [`AGENTS.md`](AGENTS.md) §«Сборка, тесты и
пересборка».

## Структура

```text
src/        крейт: main/lib (публикация), mcp, rest, core (язык и исполнение)
tests/      интеграционные тесты
docs/       документация: SPEC, GRAMMAR, журнал, требования, задачи
.opencode/  рабочая группа агентов, правила, память и почта (служебная зона)
.credo/     данные CREDO: песочница и bare-git публикаций (вне git)
```

## Документация

- [`docs/README.md`](docs/README.md) — карта документации и канонов;
- [`docs/questions/`](docs/questions/) + [`docs/decisions/`](docs/decisions/) —
  журнал «вопрос → решение» (правила — [`.opencode/rules/journal.md`](.opencode/rules/journal.md));
- [`docs/features/README.md`](docs/features/README.md) — требования, статусы,
  приоритеты, счётчики сценариев;
- [`docs/tasks/README.md`](docs/tasks/README.md) — задачи по коду.
