# features — сводка функциональных требований CREDO

Каталог содержит функциональные требования к CREDO (прототип `credo2`) в формате
**Gherkin** (`# language: ru`), разложенные **по одному файлу на фичу**.

> **Стратегия реализации (Q1, решено 2026-09-24):** требования верифицируются
> на эволюционирующем `credo2`, а не на отдельном greenfield-проекте. Целевая
> структура монорепозитория — `SPECIFICATION.md` §5; первым шагом является
> рефакторинг `credo2` в workspace без изменения поведения (Фаза 0).
>
> **Решения Q5/Q40 (2026-09-24):** этот файл — единственный канон статусов и
> приоритетов (SPEC §6 не дублирует таблицу). Статус реализации и приоритет
> MVP — две независимые оси (см. ниже). Фичи — документация, а не исполняемая
> спецификация: DoD — `cargo fmt --check`, `cargo clippy -- -D warnings`,
> `cargo test` + ручной прогон UI-сценариев; cucumber-rs — план v0.2.
> Инвентаризация (структура файлов, счётчики) проверяется тестом
> [`../tests/features_inventory.rs`](../tests/features_inventory.rs).

Требования разделены на две части: **backend** CREDO и **frontend** (UI DAR
Notebook). Открытые расхождения с `SPECIFICATION.md` и кодом сведены в
[`../OPEN_QUESTIONS.md`](../OPEN_QUESTIONS.md).

## Как читать

- Один `*.feature` — одна функция (в Gherkin — `Функция:`).
- Языковые конструкции: `Функция` (Feature), `Контекст` (Background),
  `Сценарий` (Scenario), `Структура сценария` (Scenario Outline) с `Примеры`.
- Каждый сценарий — кандидат в автотест (`Дано/Когда/Тогда/И`).
- Статусы и приоритеты файлов живут только здесь; SPEC §6 ссылается сюда,
  план по фазам — SPEC §8.

## Статусы и приоритеты

Две независимые оси: **статус** отвечает на вопрос «что уже работает»,
**приоритет** — «что и когда нужно демо».

**Статус реализации:**

| Метка | Значение |
|---|---|
| ✅ | **Реализовано.** Поведение есть в `credo2`, сценарии не противоречат коду; для backend-зоны есть автотест либо явная пометка «ручной прогон» (см. «Соответствие коду»), UI проверяется вручную по сценарию |
| 🟡 | **Частично.** Ядро реализовано, но часть сценариев расходится с кодом; расхождения перечислены в `OPEN_QUESTIONS.md` |
| ⬜ | **План.** Не реализовано |
| ⏸ | **Отложено.** Вне объёма MVP (v0.2+) |

**Приоритет MVP:**

| Метка | Значение | Срок (SPEC §8) |
|---|---|---|
| 🔴 | **Обязательно** — без этого демо невозможно | Фазы 1–5 |
| 🟡 | **Желательно** — усиливает впечатление | Фазы 5–6 |
| 🟢 | **Вау-эффект** — запоминается, но не критично | Фаза 6+ |
| ⏳ | **Отложено** — не в MVP | v0.2+ |

## Сводная таблица

### Язык DAR (лексер, парсер, исполнение)

| Файл | Категория | Сценариев | Статус | Приоритет | Что покрывает |
|---|---|---:|---|---|---|
| [`lexer.feature`](lexer.feature) | Лексер | 6 | ⬜ | ⏳ | Токены: ключевые слова, идентификаторы, числа, строки, комментарии, операторы |
| [`parser.feature`](parser.feature) | Парсер | 5 | 🟡 | 🔴 | Дерево правила, `Приоритет`, причина, ошибки синтаксиса |
| [`execution.feature`](execution.feature) | Исполнение | 3 | 🟡 | 🔴 | Срабатывание/несрабатывание, операторы сравнения |
| [`explain.feature`](explain.feature) | Объяснимость | 3 | ✅ | 🔴 | Поля объяснения: `rule_name`, `condition`, `actual_value` (латиница, Q42) |
| [`errors.feature`](errors.feature) | Ошибки | 3 | 🟡 | 🔴 | «Неизвестное поле» и «Несовместимые типы» на исполнении (Q8/Q9); «Файл пуст» и ошибки типов на парсере — v0.2 |

> **Решение Q3 (2026-09-24):** MVP использует regex-минимум — это и есть
> язык v0.1, канонизированный в [`../GRAMMAR.md`](../GRAMMAR.md). Лексер и
> полноценный AST — целевое состояние v0.2 (`lexer.feature`, сценарии AST в
> `parser.feature`); LSP поверх стабильного API `parse_rule` без ожидания
> AST (GRAMMAR.md §3). Приоритет `lexer` — ⏳.
>
> **Решение Q4 (2026-09-24):** `Приоритет` вне MVP — сценарии с
> `Приоритет: 100;` в `parser.feature`, `execution.feature`,
> `explain.feature`, `editor.feature`, `lsp.feature`,
> `client_explanation.feature` относятся к целевому состоянию v0.2.
>
> **Решения Q36/Q38 (2026-09-24):** конвейеры/скоринги/таблицы — вне MVP;
> LSP — в MVP в составе diagnostics + completion + hover + symbols +
> semanticTokens (SPEC §3.3), formatting/definition/references — v0.2.

