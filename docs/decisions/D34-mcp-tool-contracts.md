# D34: Контракты MCP-инструментов — общие правила, коды, инварианты и полные схемы

- **Статус:** accepted
- **Дата:** 2026-09-26
- **Resolves:** [Q29](../questions/Q29.md)
- **Спека:** [`SPECIFICATION.md`](../SPECIFICATION.md) §10, решение №34;
  общие правила, коды и инварианты — §4.5
- **Affects:** [`../features/mcp_tools.feature`](../features/mcp_tools.feature),
  [`../features/draft.feature`](../features/draft.feature),
  [`../features/test_draft.feature`](../features/test_draft.feature),
  [`../features/publish.feature`](../features/publish.feature),
  [`../features/deprecation.feature`](../features/deprecation.feature)
  (правки — зона `docs-writer`);
  [`../features/README.md`](../features/README.md) (заметка);
  [`SPECIFICATION.md`](../SPECIFICATION.md) §4.5, §10;
  [`../../src/mcp.rs`](../../src/mcp.rs) (конверт, коды, схемы ответов)
- **Tasks:** [T-04](../tasks/T-04-mcp-errors/README.md) (сделана),
  [T-05](../tasks/T-05-mcp-success-schemas/README.md) (открыта; см. «Сверка с
  кодом»)

## Контекст

JSON-схемы MCP-инструментов не были собраны в одном месте: конверт ошибок и
коды, поля ответов `check.test`/`check.list_published`/`check.deprecate`,
инварианты черновика (`stale`, `test_valid`, отсутствие `size`/`format`)
описывались разрозненно; `check.deprecate` требовал `reason` обязательным.
Единый контракт взаимодействия агента с сервером отсутствовал. Полный
контекст — [Q29](../questions/Q29.md).

## Решение

JSON-контракты MCP-инструментов v0.1 фиксируются как единый источник истины.
**Краткая сводка — общие правила успеха/ошибки, стабильные коды (перечень),
инварианты черновика и таблица «инструмент → вход → успех» — в
[`SPECIFICATION.md`](../SPECIFICATION.md) §4.5.** Ниже — то, что §4.5 не
дублирует: таблица «когда используется» для кодов, полные JSON-схемы
входов/успехов и перечень вне-MVP-инструментов (перенесено из
[Q29](../questions/Q29.md) без потерь).

### Общие правила

1. **Успешный ответ:** `isError = false`; `content[0].text` содержит
   JSON-объект; объект содержит поле `status`; `status` — общий `"ok"` или
   доменный (`"published"`, `"deprecated"`).
2. **Ошибочный ответ:** `isError = true`; `content[0].text` содержит
   `{"error": {"code": "...", "message": "..."}}`; `code` — стабильный
   `snake_case` латиницей; `message` — русский текст (Q11).
3. **Ключи ответов** — `snake_case`, латиница. Текстовые значения
   `decision`, `reason`, `deprecation_reason`, `message` — русские.
4. **Каноническое имя входных данных** для исполнения — `input`;
   альтернативы `data`, `params`, `payload` не используются.

### Стабильные коды ошибок (таблица «когда используется»)

| Код | Когда используется |
|---|---|
| `validation_failed` | неверные параметры или невалидный `.dar`-текст |
| `draft_not_found` | черновик не найден |
| `evaluation_failed` | ошибка исполнения правила (Q8/Q9) |
| `publish_failed` | публикация отклонена: дубль, downgrade, отсутствие успешного теста, изменение черновика после теста, ошибка git |
| `version_not_found` | опубликованная версия не найдена |
| `version_deprecated` | попытка исполнить выведенную из эксплуатации версию |
| `deprecation_conflict` | попытка повторно пометить версию как устаревшую |
| `manifest_error` | не удалось пересобрать или записать манифест |
| `unknown_tool` | неизвестное имя инструмента |
| `internal_error` | неожиданная внутренняя ошибка |

### Инварианты

Сводка — §4.5; ниже — исходные формулировки Q29 (для полноты):

1. Любой черновик в песочнице всегда парсится: `check.create` валидирует
   `source` до записи. Черновик с невалидным текстом существовать не может.
2. Черновик не раскрывает внутренний тип `Rule`. MCP-ответы возвращают
   только рендеренные строки: `source`, `condition`, `decision`, `reason`.
   Структуры `Rule/Condition/Action` остаются приватным представлением ядра
   (Q3, [`GRAMMAR.md`](../GRAMMAR.md) §3).
3. `stale` черновика: файл `rules/{name}.dar` отсутствует → `false`; файл
   существует и его `source_hash` равен `source_hash` черновика → `false`;
   отличается → `true`. `stale` не блокирует `check.test` (Q12).
