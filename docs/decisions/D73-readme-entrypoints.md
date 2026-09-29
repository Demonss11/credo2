# D73: README — корневой создать; `docs/README.md` отрефакторить

- **Статус:** accepted
- **Дата:** 2026-09-29
- **Resolves:** [Q69](../questions/Q69.md)
- **Спека:** — (§10 упраздняется решением [D70](D70-spec-reduction.md))
- **Affects:** `README.md` (корневой, новый, ~30 строк);
  [`docs/README.md`](../README.md) (лёгкая карта);
  [`AGENTS.md`](../../AGENTS.md) (§Сборка — цель ссылки);
  [D60](D60-docs-ownership-sync.md) (политика «один факт — один канон»)
- **Tasks:** — (см. «Сверка с кодом»)

## Контекст

Корневого `README.md` в репозитории нет — у прототипа отсутствует точка входа
для человека. [`docs/README.md`](../README.md) (62 строки) выполняет роль карты,
но дублирует таблицу «Канон чего» из [`../../AGENTS.md`](../../AGENTS.md)
§Документы и решения ([D60](D60-docs-ownership-sync.md)), пересказывает решения об
архиве/индексе журнала ([D61](D61-archive-removal.md)/[D63](D63-journal-index-lifecycle.md)),
содержит Q-упоминания и ссылается на `BRIEF.md` (переезд — [D71](D71-journal-rules-relocation.md)).
Полный контекст — [Q69](../questions/Q69.md).

## Решение

1. **Создать корневой `README.md`** (~30 строк): что это, быстрый старт
   (`cargo build --release` → `target/release/credo2.exe`; `opencode mcp list`;
   полная сборка/тесты/DoD — ссылкой на [`../../AGENTS.md`](../../AGENTS.md)
   §Сборка), структура (`src/ tests/ docs/ .opencode/ .credo/`), документация
   (ссылки на [`docs/README.md`](../README.md) и [`../../AGENTS.md`](../../AGENTS.md)).
2. **[`docs/README.md`](../README.md) — лёгкая карта:** файлы, «куда за чем»,
   канон — одной строкой-ссылкой; **без таблицы-дубля** канонов и **без
   Q-упоминаний**.
3. Без дублей: описание инструментов CREDO и синтаксис правила — только в
   [`../../AGENTS.md`](../../AGENTS.md); команды сборки — ссылкой.

## Следствия

- У репозитория появляется точка входа; `docs/README.md` перестаёт дублировать
  таблицу канонов ([D60](D60-docs-ownership-sync.md)).
- Ссылки на правила журнала — на новое место после [D71](D71-journal-rules-relocation.md)
  (`.opencode/rules/journal.md`).
- Объём SPEC/GRAMMAR — по [D70](D70-spec-reduction.md)/[D74](D74-grammar-normative-focus.md).

## Сверка с кодом

Вердикт: ⚪ **не применимо** (документы) — решение документное, кода прототипа не
меняет.

Что проверено (чтением, 29.09.2026), чем подтверждено:

- **Корневого `README.md` нет:** `glob README.md` в корне — файла нет;
  [`docs/README.md`](../README.md) — 62 строки (карта с дублями и Q-упоминаниями).
- **Цель ссылки — [`../../AGENTS.md`](../../AGENTS.md) §Сборка:** команды
  `cargo build --release`, `cargo fmt --check`, `cargo clippy`, `cargo test --all`.

`cargo` не запускался (§5.3, [D50](D50-dod-by-package-scope.md)): решение
документное, кода не меняет; адресный прогон не требуется.

**Задач не требуется:** правки — документные (создание корневого README и
рефакторинг `docs/README.md`), исполняются сервисной сессией.

## Альтернативы

- **Корневой расширенный** (инструменты CREDO, синтаксис) — отклонено: дубли с
  [`../../AGENTS.md`](../../AGENTS.md); ссылка вместо копии.
- **Только рефакторинг `docs/README.md`** — отклонено: точки входа для человека
  не появляется.

## Ссылки

- Вопрос: [Q69](../questions/Q69.md)
- Связанные: [D60](D60-docs-ownership-sync.md) (владение каноном);
  [D70](D70-spec-reduction.md) (объём SPEC); [D71](D71-journal-rules-relocation.md)
  (переезд правил журнала); [D61](D61-archive-removal.md)/[D63](D63-journal-index-lifecycle.md)
- Связи: [`TRACEABILITY.md`](../TRACEABILITY.md)
