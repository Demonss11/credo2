# D99: Адаптер Koda CLI — проектный `.kodacli/settings.json` вместо второй памяти

- **Статус:** accepted
- **Дата:** 2026-10-04
- **Resolves:** [Q96](../questions/Q96.md)
- **Спека:** —
- **Affects:** новый `.kodacli/settings.json` (project scope, в git);
  [`opencode.json`](../../opencode.json) (пути `D:/pyTechNotes/.../prototypes/credo2` →
  `C:/Users/Demon/pytechnotes/dar/credo2` в `mcp.servers.credo` и
  `rust-analyzer`); [`AGENTS.md`](../../AGENTS.md) (карта репозитория — строка
  `.kodacli/`; раздел «CREDO — где что и как» — `koda mcp list`);
  [`src/main.rs`](../../src/main.rs) — без изменений (`--workspace` уже поддержан)
- **Tasks:** — (операция настройки, не продуктовая задача)

## Контекст

Проект был подключён к OpenCode (`opencode.json`, `mcp.servers`); владелец поставил
цель — работать и с **Koda CLI** (`@kodadev/koda-cli` 1.2.1). Разведка живыми командами
и бандлом `koda.js` подтвердила формат конфига (`mcpServers`: `command`/`args`/`env`/
`trust`), наличие `koda mcp add|remove|list` и **баг 1.2.1**: `mcp add` теряет args
(пишет `[]`), поэтому args дописываются вручную. Полный разбор и варианты —
[Q96](../questions/Q96.md). Попутно выяснилось, что пути в `opencode.json` устарели
после переноса репозитория — оба MCP-контура требовали починки.

## Решение

1. **Память — общая, не дублируется.** `KODA.md` не вводится: Koda читает
   `AGENTS.md` (поддерживаемое имя памяти), он уже канон проекта для агентов.
2. **Конфиг Koda — проектный.** `.kodacli/settings.json` в репозитории (project
   scope), один сервер `credo`:

   ```json
   {
     "mcpServers": {
       "credo": {
         "command": "C:\\Users\\Demon\\pytechnotes\\dar\\credo2\\target\\release\\credo2.exe",
         "args": ["--workspace", "C:\\Users\\Demon\\pytechnotes\\dar\\credo2"],
         "trust": true
       }
     }
   }
   ```

   Абсолютные пути — сознательно: Koda может запускаться из другой рабочей директории,
   а `--workspace` по умолчанию (= `current_dir`) иначе укажет не туда.
   `trust: true` — локальный доверенный бинарь проекта (обходит подтверждение каждого
   tool call).
3. **Порядок создания.** `koda mcp add --trust --transport stdio credo <exe> --
   --workspace <dir>` создаёт файл, но args теряются (баг 1.2.1) — их дописывать
   вручную/скриптом. Проверка: `koda mcp list` → `✓ credo ... (stdio) - Connected`.
4. **`opencode.json` — починка путей** на текущий корень (forward slashes валидны в
   Windows). Проверка: `opencode mcp list` → `✓ credo connected`.
5. **`AGENTS.md` — две малые правки:** строка `.kodacli/` в карту репозитория и
   `koda mcp list` в раздел проверки статуса (наряду с `opencode mcp list`).
6. **Без `.kodaignore`:** `target/` и `.credo/` уже в `.gitignore`; отдельного
   ignore-файла Koda для этого набора не требуется.

## Последствия

- Второй агентной среде (Koda) доступны те же 9 инструментов `check.create` …
  `rebuild_manifest` через общий бинарь; поведение идентично OpenCode-контуру.
- Конфиг содержит machine-specific абсолютный путь — при переносе репозитория
  обновлять `.kodacli/settings.json` и `opencode.json` синхронно (оба — одна строка
  в карте репозитория).
- `trust: true` снимает подтверждения tool call — приемлемо для локального `credo2`;
  при подключении сторонних MCP-серверов не воспроизводить.
- Продуктовый код (`src/**`) не менялся; тест целостности журнала
  [`tests/docs_journal.rs`](../../tests/docs_journal.rs) покрывает новые Q/D-файлы
  штатно (нумерация сквозная, записи связны).

## Сверка с кодом

Вердикт: ⚠️ **не применим** (настройка среды) — `src/**` не менялся. Проверено
чтением, 04.10.2026:

- **`--workspace`:** [`src/main.rs`](../../src/main.rs) (~:52) —
  `cli.workspace.unwrap_or_else(current_dir)`; абсолютный путь в конфиге снимает
  зависимость от cwd.
- **Чистота stdout:** `tracing_subscriber` → `with_writer(std::io::stderr)` (~:49);
  grep `println!/print!` по `src/**` — пусто; stdio-MCP безопасен.
- **Формат конфига Koda:** бандл `koda.js` (`mcpServers`, слияние
  user/project/workspace-складов) + живой `koda mcp add` (создал
  `.kodacli/settings.json` с `command/args/trust`); баг args-потери воспроизведён
  дважды (относительный и абсолютный путь).
- **Подключение:** `koda mcp list` → `✓ credo: ...credo2.exe --workspace ...
  (stdio) - Connected`.

**Задача —** — (настройка вне цикла задач; оформление по
[`.opencode/rules/journal.md`](../../.opencode/rules/journal.md) §5).

## Альтернативы

- **(B) User-scope `~/.kodacli/settings.json`** — отклонено: теряется
  воспроизводимость конфига в репозитории.
- **(C) Только документация** — отклонено: ручная настройка при каждом клонировании.

## Ссылки

- Вопрос: [Q96](../questions/Q96.md)
- Артефакты: `.kodacli/settings.json`, `opencode.json`, `AGENTS.md`
- Связи: `src/main.rs` (`--workspace`), `~/.kodacli/settings.json` (user-scope, не
  задействован)
- Серверная операция 04.10.2026 — настройка владельца
- Связи: [`TRACEABILITY.md`](../TRACEABILITY.md)
