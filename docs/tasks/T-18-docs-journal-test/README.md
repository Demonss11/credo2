# T-18. Тест целостности журнала `tests/docs_journal.rs`

- **Статус:** ⬜ открыта
- **Приоритет:** P3 (после демо, v0.1.x — процессная гигиена документации; продукт
  не меняет)
- **Зависит от:** после исполнения [D61](../../decisions/D61-archive-removal.md),
  [D62](../../decisions/D62-brief-journal-rules.md),
  [D63](../../decisions/D63-journal-index-lifecycle.md),
  [D65](../../decisions/D65-reference-policy.md),
  [D71](../../decisions/D71-journal-rules-relocation.md) (новая структура индекса,
  шапок и правил журнала)
- **Источник:** [D64](../../decisions/D64-journal-integrity-test.md) (Q60);
  образец — [`../../tests/features_inventory.rs`](../../tests/features_inventory.rs)
  ([D20](../../decisions/D20-features-docs-dod.md)/Q40); связано: D63, D65, D61;
  дополнено [D77](../../decisions/D77-tasks-visibility-completeness.md) (Q73) — полнота задач;
  дополнено [D80](../../decisions/D80-features-visibility-completeness.md) (Q76) — полнота фич

## Контекст

Целостность журнала Q/D проверяется ручными свипами, а `P3`-дрейф номеров строк
между прогонами ловится глазами
([`../../../.opencode/rules/journal.md`](../../../.opencode/rules/journal.md) §7
обещает тест `tests/docs_journal.rs`; критерий §7 (историч.; выполнен D61)
требует «тест целостности добавлен»).
Решение [D64](../../decisions/D64-journal-integrity-test.md) закрепляет
машинную проверку по образцу
[`../../tests/features_inventory.rs`](../../tests/features_inventory.rs).

## Что сделать

Реализовать `tests/docs_journal.rs` (Rust, только `std`, без новых внешних
зависимостей):

- **v0.1 (ядро):** уникальность/целостность ID `Q`/`D`; парность `Q`↔`D` (кроме
  `dropped`/«попутных», а также ретро-D без Q — допустимы
  ([D69](../../decisions/D69-retro-decisions.md): `D1`–`D5`, `D7`–`D11`, `D13`); у `D` — запись в
  [`../../decisions/README.md`](../../decisions/README.md) (§10-адреса —
  исторические, не проверяются); у `Q` — строки в
  [`../../questions/README.md`](../../questions/README.md) и
  [`../../TRACEABILITY.md`](../../TRACEABILITY.md); вердикт «Сверка с кодом» в
  каждом `D`; согласованность `TRACEABILITY` (задачи `T-XX` существуют;
  жизненный цикл — из словаря
  [D63](../../decisions/D63-journal-index-lifecycle.md)); запреты — номера
  строк, миграционные маркеры («ожидает переноса», `OPEN_QUESTIONS.md`, «до
  конца миграции»), ссылки канона на удаляемые/сессионные данные
  (`.opencode/mail/**`, `.opencode/state/**`) ([D65](../../decisions/D65-reference-policy.md));
  `docs/analysis/**` — допустим (провенанс);
  исключение (whitelisted) — [`findings-registry.md`](../../analysis/findings-registry.md)
  (живой реестр находок, владелец `migrator`, [D48](../../decisions/D48-findings-registry-owner.md));
- полнота задач: каждая `T-XX` из [`../../tasks/README.md`](../../tasks/README.md)
  встречается хотя бы в одной строке
  [`../../TRACEABILITY.md`](../../TRACEABILITY.md); обратно — каждая `T-XX` в
  `TRACEABILITY` имеет карточку в `docs/tasks/`; статусные пометки ⬜/🚧/✅ в
  `TRACEABILITY` совпадают с реестром
  ([D77](../../decisions/D77-tasks-visibility-completeness.md));
- полнота фич: каждая `*.feature` из [`../../features/README.md`](../../features/README.md)
  встречается поимённо в [`../../TRACEABILITY.md`](../../TRACEABILITY.md) (колонка
  «Реализация»); wildcard-обобщения (`agents-*`, «и др.») не допускаются; обратно —
  каждый `*.feature` из `TRACEABILITY` существует
  ([D80](../../decisions/D80-features-visibility-completeness.md));
- **v0.2:** полный link-check относительных ссылок журнала (вне исторических зон
  `docs/reviews/**`, `docs/analysis/**`).

## Критерий готовности

`cargo test --all` включает тест `docs_journal` и он зелёный; DoD задачи
([`../README.md`](../README.md) §«DoD для любой задачи»).

## Примечания

- Исполнитель — `tester` (владелец `tests/**`); прогон `cargo test --all`
  выполняет `validator`.
- Проверки намеренно вне исторических зон `docs/reviews/**` и
  `docs/analysis/**` (там допустимы старые адреса и номера строк — они снимки).
- Зависимость от D61–D63/D65 — чтобы тест опирался на целевую структуру индекса
  и шапок, а не на переходное состояние.
- Реестр находок: `F46` (правка тянет 3–4 файла) и `F47` (ссылки канона на
  удаляемые рабочие данные) —
  [`../../analysis/findings-registry.md`](../../analysis/findings-registry.md).
