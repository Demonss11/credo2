# D22: Канонические пути REST — `/checks/{name}/versions/{version}/...`

- **Статус:** accepted
- **Дата:** 2026-09-25
- **Resolves:** [Q20](../questions/Q20.md)
- **Спека:** [`SPECIFICATION.md`](../SPECIFICATION.md) §10, решение №22
- **Affects:** [`../features/rest_api.feature`](../features/rest_api.feature),
  [`../features/evaluate.feature`](../features/evaluate.feature),
  [`../features/batch.feature`](../features/batch.feature),
  [`../features/import_export.feature`](../features/import_export.feature)
  (шапочные пометки о каноне Q20 внесены `docs-writer`);
  [`SPECIFICATION.md`](../SPECIFICATION.md) §1.3.1 (принцип атомарной
  композиции), §4.4 (состав REST); [`../../src/rest.rs`](../../src/rest.rs)
  (маршруты)
- **Tasks:** — (см. «Сверка с кодом»)

## Контекст

Три фичи описывали разные формы одного ресурса: `rest_api.feature` —
`/checks/:name/versions/:version/...`; `evaluate.feature` —
`/checks/:name/:version/evaluate` и `GET /checks/:name/9.9.9`;
`import_export.feature` — `/checks/:name/1.0.0/export`; реализация —
вариант с `/versions/`. Было неясно, какой путь нормативен и как он
соотносится с конвейерами v0.2. Полный контекст — [Q20](../questions/Q20.md).

## Решение

1. **Канонический путь** — `/checks/{name}/versions/{version}/...` (сегмент
   `/versions/` обязателен, как в коде и `rest_api.feature`). Исполнение —
   `POST /checks/{name}/versions/{version}/evaluate`.
2. **`name` и `version` — параметры пути**, а не тела: одно правило не
   создаёт новый эндпоинт, это один шаблон пути с параметрами. Плоский
   вариант («имя в теле») не вводится и не планируется: он дублировал бы
   RPC-стиль MCP и терял наблюдаемость (аудит/метрики по пути) и
   возможность кэширования GET-представлений. Алиасы не вводятся.
3. **Конвейерам в v0.2 предназначен отдельный ресурс**
   `/pipelines/{name}/versions/{version}/evaluate`; путь `/checks/...` при
   этом не меняется. Терминология: «конвейер» и pipeline — одно понятие
   (`SPECIFICATION.md` §1.3.1/§11): в текстах — «конвейер», латиница —
   только в машинных идентификаторах (`pipeline`/`pipelines`, URL).
4. **Атомарная модель исполнения** (принцип атомарной композиции,
   «принцип Макдоналдса», §1.3.1). В v0.1 единица исполнения — одно
   правило (один `evaluate` → одно `Explanation`, Q42). Полный прогон
   («вся картина») сознательно отложен в v0.2, но атом спроектирован под
   него как композируемый элемент:
   - атомарный `evaluate` — чистая функция `(правило, данные) →
     Explanation`, без состояния и без сведения решений; чистота
     гарантируется на уровне ядра (`dar-core::evaluate_rule`), транспорты —
     тонкие обёртки;
   - сведение нескольких решений к одному вердикту — слой конвейера,
     требует `Приоритет` (Q4) и конвейеров (Q36) — v0.2;
   - конвейер в v0.2 появится как **новый** ресурс
     `/pipelines/{name}/versions/{version}/evaluate`, путь `/checks/...` при
     этом не меняется.
5. **`evaluate` читает только опубликованные версии**, влитые в `main`
   (Q12/Q15); черновики в REST-исполнении не участвуют. Серверный
   агрегирующий эндпоинт в MVP не вводится; оркестрация полного прогона —
   на стороне клиента (манифест → цикл по `active`-версиям → цикл
   атомарных `evaluate`).
6. **Состав REST в MVP минимальный** (YAGNI):
   ```
   POST /checks/{name}/versions/{version}/evaluate  — атом, ядро продукта;
   GET  /checks                                     — манифест (Q21);
   GET  /health, GET /version, GET /openapi.json    — инфраструктура.
   ```
   Вне MVP-минимума (реализованы в прототипе; решение 2026-09-25 —
   **не удалять и не развивать в MVP**): `POST /checks/{name}/evaluate`
   (active-версия), `GET /checks/{name}/versions`,
   `GET /checks/{name}/versions/{version}`. Судьба этих эндпоинтов —
   вопрос пост-MVP, не задача MVP. За горизонтом MVP: `GET /checks/{name}`
   (детали), batch (Q24 — отложен), export/import (Q26 — отложен),
   конвейеры (v0.2).

## Следствия

