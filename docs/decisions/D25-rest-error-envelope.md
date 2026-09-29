# D25: Формат ошибок REST — конверт `{"error": {"code", "message"}}`

- **Статус:** accepted
- **Дата:** 2026-09-25
- **Resolves:** [Q23](../questions/Q23.md)
- **Спека:** [`SPECIFICATION.md`](../SPECIFICATION.md) §10, решение №25
- **Affects:** [`../features/rest_api.feature`](../features/rest_api.feature),
  [`../features/evaluate.feature`](../features/evaluate.feature),
  [`../features/rest_auth.feature`](../features/rest_auth.feature),
  [`../features/errors.feature`](../features/errors.feature) (шапочные
  пометки и ассершены структуры ошибок внесены `docs-writer`);
  [`SPECIFICATION.md`](../SPECIFICATION.md) §4.4 (таблица конверта и кодов);
  [`../../src/rest.rs`](../../src/rest.rs) (`err()` и обработчики ошибок)
- **Tasks:** — (см. «Сверка с кодом»)

## Контекст

Тексты ошибок были унифицированы русским каноном (Q11), но конверт не
определён: код возвращал плоский `{"error": "<сообщение>"}` без
машиночитаемого `code`, фичи структуру не описывали. Добавление `code` и
конверта — остаток, переданный из Q11. Полный контекст —
[Q23](../questions/Q23.md).

## Решение

1. **Единый конверт ошибки** для всех ответов REST с ненулевым статусом
   (401/404/409/410/422):

   ```json
   {"error": {"code": "<стабильный код>", "message": "<русский текст>"}}
   ```

   - `message` — канонический человекочитаемый текст по Q11 (русский);
   - `code` — стабильный машиночитаемый идентификатор: `snake_case`,
     латиница (принцип «машина/человек», Q11);
   - необязательное поле `details` — в v0.1 **не вводится** (YAGNI;
     вернётся при появлении сценария валидации, например batch в Q24).
2. **Стабильные коды** (канонические тексты — Q11, пути — Q20):

   | HTTP | `code` | `message` |
   |---|---|---|
   | 401 | `unauthorized` | `ошибка аутентификации: неверный или отсутствующий x-api-key` |
   | 404 | `check_not_found` | `проверка не найдена: {name}` |
   | 404 | `version_not_found` | `версия не найдена: {name}@{version}` |
   | 409 | `activation_unavailable` | `активация недоступна: {name}` |
   | 410 | `version_deprecated` | `версия выведена из эксплуатации: {name}@{version}` |
   | 422 | `evaluation_failed` | текст ошибки ядра (`EvalError`) |
3. **Клиенты связывают статус и код:** `check_not_found` / `version_not_found`
   различаются кодом при одном статусе 404; `code` стабилен и не меняется
   без major-версии API, текст `message` может уточняться.
4. **Границы решений:** Q11 — язык и тексты `message`; Q22 — заголовок
   аутентификации и текст/код 401; Q24 — схема ответов batch (отложен; при
   возврате — тот же конверт для ошибок транспорта).

## Следствия

- `err()` в `rest.rs` и обработчик `auth` переведены на конверт; в MVP не
  меняются состав эндпоинтов и тексты — только обёртка ответа.
- `SPECIFICATION.md` §4.4 (таблица конверта и кодов), §10 (решение №25);
  `rest_api.feature`, `evaluate.feature`, `rest_auth.feature` — шапочные
  заметки и ассершены структуры ошибок (зона `docs-writer`).
- Тело 401 из Q22 ([D26](D26-rest-auth-x-api-key.md)) задаётся этим
  конвертом.

## Сверка с кодом

Вердикт: ✅ **соответствует** — код отдаёт единый конверт
`{"error": {"code", "message"}}` со стабильными латинскими `snake_case`-кодами
и русскими текстами; `details` отсутствует; тесты и фичи подтверждают.

