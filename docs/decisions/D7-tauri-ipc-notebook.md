# D7: Tauri IPC для Notebook

- **Статус:** accepted
- **Дата:** 2026-09-29 (до-журнальное решение; оформлено ретроспективно 29.09.2026, [D69](D69-retro-decisions.md))
- **Resolves:** — (до-журнальное решение, вопроса в журнале нет — допустимо для ретро-D)
- **Спека:** [`SPECIFICATION.md`](../SPECIFICATION.md) §10, №7 (историч.; §10 упраздняется решением [D70](D70-spec-reduction.md))
- **Affects:** [`notebook_ui.feature`](../features/notebook_ui.feature), [`file_management.feature`](../features/file_management.feature)
- **Tasks:** — (приложение Notebook вне периметра `credo2`; см. «Сверка с кодом»)

## Контекст

До-журнальное решение (строка №7 [`SPECIFICATION.md`](../SPECIFICATION.md) §10).
Формулировка: «Tauri IPC для Notebook»; обоснование: «Нативный доступ к файлам и
git». Ретроспективное оформление — [D69](D69-retro-decisions.md).

## Решение

Приложение Notebook строится на **Tauri**; доступ к файловой системе и git
выполняется через нативные команды по Tauri IPC, а не из веб-слоя.

## Следствия

- Файловые/git-операции workspace — на нативной стороне приложения Notebook.
- Серверная сторона прототипа — MCP/REST ([`mcp.rs`](../../src/mcp.rs),
  [`rest.rs`](../../src/rest.rs)); связь Notebook ↔ CREDO — по MCP
  ([D29](D29-notebook-mcp-transport.md)/[D30](D30-execution-mechanism.md)).

## Сверка с кодом

Вердикт: ⚪ **не применимо** — Tauri-приложение Notebook вне кода прототипа
`credo2`.

- **Приложения Notebook нет:** [`src/`](../../src) — Rust-крейт MCP/REST-сервера;
  Tauri/JS-слоя нет.
- **Требование — целевое:** [`features/notebook_ui.feature`](../features/notebook_ui.feature) ⬜,
  [`features/file_management.feature`](../features/file_management.feature) ⬜.

`cargo` не запускался (§5.3, [D50](D50-dod-by-package-scope.md)): ретро-оформление,
кода не меняет.

**Задач не требуется:** решение относится к приложению Notebook, вне периметра
`credo2`.

## Альтернативы

Историей не зафиксированы; Tauri выбран ради нативного доступа к файлам и git
(§10 №9 — Next.js убран как избыточный для desktop).

## Ссылки

- Ретро-оформление: [D69](D69-retro-decisions.md) (Q65)
- Связанные: [D9](D9-nextjs-removed.md) (Next.js убран);
  [D29](D29-notebook-mcp-transport.md) (транспорт MCP в Notebook);
  [D30](D30-execution-mechanism.md) (механизм исполнения); [D70](D70-spec-reduction.md)
- Строка канона: [`SPECIFICATION.md`](../SPECIFICATION.md) §10, №7 (историч.)
- Связи: [`TRACEABILITY.md`](../TRACEABILITY.md)
