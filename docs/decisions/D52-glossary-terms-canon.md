# D52: Терминологический канон глоссария (`SPECIFICATION.md` §11)

- **Статус:** accepted
- **Дата:** 2026-09-26
- **Resolves:** [Q7](../questions/Q7.md)
- **Спека:** [`SPECIFICATION.md`](../SPECIFICATION.md) §10, решение №52
- **Affects:** [`SPECIFICATION.md`](../SPECIFICATION.md) §11 (глоссарий —
  терминологический канон); [`features/publish.feature`](../features/publish.feature)
  (шапочная пометка о расхождении `meta.json`; правку фичи вносит
  `docs-writer`); [`docs/tasks/T-07-meta-fields/README.md`](../tasks/T-07-meta-fields/README.md)
  (связанная задача: расхождение `meta.json` уже покрыто)
- **Tasks:** — (расхождение `meta.json` покрыто
  [T-07](../tasks/T-07-meta-fields/README.md), Источник: Q13, Q7)

## Контекст

Глоссарий [`SPECIFICATION.md`](../SPECIFICATION.md) §11 не содержал терминов,
которыми оперируют требования и код: «check», «active», «supported», «kind»,
«service_hash», «контракт», «песочница», «манифест». Фичи, SPEC и код
описывали одни и те же понятия по отдельности — против политики «один факт —
один канон» (Q41). Полный контекст — [Q7](../questions/Q7.md).

## Решение

1. **Терминологический канон** — глоссарий `SPECIFICATION.md` §11. Одно
   понятие описывается ровно одной статьёй; остальные документы ссылаются, а не
   заново определяют термин.
2. **14 новых статей** (с привязкой к структурам `ManifestEntry`, `CheckMeta`,
   `CheckContract`):
   - реестр и публикация: `check (проверка)`, `name` (явно: «термин `rule_id`
     не используется»), `Контракт (CheckContract)`, `Метаданные версии
     (CheckMeta)`, `Песочница (sandbox)`, `published-repo`;
   - манифест и API: `Манифест (manifest.json)`, `ManifestEntry`, `active`,
     `supported`, `deprecated`, `service_hash`, `kind`;
   - среда: `Workspace`.
3. **Уточнены две существующие статьи:** `Черновик` — вычисляемые метки
   `stale`/`test_valid` (Q12/Q29); `Публикация` — ветка
   `publish/{name}-{version}` (Q14) и отдельный шаг `credo merge` (Q15).
4. **Определения сверены с кодом и решениями:** `ManifestEntry`, расчёт
   `service_hash` (`lib.rs`); Q13/Q17/Q19/Q21/Q29/Q32/Q36.
5. **Попутно зафиксировано расхождение** (политика Q41 — расхождения не
   «молчат»): [`features/publish.feature`](../features/publish.feature)
   требует в `meta.json` поля `display_name`, `source_hash`,
   `compiler_version`, но `CheckMeta` в коде их пока не пишет. Факт занесён
   «Следствием для кода» в Q13 и шапочной пометкой в `publish.feature`;
   расхождение покрыто задачей [T-07](../tasks/T-07-meta-fields/README.md).

## Следствия

- Термины фич, SPEC и кода сведены к единственному месту — §11; сокращается
  дублирование определений между документами (Q41).
- Новое понятие заводится статьёй в §11, а не определяется заново в разных
  документах; краткая формулировка решения — §10 №52.
- Расхождение `meta.json` не «висит»: оно зафиксировано в Q13 и
  `publish.feature` и закрывается задачей
  [T-07](../tasks/T-07-meta-fields/README.md).
- Правки фичи (`features/publish.feature`) и корневого канона — зона
  `docs-writer`/сервисной сессии, не `migrator`.

## Сверка с кодом

Вердикт: ⚪ **не применимо** — решение документное (терминологический канон
`SPECIFICATION.md` §11); продуктовое поведение и код `src/**` не меняет.
Выявленное расхождение `meta.json` лежит в коде и уже покрыто задачей
[T-07](../tasks/T-07-meta-fields/README.md).

Что проверено (29.09.2026, чтением документов и кода), чем подтверждено:

- [`SPECIFICATION.md`](../SPECIFICATION.md) §11 — все статьи решения на месте:
  новые — `check (проверка)`, `name`, `Контракт (CheckContract)`,
  `Метаданные версии (CheckMeta)`, `Песочница (sandbox)`, `published-repo`,
  `Манифест (manifest.json)`, `ManifestEntry`, `active`, `supported`,
  `deprecated`, `service_hash`, `kind`, `Workspace` (14); уточнённые —
  `Черновик`, `Публикация` (2). Соответствует перечню решения.
- [`features/publish.feature`](../features/publish.feature) — шапочная пометка
  (строки 7–9) фиксирует: канонический `meta.json` содержит `display_name`,
  `source_hash`, `compiler_version`, «в коде `CheckMeta` их пока нет …
  расхождение зафиксировано в Q13, Q7, 2026-09-26». Пометка на месте.
- `src/core.rs` — `CheckMeta` содержит `name`, `version`, `published_at`,
  `published_by`, `checksum` (+ `deprecated_at`/`deprecation_reason`);
  `display_name`, `source_hash`, `compiler_version` отсутствуют. Расхождение
  подтверждено, покрыто [T-07](../tasks/T-07-meta-fields/README.md).
- `src/lib.rs` — `ManifestEntry` = `name`/`active`/`supported`/`deprecated`,
  `Manifest` = `schema_version`/`generated_at`/`service_hash`/`checks`;
  определения §11 (статьи `ManifestEntry`, `active`, `supported`, `deprecated`,
  `service_hash`, `Манифест (manifest.json)`) сверены с кодом — соответствуют.
- Серия сверок Q13/Q17/Q19/Q21/Q29/Q32/Q36 — источник формулировок статей
  (ссылки в самом глоссарии).

`cargo` не запускался (§5.3): решение документное, адресный прогон не нужен.

**Задач не требуется:** терминологический канон — документы; выявленное
расхождение `meta.json` уже покрыто задачей
[T-07](../tasks/T-07-meta-fields/README.md).

## Альтернативы

- **Оставить глоссарий как есть** (вариант (а) [Q7](../questions/Q7.md)) —
  термины фич и кода остаются без канонического определения, дублирование и
  расхождения копятся. Отклонено: противоречит Q41.
- **Описать термины по месту (в фичах)** — отклонено: размывает «один факт —
  один канон», один и тот же термин получает разные формулировки; §11 — единая
  точка входа.
- **Свести в глоссарий все термины продукта, включая целевые v0.2** —
  отклонено: §11 описывает действующий канон (v0.1) и явно помечает отложенное
  (`kind` — «в v0.1 не вводится»).

## Ссылки

- Вопрос: [Q7](../questions/Q7.md)
- Связанные: Q13 (публикация, «Следствие для кода»), Q17/Q19/Q21/Q29/Q32/Q36
  (источники определений), Q41 (один факт — один канон)
- Задача: [T-07](../tasks/T-07-meta-fields/README.md) (расхождение `meta.json`)
- Краткий канон: [`SPECIFICATION.md`](../SPECIFICATION.md) §10, решение №52;
  глоссарий — §11
- Связи: [`TRACEABILITY.md`](../TRACEABILITY.md)
