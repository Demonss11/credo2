# T-10. Фаза 0: workspace `dar-core` + `credo-server`

- **Статус:** ⬜ открыта
- **Приоритет:** P0 (структурный фундамент, до контрактов)
- **Зависит от:** —
- **Источник:** D15 (Q1); SPEC §5 (целевая структура), §8 (Фаза 0);
  `../../features/README.md` (шапка).

## Что сделать

Превратить одиночный крейт `credo2` в workspace **без изменения поведения** и
MCP/REST-контрактов.

### Матрица переноса

| Сейчас | После | Примечание |
|---|---|---|
| `Cargo.toml` (package) | `Cargo.toml` (virtual workspace) + `crates/*/Cargo.toml` | общие версии — по желанию в `[workspace.dependencies]` |
| `src/core.rs` | `crates/dar-core/src/lib.rs` | ядро: домен, парсер, исполнение, semver, `checksum_of` |
| `src/lib.rs` | `crates/credo-server/src/lib.rs` | публикация, манифест, кэш, `AppState`/`ServiceCache` |
| `src/mcp.rs` | `crates/credo-server/src/mcp.rs` | |
| `src/rest.rs` | `crates/credo-server/src/rest.rs` | |
| `src/main.rs` | `crates/credo-server/src/main.rs` | пакет `credo-server`, `[[bin]] name = "credo2"` |
| `tests/publish.rs`, `tests/rest.rs` | `crates/credo-server/tests/` | |
| `tests/features_inventory.rs` | `crates/credo-server/tests/` | путь к фичам — `../../docs/features` |

### Детали

- **Пакет и бинарник:** пакет называется `credo-server`, но бинарник остаётся
  `credo2.exe` (`[[bin]] name = "credo2"`) — чтобы `opencode.json` (MCP `credo`)
  и запуск не менялись. Переименование в `credo-server` — отдельная задача
  (после демо).
- **Границы ядра:** `dar-core` не знает о git/fs/сети; зависимости — только
  нужные `core.rs` (anyhow, regex, serde, sha2, hex).
- **Видимость:** при переносе согласовать `pub(crate)` с публичным API ядра
  (`pub use` в `dar-core/src/lib.rs`); логику не менять.
- **CI** (`.github/workflows/ci.yml`) не меняется: `cargo fmt/clippy/test --all`
  из корня workspace.
- **Rust-analyzer и MCP** в `opencode.json` остаются настроены на
  `prototypes/credo2` (workspace root).

## Критерий готовности

- `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`,
  `cargo test --all` — зелёные (60/60 на момент старта).
- `cargo build --release` из корня даёт `target/release/credo2.exe`.
- Ручной прогон: MCP-цикл «создать черновик → тест → публикация» и REST-режим
  (`--rest`) ведут себя как до переноса; `opencode.json` не менялся.
- Документация актуализирована: `AGENTS.md` («где искать»),
  `../../features/README.md` («Соответствие коду»), ссылки в `../`.

## Примечания

- **Границы:** только перемещение. Не создавать `lsp-dar`/`notebook` (свои фазы),
  не переименовывать бинарник, не реструктурировать `../../features/`, не
  переносить каталог репозитория в `dar/`.
- **Оценка:** 1–2 сессии; при неожиданностях — откат, объём не расширять.
- Follow-up (v0.x): перенос `tests/features_inventory.rs` в отдельный
  `conformance/`-крейт, когда появятся тесты соответствия.
