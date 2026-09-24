# features — сводка функциональных требований CREDO

Каталог содержит функциональные требования к CREDO (прототип `credo2`) в формате
**Gherkin** (`# language: ru`), разложенные **по одному файлу на фичу**.

> **Стратегия реализации (Q1, решено 2026-09-24):** требования верифицируются
> на эволюционирующем `credo2`, а не на отдельном greenfield-проекте. Целевая
> структура монорепозитория — `SPECIFICATION.md` §5; первым шагом является
> рефакторинг `credo2` в workspace без изменения поведения (Фаза 0).

Требования разделены на две части: **backend** CREDO и **frontend** (UI DAR
Notebook). Открытые расхождения с `SPECIFICATION.md` и кодом сведены в
[`../OPEN_QUESTIONS.md`](../OPEN_QUESTIONS.md).

## Как читать

- Один `*.feature` — одна функция (в Gherkin — `Функция:`).
- Языковые конструкции: `Функция` (Feature), `Контекст` (Background),
  `Сценарий` (Scenario), `Структура сценария` (Scenario Outline) с `Примеры`.
- Каждый сценарий — кандидат в автотест (`Дано/Когда/Тогда/И`).

## Статус

| Метка | Значение |
|---|---|
| ✅ | Поведение реализовано в `credo2` (возможны отличия в деталях) |
| 🟡 | Реализовано ядро, но формулировки сценариев расходятся с текущим кодом |
| ⬜ | Не реализовано — план |
| ⏸ | Явно отложено (вне текущего объёма) |

## Сводная таблица

### Язык DAR (лексер, парсер, исполнение)

| Файл | Категория | Сценариев | Статус | Что покрывает |
|---|---|---:|---|---|
| [`lexer.feature`](lexer.feature) | Лексер | 6 | ⬜ | Токены: ключевые слова, идентификаторы, числа, строки, комментарии, операторы |
| [`parser.feature`](parser.feature) | Парсер | 5 | 🟡 | Дерево правила, `Приоритет`, причина, ошибки синтаксиса |
| [`execution.feature`](execution.feature) | Исполнение | 3 | 🟡 | Срабатывание/несрабатывание, операторы сравнения |
| [`explain.feature`](explain.feature) | Объяснимость | 4 | 🟡 | Поля объяснения: правило, приоритет, условие, значение |
| [`errors.feature`](errors.feature) | Ошибки | 3 | 🟡 | Понятные сообщения для риск-технолога |

> `credo2` использует минимальный DSL на регулярных выражениях (`core::parse_rule`):
> только одно сравнение, без `Приоритет`, без типизированных ошибок. Поэтому
> языковые фичи — это целевое состояние DAR, а не текущая реализация.

### Черновики, публикация, версионирование

| Файл | Категория | Сценариев | Статус | Что покрывает |
|---|---|---:|---|---|
| [`draft.feature`](draft.feature) | Управление черновиками | 6 | 🟡 | `check.create/list_drafts/get_draft/delete_draft`, перезапись |
| [`test_draft.feature`](test_draft.feature) | Тестирование черновиков | 4 | 🟡 | `check.test` на конкретных данных |
| [`publish.feature`](publish.feature) | Публикация и версии | 6 | 🟡 | `check.publish`, метаданные, deprecated, запрет дублей |
| [`publish_rules.feature`](publish_rules.feature) | Правила публикации | 6 | ✅ | Ветка вместо main, downgrade, MAJOR bump при смене контракта |
| [`immutability.feature`](immutability.feature) | Иммутабельность | 2 | ✅ | Повторная публикация версии отклоняется |
| [`storage_paths.feature`](storage_paths.feature) | Хранилище версий | 4 | ✅ | Версия = путь `checks/{name}/{version}/`, `meta.json` |

### REST API

| Файл | Категория | Сценариев | Статус | Что покрывает |
|---|---|---:|---|---|
| [`evaluate.feature`](evaluate.feature) | REST-исполнение | 7 | 🟡 | `POST .../evaluate`, ключ, 404, манифест, Swagger |
| [`rest_api.feature`](rest_api.feature) | REST API | 10 | ✅ | `/version`, `/checks`, версии, active/evaluate, OpenAPI, кэш |
| [`rest_auth.feature`](rest_auth.feature) | Аутентификация REST | 5 | ✅ | API-key, 401, `/health` и `/docs` без ключа |

### Манифест и жизненный цикл

| Файл | Категория | Сценариев | Статус | Что покрывает |
|---|---|---:|---|---|
| [`manifest.feature`](manifest.feature) | Манифест сервиса | 5 | ✅ | active/supported/deprecated, `service_hash`, `manifest.json` |
| [`manifest_sync.feature`](manifest_sync.feature) | Синхронизация манифеста | 5 | ✅ | Watcher, перечитывание при изменении main, ошибки |
| [`deprecation.feature`](deprecation.feature) | Deprecation версии | 3 | ✅ | `check.deprecate`, пересборка манифеста, сохранность данных |
| [`semver.feature`](semver.feature) | Строгий semver | 4 | ✅ | Парсинг/нормализация, сравнение, pre-release |

