# D31: Контракт `check.create` — `{name, source}`

- **Статус:** accepted
- **Дата:** 2026-09-26
- **Resolves:** [Q28](../questions/Q28.md)
- **Спека:** [`SPECIFICATION.md`](../SPECIFICATION.md) §10, решение №31
  (схемы — §4.5)
- **Affects:** [`../features/draft.feature`](../features/draft.feature),
  [`../features/agent_minimal.feature`](../features/agent_minimal.feature)
  (правки — зона `docs-writer`);
  [`../features/README.md`](../features/README.md) (строка и заметка);
  [`SPECIFICATION.md`](../SPECIFICATION.md) §7 (жизненный цикл), §10;
  [`../../src/mcp.rs`](../../src/mcp.rs) (`check.create`, tool spec)
- **Tasks:** [T-03](../tasks/T-03-check-create/README.md) (сделана; см.
  «Сверка с кодом»)

## Контекст

Контракт входа `check.create` расходился между фичами и кодом:
`draft.feature` описывала `{name, text, expected_kind}`, код принимал только
`source` и возвращал `{status, draft, overwritten, sandbox_file}`; ответ
`check.list_drafts` в фиче упоминал размер в байтах и формат `"text"`. Для
главного инструмента агента (создание черновика из текста правила) не было
единого канона. Полный контекст — [Q28](../questions/Q28.md).

## Решение

Контракт `check.create` для MVP:

```json
{ "name": "string", "source": "string" }
```

1. **Оба параметра обязательны.** `source` — весь текст `.dar`-файла; сервер
   парсит и валидирует его, ошибки парсинга возвращаются как `error`.
2. **`name` сверяется с заголовком правила** (первая строка `Правило {name}`
   согласно [`GRAMMAR.md`](../GRAMMAR.md)); если заголовка нет или имя в
   заголовке не совпадает с переданным `name` — ошибка валидации.
3. **Идемпотентность:** повторный `check.create` с тем же `name`
   перезаписывает черновик (обновляет `source`) — ментальная модель
   «сохранить = обновить»; отдельного флага перезаписи нет.
4. **Ответ:** `{ "status": "ok", "name": "..." }`. Метаинформация (список
   полей) — вне MVP, задача LSP.
5. **Без `expected_kind` и `contract`** — типы выводятся автоматически из
   `source`.

## Следствия

- У агента и UI один предсказуемый вход: `name` передаётся явно и
  подтверждается заголовком; типы (kind/contract) не запрашиваются.
- «Сохранить = обновить» — простая модель и для UI, и для агента: повторный
  вызов не требует отдельного флага и не создаёт вторую запись.
- `check.list_drafts` не отдаёт `size`/`format` (общие инварианты §4.5);
  производные `stale`/`test_valid` — вычисляемые.
- Документы приводятся к канону (правки — зона `docs-writer`):
  `draft.feature` (контракт, «сохранить = обновить», ошибка невалидного
  `source`), `agent_minimal.feature` (агент передаёт `name` и `source`),
  `features/README.md` (строка, заметка), `SPECIFICATION.md` §7 и §10
  (решение №31).
- Схемы ответов MCP-инструментов зафиксированы в §4.5 (решение [Q29](Q29.md)/
  [D34](D34-mcp-tool-contracts.md)); `check.create` — `{name, source}` → `ok`.

## Сверка с кодом

Вердикт: ✅ **соответствует** — код реализует контракт `{name, source}` →
`{status, name}`, имя сверяется с заголовком, повторный вызов перезаписывает
черновик; `expected_kind`/`contract` не используются. Расхождение из Q28
устранено задачей [T-03](../tasks/T-03-check-create/README.md).

Что проверено (чтением кода, тестов и фич, 29.09.2026), чем подтверждено:

- [`src/mcp.rs`](../../src/mcp.rs) — хендлер `create` (155–184):
  - оба параметра обязательны: `name` (158–161) и `source` (162–165), иначе
    `validation_failed` («Нужен параметр 'name'/'source'»);
  - `source` парсится (`parse_rule`, 167) — невалидный текст → ошибка
    валидации;
  - `name` сверяется с заголовком: `if rule.name != name` → ошибка
    «Имя '{name}' не совпадает с заголовком '{rule.name}'» (168–173);
  - результат — ровно `{ "status": "ok", "name": name }` (183) — без
    `expected_kind`/`contract`/`sandbox_file`;
  - перезапись: `upsert_draft` (174–182), существующий черновик берётся
    `get_draft` и передаётся в `make_draft` («сохранить = обновить»);
  - tool spec `check.create` (472–477): `properties` — только `name`/`source`,
    `required` — оба.
