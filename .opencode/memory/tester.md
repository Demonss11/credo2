# Память: tester (тесты задачи T-XX)

- **Канон:** `docs/features/*.feature` (сценарии задачи),
  `.opencode/rules/review.md`.
- **Правило:** чекпойнт — какие тесты добавлены, каким сценариям/кейсам
  соответствуют, что компилируется. Прогон — у `validator` (R2). Кратко.
- **Знание:** `git -C <путь>` отклоняется всегда (проба 28.09.2026) — рабочая
  директория задаётся полем `workdir`, с `-C` не экспериментировать.

## Чекпойнты

- **T-24 (02.10.2026, до проверок).** Добавлен `tests/rest_cli.rs` —
  интеграционный тест CLI-контура REST на реальном бинарнике
  (`env!("CARGO_BIN_EXE_credo2")`), запросы по TCP вручную (reqwest в
  dev-deps нет; tokio/axum есть). Покрытие: запуск `--rest`+`--addr :<free>` и
  готовность `GET /health` (поллинг); `--api-key` → 401 без заголовка и с
  неверным, 200 с верным; env `CREDO_API_KEY` → тот же контракт; открытый режим
  без ключа → 200; открытые пути `/health`/`/docs`/`/openapi.json` без ключа;
  `--no-rest`+`--rest`/`--addr` → отказ clap. Процесс держит stdin открытым
  (иначе MCP на EOF гасит REST). `src/**`, `Cargo.toml`, `features/**` не
  трогались. **Проверки после:** `cargo check --all-targets` — ok;
  `cargo fmt --check` — ok. Прогон не делался (R2). Грабли: HTTP-клиента
  (reqwest/hyper-client) в dev-deps нет — запросы сырым TCP; процесс обязан
  держать stdin открытым; `--rest` без `--addr` слушает фиксированный 8080 —
  в тесте сознательно не берётся (флейк), `--rest` упражняется вместе с
  `--addr` и проверкой конфликта.
- **T-24 (02.10.2026, rework P1, iteration 2, до проверок).** Одна правка
  `tests/rest_cli.rs:47` (fn `wait_ready`): вложенный
  `if let Ok((200, body)) = request(...) { if body.contains("ok") { return; } }`
  схлопнут в let-chain
  `if let Ok(...) && body.contains("ok") { return; }` — снят
  `clippy::collapsible_if` (rust-1.96.0, `-D warnings`). Иных правок нет.
  **Проверки после:** `cargo fmt --check` — ok; `cargo check --all-targets` —
  ok. `cargo clippy` не запускался (не в правах роли, R2) — повторная приёмка
  за `validator` (`-r2`). Прогон тестов не делался (R2).
