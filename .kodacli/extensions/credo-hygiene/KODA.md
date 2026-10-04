# KODA.md — Koda CLI: специфика окружения CREDO

Дополнение к `AGENTS.md` для сессий **Koda CLI**. Общие правила проекта,
роли и канон — в `AGENTS.md`; здесь только отличия среды Koda.

## Что НЕ работает в Koda

- **Плагины OpenCode** (`.opencode/plugins/*.ts`: `token-guard`, `wave0-guard`,
  `wave0-observe`) не загружаются: у Koda CLI нет исполняемого plugin API.
  Не рассчитывай на:
  - срез вывода инструментов `token-guard` (B1) и снятие tool-схем (B2);
  - контекстные deny-правила `wave0-guard` и журнал `target/wave0-guard.jsonl`;
  - поток событий субагентов `wave0-observe` и сводки родителю.
- **Per-agent permissions** (`.opencode/agents/*.md` → `permissions`) — модель
  прав OpenCode; в Koda роль-агента так не ограничивается.
- **Code Mode** (`execute`, `tools.credo.*`) — конструкция OpenCode. В Koda
  MCP-инструменты вызываются напрямую (см. ниже).

## Инструменты CREDO в Koda

MCP-серверы `credo` и `rust-analyzer` подключены через `.kodacli/settings.json`.
Инструменты доступны напрямую под именами вида `<сервер>__<инструмент>`:

| Инструмент Koda | Назначение |
|---|---|
| `credo__check_create({ source })` | создать/перезаписать черновик |
| `credo__check_list_drafts()` | список черновиков |
| `credo__check_get_draft({ name })` | показать черновик |
| `credo__check_test({ name, input })` | исполнить черновик |
| `credo__check_delete_draft({ name })` | удалить черновик |
| `credo__check_publish({ name, version, published_by? })` | опубликовать версию |
| `credo__check_deprecate({ name, version, reason })` | пометить deprecated |
| `credo__check_list_published()` | список из `main` |
| `credo__check_rebuild_manifest()` | пересобрать манифест |

Семантика и подводные камни инструментов — `AGENTS.md` §«Как вызывать
инструменты CREDO» (пример полного цикла, ветки публикаций, semver, Q8/Q9).

## Расширение `credo-hygiene`

Расширение `.kodacli/extensions/credo-hygiene` даёт Koda то, что выражается
декларативно:

- **`excludeTools`** — жёсткие запреты, заменяющие часть `wave0-guard`:
  `git push --force`, `git push -f`, `git reset --hard`, `rm -rf`,
  `Remove-Item -Recurse -Force`, чтение `.env`. Запрет действует на всю
  сессию (не per-agent, в отличие от OpenCode).
- **Слеш-команды** `/git:checkpoint` и `/git:status` — порт
  `.opencode/commands/git/*`. Привязки `agent: git` нет: команда подставляет
  промпт в текущую сессию.

## Гигиена контекста

Плагина `token-guard` нет, но Koda **сама** управляет контекстом:

- сжатие истории срабатывает на пороге
  `chatCompression.contextPercentageThreshold` (по умолчанию 50% лимита);
- при сжатии старые выводы инструментов обрезаются до последних 30 строк.

Если вывод может быть срезан — сужай команды (`rg -m`, `offset/limit`,
фильтры) и дочитывай точечно; факт среза отмечай в отчёте.

## Субагенты

Инструмент Koda — `run_subagent` (не `subagent`/`task`). По умолчанию
субагент только читает; пишущие инструменты включаются настройкой `subagents`
в `.kodacli/settings.json`. Роли-агенты OpenCode в Koda недоступны — роль
задаётся параметром `agent` (по имени) либо инструкциями контекста.
