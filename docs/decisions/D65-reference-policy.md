# D65: Политика ссылок и дублей — «ссылка, не копия» и время жизни адреса

- **Статус:** accepted
- **Дата:** 2026-09-29
- **Resolves:** [Q61](../questions/Q61.md)
- **Спека:** [`SPECIFICATION.md`](../SPECIFICATION.md) §10, решение №65
- **Affects:** [`BRIEF.md`](../BRIEF.md) (антипаттерны/рецепты — [D62](D62-brief-journal-rules.md));
  [`TRACEABILITY.md`](../TRACEABILITY.md), [`questions/README.md`](../questions/README.md),
  [`decisions/README.md`](../decisions/README.md) (структура — [D63](D63-journal-index-lifecycle.md));
  [`SPECIFICATION.md`](../SPECIFICATION.md) §10; проводники правила —
  [`features/README.md`](../features/README.md), [`tasks/README.md`](../tasks/README.md),
  `tests/docs_journal.rs` ([D64](D64-journal-integrity-test.md) — задача
  [T-18](../tasks/T-18-docs-journal-test/README.md)); реестр находок (`F47`, `F46`)
- **Tasks:** — (см. «Сверка с кодом»)

## Контекст

Документация пронизана перекрёстными ссылками и перечнями; одна правка тянет
3–4 файла (кейс `F46` в
[`findings-registry.md`](../analysis/findings-registry.md)). Запрет ссылок на
номера строк уже зафиксирован в антипаттернах [`BRIEF.md`](../BRIEF.md) §8, но
политика «какие ссылки обязательны, а какие избыточны» и правило «время жизни
адреса» не были зафиксированы. Дополнительный риск — ссылки канона на рабочие
данные, которые удаляются штатно (`F47`). Полный контекст — [Q61](../questions/Q61.md).

## Решение

1. **«Ссылка, не копия»:** формулировка факта живёт в одном каноне
   ([D60](D60-docs-ownership-sync.md) и §10), остальные документы ссылаются, а не
   повторяют текст.
2. **Обязательные ссылки:**
   - `Q` ↔ `D` (взаимные: `Resolves` в D — ссылка на Q, обратно в Q — на D);
   - `D` → задача (`Tasks` в D-файле, `T-XX` в `docs/tasks/`);
   - пометка `# Dn (Qn)` в шапке фичи
     ([`features/*.feature`](../features/), [`BRIEF.md`](../BRIEF.md) §5.6);
   - §10 → `D` (каждая строка решения ведёт на D-файл);
   - связи централизует [`TRACEABILITY.md`](../TRACEABILITY.md) — единая таблица
     жизненного цикла ([D63](D63-journal-index-lifecycle.md)).
3. **Запрещено:**
   - ссылки на номера строк (антипаттерн [`BRIEF.md`](../BRIEF.md) §8);
   - дублирующие перечни правил/формулировок в шапках документов;
   - копии таблицы «Канон чего» — один канон ([D60](D60-docs-ownership-sync.md)/§10),
     остальное — ссылки (находка 3 разведки);
   - пересказ решений в нотах [`features/README.md`](../features/README.md) —
     краткая суть + ссылка на `Dn`, не копия текста (находка 2).
4. **«Время жизни адреса»:** ссылка из канона допустима на артефакт, который
   **штатно не удаляется**. Запрещены ссылки на **удаляемые/сессионные рабочие
   данные** — почту `.opencode/mail/**` (её удаляет `clean-logs.mjs --mail-only`)
   и состояние `.opencode/state/**` (сессионное состояние). Ссылки на
   `docs/analysis/**` **допустимы** как провенанс («улики, не канон»): разборы
   лежат в git и штатно не удаляются (проверено 29.09.2026: `clean-logs.mjs`
   чистит только `memory`/`mail`). **Исключение-реестр:**
   [`docs/analysis/findings-registry.md`](../analysis/findings-registry.md)
   — живой реестр находок (владелец `migrator`, [D48](D48-findings-registry-owner.md)),
   ссылки на него из канона и журнала допустимы.
   Существующие живые адреса `mail/**` в каноне (полный список — исчерпывающий
   свип 29.09.2026) снимаются **исполнением** решения:
   - `docs/questions/`: Q54 (`service-t11-closeout.md`), Q55 (`service-dod-scope.md`),
     Q56 (`service-agent-tools.md`), Q57–Q61 (`service-docs-hygiene.md`);
   - [`questions/README.md`](../questions/README.md) — строка Q56
     (`service-agent-tools.md`), `docs/decisions/`: D49 (`service-t11-closeout.md`),
     D50 (`service-dod-scope.md`), D51 (`service-agent-tools.md`),
     [`decisions/README.md`](../decisions/README.md) — строка D43 (упоминание
     права, не адрес ленты);
   - запись «Чистка документации: заведены вопросы Q57–Q61» в
     [`CHANGELOG.md`](../CHANGELOG.md) (`service-docs-hygiene.md`).
   В [`CHANGELOG.md`](../CHANGELOG.md) — текстом без ссылки на ленту. Адреса
   `mail/**` в задачах `T-15` — вне канона (вне охвата свипа). Проверку правила
   реализует тест [D64](D64-journal-integrity-test.md).
