# D9: Next.js убран

- **Статус:** accepted
- **Дата:** 2026-09-29 (до-журнальное решение; оформлено ретроспективно 29.09.2026, [D69](D69-retro-decisions.md))
- **Resolves:** — (до-журнальное решение, вопроса в журнале нет — допустимо для ретро-D)
- **Спека:** [`SPECIFICATION.md`](../SPECIFICATION.md) §10, №9 (историч.; §10 упраздняется решением [D70](D70-spec-reduction.md))
- **Affects:** [`notebook_ui.feature`](../features/notebook_ui.feature) (целевой стек приложения)
- **Tasks:** — (отрицательное решение; см. «Сверка с кодом»)

## Контекст

До-журнальное решение (строка №9 [`SPECIFICATION.md`](../SPECIFICATION.md) §10).
Формулировка: «Next.js убран»; обоснование: «Избыточен для desktop».
Ретроспективное оформление — [D69](D69-retro-decisions.md).

## Решение

Next.js **не используется**: Notebook — desktop-приложение (Tauri,
[D7](D7-tauri-ipc-notebook.md)), а не веб-фронтенд на Next.js.

## Следствия

- Стек приложения Notebook — Tauri/Editor без Next.js-слоя.
- Отрицательное решение: кода прототипа не касается.

## Сверка с кодом

Вердикт: ⚪ **не применимо** — решение отрицательное (отказ от технологии); в
коде прототипа `credo2` нечего проверять.

- **Next.js-фронтенда в репозитории нет:** корень — Rust-крейт
  ([`src/`](../../src)) без Node-приложения; в `.opencode/` Node-пакеты служебные
  (плагин/скрипты), не frontend.
- Требование UI отслеживается фичей [`notebook_ui.feature`](../features/notebook_ui.feature) ⬜.

`cargo` не запускался (§5.3, [D50](D50-dod-by-package-scope.md)): ретро-оформление,
кода не меняет.

**Задач не требуется:** отказ от Next.js — свойство целевого стека приложения
Notebook, вне периметра `credo2`.

## Альтернативы

Историей не зафиксированы; отклонён сам Next.js (избыточен для desktop).

## Ссылки

- Ретро-оформление: [D69](D69-retro-decisions.md) (Q65)
- Связанные: [D7](D7-tauri-ipc-notebook.md) (Tauri — целевой стек);
  [D70](D70-spec-reduction.md)
- Строка канона: [`SPECIFICATION.md`](../SPECIFICATION.md) §10, №9 (историч.)
- Связи: [`TRACEABILITY.md`](../TRACEABILITY.md)