- [`src/lib.rs`](../../src/lib.rs) — `upsert_draft` реализует вставку/замену
  по имени (одна запись на `name`).
- Тесты [`../../tests/mcp_draft.rs`](../../tests/mcp_draft.rs) (через реальный
  бинарник, MCP-stdio):
  - `create_scenario_saves_draft_to_sandbox_with_hash` (369–402) — ответ
    ровно `{status: "ok", name}`; текст хранится без изменений, есть
    `source_hash`; `.dar` не создаётся (draft-first, Q33);
  - `stale_and_hash_after_overwrite_q12` (63–99) — повторный `check.create`
    перезаписывает текст, `count == 1` (не вторая запись), `created_at` не
    сбрасывается;
  - `get_draft_canonical_fields_and_no_internals_q29_inv2_inv5` (104–141) —
    ответ `check.create` содержит ровно ключи `["name", "status"]`; нет
    `size`/`format`/внутреннего `rule`;
  - `create_rejects_source_without_rule_header` (408–426) — `source` без
    заголовка → `validation_failed`, черновик не создаётся;
  - `create_rejects_name_title_mismatch` (431–453) — расхождение `name` и
    заголовка → `validation_failed`, черновик не создаётся ни под каким
    именем;
  - `create_without_name_param_is_error` (457–467) и
    `create_without_source_param_is_error` (471–481) — отсутствие параметра →
    `validation_failed`;
  - `agent_create_appears_in_drafts_list` (487–499) — сценарий
    `agent_minimal.feature`: `check.create` с `name`+`source` → черновик в
    `check.list_drafts`.
- Юнит-тесты [`src/mcp.rs`](../../src/mcp.rs): `create_requires_name_param`
  (676–686), `create_rejects_name_title_mismatch` (689–708),
  `create_returns_status_and_name` (710–726),
  `check_create_tool_spec_requires_name_and_source` (728–733).
- Фичи: [`draft.feature`](../features/draft.feature) (2–4, 16–25, 58–71) —
  `{name, source}`, «сохранить = обновить», отказ на невалидном `source`;
  [`agent_minimal.feature`](../features/agent_minimal.feature) (40–41) — агент
  передаёт `name` и `source`.
- [`SPECIFICATION.md`](../SPECIFICATION.md) §7 (663–715) — `check.create` в
  жизненном цикле; §10 №31 (854) — формулировка решения; §4.5 (485–541) —
  таблица `check.create` `{name, source}` → `ok`.

`cargo` не запускался (§5.3, [D50](D50-dod-by-package-scope.md)): решение
сверено чтением кода, тестов и фич; адресный прогон не требуется (правок код
не порождает).

**Задач новых не требуется:** контракт реализован; исходное расхождение Q28
устранено задачей [T-03](../tasks/T-03-check-create/README.md) (сделана).

## Альтернативы

- **`source`-only** (вариант (а) [Q28](../questions/Q28.md), рекомендация) —
  кода не меняет, но агент не может сослаться на имя до парсинга и
  подтвердить его; явный `name` делает контракт самодокументируемым.
  Отклонено.
- **Архивная форма `{name, text, expected_kind}`** (вариант (б)) — вводит
  обязательные «размер»/«формат»/kind, которые MVP не несёт (типы выводятся
  из текста); дублирует парсинг на стороне клиента. Отклонено.
- **Ответ с `draft`/`overwritten`/`sandbox_file`** — раскрывает внутреннее
  устройство (путь песочницы) и метаинформацию; канон — ровно
  `{status, name}`. Отклонено.
- **Отдельный флаг перезаписи** — «сохранить = обновить» проще для UI и
  агента; лишний параметр в контракте не нужен. Отклонено.

## Ссылки

- Вопрос: [Q28](../questions/Q28.md)
- Связанные: [Q29](../questions/Q29.md) (контракты MCP-инструментов, §4.5),
  Q33 (draft-first), [Q12](../questions/Q12.md) (поток
  `файл → черновик → публикация`), [Q11](../questions/Q11.md) (язык сообщений —
  ответственность [D53](D53-error-messages-language.md));
  [Q27](../questions/Q27.md) (запуск и адрес REST),
  [Q30](../questions/Q30.md) (транспорт MCP в Notebook); Q35+ — ожидают переноса
- Задача: [T-03](../tasks/T-03-check-create/README.md) (сделана)
- Краткий канон: [`SPECIFICATION.md`](../SPECIFICATION.md) §10, решение №31;
  схемы — §4.5
- Связи: [`TRACEABILITY.md`](../TRACEABILITY.md)
