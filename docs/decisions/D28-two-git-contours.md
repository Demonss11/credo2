# D28: Два git-контура — workspace-репозиторий и реестр публикаций; мультиверсионность

- **Статус:** accepted
- **Дата:** 2026-09-26
- **Resolves:** [Q32](../questions/Q32.md)
- **Спека:** [`SPECIFICATION.md`](../SPECIFICATION.md) §10, решение №28
- **Affects:** [`SPECIFICATION.md`](../SPECIFICATION.md) §4.1, §4.2, §7, §10;
  [`../features/storage_paths.feature`](../features/storage_paths.feature),
  [`../features/git_integration.feature`](../features/git_integration.feature),
  [`../features/agent_minimal.feature`](../features/agent_minimal.feature),
  [`../features/publish.feature`](../features/publish.feature),
  [`../features/publish_rules.feature`](../features/publish_rules.feature),
  [`../features/immutability.feature`](../features/immutability.feature),
  [`../features/deprecation.feature`](../features/deprecation.feature),
  [`../features/mcp_tools.feature`](../features/mcp_tools.feature)
  (целевое состояние; шапки/статусы — зона `docs-writer`);
  [`../features/README.md`](../features/README.md);
  [`../../src/lib.rs`](../../src/lib.rs) (путь реестра — расхождение,
  [T-06](../tasks/T-06-registry-path-xyz/README.md))
- **Tasks:** [T-06](../tasks/T-06-registry-path-xyz/README.md) (открыта; см.
  «Сверка с кодом»)

## Контекст

SPEC §4.1 описывал у Notebook локальный `git CLI` и «локальный репозиторий»;
§4.2 — у сервера свой bare-репозиторий публикаций. `git_integration.feature`
показывает статус ветки, diff, commit и «Опубликовать» — было непонятно, к
какому репозиторию относятся операции UI. Полный контекст —
[Q32](../questions/Q32.md).

## Решение

1. **Три уровня источника правды:**
   - грязный буфер редактора → черновик (draft-first, `check.create`) →
     локальный прогон `check.test`; единый механизм —
     [Q33](../questions/Q33.md) ([D30](D30-execution-mechanism.md));
   - файл на диске (`rules/*.dar`) → для git-операций (`commit` / `publish`);
   - опубликованная версия → для исполнения «как в проде» (явный выбор версии).
   По умолчанию: грязный буфер → черновик; иначе → файл на диске.
2. **Панель git в Notebook работает только с workspace-репозиторием.**
   Опубликованный bare-репозиторий — внутренняя деталь сервера; пользователь
   видит его содержимое через панель «Версии» (read-only, источник — манифест /
   `check.list_published`).
3. **Переключение версий** — выпадающий список в тулбаре редактора, группировка
   по major.minor (1.x, 2.x). Workspace при переключении не затирается.
4. **Структура реестра публикаций** (уточнение к [Q13](../questions/Q13.md)):
   `checks/{name}/{X}/{Y}/{Z}/`, где X = major, Y = minor, Z = patch (без
   избыточности). Внутри: `rule.json` + `contract.json` + `meta.json`.
5. **REST** — без изменений: канон [Q20](../questions/Q20.md)
   ([D22](D22-rest-paths-canon.md)) `/checks/{name}/versions/{version}/evaluate`;
   сервер внутренне маппит `{version}` → `{X}/{Y}/{Z}/` для доступа к файлам
   реестра.
6. **Публикация только с новым номером версии:** существующую версию
   переопубликовать нельзя (отказ с предложением следующего номера); downgrade
   запрещён.
7. **Сопровождение нескольких версий** (хотфикс 1.0.x при живой разработке
   2.0.0): рабочая ветка создаётся от линии 1.0.x, правка, публикация с
   bumped-номером (1.0.1); линия 2.0.0 не затрагивается.
8. **Pre-release версии** (1.0.0-rc.1) — вне MVP, возвращаются в v0.2
   ([Q18](../questions/Q18.md) ([D35](D35-semver-v01.md)): сравнение — числовое
   по семантике semver; публикация — по решению Q32 остаётся вне MVP). Когда
   дойдём — вариант А: `checks/{name}/{X}/{Y}/{Z-prerelease}/`.

