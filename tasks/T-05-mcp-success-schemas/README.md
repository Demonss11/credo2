# T-05. MCP-схемы успеха инструментов

- **Статус:** ⬜ открыта
- **Приоритет:** P1
- **Зависит от:** T-01, T-02
- **Источник:** Q29 (§4.5), Q18 (сортировка).

## Что сделать

- `check.list_drafts`/`check.get_draft` — без `size`/`format`, без объекта
  `rule`; + `stale`, `test_valid`, `source_hash`, `last_test_checksum`,
  `tested_at`, `created_at`/`updated_at`.
- `check.test` — поля объяснения (Q42) и метки теста на верхнем уровне,
  без вложенного `explanation`.
- `check.delete_draft` — `{"status":"ok","name":...,"deleted":bool}`,
  идемпотентно (`deleted:false` — не ошибка).
- `check.publish` — `status:"published"` + `name`, `version`, `branch`,
  `path`, `published_at`, `published_by`, `commit_msg`, `next_step`
  (необязательная подсказка).
- `check.deprecate` — `reason` обязателен и непуст
  (`validation_failed`), ответ `deprecated` + `service_hash`, повтор —
  `deprecation_conflict`.
- `check.list_published` — `service_hash`, `path`, `status`, `active`,
  `deprecated_at`/`deprecation_reason`, сортировка semver, `count`.
- `check.rebuild_manifest` — `count` (согласован с `GET /checks`).

## Критерий готовности

Соответствующие сценарии `../../features/mcp_tools.feature`,
`../../features/draft.feature`, `../../features/test_draft.feature`,
`../../features/publish.feature`, `../../features/deprecation.feature`.
