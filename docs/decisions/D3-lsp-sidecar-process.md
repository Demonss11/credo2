# D3: LSP как sidecar-процесс

- **Статус:** accepted
- **Дата:** 2026-09-29 (до-журнальное решение; оформлено ретроспективно 29.09.2026, [D69](D69-retro-decisions.md))
- **Resolves:** — (до-журнальное решение, вопроса в журнале нет — допустимо для ретро-D)
- **Спека:** [`SPECIFICATION.md`](../SPECIFICATION.md) §10, №3 (историч.; §10 упраздняется решением [D70](D70-spec-reduction.md))
- **Affects:** [`lsp.feature`](../features/lsp.feature), [`lsp_notebook.feature`](../features/lsp_notebook.feature)
- **Tasks:** — (вне периметра MVP; см. «Сверка с кодом»)

## Контекст

До-журнальное решение (строка №3 [`SPECIFICATION.md`](../SPECIFICATION.md) §10).
Формулировка: «LSP как sidecar-процесс»; обоснование: «Единый код для Notebook и
внешних IDE». Ретроспективное оформление — [D69](D69-retro-decisions.md).
Поведение при падении sidecar — [D6](D6-lsp-degradation.md).

## Решение

Языковой сервер (`dar-core`/LSP) поставляется как **отдельный
sidecar-процесс**, а не как библиотека, встроенная в клиентов: один движок
обслуживает Notebook и внешние IDE.

## Следствия

- Клиенты (Notebook, IDE) общаются с LSP по протоколу, а не через прямой API.
- Диагностика/completion/hover и деградация без fallback — [D6](D6-lsp-degradation.md).

## Сверка с кодом

Вердикт: ⬜ **не реализовано** — LSP-сервер в прототипе — целевое v0.2.

- **Кода LSP нет:** [`src/`](../../src) — только `main.rs`, `mcp.rs`, `rest.rs`,
  `core.rs`, `lib.rs`; sidecar-процесса/`lsp`-крейта нет.
- **Требование — целевое:** [`features/lsp.feature`](../features/lsp.feature) ⬜,
  [`features/lsp_notebook.feature`](../features/lsp_notebook.feature) ⬜
  ([`features/README.md`](../features/README.md) `:214–215`).
- Вне периметра MVP ([D18](D18-pipelines-out-lsp-mvp.md) — состав LSP MVP;
  реализация LSP-сервера — v0.2).

`cargo` не запускался (§5.3, [D50](D50-dod-by-package-scope.md)): ретро-оформление,
кода не меняет.

**Задач не требуется:** решение — архитектурный ориентир вне периметра MVP;
требование отслеживается фичей [`lsp.feature`](../features/lsp.feature) (v0.2).

## Альтернативы

Историей не зафиксированы; выбор «sidecar-процесс» мотивирован единым кодом для
Notebook и внешних IDE.

## Ссылки

- Ретро-оформление: [D69](D69-retro-decisions.md) (Q65)
- Связанные: [D6](D6-lsp-degradation.md) (деградация при падении LSP);
  [D18](D18-pipelines-out-lsp-mvp.md) (состав LSP MVP); [D4](D4-lsp-transport-stdio.md),
  [D5](D5-lsp-client-codemirror.md) (транспорт/клиент); [D70](D70-spec-reduction.md)
- Строка канона: [`SPECIFICATION.md`](../SPECIFICATION.md) §10, №3 (историч.)
- Связи: [`TRACEABILITY.md`](../TRACEABILITY.md)
