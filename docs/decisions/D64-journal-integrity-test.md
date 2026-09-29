# D64: Тест целостности журнала — `tests/docs_journal.rs`

- **Статус:** accepted
- **Дата:** 2026-09-29
- **Resolves:** [Q60](../questions/Q60.md)
- **Спека:** [`SPECIFICATION.md`](../SPECIFICATION.md) §10, решение №64
- **Affects:** новый `tests/docs_journal.rs` (образец — `tests/features_inventory.rs`,
  [Q40](../questions/Q40.md)/[D20](D20-features-docs-dod.md));
  [`TRACEABILITY.md`](../TRACEABILITY.md), [`questions/README.md`](../questions/README.md),
  [`decisions/README.md`](../decisions/README.md), §10 [`SPECIFICATION.md`](../SPECIFICATION.md);
  [`BRIEF.md`](../BRIEF.md) (раздел «Целостность журнала» — [D62](D62-brief-journal-rules.md));
  задача [`T-18`](../tasks/T-18-docs-journal-test/README.md); согласовано с
  [D61](D61-archive-removal.md), [D63](D63-journal-index-lifecycle.md),
  [D65](D65-reference-policy.md)
- **Tasks:** [T-18](../tasks/T-18-docs-journal-test/README.md) (см. «Сверка с кодом»)

## Контекст

[`BRIEF.md`](../BRIEF.md) §7 описывает проверку целостности журнала вручную и
обещает тест `tests/docs_journal.rs` из «плана v0.x»; критерий завершения
миграции ([`BRIEF.md`](../BRIEF.md) §7, историч.; выполнен [D61](D61-archive-removal.md)) требует «тест целостности добавлен».
Сейчас целостность проверяется ручными свипами, а `P3`-дрейф номеров строк
между прогонами ловится глазами. Машинная проверка по образцу
[`tests/features_inventory.rs`](../../tests/features_inventory.rs) убрала бы
ручные свипы. Полный контекст — [Q60](../questions/Q60.md).

## Решение

1. **Реализовать интеграционный тест `tests/docs_journal.rs`** (Rust, только
   `std` — по образцу [`tests/features_inventory.rs`](../../tests/features_inventory.rs);
   новые внешние зависимости не требуются). Проверки:
   1. **Уникальность и целостность ID** Q/D: файлы `docs/questions/Qn.md` и
      `docs/decisions/Dn-<слаг>.md`, номера не дублируются и не
      переиспользуются.
   2. **Парность Q↔D:** у `Q` есть решение (`Resolves`) либо явное исключение
      (`dropped`/«закрыт попутно»); у каждого `D` — существующий вопрос `Q`.
   3. **Строки-представления:** у `D` — строка §10 со ссылкой `[Dn]` и строка в
      [`decisions/README.md`](../decisions/README.md); у `Q` — строки в
      [`questions/README.md`](../questions/README.md) и
      [`TRACEABILITY.md`](../TRACEABILITY.md).
   4. **Вердикт «Сверка с кодом»** присутствует в каждом `D`.
   5. **Согласованность [`TRACEABILITY.md`](../TRACEABILITY.md):** задачи `T-XX`
      существуют в `docs/tasks/`; значение жизненного цикла — из словаря
      [D63](D63-journal-index-lifecycle.md) (`open` · `resolved` · `in work` ·
      `done` · `dropped`).
   6. **Живые относительные ссылки** журнала (вне исторических зон
      `docs/reviews/**`, `docs/analysis/**`).
   7. **Запреты:** адреса с номерами строк; миграционные маркеры («ожидает
      переноса», `OPEN_QUESTIONS.md`, «до конца миграции») вне исторических зон.
   8. **«Время жизни адреса»** ([D65](D65-reference-policy.md)): нет ссылок
      канона на **удаляемые/сессионные** рабочие данные — `.opencode/mail/**`,
      `.opencode/state/**`; ссылки на `docs/analysis/**` — **допустимы** как
      провенанс (не проверяются), исключение (whitelisted) —
      [`findings-registry.md`](../analysis/findings-registry.md) (живой реестр,
      владелец `migrator`, [D48](D48-findings-registry-owner.md)).
2. **Поэтапность.** v0.1 — проверки 1–5, 7–8 (ID/парность/таблицы/запреты);
   полный link-check (п.6) — v0.2 (сначала ссылки-кандидаты, затем блокирующая
   проверка), чтобы тест не был хрупким на исторических зонах.
