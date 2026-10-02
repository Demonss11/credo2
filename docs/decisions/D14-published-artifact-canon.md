# D14: Артефакт публикации — неизменяемая JSON-тройка; «один check = один каталог версии»

- **Статус:** accepted
- **Дата:** 2026-09-25
- **Resolves:** [Q13](../questions/Q13.md)
- **Спека:** [`SPECIFICATION.md`](../SPECIFICATION.md) §10, решение №14
  (структура пути уточнена в решении №28)
- **Affects:** [`SPECIFICATION.md`](../SPECIFICATION.md) §10 (решения №14, №28),
  §11 (статьи `check`, `name`, `Контракт`, `Метаданные версии`);
  [`../features/publish.feature`](../features/publish.feature),
  [`../features/storage_paths.feature`](../features/storage_paths.feature),
  [`../features/publish_rules.feature`](../features/publish_rules.feature),
  [`../features/immutability.feature`](../features/immutability.feature),
  [`../features/mcp_tools.feature`](../features/mcp_tools.feature)
  (шапочные пометки внесены `docs-writer`);
  [`../../src/core.rs`](../../src/core.rs) (`CheckMeta`),
  [`../../src/lib.rs`](../../src/lib.rs) (путь и запись артефакта)
- **Tasks:** [T-06](../tasks/T-06-registry-path-xyz/README.md),
  [T-07](../tasks/T-07-meta-fields/README.md) (см. «Сверка с кодом»)

## Контекст

Четыре документа описывали артефакт публикации по-разному: `publish.feature` —
файл «Правило … 1.0.0», `storage_paths.feature` — `checks/{name}/{version}/`,
`publish_rules.feature` — `checks/CreditAgeMin/2.0.0/`, SPEC §10 п. 14 — «одно
правило = один файл»; код писал JSON-тройку. Нужен был единый канон: что именно
попадает в реестр и по какому пути. Полный контекст —
[Q13](../questions/Q13.md).

## Решение

1. **Артефакт публикации — неизменяемый скомпилированный артефакт**, а не
   исходный DSL-текст. Каталог версии `checks/{name}/{version}/` содержит:
   - `rule.json` — исполняемое представление правила (AST или иной
     машинно-читаемый формат);
   - `contract.json` — контракт проверки: входные поля с типами и формат
     результата;
   - `meta.json` — метаданные издания: `name`, `version`, `display_name`,
     `published_at`, `published_by`, `checksum`, `source_hash`,
     `compiler_version`.
2. **Исходный `.dar`** остаётся источником истины в рабочем контуре (см.
   [D54](D54-source-of-truth-flow.md)); в v0.1 в реестр публикаций не попадает.
   Добавление `source.dar` в каталог версии — v0.2; связь артефакта с
   исходником обеспечивает `source_hash`.
3. **`name`** — машиночитаемый латинский идентификатор (например,
   `CreditAgeMin`); человекочитаемое имя — `display_name` в `meta.json`.
4. **Каталог версии неизменяем:** изменение правила требует новой версии.
   Формулировка «одно правило = один файл» устарела; канон — «один check =
   один каталог версии».
5. **Структура каталога — по сегментам semver** (уточнение
   [Q32](../questions/Q32.md), 2026-09-26): `checks/{name}/{X}/{Y}/{Z}/`, где X = major,
   Y = minor, Z = patch, без избыточности; публичный REST передаёт версию одной
   строкой, сервер маппит `{version}` → `{X}/{Y}/{Z}`. Pre-release
   (`1.0.0-rc.1`) — вне MVP (вариант пути
   `checks/{name}/{X}/{Y}/{Z-prerelease}/`; итог [Q18](../questions/Q18.md):
   публикация остаётся вне MVP, сравнение — числовое). Обновлены
   `storage_paths.feature` и фичи с путём, `SPECIFICATION.md` §10 (решения
   №14/№28).

## Следствия

- Единый артефакт реестра — три JSON-файла в каталоге версии; DSL-текст в
  реестр не попадает (согласовано с [D54](D54-source-of-truth-flow.md), п. 2).
- Формулировка «одно правило = один файл» выведена из употребления; канон —
  «один check = один каталог версии» (статьи глоссария §11 — `check`, `name`,
  `Контракт`, `Метаданные версии`).
- Раскладка пути `checks/{name}/{X}/{Y}/{Z}/` — предпосылка задач
  [T-06](../tasks/T-06-registry-path-xyz/README.md) (путь) и
  [T-07](../tasks/T-07-meta-fields/README.md) (поля `meta.json`).
- **«Следствие для кода» (зафиксировано 2026-09-26, [Q7](../questions/Q7.md)):**
  метаданные не соответствуют канону — `CheckMeta` (`src/core.rs`) пишет
  только `name`, `version`, `published_at`, `published_by`, `checksum`
  (+ `deprecated_at`/`deprecation_reason`; запись — `src/lib.rs`),
  канонические `display_name`, `source_hash`, `compiler_version` не
  записываются — сценарий «Публикация сохраняет метаданные»
  ([`publish.feature`](../features/publish.feature)) ожидает кода. Покрыто
  задачей [T-07](../tasks/T-07-meta-fields/README.md).

