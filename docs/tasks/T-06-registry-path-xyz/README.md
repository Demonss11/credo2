# T-06. Реестр: путь `checks/{name}/{X}/{Y}/{Z}/`

- **Статус:** ⬜ открыта
- **Приоритет:** P2
- **Зависит от:** —
- **Источник:** Q13, Q32 (SPEC §10 №14/№28).

## Что сделать

- `lib.rs` строит и читает путь по сегментам `X/Y/Z` (без `+build`, Q18).
- REST-слой маппит `{version}` → `{X}/{Y}/{Z}`.
- Обновить `tests/publish.rs` (ожидания плоского пути).

## Критерий готовности

Сценарии `../../features/storage_paths.feature`,
`../../features/publish.feature`, `../../features/publish_rules.feature`,
`../../features/immutability.feature`, `../../features/deprecation.feature`,
`../../features/mcp_tools.feature`.