### Черновики, публикация, версионирование

| Файл | Категория | Сценариев | Статус | Приоритет | Что покрывает |
|---|---|---:|---|---|---|
| [`draft.feature`](draft.feature) | Управление черновиками | 9 | 🟡 | 🔴 | `check.create/list_drafts/get_draft/delete_draft`, перезапись; производные от `.dar`, `stale` по `source_hash` |
| [`test_draft.feature`](test_draft.feature) | Тестирование черновиков | 5 | 🟡 | 🔴 | `check.test` на конкретных данных, включая устаревшие черновики |
| [`publish.feature`](publish.feature) | Публикация и версии | 6 | 🟡 | 🔴 | `check.publish`: ветка `publish/{name}-{version}`, артефакт `checks/{name}/{version}/`, метаданные, deprecated, запрет дублей |
| [`publish_rules.feature`](publish_rules.feature) | Правила публикации | 6 | ✅ | 🔴 | Ветка вместо main, downgrade, MAJOR bump при смене контракта |
| [`immutability.feature`](immutability.feature) | Иммутабельность | 2 | ✅ | 🟡 | Повторная публикация версии отклоняется |
| [`storage_paths.feature`](storage_paths.feature) | Хранилище версий | 4 | ✅ | 🔴 | Версия = путь `checks/{name}/{version}/` (`rule.json`, `contract.json`, `meta.json`) |

### REST API

| Файл | Категория | Сценариев | Статус | Приоритет | Что покрывает |
|---|---|---:|---|---|---|
| [`evaluate.feature`](evaluate.feature) | REST-исполнение | 7 | 🟡 | 🔴 | `POST .../versions/{version}/evaluate`, ключ, 404, манифест (Q21), отказ deprecated, Swagger |
| [`rest_api.feature`](rest_api.feature) | REST API | 11 | 🟡 | 🔴 | `/version`, `/checks`, версии, active/evaluate, отказ deprecated (410), «активация недоступна» (409), OpenAPI, кэш |
| [`rest_auth.feature`](rest_auth.feature) | Аутентификация REST | 6 | ✅ | 🔴 | API-key, 401 (текст Q22), `/health`, `/docs`, `/openapi.json` без ключа, режим демо |

> **Решения Q11/Q20/Q21 (2026-09-25):** канон пути —
> `/checks/{name}/versions/{version}/...`. MVP-минимум REST:
> `POST /checks/{name}/versions/{version}/evaluate`, `GET /checks`
> (манифест: `schema_version`, `count`, `service_hash`, `checks[]` с
> `name`/`active`/`supported`/`deprecated`) и инфраструктурные
> `GET /health`, `GET /version`, `GET /openapi.json`. Тексты ошибок —
> русские: 404 «проверка не найдена: {name}», 410 «версия выведена из
> эксплуатации», 409 «активация недоступна» (401 — Q22). Эндпоинты
> active-evaluate и GET-детали — вне MVP-минимума (реализованы в
> прототипе; судьба — пост-MVP, в MVP не удаляются и не развиваются).

### Манифест и жизненный цикл

| Файл | Категория | Сценариев | Статус | Приоритет | Что покрывает |
|---|---|---:|---|---|---|
| [`manifest.feature`](manifest.feature) | Манифест сервиса | 5 | ✅ | 🔴 | active/supported/deprecated, `service_hash`, `manifest.json` |
| [`manifest_sync.feature`](manifest_sync.feature) | Синхронизация манифеста | 5 | ✅ | 🟡 | Watcher, перечитывание при изменении main, ошибки |
| [`deprecation.feature`](deprecation.feature) | Deprecation версии | 3 | ✅ | 🟡 | `check.deprecate`, пересборка манифеста, сохранность данных |
| [`semver.feature`](semver.feature) | Строгий semver | 4 | ✅ | 🔴 | Парсинг/нормализация, сравнение, pre-release |

### Объяснимость, MCP, качество

| Файл | Категория | Сценариев | Статус | Приоритет | Что покрывает |
|---|---|---:|---|---|---|
| [`explain_full.feature`](explain_full.feature) | Объяснимость | 3 | ✅ | 🔴 | Полная схема: `rule_name`, `condition`, `actual_value`, `matched`, `decision`, `reason`; ошибка при отсутствии поля |
| [`mcp_tools.feature`](mcp_tools.feature) | MCP инструменты | 4 | ✅ | 🔴 | `check.publish/deprecate/rebuild_manifest/list_published` |
| [`testing.feature`](testing.feature) | Тестирование | 5 | ✅ | 🔴 | Юнит- и интеграционные тесты, CI, инвентаризация фич |