4. `test_valid = (last_test_checksum != null && last_test_checksum ==
   source_hash)`.
5. Поля `size` и `format` черновика не вводятся: формат всегда `.dar`,
   размер при необходимости вычисляется клиентом из `source`.

### Полные схемы инструментов

Сводная таблица «вход → успех» — §4.5. Ниже — полные схемы.

#### 1. `check.create`

Вход: `{name, source}`, оба обязательны. Повторный вызов с тем же `name`
перезаписывает черновик («сохранить = обновить», Q28/[D31](D31-check-create-contract.md)).

Успех:
```json
{ "status": "ok", "name": "..." }
```

#### 2. `check.list_drafts`

Вход: `{}`.

Успех:
```json
{
  "status": "ok",
  "count": 1,
  "drafts": [
    {
      "name": "...",
      "updated_at": "...",
      "condition": "...",
      "decision": "...",
      "stale": false
    }
  ]
}
```

`condition` и `decision` — рендеренные строки, не объекты.

#### 3. `check.get_draft`

Вход: `{name}`.

Успех:
```json
{
  "status": "ok",
  "draft": {
    "name": "...",
    "source": "...",
    "source_hash": "sha256:...",
    "condition": "...",
    "decision": "...",
    "reason": "...",
    "created_at": "...",
    "updated_at": "...",
    "last_test_checksum": "sha256:...",
    "tested_at": "...",
    "test_valid": true,
    "stale": false
  }
}
```

`last_test_checksum` и `tested_at` могут быть `null`, если тест не
выполнялся. Поле `rule` не возвращается. `condition` — результат
`condition_to_string`; `decision` и `reason` — значения действия.
`test_valid` — инвариант 4, `stale` — инвариант 3.

#### 4. `check.test`

Вход: `{name, input}`.

Успех возвращает поля канонического объяснения (Q42) и метаданные теста на
верхнем уровне, без вложенного `explanation`:
```json
{
  "status": "ok",
  "rule_name": "...",
  "condition": "...",
  "actual_value": 19,
  "matched": true,
  "decision": "...",
  "reason": "...",
  "source_hash": "sha256:...",
  "tested_at": "...",
  "last_test_checksum": "sha256:..."
}
```

Успешный `check.test` обновляет `last_test_checksum` и `tested_at`
черновика; `last_test_checksum` равен `source_hash` черновика на момент
успешного теста. `stale` не блокирует `check.test`. Ошибки исполнения
возвращаются как `evaluation_failed`.

#### 5. `check.delete_draft`

Вход: `{name}`.

Успех:
```json
{ "status": "ok", "name": "...", "deleted": true }
```

Если черновика не было — `deleted: false`; отсутствие черновика не является
ошибкой (идемпотентность).

#### 6. `check.publish`

Вход: `{name, version, published_by?}`.

Успех:
```json
{
  "status": "published",
  "name": "...",
  "version": "...",
  "branch": "publish/{name}-{version}",
  "path": "checks/{name}/{X}/{Y}/{Z}/",
  "published_at": "...",
  "published_by": "...",
  "commit_msg": "...",
  "next_step": "..."
}
```

Поля `branch`, `path`, `name`, `version` стабильны. Поле `next_step` —
необязательная человекочитаемая подсказка; формат текста не гарантируется.
Публикация без успешного теста и публикация после изменения черновика
отклоняются с кодом `publish_failed`.

#### 7. `check.deprecate`

Вход: `{name, version, reason}`, все поля обязательны, `reason` непустой.

Успех:
```json
{
  "status": "deprecated",
  "name": "...",
  "version": "...",
  "reason": "...",
  "deprecated_at": "...",
  "service_hash": "..."
}
```

Инструмент помечает версию как устаревшую, пересобирает манифест и
возвращает новый `service_hash`. Повторная попытка депрекации возвращает
`deprecation_conflict`.

#### 8. `check.list_published`

Вход: `{}`.

Успех:
```json
{
  "status": "ok",
  "count": 2,
  "service_hash": "...",
  "versions": [
    {
      "name": "...",
      "version": "...",
      "path": "checks/{name}/{X}/{Y}/{Z}/",
      "status": "supported",
      "active": true,
      "published_at": "...",
      "published_by": "...",
      "deprecated_at": null,
      "deprecation_reason": null
    }
  ]
}
```

`count` — число версий в списке; `active` вычисляется из манифеста.
Сортировка: имя по возрастанию, версия по убыванию **по семантике semver**
([`semver.feature`](../features/semver.feature), Q18).

#### 9. `check.rebuild_manifest`

Вход: `{}`.

Успех:
```json
{ "status": "ok", "service_hash": "...", "written": true, "count": 2 }
```

