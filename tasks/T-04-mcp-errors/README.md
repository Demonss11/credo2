# T-04. MCP-ошибки: конверт и коды

- **Статус:** ⬜ открыта
- **Приоритет:** P1
- **Зависит от:** —
- **Источник:** Q29 (§4.5), Q11 (язык), Q23 (аналогия конверта REST).

## Что сделать

- Все ошибочные ответы MCP: `isError = true` и
  `{"error":{"code","message"}}`.
- Коды: `validation_failed`, `draft_not_found`, `evaluation_failed`,
  `publish_failed`, `version_not_found`, `version_deprecated`,
  `deprecation_conflict`, `manifest_error`, `unknown_tool`,
  `internal_error`.
- `message` — русский (канон Q11), `code` — латиница `snake_case`.

## Критерий готовности

Сценарии с кодами в `../../features/mcp_tools.feature`,
`../../features/draft.feature`, `../../features/test_draft.feature`,
`../../features/publish.feature`, `../../features/deprecation.feature`.
