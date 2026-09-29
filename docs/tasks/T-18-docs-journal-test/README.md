# T-18. Тест целостности журнала `tests/docs_journal.rs`

- **Статус:** ⬜ открыта
- **Приоритет:** P3 (после демо, v0.1.x — процессная гигиена документации; продукт
  не меняет)
- **Зависит от:** после исполнения [D61](../../decisions/D61-archive-removal.md),
  [D62](../../decisions/D62-brief-journal-rules.md),
  [D63](../../decisions/D63-journal-index-lifecycle.md),
  [D65](../../decisions/D65-reference-policy.md) (новая структура индекса и шапок)
- **Источник:** [D64](../../decisions/D64-journal-integrity-test.md) (Q60);
  образец — [`../../tests/features_inventory.rs`](../../tests/features_inventory.rs)
  ([D20](../../decisions/D20-features-docs-dod.md)/Q40); связано: D63, D65, D61

## Контекст

Целостность журнала Q/D проверяется ручными свипами, а `P3`-дрейф номеров строк
между прогонами ловится глазами ([`../../BRIEF.md`](../../BRIEF.md) §7 обещает
тест `tests/docs_journal.rs`; критерий §7 (историч.; выполнен D61) требует «тест целостности добавлен»).
Решение [D64](../../decisions/D64-journal-integrity-test.md) закрепляет
машинную проверку по образцу
[`../../tests/features_inventory.rs`](../../tests/features_inventory.rs).

## Что сделать

Реализовать `tests/docs_journal.rs` (Rust, только `std`, без новых внешних
зависимостей):

- **v0.1 (ядро):** уникальность/целостность ID `Q`/`D`; парность `Q`↔`D` (кроме
  `dropped`/«попутных»); у `D` — строка §10 со ссылкой `[Dn]` и строка в
  [`../../decisions/README.md`](../../decisions/README.md); у `Q` — строки в
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
