# D5: Клиент LSP — `codemirror-languageserver`

- **Статус:** accepted
- **Дата:** 2026-09-29 (до-журнальное решение; оформлено ретроспективно 29.09.2026, [D69](D69-retro-decisions.md))
- **Resolves:** — (до-журнальное решение, вопроса в журнале нет — допустимо для ретро-D)
- **Спека:** [`SPECIFICATION.md`](../SPECIFICATION.md) §10, №5 (историч.; §10 упраздняется решением [D70](D70-spec-reduction.md))
- **Affects:** [`lsp_notebook.feature`](../features/lsp_notebook.feature), [`notebook_ui.feature`](../features/notebook_ui.feature)
- **Tasks:** — (клиентский слой вне периметра `credo2`; см. «Сверка с кодом»)

## Контекст

До-журнальное решение (строка №5 [`SPECIFICATION.md`](../SPECIFICATION.md) §10).
Формулировка: «Клиент LSP: `codemirror-languageserver`»; обоснование:
«Проверенное решение для начала». Ретроспективное оформление —
[D69](D69-retro-decisions.md).

## Решение

На стороне Notebook LSP-клиент строится на `codemirror-languageserver`
(Editor-компонент Notebook), а не на самописном протокольном клиенте.

## Следствия

- Выбор клиента — свойство приложения Notebook (frontend), а не Rust-прототипа.
- Связка диагностика/completion/деградация — [D6](D6-lsp-degradation.md),
  [`lsp_notebook.feature`](../features/lsp_notebook.feature).

## Сверка с кодом

Вердикт: ⚪ **не применимо** — клиентская библиотека относится к приложению
Notebook, вне кода прототипа `credo2`.

- **Frontend/Notebook в репозитории нет:** [`src/`](../../src) — только Rust-крейт
  прототипа; Node/CodeMirror-клиента нет.
- **Требование — целевое:** [`features/lsp_notebook.feature`](../features/lsp_notebook.feature)
  ⬜ (sidecar/диагностика в CodeMirror); [`features/notebook_ui.feature`](../features/notebook_ui.feature) ⬜.

`cargo` не запускался (§5.3, [D50](D50-dod-by-package-scope.md)): ретро-оформление,
кода не меняет.

**Задач не требуется:** решение — архитектурный ориентир для приложения Notebook;
вне периметра `credo2` (реализация UI — по [`lsp_notebook.feature`](../features/lsp_notebook.feature)).

## Альтернативы

Историей не зафиксированы; библиотека выбрана как проверенное решение для старта.

## Ссылки

- Ретро-оформление: [D69](D69-retro-decisions.md) (Q65)
- Связанные: [D3](D3-lsp-sidecar-process.md) (sidecar-процесс);
  [D4](D4-lsp-transport-stdio.md) (транспорт stdio); [D6](D6-lsp-degradation.md)
  (деградация без fallback); [D70](D70-spec-reduction.md)
- Строка канона: [`SPECIFICATION.md`](../SPECIFICATION.md) §10, №5 (историч.)
- Связи: [`TRACEABILITY.md`](../TRACEABILITY.md)