**Примечание:** п. 5 не отменяет и не изменяет канон [Q20](../questions/Q20.md)
(путь с `/versions/`). «Короткий формат» означает передачу версии одной строкой
вместо трёх сегментов пути, а не изменение URL-шаблона.

**Следствие для кода:** `lib.rs` строит путь `checks/{name}/{version}`;
требование Q13/Q32 — `checks/{name}/{X}/{Y}/{Z}`. Правка кода — отдельная
задача [T-06](../tasks/T-06-registry-path-xyz/README.md); в этих правках код не
менялся, требование зафиксировано в фичах, статусы затронутых файлов понижены до
🟡 (`features/README.md`).

## Следствия

- Разделены зоны ответственности двух git-контуров: git-панель UI — workspace-
  репозиторий; реестр публикаций (`published-repo`) — внутренняя деталь сервера,
  видимая read-only через «Версии» (манифест / `check.list_published`).
- Workspace не затирается при просмотре/выборе версий; хотфикс-линии позволяют
  сопровождать старые версии, не останавливая разработку.
- Публикация неизменяема: только новый номер, без переопубликации и downgrade
  (согласовано с [Q17](../questions/Q17.md)/[D57](D57-bare-git-immutability.md),
  [Q16](../questions/Q16.md)/[D32](D32-test-gate-mvp.md)).
- Структура реестра `checks/{name}/{X}/{Y}/{Z}/` — предпосылка задачи
  [T-06](../tasks/T-06-registry-path-xyz/README.md); REST-шаблон `{version}`
  сохраняется, маппинг в `{X}/{Y}/{Z}` — на сервере (канон
  [Q20](../questions/Q20.md)/[D22](D22-rest-paths-canon.md)).
- Документы приводятся к канону (правки — зона `docs-writer`):
  `storage_paths.feature`, `git_integration.feature`, `agent_minimal.feature`,
  `publish.feature`, `publish_rules.feature`, `immutability.feature`,
  `deprecation.feature`, `mcp_tools.feature`; `features/README.md`;
  `SPECIFICATION.md` §4.1/§4.2/§7/§10.

## Сверка с кодом

Вердикт: 🟡 **расхождение** — два git-контура, неизменяемость и разрешение версий
реализованы; путь реестра в коде плоский (`checks/{name}/{version}`) вместо
канонического `checks/{name}/{X}/{Y}/{Z}/` — расхождение в периметре MVP покрыто
задачей [T-06](../tasks/T-06-registry-path-xyz/README.md). UI-части решения
(панель workspace-only, read-only «Версии», переключатель версий, хотфикс-линия)
— целевое состояние Notebook, вне кода `credo2`.

Что проверено (чтением кода, тестов и SPEC, 29.09.2026), чем подтверждено:

- **Путь реестра — 🟡 расхождение:** [`src/lib.rs`](../../src/lib.rs) строит
  `format!("checks/{}/{}", rule.name, vstr)` (`:342`), `debug_assert` того же
  пути (`:383`), `deprecate` — `checks/{name}/{vstr}/meta.json` (`:462`);
  комментарий структуры — `// checks/Name/1.0.0` (`:317`); чтение
  `list_from_ref` разбирает `checks/`-префикс и ожидает три сегмента
  (`:241–248`) — плоская модель, а не `X/Y/Z`. Тесты пути отсутствуют:
  [`tests/publish.rs`](../../tests/publish.rs) проверяет версии/дубликаты по
  полям, не по пути; [`tests/rest.rs`](../../tests/rest.rs) — только REST-шаблон
  `/checks/{name}/versions/{version}/...`. → задача
  [T-06](../tasks/T-06-registry-path-xyz/README.md) (P2).
- **REST-шаблон `{version}` — ✅ (канон Q20 не меняется):**
  [`src/rest.rs`](../../src/rest.rs) маршруты `/checks/{name}/versions`,
  `/checks/{name}/versions/{version}`,
  `/checks/{name}/versions/{version}/evaluate` (`:25–29`); маппинг `{version}` →
  `{X}/{Y}/{Z}/` — часть пути реестра, покрыт
  [T-06](../tasks/T-06-registry-path-xyz/README.md).