`count` — число проверок в манифесте, согласовано с `GET /checks` (Q21).

#### 10. `check.run`

Вход: `{name, version, input}`, все поля обязательны.

Успех возвращает поля объяснения на верхнем уровне:
```json
{
  "status": "ok",
  "name": "...",
  "version": "...",
  "rule_name": "...",
  "condition": "...",
  "actual_value": 19,
  "matched": true,
  "decision": "...",
  "reason": "..."
}
```

`check.run` исполняет только опубликованные версии из `main`. Обращение к
выведенной из эксплуатации версии возвращает `version_deprecated`.

### Вне Q29

`check.merge` (Q15; план — команда `credo merge` и будущая кнопка UI)
остаётся за Q15 и в контракты Q29 не входит. `check.explain_client` (Q25) —
отложен в v0.2.

## Следствия

- Агент и Notebook получают один стабильный источник истины: успех — объект
  со `status`, ошибка — конверт `{"error": {"code","message"}}`; различать
  ошибки следует по `code`, а не по тексту `message`.
- Ключи — `snake_case` латиницей; тексты `decision`/`reason`/
  `deprecation_reason`/`message` — русские (Q11, [D53](D53-error-messages-language.md)).
- Черновик не раскрывает внутренний `Rule`; `size`/`format` не вводятся;
  `stale`/`test_valid` — вычисляемые (Q12/[D54](D54-source-of-truth-flow.md),
  [T-01](../tasks/T-01-draft-source-hash/README.md)).
- Общие правила, коды и инварианты — §4.5; полные схемы — настоящий D34
  (указатель §4.5 ведёт к нему).
- Документы приводятся к канону (правки — зона `docs-writer`):
  `mcp_tools.feature`, `draft.feature`, `test_draft.feature`,
  `publish.feature`, `deprecation.feature`, `features/README.md`; §4.5 и §10.
- Кодинг: конверт и коды — [T-04](../tasks/T-04-mcp-errors/README.md)
  (сделана); поля черновиков — [T-01](../tasks/T-01-draft-source-hash/README.md)
  (сделана); остальные схемы успеха — [T-05](../tasks/T-05-mcp-success-schemas/README.md)
  (открыта).

## Сверка с кодом

Вердикт: 🟡 **расхождение** — общие правила, конверт ошибок, 10 стабильных
кодов и инварианты черновика реализованы; **схемы успеха частично отстают** от
канона (черновики приведены, остальное — [T-05](../tasks/T-05-mcp-success-schemas/README.md)).
Расхождение в периметре MVP покрыто задачей.

Что проверено (чтением кода и тестов, 29.09.2026), чем подтверждено:

- **Конверт ошибок и коды — ✅ ([T-04](../tasks/T-04-mcp-errors/README.md),
  сделана):** [`src/mcp.rs`](../../src/mcp.rs) — `ErrorCode` и `as_str`
  (38–67) дают 10 латинских `snake_case`-кодов ровно по списку Q29;
  `ToolError` (72–131) несёт русский `message`; `to_json` (128–130) рендерит
  единый конверт `{"error": {"code","message"}}`;
  [`response`](../../src/mcp.rs) (533–546) выставляет `isError` и
  `content[0].text`. Тесты
  [`../../tests/mcp_errors.rs`](../../tests/mcp_errors.rs) проверяют
  `validation_failed`, `draft_not_found`, `publish_failed`, `unknown_tool`,
  `version_not_found`, `deprecation_conflict`;
  [`../../tests/common/mod.rs`](../../tests/common/mod.rs) —
  `assert_error_envelope` (181–225) требует латинский `snake_case` `code` и
  русский `message`.
- **Инварианты 1–5 — ✅ (черновики):**
  - `check.create` валидирует `source` до записи (155–184, инвариант 1);
  - `draft_json` (438–455) отдаёт только рендеренные строки, внутренний `rule`
    не публикуется (инвариант 2);
  - `stale` — `is_stale` ([`src/lib.rs`](../../src/lib.rs)), не блокирует
    `check.test` (комментарий 234, инвариант 3);
  - `test_valid` — вычисляется в `draft_json` (439–440, инвариант 4);
  - `size`/`format` не отдаются (инвариант 5).
  Тесты [`../../tests/mcp_draft.rs`](../../tests/mcp_draft.rs) —
  `get_draft_canonical_fields_and_no_internals_q29_inv2_inv5` (104–141),
  `list_drafts_canonical_fields_q29` (145+), `stale_*` (32–99).