### Объяснимость, MCP, качество

| Файл | Категория | Сценариев | Статус | Что покрывает |
|---|---|---:|---|---|
| [`explain_full.feature`](explain_full.feature) | Объяснимость | 3 | 🟡 | `rule_name`, `condition`, `actual_value`, `matched`, `decision`, `reason`, `priority` |
| [`mcp_tools.feature`](mcp_tools.feature) | MCP инструменты | 4 | ✅ | `check.publish/deprecate/rebuild_manifest/list_published` |
| [`testing.feature`](testing.feature) | Тестирование | 3 | ✅ | Юнит- и интеграционные тесты, CI |

### Интерфейс DAR Notebook (frontend, план)

> Эти требования описывают **UI-часть** продукта (DAR Notebook) и пока не
> реализованы в `credo2` — все помечены ⬜. Они работают поверх backend-фич
> (`check.*`, REST, манифест) через MCP-сервер.

| Файл | Категория | Сценариев | Статус | Что покрывает |
|---|---|---:|---|---|
| [`notebook_ui.feature`](notebook_ui.feature) | Основной интерфейс | 5 | ⬜ | Трёхколоночный layout, открытие/создание workspace, командная палитра |
| [`editor.feature`](editor.feature) | Редактор `.dar` | 7 | ⬜ | Подсветка, автодополнение, диагностика, табы, сохранение |
| [`inline_execution.feature`](inline_execution.feature) | Инлайн-исполнение | 7 | ⬜ | Кнопка «Выполнить», ввод JSON, блоки результатов, несколько тестов |
| [`git_integration.feature`](git_integration.feature) | Git в UI | 6 | ⬜ | Статус, визуальный diff, commit, публикация, история версий |
| [`agent_minimal.feature`](agent_minimal.feature) | Чат с агентом | 7 | ⬜ | MCP через stdio, `check.create/test/publish`, отображение вызовов |
| [`file_management.feature`](file_management.feature) | Управление файлами | 6 | ⬜ | Создание/переименование/удаление, drag-and-drop, поиск |
| [`graph_view.feature`](graph_view.feature) | Граф связей | 4 | ⬜ | Визуализация зависимостей правил конвейера |
| [`lsp.feature`](lsp.feature) | Language Server Protocol | 15 | ⬜ | Initialize, диагностика, completion, hover, definition, formatting, токены |
| [`lsp_notebook.feature`](lsp_notebook.feature) | LSP в Notebook (уточнения) | 6 | ⬜ | Sidecar-процесс, диагностика/completion в CodeMirror, перезапуск |

### Аналитика и интеграции (план / вау)

| Файл | Категория | Сценариев | Статус | Что покрывает |
|---|---|---:|---|---|
| [`dashboard.feature`](dashboard.feature) | Обзор проверок | 3 | 🟡 | Активные/deprecated, история версий, `latest` |
| [`batch.feature`](batch.feature) | Массовый прогон | 3 | ⬜ | Прогон набора заявок, агрегация, частичные ошибки |
| [`client_explanation.feature`](client_explanation.feature) | Объяснение для клиента | 3 | ⬜ | Человекочитаемый текст отказа/одобрения |
| [`import_export.feature`](import_export.feature) | Импорт/экспорт `.dar` | 5 | ⬜ | Экспорт версии/черновика, импорт файла |
| [`wasm.feature`](wasm.feature) | WASM в браузере | 4 | ⬜ | Локальное исполнение правил в браузере |

### Отложено

| Файл | Категория | Сценариев | Статус | Что покрывает |
|---|---|---:|---|---|
| [`deferred.feature`](deferred.feature) | Явно отложенные требования | 6 | ⏸ | PR через GitHub API, внешний кэш, OAuth/mTLS, multi-tenant/region |

**Итого: 36 файлов, 185 сценариев** (backend — 27 файлов / 122 сценария,
frontend DAR Notebook — 9 файлов / 63 сценария).

## Соответствие коду

| Зона | Где в коде |
|---|---|
| Домен, DSL, semver | [`../src/core.rs`](../src/core.rs) |
| Git-хранилище, публикация, манифест, состояние | [`../src/lib.rs`](../src/lib.rs) |
| MCP-инструменты | [`../src/mcp.rs`](../src/mcp.rs) |
| REST + OpenAPI + аутентификация | [`../src/rest.rs`](../src/rest.rs) |
| CLI | [`../src/main.rs`](../src/main.rs) |
| Интеграционные тесты публикации | [`../tests/publish.rs`](../tests/publish.rs) |