3. **Задача на реализацию** — [`T-18`](../tasks/T-18-docs-journal-test/README.md)
   (исполнитель `tester`; источник `D64 (Q60)`; зависимость — после исполнения
   [D61](D61-archive-removal.md)/[D62](D62-brief-journal-rules.md)/[D63](D63-journal-index-lifecycle.md)/[D65](D65-reference-policy.md),
   чтобы проверки опирались на новую структуру индекса и шапок).

## Следствия

- Ручные свипы целостности журнала заменяются машинной проверкой в `cargo test`
  (DoD прототипа); `P3`-дрейф номеров строк и битые ссылки ловятся тестом.
- Тест — часть `tests/`, поэтому его прогон входит в DoD `cargo test --all`
  ([`AGENTS.md`](../../AGENTS.md) §Сборка, [D20](D20-features-docs-dod.md));
  отдельного прогона `validator` для документации решение не вводит.
- Раздел «Целостность журнала» в [`BRIEF.md`](../BRIEF.md) получает машинный
  основной механизм ([D62](D62-brief-journal-rules.md)); ручной чек-лист
  становится вспомогательным.
- Тест зависит от структуры, вводимой [D63](D63-journal-index-lifecycle.md)
  (жизненный цикл, каталоги) и [D65](D65-reference-policy.md) (время жизни
  адреса) — отсюда порядок реализации.

## Сверка с кодом

Вердикт: ⬜ **не реализовано** — теста `tests/docs_journal.rs` нет; решение
порождает задачу [T-18](../tasks/T-18-docs-journal-test/README.md).

Что проверено (чтением, 29.09.2026), чем подтверждено:

- **Теста нет:** `glob tests/*.rs` → `rest.rs`, `publish.rs`, `mcp_errors.rs`,
  `mcp_draft.rs`, `features_inventory.rs`; `docs_journal.rs` отсутствует.
- **Образец есть:** [`tests/features_inventory.rs`](../../tests/features_inventory.rs)
  — интеграционный тест на `std` без внешних зависимостей (`env!("CARGO_MANIFEST_DIR")`,
  `fs`, `BTreeMap`), doc-комментарий с источником (Q40); тот же подход применим
  к журналу.
- **Проверяемые артефакты существуют:** `docs/questions/Qn.md`,
  `docs/decisions/Dn-<слаг>.md`, [`TRACEABILITY.md`](../TRACEABILITY.md),
  [`questions/README.md`](../questions/README.md),
  [`decisions/README.md`](../decisions/README.md), `docs/tasks/`,
  §10 [`SPECIFICATION.md`](../SPECIFICATION.md).

`cargo` не запускался (§5.3, [D50](D50-dod-by-package-scope.md)): тест ещё не
написан, прогонять нечего; реализация — `tester` (задача T-18).

**Задача:** [T-18](../tasks/T-18-docs-journal-test/README.md) — реализация теста
целостности журнала (исполнитель `tester`). Это единственная задача решения.

## Альтернативы

- **Минимальный тест** (только уникальность ID и живость ссылок) — отклонено:
  не покрывает парность Q↔D, строки §10/сводок и согласованность задач —
  основной источник дрейфа F46.
- **Отложить** (оставить ручные свипы) — отклонено: критерий завершения миграции
  ([`BRIEF.md`](../BRIEF.md) §7, историч.; выполнен [D61](D61-archive-removal.md)) прямо требует тест целостности; ручные свипы
  дороги и ловят `P3`-дрейф глазами.

## Ссылки

- Вопрос: [Q60](../questions/Q60.md)
- Связанные: [D63](D63-journal-index-lifecycle.md) (структура индекса/жизненный
  цикл); [D65](D65-reference-policy.md) (время жизни адреса); [D61](D61-archive-removal.md)
  (судьба архива); [D62](D62-brief-journal-rules.md) (раздел «Целостность
  журнала»); [Q40](../questions/Q40.md)/[D20](D20-features-docs-dod.md) (образец
  теста и DoD)
- Задача: [T-18](../tasks/T-18-docs-journal-test/README.md)
- Краткий канон: [`SPECIFICATION.md`](../SPECIFICATION.md) §10, решение №64
- Связи: [`TRACEABILITY.md`](../TRACEABILITY.md)