- **Схемы успеха — 🟡 расхождение ([T-05](../tasks/T-05-mcp-success-schemas/README.md),
  открыта):**
  - `check.list_drafts` — ✅: `{status, count, drafts:[name, updated_at,
    condition, decision, stale]}` (186–199) — совпадает с каноном;
  - `check.get_draft` — ✅: `draft_json` (438–455) — ровно канонические поля;
  - `check.test` — ✅: `{status, rule_name, condition, actual_value, matched,
    decision, reason, source_hash, tested_at, last_test_checksum}` (250–261) —
    поля объяснения и метки на верхнем уровне, без `explanation`;
  - `check.delete_draft` — 🟡: код отдаёт
    `{status: "deleted"|"not_found", name}` (277–279), канон —
    `{status:"ok", name, deleted: bool}` (идемпотентно);
  - `check.publish` — 🟡: код отдаёт `{status:"published", name, version,
    branch, path, commit_msg, next_step}` (328–341), канон добавляет
    `published_at`, `published_by`;
  - `check.deprecate` — 🟡: код отдаёт `{status:"deprecated", name, version,
    reason}` (391–396), канон добавляет `deprecated_at`, `service_hash`;
    обязательность/непустота `reason` — ✅ (353–357);
  - `check.list_published` — 🟡: код отдаёт `{count, checks:[…]}` (399–411),
    канон — `{status:"ok", count, service_hash, versions:[… path, status,
    active, published_at, published_by, deprecated_at, deprecation_reason]}`;
  - `check.rebuild_manifest` — 🟡: код отдаёт `{status:"ok", service_hash,
    written}` (413–419), канон добавляет `count`;
  - `check.run` — вне MVP (Q33/[T-09](../tasks/T-09-check-run/README.md)).
- **Общие правила:** успех — объект со `status` (`response` 533–546);
  `status` доменный `"published"`/`"deprecated"` в соответствующих
  инструментах; ключи `snake_case`, тексты русские (Q11) — ✅.
- Tool spec'и ([`src/mcp.rs`](../../src/mcp.rs) 470–531) совпадают с таблицей
  входов §4.5.

`cargo` не запускался (§5.3, [D50](D50-dod-by-package-scope.md)): решение
сверено чтением кода и тестов; адресный прогон не требуется (правок код не
порождает).

**Задач новых не требуется:** конверт/коды и инварианты реализованы
([T-04](../tasks/T-04-mcp-errors/README.md) — сделана,
[T-01](../tasks/T-01-draft-source-hash/README.md) — сделана); остаток схем
успеха покрыт открытой [T-05](../tasks/T-05-mcp-success-schemas/README.md).

## Альтернативы

- **Схемы по фичам, без единого места** (вариант (б)) — дублирование и дрейф
  между фичами, агентом и кодом; отклонено.
- **Полные схемы только в коде (`tool_specs`)** (вариант (в)) — канон
  недоступен документации и агентному контексту; отклонено.
- **Дублировать все схемы в §4.5 и в D34** — нарушает «один факт — один
  канон» (Q41); принято: сводка/правила/коды/инварианты — §4.5, полные
  схемы — D34.
- **Оставить `size`/`format` черновика** — противоречит инварианту 5 и
  `draft.feature`; отклонено.
- **Сделать `reason` необязательным в `check.deprecate`** — аудит требует
  причины депрекации; отклонено (обязателен и непуст).

## Ссылки

- Вопрос: [Q29](../questions/Q29.md)
- Связанные: [Q28](../questions/Q28.md) / [D31](D31-check-create-contract.md)
  (контракт `check.create`); [Q11](../questions/Q11.md) /
  [D53](D53-error-messages-language.md) (язык ошибок);
  [Q23](../questions/Q23.md) / [D25](D25-rest-error-envelope.md) (конверт
  REST-ошибок); [Q12](../questions/Q12.md) /
  [D54](D54-source-of-truth-flow.md) (черновик, `stale`);
  [Q42](../questions/Q42.md) / [D21](D21-core-semantics-v01.md) (схема
  объяснения); [Q16](../questions/Q16.md) / [D32](D32-test-gate-mvp.md)
  (тест-гейт); [Q18](../questions/Q18.md) / [D35](D35-semver-v01.md)
  (semver), Q33 (draft-first) — ожидает переноса
- Задачи: [T-04](../tasks/T-04-mcp-errors/README.md) (сделана),
  [T-05](../tasks/T-05-mcp-success-schemas/README.md) (открыта),
  [T-01](../tasks/T-01-draft-source-hash/README.md) (сделана),
  [T-09](../tasks/T-09-check-run/README.md) (`check.run`, вне MVP)
- Краткий канон: [`SPECIFICATION.md`](../SPECIFICATION.md) §10, решение №34;
  общие правила, коды и инварианты — §4.5
- Связи: [`TRACEABILITY.md`](../TRACEABILITY.md)
