# D45: Судьба волны 0 и конфиг-пакет качества

- **Статус:** accepted
- **Дата:** 2026-09-28
- **Resolves:** [Q50](../questions/Q50.md)
- **Спека:** [`SPECIFICATION.md`](../SPECIFICATION.md) §10, решение №45
- **Affects:** [`opencode.json`](../../opencode.json);
  [`.opencode/plugins/token-guard.ts`](../../.opencode/plugins/token-guard.ts);
  [`.opencode/.gitignore`](../../.opencode/.gitignore);
  [`docs-writer.md`](../../.opencode/agents/docs-writer.md),
  [`git.md`](../../.opencode/agents/git.md); [`AGENTS.md`](../../AGENTS.md)
  §«Сборка, тесты и пересборка»; `rustfmt.toml`, `rust-toolchain.toml`,
  `.cargo/config.toml`, `.gitattributes`; `src/**`, `tests/**` (реформат);
  [`findings-registry.md`](../analysis/findings-registry.md)
- **Tasks:** — (см. «Сверка с кодом»)

## Контекст

Волна 0 (T-15) прошла боевую проверку в Run 5. Судьба её решений оставалась
открытой в `wave0-report.md` §6; 28.09.2026 владелец принял окончательные решения.
Run 5 также вскрыл расхождения конфиг-пакета качества (F20/F23/F33) и
невоспроизводимость B1 при зависимостях плагина в `.gitignore` (F29). Полный
контекст — [Q50](../questions/Q50.md),
[`memorandum-W8-run5.md`](../analysis/memorandum-W8-run5.md) §2, §4.6, §4.11.

## Решение

1. **A (`formatter`) — закрепить:** харнесс-форматтер входит в норму; `rustfmt.toml`
   (`edition = "2024"`, `max_width = 80`, `tab_spaces = 4`,
   `match_block_trailing_comma`, `merge_derives`); стиль применён (`cargo fmt`,
   10 файлов, +374/−157).
2. **B1 (`token-guard` срез) — норма:** протокол работы с маркером среза — в
   `AGENTS.md` (§4.9 меморандума / [D44](D44-run5-refinements.md) п. 4).
   Воспроизводимость (F29): зависимости плагина — в git (`.opencode/package.json` +
   `package-lock.json`; `node_modules` игнорируется), шаг установки `npm ci` в
   `.opencode/` — в `AGENTS.md` §«Сборка, тесты и пересборка».
3. **B2 — отключить:** таблица `B2_PREFIXES` пуста; вернуть при прямой экспозиции
   MCP (`codemode:false`).
4. **deny `execute`** для [`docs-writer`](../../.opencode/agents/docs-writer.md) и
   [`git`](../../.opencode/agents/git.md).
5. **Конфиг-пакет качества:** `.cargo/config.toml` (`-D warnings`) — оставить;
   `clippy.toml` — удалён (ключи инертны при текущем наборе линтов);
   `rustfmt.toml` — стиль 80 + `edition = "2024"` (F20); `.gitattributes`
   (`* text=auto eol=lf`) — добавлен (F23); `rust-toolchain.toml` — 1.96.0.
6. **Отметки F28** в `wave0-report.md` §6 и карточке T-15 — зона `docs-writer`.

## Следствия

- Волна 0 закреплена как норма (A/B1), B2 отключён до условия возврата; markdown
  судьбы волны 0 снимает «две истины» (F28).
- Конфиг-пакет качества введён в норму; принят реформат стиля 80 (F33).
- Приёмка пакета (`validator`): `cargo fmt --check`; `cargo clippy --all-targets
  -- -D warnings`; `cargo test --all` — зелёные; smoke харнесс-форматтера (сохранить
  файл с mixed-case импортами → `git diff` пуст).
- Обновление доков T-15 (`wave0-report`, карточка) — за `docs-writer`.

## Сверка с кодом

Вердикт: ⚪ **не применимо** — решение о конфиге и правах агентов, продуктовое
поведение не меняет. Что проверено (28.09.2026): состав прав и ролей резолвится
движком (`opencode debug agents`); deny `execute` не ломает read/shell-путь
(смоук — [`rights-probe-2026-09-28.md`](../analysis/rights-probe-2026-09-28.md)
§2); `rustfmt --check src/mcp.rs` без edition падал, с `edition = "2024"` — чисто
(F20). Полная приёмка конфиг-пакета (DoD: `fmt`/`clippy`/`test`) — за
`validator`, не за `migrator`; здесь `cargo` не запускался. Задач не требуется:
канон/конфиг внесены сервисной сессией 28.09.2026; доки T-15 — `docs-writer`.

## Альтернативы

- **Оставить волну 0 пилотом** (вариант (а) [Q50](../questions/Q50.md)) — «две
  истины» в документах. Отклонено.
- **Откатить волну 0** (вариант (в)) — противоречит решениям владельца и фактам
  боевой проверки. Отклонено.
- **Оставить `clippy.toml`/`rustfmt.toml` без edition** — инертные ключи и
  fmt-петли (F20/F33). Отклонено.

## Ссылки

- Вопрос: [Q50](../questions/Q50.md)
- Краткий канон: [`SPECIFICATION.md`](../SPECIFICATION.md) §10, решение №45
- Основание: [`memorandum-W8-run5.md`](../analysis/memorandum-W8-run5.md) §4.6,
  §4.11; [`wave0-report.md`](../tasks/T-15-mcp-ready-process/wave0-report.md) §6
- Пробы: [`rights-probe-2026-09-28.md`](../analysis/rights-probe-2026-09-28.md)
- Связи: [`TRACEABILITY.md`](../TRACEABILITY.md)
