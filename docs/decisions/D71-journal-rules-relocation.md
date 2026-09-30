# D71: Правила журнала → `.opencode/rules/journal.md`; `docs/BRIEF.md` удаляется

- **Статус:** accepted
- **Дата:** 2026-09-29
- **Resolves:** [Q67](../questions/Q67.md)
- **Спека:** — (§10 упраздняется решением [D70](D70-spec-reduction.md))
- **Affects:** `.opencode/rules/journal.md` (новый, ~90 строк);
  `docs/BRIEF.md` (удаляется);
  [`AGENTS.md`](../../AGENTS.md), [`../../.opencode/agents/*`](../../.opencode/agents),
  [`docs/README.md`](../README.md), [`docs/tasks/README.md`](../tasks/README.md),
  [T-18](../tasks/T-18-docs-journal-test/README.md)/[T-19](../tasks/T-19-doc-quality-checks/README.md),
  [`SPECIFICATION.md`](../SPECIFICATION.md), D-файлы (ссылки на `BRIEF.md`);
  [D66](D66-doc-quality-checks.md) (исключения doc-size)
- **Tasks:** — (правки — в тексте решения; служебная зона — по протоколу)

## Контекст

`docs/BRIEF.md` (307 строк; историч.) вырос вокруг миграции архива Q/D: живые
правила (ID, жизненный цикл, шаблоны Q/D, §5.3 сверка, антипаттерны) соседствуют
с миграционным осадком и дублями с [D50](D50-dod-by-package-scope.md) и
[`../../.opencode/rules/review.md`](../../.opencode/rules/review.md). Место
процессных правил — рядом с правилами ролей в `.opencode/rules/`, а не в `docs/`,
где живёт продуктовый канон. На `BRIEF.md` ссылаются ~50 мест в ~19 файлах ядра;
удаление в одиночку рвёт ссылки (ловит link-check, [T-18](../tasks/T-18-docs-journal-test/README.md)).
Полный контекст — [Q67](../questions/Q67.md).

## Решение

1. **Перенести живые правила** в `.opencode/rules/journal.md` (~90 строк):
   принципы, ID и **сквозная нумерация D** (согласовано с
   [D70](D70-spec-reduction.md)), жизненный цикл, шаблоны Q/D, рецепты §5.1–§5.6,
   связи/целостность, антипаттерны.
2. **`docs/BRIEF.md` удалить.**
3. **Обновить ссылки одним пакетом** (~50 мест в ~19 файлах ядра): роли
   `.opencode/agents/*`, [`AGENTS.md`](../../AGENTS.md), [`docs/README.md`](../README.md),
   [`docs/tasks/README.md`](../tasks/README.md), карточки T-18/T-19,
   [`SPECIFICATION.md`](../SPECIFICATION.md), D-файлы. Исторические
   `Q`/`reviews`/`analysis`/`mail` — **не трогать**.
4. **[D66](D66-doc-quality-checks.md):** убрать `BRIEF.md` из исключений doc-size
   (файл удаляется); `journal.md` — в охвате зоны правил, отдельное исключение не
   требуется (короче лимита); ссылки D66 на `BRIEF.md` → `journal.md`.
5. **Правки канона агентов** (`.opencode/**`, [`AGENTS.md`](../../AGENTS.md)) —
   по протоколу: журнал → правки → аудит `auditor` → приёмка `validator`.

## Следствия

- Процессные правила — рядом с правилами ролей; в `docs/` остаётся продуктовый
  канон.
- [D62](D62-brief-journal-rules.md) **superseded в части BRIEF**: правило «BRIEF.md
  — компактные правила журнала» заменяется переездом в `.opencode/rules/journal.md`.
- Удаление `BRIEF.md` и правки ссылок идут одним пакетом (риск link-check).
- Исключения doc-size в [D66](D66-doc-quality-checks.md) теряют `BRIEF.md`.

## Сверка с кодом

Вердикт: ⚪ **не применимо** (процесс/документы/служебная зона) — решение
документно-процессное, кода прототипа не меняет.

Что проверено (чтением, 29.09.2026), чем подтверждено:

- **`BRIEF.md` существует и объёмный:** `docs/BRIEF.md` (историч.) — 307 строк;
  ссылки на него — ~50 мест в ~19 файлах (карта разведки [Q67](../questions/Q67.md)).
- **Место-цель существует:** [`.opencode/rules/`](../../.opencode/rules) —
  `review.md`, `workspace.md`, `dispatch-loop.md`, `git-workflow.md`.
- **Исключения doc-size:** [D66](D66-doc-quality-checks.md) перечисляет `BRIEF.md`
  среди исключений-«книг»; в `journal.md` — < 300 строк.

`cargo` не запускался (§5.3, [D50](D50-dod-by-package-scope.md)): решение
документное, кода не меняет; адресный прогон не требуется.

**Задач не требуется:** правки — документные и служебные, исполняются сервисной
сессией по тексту решения; правки канона агентов — по протоколу (аудит `auditor`).

## Альтернативы

- **Сжать `BRIEF.md` на месте (~100 строк)** — отклонено владельцем: процессные
  правила не место в `docs/`.
- **Удалить без переноса** (правила — в `review.md`/`AGENTS.md`) — отклонено:
  смешение зон ответственности и перегрузка правил ревью.

## Ссылки

- Вопрос: [Q67](../questions/Q67.md)
- Связанные: [D62](D62-brief-journal-rules.md) (superseded в части BRIEF);
  [D70](D70-spec-reduction.md) (сквозная нумерация D); [D66](D66-doc-quality-checks.md)
  (исключения doc-size); [D63](D63-journal-index-lifecycle.md) (индекс журнала);
  [D65](D65-reference-policy.md) (ссылки)
- Задача: [T-18](../tasks/T-18-docs-journal-test/README.md) (link-check)
- Связи: [`TRACEABILITY.md`](../TRACEABILITY.md)