- **Два git-контура — ✅ (в объёме `credo2`):** серверный контур — bare-git
  `published-repo` ([`src/lib.rs`](../../src/lib.rs) `ensure_repo`, `:207–229`),
  черновики — `.credo/sandbox.json` ([D58](D58-workspace-data-dirs.md));
  workspace-репозиторий — сторона Notebook, вне `credo2`. MCP-поверхность
  публикаций: [`src/mcp.rs`](../../src/mcp.rs) `check.publish` (`:282–340`),
  `check.deprecate` (`:344–397`), `check.list_published` (`:399–411`),
  `check.rebuild_manifest` (`:413+`).
- **Публикация только с новым номером, без downgrade — ✅:**
  [`src/lib.rs`](../../src/lib.rs) `publish` (`:321–409`): отказ на дубль версии
  (`:346–351`), запрет downgrade (`:353–360`); тесты `tests/publish.rs`
  (`rejects_duplicate_version`, `rejects_downgrade`).
- **Три уровня источника правды (draft-first) — ✅ (согласовано):** черновик —
  `check.create` ([D54](D54-source-of-truth-flow.md)); локальный прогон —
  `check.test` (единый механизм [D30](D30-execution-mechanism.md)); публикация
  читается из `main` (`list_from_ref`, `:233+`).
- **UI-части — вне кода `credo2`:** `src/` содержит только
  `main`/`mcp`/`rest`/`core`/`lib`; интерфейса нет — панель workspace-only,
  read-only «Версии», переключатель версий и хотфикс-линия относятся к целевому
  состоянию Notebook, а не к расхождению кода прототипа.

`cargo` не запускался (§5.3, [D50](D50-dod-by-package-scope.md)): сверено
чтением кода и тестов; адресный прогон не требуется (правок код не порождает).

**Задач новых не требуется:** расхождение пути реестра в периметре MVP уже
покрыто задачей [T-06](../tasks/T-06-registry-path-xyz/README.md) (открыта);
UI-части — целевое состояние Notebook, вне периметра `credo2`.

## Альтернативы

- **UI работает с обоими репозиториями в одной панели** — смешивает рабочий
  контур и реестр публикаций, позволяя Commit'ить в `published-repo`; отклонено.
- **`published-repo` — публичный/общий сервер** — требует развёртывания и общего
  состояния; совместная работа вне MVP (согласуется с
  [D29](D29-notebook-mcp-transport.md)); отклонено (v0.2+).
- **Плоский путь `checks/{name}/{version}`** — не даёт файловой раскладки по
  semver и естественной структуры мультиверсионности; отклонено в пользу
  `{X}/{Y}/{Z}`.
- **Переопубликация/перезапись версии** — противоречит иммутабельности
  ([Q17](../questions/Q17.md)/[D57](D57-bare-git-immutability.md)); отклонено.
- **Pre-release в MVP** — отложено в v0.2 (вариант пути
  `checks/{name}/{X}/{Y}/{Z-prerelease}/`).

## Ссылки

- Вопрос: [Q32](../questions/Q32.md)
- Связанные: [Q13](../questions/Q13.md) ([D14](D14-published-artifact-canon.md)
  — артефакт и путь), [Q15](../questions/Q15.md)
  ([D56](D56-merge-step.md) — merge в `main`), [Q16](../questions/Q16.md)
  ([D32](D32-test-gate-mvp.md) — тест-гейт), [Q17](../questions/Q17.md)
  ([D57](D57-bare-git-immutability.md) — иммутабельность),
  [Q18](../questions/Q18.md) ([D35](D35-semver-v01.md) — pre-release и semver),
  [Q20](../questions/Q20.md) ([D22](D22-rest-paths-canon.md) — канон REST),
  [Q22](../questions/Q22.md) ([D26](D26-rest-auth-x-api-key.md));
  [Q33](../questions/Q33.md) ([D30](D30-execution-mechanism.md) — единый механизм
  исполнения); [D58](D58-workspace-data-dirs.md) (каталоги данных)
- Задача: [T-06](../tasks/T-06-registry-path-xyz/README.md) (открыта)
- Краткий канон: [`SPECIFICATION.md`](../SPECIFICATION.md) §10, решение №28;
  §4.1, §4.2, §7
- Связи: [`TRACEABILITY.md`](../TRACEABILITY.md)
