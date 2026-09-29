# D55: Имя ветки публикации — `publish/{name}-{version}`

- **Статус:** accepted
- **Дата:** 2026-09-25
- **Resolves:** [Q14](../questions/Q14.md)
- **Спека:** [`SPECIFICATION.md`](../SPECIFICATION.md) §10, решение №55
- **Affects:** [`../features/publish.feature`](../features/publish.feature),
  [`../features/publish_rules.feature`](../features/publish_rules.feature),
  [`../features/README.md`](../features/README.md) (заметка о каноне) —
  правки зона `docs-writer`;
  [`../../src/lib.rs`](../../src/lib.rs) (формирование ветки),
  [`../../src/mcp.rs`](../../src/mcp.rs) (ответ `check.publish`)
- **Tasks:** — (см. «Сверка с кодом»)

## Контекст

Документы расходились в имени ветки публикации: `publish.feature` называл
`checks/{name}/{version}`, `publish_rules.feature` и код —
`publish/{name}-{version}`. Смешивались пространства артефактов (каталог в
реестре) и доставки (ветка). Полный контекст — [Q14](../questions/Q14.md).

## Решение

1. **Каноническое имя ветки — `publish/{name}-{version}`** (например,
   `publish/CreditAgeMin-2.0.0`), как в коде и `publish_rules.feature`.
2. **`checks/{name}/{version}/` — путь артефакта внутри реестра**, а не имя
   ветки; `publish/{name}-{version}` — имя ветки доставки.
3. **Использовать `checks/...` как имя ветки запрещено** — пространства
   артефактов и доставки не смешиваются.

## Следствия

- Имена артефакта и ветки разведены и не пересекаются: до слияния в `main`
  потребители видят только артефакт по пути реестра, а ветка — транспортная
  единица ([D56](D56-merge-step.md)).
- `publish.feature` приведён к канону (`publish/{name}-{version}`);
  `publish_rules.feature` уже соответствовал; заметка о каноне — в
  `features/README.md` (правки — зона `docs-writer`).
- Имя ветки фигурирует в `next_step` ответа `check.publish` и в команде
  слияния `credo merge` ([D56](D56-merge-step.md)).

## Сверка с кодом

Вердикт: ✅ **соответствует** — код формирует и возвращает каноническое имя
ветки; расхождение было только в документации.

Что проверено (чтением кода, тестов и фич, 29.09.2026), чем подтверждено:

- [`src/lib.rs`](../../src/lib.rs) `publish(...)` — `let branch =
  format!("publish/{}-{}", rule.name, vstr);` (397); ветка создаётся от `main`
  (`parent = rev_parse(repo, "main")`, 396; `create_ref`, 400).
- [`src/mcp.rs`](../../src/mcp.rs) `publish(...)` — ответ `check.publish`
  содержит `"branch": outcome.branch` (326, 332); `next_step` использует то же
  имя ветки (335–340).
- Тесты [`../../tests/publish.rs`](../../tests/publish.rs) — слияние по имени
  `publish/CreditAgeMin-1.0.0` (27, 38, 50 и далее), т. е. имя закреплено в
  тестах.
- Фичи: [`publish_rules.feature`](../features/publish_rules.feature) —
  «создаётся ветка "publish/CreditAgeMin-1.0.0"» (10);
  [`publish.feature`](../features/publish.feature) — «в репозитории создаётся
  ветка "publish/CreditAgeMin-1.0.0"» (25), т. е. документ уже приведён к
  канону (соответствует `Affects`).
- Глоссарий [`SPECIFICATION.md`](../SPECIFICATION.md) §11, статья «Публикация»
  (ветка `publish/{name}-{version}`) — согласована.

`cargo` не запускался (§5.3, [D50](D50-dod-by-package-scope.md)): решение сверено
чтением кода, тестов и фич; адресный прогон не требуется (правок код не
порождает).

**Задач не требуется:** рекомендация уже реализована в коде и описана в
`publish_rules.feature`; решение снимает расхождение документации
(`publish.feature`).

## Альтернативы

- **`checks/{name}/{version}` как имя ветки** (вариант (а)
  [Q14](../questions/Q14.md)) — смешивает пространство артефактов внутри
  реестра и пространство транспортных веток; отклонено.
- **Имя из артефакта (`checks/{name}/{X}/{Y}/{Z}`)** — усложняет восприятие и
  дублирует структуру пути; отклонено.
- **Два разных имени (ветка и артефакт совпадают)** — порождает повтор
  расхождения; отклонено в пользу явного разделения.

## Ссылки

- Вопрос: [Q14](../questions/Q14.md)
- Связанные: [Q13](../questions/Q13.md) (путь артефакта),
  [Q15](../questions/Q15.md) (шаг слияния), Q32 (опубликованный репозиторий) —
  ожидает переноса
- Краткий канон: [`SPECIFICATION.md`](../SPECIFICATION.md) §10, решение №55;
  §11 (статья «Публикация»)
- Связи: [`TRACEABILITY.md`](../TRACEABILITY.md)