Что проверено (чтением кода и тестов, 29.09.2026), чем подтверждено:

- [`src/rest.rs`](../../src/rest.rs) — `err()` (`52–61`) формирует ровно
  `{"error": {"code", "message"}}`; коды: `unauthorized` (`77`),
  `check_not_found` (`119`, `156`), `version_not_found` (`133`, `189`),
  `activation_unavailable` (`164`), `version_deprecated` (`197`),
  `evaluation_failed` (`206`) — все латиница `snake_case`; ключа `details`
  в коде нет.
- [`src/rest.rs`](../../src/rest.rs) — тексты русские и канонические
  («проверка не найдена: {name}», «версия не найдена: {name}@{version}»,
  «активация недоступна: {name}», «версия выведена из эксплуатации:
  {name}@{version}», 401 из Q22).
- Тесты: [`../../tests/common/mod.rs`](../../tests/common/mod.rs) —
  `assert_error_envelope` (`184–225`) проверяет, что ключи конверта —
  ровно `["code","message"]`, `code` — латиница `snake_case`, `message` —
  непустой русский;
  [`../../tests/rest.rs`](../../tests/rest.rs) — `assert_error` (`95+`)
  (Q23-конверт), `error_envelope_is_never_flat_q23` (`376+`) прогоняет
  401/404/409/410/422 и отклоняет плоский конверт,
  `unknown_check_is_404_check_not_found` (`220–240`),
  `deprecated_version_is_gone_410` (`185–201`),
  `all_deprecated_gives_409_activation_unavailable` (`204–217`),
  `missing_field_is_422_evaluation_failed` (`278–291`),
  `unauthorized_401_uses_q23_envelope` (`293–310`);
  `openapi_has_canonical_paths_error_schema_and_no_import_export`
  (`315+`) проверяет `components.schemas.ErrorResponse`
  (`required = ["code","message"]`) и привязку конверта к 401/404/422.
- Фичи: [`rest_api.feature`](../features/rest_api.feature) (`7–9`, `44–47`,
  `64–76`), [`evaluate.feature`](../features/evaluate.feature) (`8`, `43–52`,
  `61–71`), [`rest_auth.feature`](../features/rest_auth.feature),
  [`errors.feature`](../features/errors.feature) — структура ошибок и
  русские тексты соответствуют конверту.
- [`SPECIFICATION.md`](../SPECIFICATION.md) §4.4 — таблица конверта и кодов;
  §10 — решение №25.

`cargo` не запускался (§5.3, D50): правок кода решение не порождает;
адресный прогон не требуется (сверено чтением кода и тестов; тексты/коды
ранее сверял [D53](D53-error-messages-language.md)).

**Задач не требуется:** код уже реализует конверт, стабильные коды и русские
тексты; `details` не вводится; тесты `assert_error_envelope`/`error_envelope_is_never_flat_q23`
подтверждают.

## Альтернативы

- **Плоский `{"error": "<message>"}`** — фактическое состояние до решения;
  нет машиночитаемого `code`, клиент не может различать 404-случаи.
  Отклонено.
- **`problem+json` (RFC 7807)** — избыточен для v0.1, ломает единый
  латинский машинный контракт Q11; отклонено.
- **Ввести `details` сразу** — сценарий валидации (batch) вне MVP; YAGNI.
  Отложено (Q24).

## Ссылки

- Вопрос: [Q23](../questions/Q23.md)
- Связанные: [Q11](../questions/Q11.md) (язык и тексты `message`),
  [Q22](../questions/Q22.md) (заголовок и код 401),
  [D26](D26-rest-auth-x-api-key.md), [Q20](../questions/Q20.md) (пути),
  Q24 (batch — отложен)
- Краткий канон: [`SPECIFICATION.md`](../SPECIFICATION.md) §10, решение
  №25; таблица конверта — §4.4
- Связи: [`TRACEABILITY.md`](../TRACEABILITY.md)