5. **Ориентир бюджета закрытия `Q`:** механические точки — `Q`, `D`, §10,
   [`decisions/README.md`](../decisions/README.md),
   [`questions/README.md`](../questions/README.md),
   [`TRACEABILITY.md`](../TRACEABILITY.md) (≤6 файлов) плюс затронутые
   фичи/задачи; рецепт в затронутых разделах [`BRIEF.md`](../BRIEF.md)
   ([D62](D62-brief-journal-rules.md)).

## Следствия

- Одна правка факта не расходится по 3–4 документам: канон один, остальное —
  ссылки (причина `F46` устранена политикой).
- Ссылки на удаляемые рабочие данные исчезают из канона; провенанс остаётся в
  лентах/памяти и не теряется при штатных чистках (`F47`).
- Обязательные связи фиксируются в [`BRIEF.md`](../BRIEF.md) и проверяются
  тестом [D64](D64-journal-integrity-test.md); бюджет закрытия `Q` (≤6 файлов)
  задаёт ориентир.
- Исполнение (снятие адресов `mail/**` из журнала, ссылки-копии в шапках) — документные
  правки `docs-writer`/`migrator`, отдельными шагами; код прототипа не меняется.

## Сверка с кодом

Вердикт: ⚪ **не применимо** (процесс/документы) — политика ссылок и связей
касается документации, кода прототипа не меняет; машинная часть (проверка
правила) покрывается тестом [D64](D64-journal-integrity-test.md) (задача
[T-18](../tasks/T-18-docs-journal-test/README.md)).

Что проверено (чтением, 29.09.2026), чем подтверждено:

- **Причина (кейс `F46`)** и находки 2/3 разведки — в
  [`findings-registry.md`](../analysis/findings-registry.md): правка одной
  записи тянет 3–4 файла; пересказ решений в нотах
  [`features/README.md`](../features/README.md); 4–5 копий «Канон чего».
- **Запрет номеров строк** уже в [`BRIEF.md`](../BRIEF.md) §8; политика его
  воспроизводит, а не вводит заново.
- **Существующие адреса `mail/**` в каноне** (полный список):
  `docs/questions/` Q54 (`service-t11-closeout.md`), Q55 (`service-dod-scope.md`),
  Q56 (`service-agent-tools.md`), Q57–Q61 (`service-docs-hygiene.md`); строка Q56
  в [`questions/README.md`](../questions/README.md); `docs/decisions/` D49
  (`service-t11-closeout.md`), D50 (`service-dod-scope.md`), D51
  (`service-agent-tools.md`); запись в [`CHANGELOG.md`](../CHANGELOG.md) —
  объект исполнения (п.4).
- **Тест-проводник:** `tests/docs_journal.rs` не существует (см. «Сверка» D64);
  правило «время жизни адреса» станет его проверкой п.8.

`cargo` не запускался (§5.3, [D50](D50-dod-by-package-scope.md)): решение
документное, кода не касается; адресный прогон не требуется (проверка —
задача T-18 по D64).

**Задач не требуется:** снятие адресов `mail/**` и дублей — документные правки
`docs/**`; машинная проверка правила реализуется тестом
[D64](D64-journal-integrity-test.md) (задача
[T-18](../tasks/T-18-docs-journal-test/README.md)). Отдельной задачи по коду
политика не порождает.

## Альтернативы

- **Минимум ссылок** (только прямые Q↔D) — отклонено: дешевле в поддержке, но
  теряется навигация и связь с задачами/фичами.
- **Оставить как есть** (практика сложилась) — отклонено: перегрузка и
  дублирование сохраняются, кейс `F46` воспроизводится, ссылки на удаляемые
  рабочие данные остаются.

## Ссылки

- Вопрос: [Q61](../questions/Q61.md)
- Связанные: [D63](D63-journal-index-lifecycle.md) (единая таблица/каталоги);
  [D64](D64-journal-integrity-test.md) (тест — проводник правила);
  [D62](D62-brief-journal-rules.md) (антипаттерны/рецепты [`BRIEF.md`](../BRIEF.md));
  [D60](D60-docs-ownership-sync.md) (политика «один факт — один канон»)
- Реестр: `F47`/`F46` в [`findings-registry.md`](../analysis/findings-registry.md)
- Краткий канон: [`SPECIFICATION.md`](../SPECIFICATION.md) §10, решение №65
- Связи: [`TRACEABILITY.md`](../TRACEABILITY.md)