### Интерфейс DAR Notebook (frontend, план)

> Эти требования описывают **UI-часть** продукта (DAR Notebook) и пока не
> реализованы в `credo2` — все помечены ⬜. Они работают поверх backend-фич
> (`check.*`, REST, манифест) через MCP-сервер.

| Файл | Категория | Сценариев | Статус | Приоритет | Что покрывает |
|---|---|---:|---|---|---|
| [`notebook_ui.feature`](notebook_ui.feature) | Основной интерфейс | 5 | ⬜ | 🔴 | Трёхколоночный layout, открытие/создание workspace, командная палитра |
| [`editor.feature`](editor.feature) | Редактор `.dar` | 7 | ⬜ | 🔴 | Подсветка, автодополнение, диагностика, табы, сохранение |
| [`inline_execution.feature`](inline_execution.feature) | Инлайн-исполнение | 7 | ⬜ | 🔴 | Кнопка «Выполнить», ввод JSON, блоки результатов, несколько тестов |
| [`git_integration.feature`](git_integration.feature) | Git в UI | 7 | ⬜ | 🔴 | Статус, визуальный diff, commit, публикация, слияние `credo merge`, история версий |
| [`agent_minimal.feature`](agent_minimal.feature) | Чат с агентом | 7 | ⬜ | 🔴 | MCP через stdio, `check.create/test/publish`, отображение вызовов |
| [`file_management.feature`](file_management.feature) | Управление файлами | 6 | ⬜ | 🟡 | Создание/переименование/удаление, drag-and-drop, поиск |
| [`graph_view.feature`](graph_view.feature) | Граф связей | 4 | ⬜ | ⏳ | Визуализация зависимостей правил конвейера |
| [`lsp.feature`](lsp.feature) | Language Server Protocol | 15 | ⬜ | 🔴 | Initialize, диагностика, completion, hover, definition, formatting, токены |
| [`lsp_notebook.feature`](lsp_notebook.feature) | LSP в Notebook (уточнения) | 6 | ⬜ | 🔴 | Sidecar-процесс, диагностика/completion в CodeMirror, перезапуск |

### Аналитика и интеграции (план / вау)

| Файл | Категория | Сценариев | Статус | Приоритет | Что покрывает |
|---|---|---:|---|---|---|
| [`dashboard.feature`](dashboard.feature) | Обзор проверок | 3 | 🟡 | 🟡 | Активные/deprecated и состав версий из манифеста (`active`/`supported`/`deprecated`) |
| [`batch.feature`](batch.feature) | Массовый прогон | 3 | ⬜ | 🟡 | Прогон набора заявок, агрегация, частичные ошибки |
| [`client_explanation.feature`](client_explanation.feature) | Объяснение для клиента | 3 | ⬜ | 🟢 | Человекочитаемый текст отказа/одобрения |
| [`wasm.feature`](wasm.feature) | WASM в браузере | 4 | ⬜ | 🟢 | Локальное исполнение правил в браузере |

### Отложено

| Файл | Категория | Сценариев | Статус | Приоритет | Что покрывает |
|---|---|---:|---|---|---|
| [`deferred.feature`](deferred.feature) | Явно отложенные требования | 6 | ⏸ | ⏳ | PR через GitHub API, внешний кэш, OAuth/mTLS, multi-tenant/region |
| [`import_export.feature`](import_export.feature) | Импорт/экспорт `.dar` | 5 | ⏸ | ⏳ | Экспорт версии/черновика, импорт файла — пост-MVP (Q26) |

**Итого: 36 файлов, 193 сценария** (backend — 27 файлов / 129 сценариев,
frontend DAR Notebook — 9 файлов / 64 сценария).

## Соответствие коду

> **Q13 (решено 2026-09-25):** артефакт публикации — каталог
> `checks/{name}/{version}/` (`rule.json`, `contract.json`, `meta.json`);
> `name` — машиночитаемый латинский идентификатор (например, `CreditAgeMin`),
> человекочитаемое имя — `display_name` в `meta.json`. Исходный `.dar` в
> реестр не попадает, связь — через `source_hash`.

| Зона | Где в коде | Тесты / проверка |
|---|---|---|
| Домен, DSL, semver | [`../src/core.rs`](../src/core.rs) | юнит-тесты в `core.rs` |
| Git-хранилище, публикация, манифест, состояние | [`../src/lib.rs`](../src/lib.rs) | юнит-тесты в `lib.rs`, [`../tests/publish.rs`](../tests/publish.rs) |
| MCP-инструменты | [`../src/mcp.rs`](../src/mcp.rs) | ручной прогон |
| REST + OpenAPI + аутентификация | [`../src/rest.rs`](../src/rest.rs) | ручной прогон |
| CLI | [`../src/main.rs`](../src/main.rs) | — |
| Инвентаризация фич | [`README.md`](README.md) и `features/*.feature` | [`../tests/features_inventory.rs`](../tests/features_inventory.rs) |