## Сверка с кодом

Вердикт: 🟡 **расхождение** — артефакт-тройка и неизменяемость реализованы, но
путь плоский, а поля `meta.json` неполны; расхождения в периметре MVP покрыты
задачами.

Что проверено (чтением кода, тестов и фич, 29.09.2026), чем подтверждено:

- **Состав артефакта — ✅:** [`src/lib.rs`](../../src/lib.rs) `publish(...)`
  пишет ровно `rule.json`, `contract.json`, `meta.json` (389–393); исходный
  `.dar` не попадает в артефакт (согласуется с `publish.feature:28`
  «исходный текст „.dar“ в артефакт публикации не попадает»).
- **Путь реестра — 🟡 расхождение:** [`src/lib.rs`](../../src/lib.rs) строит
  плоский путь `format!("checks/{}/{}", rule.name, vstr)` (342; сверка с
  `meta` — 383; чтение meta — 462), тогда как канон — сегменты
  `checks/{name}/{X}/{Y}/{Z}/`. → задача
  [T-06](../tasks/T-06-registry-path-xyz/README.md) (P2).
- **Поля `meta.json` — 🟡 расхождение:** `CheckMeta`
  ([`src/core.rs`](../../src/core.rs) 70–81) содержит `name`, `version`,
  `published_at`, `published_by`, `checksum`, `deprecated_at`,
  `deprecation_reason`; `display_name`, `source_hash`, `compiler_version`
  отсутствуют; запись — [`src/lib.rs`](../../src/lib.rs) 373–381. → задача
  [T-07](../tasks/T-07-meta-fields/README.md) (P2).
- **Неизменяемость — ✅:** дубликат версии отклоняется (`src/lib.rs:346-351`),
  downgrade — `src/lib.rs:353-360`; тесты `tests/publish.rs`
  (`rejects_duplicate_version`, `rejects_downgrade`,
  `rejects_contract_change_without_major`).
- **Фичи обновлены по решению (чтением, не правились):**
  [`publish.feature`](../features/publish.feature) — шапка 7–11 (Q13: канонический
  `meta.json`), сценарий 26–28 (каталог `checks/CreditAgeMin/1/0/0/`,
  `.dar` не попадает);
  [`storage_paths.feature`](../features/storage_paths.feature) — шапка 2–7
  (Q32: `checks/{name}/{X}/{Y}/{Z}/`), сценарий 17–20;
  [`../features/README.md`](../features/README.md) — пометка у
  `publish.feature`/`storage_paths.feature` (93, 96, 309).
- **Краткий канон:** [`SPECIFICATION.md`](../SPECIFICATION.md) §10 решение №14
  (ныне со ссылкой на этот D-файл), §11 статьи `check`/`name`/`Контракт`/
  `Метаданные версии` (887–890); путь уточнён решением №28.

`cargo` не запускался (§5.3, [D50](D50-dod-by-package-scope.md)): решение сверено
чтением кода, тестов и фич; адресный прогон не требуется (правок код не
порождает).

**Задач новых не требуется:** расхождения в периметре MVP уже покрыты
[T-06](../tasks/T-06-registry-path-xyz/README.md) (путь `X/Y/Z`) и
[T-07](../tasks/T-07-meta-fields/README.md) (поля `meta.json`). Отложенное
(`source.dar` в каталоге версии, pre-release) — вне MVP.

## Альтернативы

- **Публиковать `.dar`-текст** (вариант (б) [Q13](../questions/Q13.md)) —
  «один файл» буквально, но теряется скомпилированное представление, снижающее
  дрейф исполнения; отклонено.
- **JSON-тройка + `source.dar` в каталоге версии** (вариант (в)) — сохраняет
  исходник рядом с артефактом, но удваивает хранение и связывает реестр с
  рабочим контуром; отложено в v0.2 (связь обеспечивает `source_hash`).
- **Оставить прежние противоречивые формулировки** (файл «Правило … 1.0.0»,
  «одно правило = один файл», разные пути) — нарушает «один факт — один канон»
  (Q41); отклонено.
- **Оставить плоский путь `checks/{name}/{version}/`** — не даёт файловой
  раскладки по semver и единого маппинга REST `{version}` → `{X}/{Y}/{Z}`;
  отклонено в пользу уточнения Q32.

## Ссылки

- Вопрос: [Q13](../questions/Q13.md)
- Связанные: [Q7](../questions/Q7.md) («Следствие для кода», терминология),
  [Q12](../questions/Q12.md) (источник истины), [Q18](../questions/Q18.md)
  (pre-release); [Q29](../questions/Q29.md) (контракт ответа публикации),
  [Q32](../questions/Q32.md) (структура пути); [Q34](../questions/Q34.md) (тест-гейт)
- Задачи: [T-06](../tasks/T-06-registry-path-xyz/README.md),
  [T-07](../tasks/T-07-meta-fields/README.md)
- Краткий канон: [`SPECIFICATION.md`](../SPECIFICATION.md) §10, решение №14
  (путь — решение №28); глоссарий §11
- Связи: [`TRACEABILITY.md`](../TRACEABILITY.md)
