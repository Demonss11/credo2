# D79: Полнота задач — в канон журнала §7

- **Статус:** accepted
- **Дата:** 2026-09-30
- **Resolves:** [Q75](../questions/Q75.md)
- **Спека:** —
- **Affects:** [`.opencode/rules/journal.md`](../../.opencode/rules/journal.md) §7
  (перечень проверок); уточнение к [D77](D77-tasks-visibility-completeness.md)
- **Tasks:** — (служебная зона; правка сервисной сессией; машинная проверка —
  [T-18](../tasks/T-18-docs-journal-test/README.md))

## Контекст

[D77](D77-tasks-visibility-completeness.md) п.6 отложил внесение полноты задач в
[`.opencode/rules/journal.md`](../../.opencode/rules/journal.md) §7 «отдельно»
(служебная зона + аудит). При этом полнота задач уже закреплена в
[`tasks/README.md`](../tasks/README.md) (преамбула) и в карточке
[`T-18`](../tasks/T-18-docs-journal-test/README.md) (проверки v0.1), а §7 канона
журнала её не называет. Полный контекст — [Q75](../questions/Q75.md).

## Решение

1. В §7 канона журнала
   ([`.opencode/rules/journal.md`](../../.opencode/rules/journal.md)) добавить в
   перечень проверок **полноту задач**: каждая `T-XX` из
   [`tasks/README.md`](../tasks/README.md) видна в
   [`TRACEABILITY.md`](../TRACEABILITY.md) (хотя бы одной строкой), статусные
   пометки ⬜/🚧/✅ синхронны реестру — со ссылками
   [D77](D77-tasks-visibility-completeness.md)/D79.
2. Правку вносит **сервисная сессия** по тексту решения, по протоколу
   `AGENTS.md` §«Служебная зона и аудит» (аудит `auditor`, приёмка `validator`).
3. Машинная реализация — [`T-18`](../tasks/T-18-docs-journal-test/README.md)
   (карточка дополнена [D77](D77-tasks-visibility-completeness.md)).

## Следствия

- Канон процесса и тест согласованы: одно требование — один канон (§1
  журнала; [Q41](../questions/Q41.md)/[D60](D60-docs-ownership-sync.md)).
- Вопрос, отложенный [D77](D77-tasks-visibility-completeness.md) п.6, закрыт.
- Правка служебной зоны: до коммита обязателен аудит «инструкция ↔ права»
  (`auditor`).
- **Обновление 30.09.2026** (закрытие P3 аудита операции №4): критерий в §7 выражен ссылкой на абзац «Полнота» в [`tasks/README.md`](../tasks/README.md) (строгий Q41); упоминание теста помечено как план (`T-18` ⬜).

## Сверка с кодом

Вердикт: ⚪ **не применимо** (процесс/служебная зона) — решение правит канон
ведения журнала; продуктовый код прототипа не меняет. Факты по §5.3 (чтением,
30.09.2026):

- [`journal.md`](../../.opencode/rules/journal.md) §7 — текущий перечень проверок
  называет тест [`T-18`](../tasks/T-18-docs-journal-test/README.md) (D64) и не
  содержит пункта о полноте задач;
- карточка [`T-18`](../tasks/T-18-docs-journal-test/README.md) — проверки v0.1
  дополнены полнотой задач ([D77](D77-tasks-visibility-completeness.md));
- [D77](D77-tasks-visibility-completeness.md) п.6 — вопрос «§7» отложен.

`cargo` не запускался (§5.3, [D50](D50-dod-by-package-scope.md)): запись
документная, кода прототипа не меняет.

**Задач не требуется:** правка — служебная зона (сервисная сессия по тексту
решения); машинная проверка — [`T-18`](../tasks/T-18-docs-journal-test/README.md).

## Альтернативы

- **(б) Оставить в `docs/` и в тесте** — отклонено: тест знает о требовании,
  канон — нет (два места истины, дрейф).

## Ссылки

- Вопрос: [Q75](../questions/Q75.md)
- Связанные: [D77](D77-tasks-visibility-completeness.md) (правило полноты; п.6 —
  «§7 отдельно»), [D64](D64-journal-integrity-test.md) (тест целостности, T-18),
  [D60](D60-docs-ownership-sync.md) (один факт — один канон)
- Артефакты: [`.opencode/rules/journal.md`](../../.opencode/rules/journal.md);
  [`TRACEABILITY.md`](../TRACEABILITY.md)