- Пути в `evaluate.feature` и `import_export.feature` приведены к канону
  (шапочные пометки и сценарии — зона `docs-writer`); `rest_api.feature`
  сверена и пометку получила.
- `import_export.feature` — отложен по решению Q26 (способ B): файл
  остаётся отдельным, статус ⏸ и приоритет ⏳, в `deferred.feature` не
  вливается, счётчики не меняются; путь сценария экспорта приведён к
  канону `/checks/{name}/versions/{version}/export`.
- Принцип атомарной композиции закреплён в `SPECIFICATION.md` §1.3.1,
  состав REST — в §4.4; краткий канон — §10, решение №22.
- Клиентская оркестрация полного прогона опирается на манифест
  `GET /checks` (Q21): итерация по `active`-версиям.

## Сверка с кодом

Вердикт: ✅ **соответствует** — `src/rest.rs` реализует канонический путь
с обязательным сегментом `/versions/`; тесты и фичи подтверждают.

Что проверено (чтением кода, фич и тестов, 29.09.2026), чем подтверждено:

- [`src/rest.rs`](../../src/rest.rs) — маршруты (`20–29`): `/health`,
  `/docs`, `/openapi.json`, `/version`, `/checks`,
  `/checks/{name}/versions`, `/checks/{name}/versions/{version}`,
  `/checks/{name}/evaluate`,
  `/checks/{name}/versions/{version}/evaluate`; сегмент `/versions/`
  присутствует во всех versioned-путях.
- [`src/rest.rs`](../../src/rest.rs) — генерация OpenAPI (`235–360`)
  строит пути `/checks/{name}/versions/{v}/evaluate`,
  `/checks/{name}/versions/{v}`, `/checks/{name}/evaluate`; ресурса
  `/pipelines` в коде нет (конвейеры — v0.2).
- [`src/rest.rs`](../../src/rest.rs) — middleware `auth` (`63–85`): без
  ключа открыты `/health`, `/docs`, `/openapi.json` — согласуется с
  составом MVP-минимума и решением Q22.
- Тесты: [`../../tests/rest.rs`](../../tests/rest.rs) —
  `openapi_has_canonical_paths_error_schema_and_no_import_export` (`315+`),
  пути `/checks/CreditAgeMin/versions/1.0.0/evaluate` (`175`),
  `/checks/CreditAgeMin/versions/9.9.9/evaluate` (`251`).
- Фичи: [`rest_api.feature`](../features/rest_api.feature) (`2–5`,
  `58–62`), [`evaluate.feature`](../features/evaluate.feature) (`2–3`,
  `17`), [`batch.feature`](../features/batch.feature) (`7`),
  [`import_export.feature`](../features/import_export.feature) (`4–5`,
  `13`) — пути соответствуют канону. «Конвейеры — v0.2» в фичах
  (`parser`/`execution`/`editor`/`lsp`/`client_explanation`/`graph_view`)
  канону не противоречат: `evaluate.feature:13` — «кредитный конвейер» как
  цель интеграции пользователя, не маршрут.
- [`SPECIFICATION.md`](../SPECIFICATION.md) §1.3.1 (`48–96`) — принцип
  атомарной композиции и терминология; §4.4 (`404+`) — состав REST.

`cargo` не запускался (§5.3, D50): правок кода решение не порождает;
адресный прогон не требуется.

**Задач не требуется:** код REST уже реализует канонический путь;
расхождение было только в текстах фич (`evaluate.feature`,
`import_export.feature`), приведённых к канону шапочными пометками
`docs-writer`.

## Альтернативы

- **Плоский путь `/checks/{name}/{version}/...`** (как в раннем
  `evaluate.feature`) — терялась бы явная семантика пространства версий.
  Отклонено.
- **Имя/версия в теле запроса** (RPC-стиль MCP) — дублирует MCP, теряет
  наблюдаемость по пути и кэширование GET-представлений. Отклонено.
- **Алиасы** (оба варианта пути) — два канона одного ресурса, нарушает
  Q41. Отклонено.
- **Агрегирующий серверный эндпоинт полного прогона** — нарушает
  атомарность и YAGNI; оркестрация оставлена клиенту. Отклонено.

## Ссылки

- Вопрос: [Q20](../questions/Q20.md)
- Связанные: [Q21](../questions/Q21.md) (схема манифеста), Q24 (batch —
  отложен), [Q26](../questions/Q26.md) (import/export — отложен),
  [Q36](../questions/Q36.md) (конвейеры — v0.2),
  Q4 (`Приоритет`), Q12/Q15 (источник истины, merge)
- Краткий канон: [`SPECIFICATION.md`](../SPECIFICATION.md) §10, решение
  №22; терминология и принцип — §1.3.1
- Связи: [`TRACEABILITY.md`](../TRACEABILITY.md)
