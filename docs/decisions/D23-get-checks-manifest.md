# D23: Схема ответа `GET /checks` — манифест

- **Статус:** accepted
- **Дата:** 2026-09-25
- **Resolves:** [Q21](../questions/Q21.md)
- **Спека:** [`SPECIFICATION.md`](../SPECIFICATION.md) §10, решение №23
- **Affects:** [`../features/manifest.feature`](../features/manifest.feature),
  [`../features/rest_api.feature`](../features/rest_api.feature),
  [`../features/evaluate.feature`](../features/evaluate.feature),
  [`../features/dashboard.feature`](../features/dashboard.feature)
  (шапочные пометки о каноне Q21 внесены `docs-writer`);
  [`SPECIFICATION.md`](../SPECIFICATION.md) §4.4 (схема API);
  [`../../src/rest.rs`](../../src/rest.rs) (хендлер `GET /checks`),
  [`../../src/lib.rs`](../../src/lib.rs) (`ManifestEntry`, сборка манифеста)
- **Tasks:** — (см. «Сверка с кодом»)

## Контекст

Три фичи описывали один ответ по-разному: `rest_api.feature` —
`count`/`service_hash`/`checks` (`ManifestEntry`:
`name`/`active`/`supported`/`deprecated`); `dashboard.feature` — «три записи
со статусами активна/deprecated»; `evaluate.feature` —
`name`/`kind`/`versions`/`latest`. Было неясно, какая схема нормативна и
нужно ли поле `kind`. Полный контекст — [Q21](../questions/Q21.md).

## Решение

1. **Канон схемы `GET /checks`** — манифест как есть: ответ идентичен
   `manifest.json` в `main` (один источник истины):
   `{ "schema_version": 1, "count", "service_hash", "checks": [
   ManifestEntry ] }`.
2. **`ManifestEntry`** (канонические сценарии — `manifest.feature`):
   - `name: string` — машиночитаемый идентификатор (латиница, Q13);
   - `active: string` — максимальная версия среди **не** помеченных
     deprecated (пример: 1.1.0 в deprecated → active = 1.0.1); если все
     версии deprecated — пустая строка (конвенция «нет значения = пустая
     строка», Q10/Q42); обращение к active-эндпоинту в этом состоянии —
     409 «активация недоступна: {name}» (Q11);
   - `supported: string[]` — все не-deprecated версии по убыванию, включая
     active (денормализация намеренная);
   - `deprecated: string[]` — версии, выведенные из эксплуатации.
   **Инварианты:** каждая версия ровно в одном из `supported`/`deprecated`;
   при непустом `supported` `active == supported[0]`.
3. **Поле `kind` в манифест не добавляется:** в v0.1 единственный вид
   проверок — правила (конвейеры/скоринги/таблицы вне MVP, Q36), `kind` —
   YAGNI, вернётся в v0.2.
4. **Отдельный `GET /checks/{name}` (детали) в MVP не нужен** — манифеста
   достаточно для дашборда и для клиентской оркестрации полного прогона
   (Q20): итерация по `active`-версиям.

## Следствия

- `dashboard.feature` и `evaluate.feature` приведены к канону: из схемы
  списка убраны `kind`/`versions`/`latest` (шапочные пометки — зона
  `docs-writer`).
- Схема описана в `SPECIFICATION.md` §4.4 (схема API), краткий канон —
  §10, решение №23; ссылка на манифест добавлена в Q20.
- Состояние «все версии deprecated» (пустой `active`) согласовано с
  текстом 409 и кодом active-эндпоинта (Q11, D53).
- `service_hash` остаётся отпечатком состояния и используется для
  инвалидации кэша; ответ `GET /checks` — из кэша в памяти, без чтения git
  на каждый запрос.

## Сверка с кодом

Вердикт: ✅ **соответствует** — код строит манифест ровно по канонической
схеме и инвариантам; unit- и интеграционные тесты подтверждают.

Что проверено (чтением кода и тестов, 29.09.2026), чем подтверждено:

- [`src/lib.rs`](../../src/lib.rs) — `ManifestEntry` (`525–531`):
  `name`/`active`/`supported`/`deprecated`; `Manifest` (`533–539`):
  `schema_version`/`generated_at`/`service_hash`/`checks`.
- [`src/lib.rs`](../../src/lib.rs) — `build_manifest` (`541–608`):
  `supported`/`deprecated` собираются по возрастанию и реверсятся — по
  убыванию (`580–581`); `active` — первая `supported`, иначе `""`
  (`583–585`); `schema_version: 1` (`603`).
- [`src/rest.rs`](../../src/rest.rs) — `manifest_response` (`96–111`)
  отдаёт `{schema_version, count, service_hash, checks}` без `generated_at`
  (он остаётся в `GET /version`); `list_checks` (`108–111`) — из кэша.
- [`src/rest.rs`](../../src/rest.rs) — unit-тест
  `manifest_response_has_canonical_keys_q21` (`421–450`).
- [`src/lib.rs`](../../src/lib.rs) — unit-тесты `supported_is_descending_q21`
  (`927`), `active_is_first_supported_q21` (`938`),
  `active_is_empty_when_all_deprecated_q21` (`948`).
- [`src/rest.rs`](../../src/rest.rs) — `eval_active` (`149–170`): пустой
  `active` → 409 `activation_unavailable` («активация недоступна: {name}»).
- Тесты: [`../../tests/rest.rs`](../../tests/rest.rs) —
  `get_checks_returns_q21_manifest` (`126–144`): `schema_version == 1`,
  `count`, `active == "1.0.1"`, `supported == ["1.0.1","1.0.0"]`,
  `deprecated == ["1.1.0"]`.
- Фичи: [`manifest.feature`](../features/manifest.feature) (сценарии
  `active`/`supported`/`deprecated`, `schema_version`),
  [`rest_api.feature`](../features/rest_api.feature) (`25–36`),
  [`evaluate.feature`](../features/evaluate.feature) (`54–59`),
  [`dashboard.feature`](../features/dashboard.feature) (`2–4`) — без
  `kind`/`versions`/`latest`.
- [`SPECIFICATION.md`](../SPECIFICATION.md) §4.4 — схема манифеста и
  оговорка, что `generated_at` остаётся в `GET /version`.

`cargo` не запускался (§5.3, D50): правок кода решение не порождает;
адресный прогон не требуется (сверено чтением кода и тестов).

**Задач не требуется:** код уже реализует каноническую схему и инварианты
манифеста; расхождение было только в текстах фич
(`dashboard.feature`/`evaluate.feature`), приведённых к канону.

## Альтернативы

- **Добавить `kind` в `ManifestEntry`** — в v0.1 единственный вид проверок
  (правило), поле не используется публикацией; YAGNI. Отклонено, вернётся
  в v0.2.
- **Облегчённый список `name`/`kind`/`versions`/`latest`** (ранний
  `evaluate.feature`) — теряет инварианты и статусы, дублирует данные.
  Отклонено.
- **Отдельный `GET /checks/{name}` (детали) в MVP** — манифеста достаточно
  для дашборда и клиентской оркестрации. Отклонено (пост-MVP).

## Ссылки

- Вопрос: [Q21](../questions/Q21.md)
- Связанные: [Q20](../questions/Q20.md) (канонические пути), Q10/Q42
  (пустая строка как «нет значения»), Q11 (тексты 401/409),
  Q13 (артефакт публикации), Q24 (batch), Q36 (конвейеры)
- Краткий канон: [`SPECIFICATION.md`](../SPECIFICATION.md) §10, решение
  №23; схема — §4.4
- Связи: [`TRACEABILITY.md`](../TRACEABILITY.md)
