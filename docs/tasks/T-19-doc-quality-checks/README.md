# T-19. Doc-quality проверки: `doc-size` + `markdownlint-cli2` (локальный композит)

- **Статус:** ⬜ открыта
- **Приоритет:** P3 (после демо, v0.1.x — процессная гигиена документации; продукт
  не меняет)
- **Зависит от:** —
- **Источник:** [D66](../../decisions/D66-doc-quality-checks.md) (Q62);
  связано: [D65](../../decisions/D65-reference-policy.md) (политика ссылок),
  [D64](../../decisions/D64-journal-integrity-test.md) (link-check — в T-18)

## Контекст

Ссылочная гниль и рост документов — ручная боль (29.09.2026: `docs/BRIEF.md`
сжат 395→304 строки — историч.; правила журнала перенесены в
[`../../../.opencode/rules/journal.md`](../../../.opencode/rules/journal.md)
решением [D71](../../decisions/D71-journal-rules-relocation.md)). Соседний проект
применяет композит doc-проверок (`markdownlint-cli2`, лимит строк, `cspell`,
`markdown-link-check`) как единый `npm run check`. Решение
[D66](../../decisions/D66-doc-quality-checks.md) закрепляет для credo2 Node-
инструменты `doc-size` + `markdownlint-cli2` и композит «быстрые раньше»;
link-check — единый владелец Rust-тест `tests/docs_journal.rs`
([T-18](../T-18-docs-journal-test/README.md),
[D64](../../decisions/D64-journal-integrity-test.md)), Node-
`markdown-link-check` отклонён.

## Что сделать

1. **`.opencode/scripts/doc-checks/`** (новое):
   - `check-doc-size.mjs` — лимит по умолчанию **300 строк**; фиксированные
     исключения-«книги»: `SPECIFICATION.md`, `GRAMMAR.md`, `CHANGELOG.md`,
     `features/README.md`; охват — `AGENTS.md`, `docs/*.md`,
     `docs/questions/**`, `docs/decisions/**`, `docs/tasks/**/README.md`,
     `docs/features/README.md`, `docs/README.md`, `.opencode/agents/**`,
     `.opencode/rules/**`; исключения — `docs/reviews/**`, `docs/analysis/**`,
     `docs/research/**`, `.opencode/mail/**`, `.opencode/memory/**`,
     `.opencode/state/**`; приём списка файлов в argv → режим «только
     изменённые»;
   - `.markdownlint-cli2.jsonc` — ruleset: `MD001`, `MD025`, `MD040`, `MD047` on;
     `MD024` siblings-only; `MD013`/`MD033`/`MD041`/`MD060` off; запуск с явным
     `--config` (конфиг рядом со скриптами);
   - композит `check-doc-quality.mjs` (или `npm run check:docs`) — порядок
     «быстрые раньше» (size → lint), единый exit-code.
2. **`.opencode/package.json`** — поле `scripts` (`check:docs`), dev-зависимость
   `markdownlint-cli2` с фиксацией lock-файлом.
3. **Пилот до включения** (без `--fix`): сухой прогон с корня → baseline и
   счётчики по правилам; правила с шумом > 0 на каноне — либо отключить, либо
   разово исправить (отдельной задачей); утвердить таблицу лимитов/исключений.
4. **Включение:** локальный прогон перед коммитами документов у ролей-писателей
   (`docs-writer`, `migrator`, `validator`); затем отдельный CI-job `docs` в
   `.github/workflows/ci.yml` (`npm ci` в `.opencode` + прогон) — отдельно от
   job `test` (не смешивать с Rust-DoD).
5. **Link-check — не здесь:** остаётся за [T-18](../T-18-docs-journal-test/README.md)
   ([D64](../../decisions/D64-journal-integrity-test.md), v0.2) — единый владелец
   относительных ссылок.

## Критерий готовности

Пилот с корня даёт **0 нарушений** на включённом охвате, исключения
документированы, время прогона — секунды; композит запускается одной командой;
[`../../../.opencode/rules/journal.md`](../../../.opencode/rules/journal.md)
§7/§8 обновлён ссылкой на инструменты
(`docs-writer`); CI-job `docs` добавлен отдельным изменением. DoD —
[`../README.md`](../README.md) §«DoD для любой задачи» с поправкой
[D50](../../decisions/D50-dod-by-package-scope.md): Rust-код не меняется →
`cargo`-прогон не требуется, проверка — прогон композита и `npm ci`.

## Примечания

- **Исполнитель** — сервисная сессия: правки в служебной зоне `.opencode/**`
  (Node), не в `src/**`/`tests/**`.
- **D50:** изменения кода прототипа нет — `cargo`-прогон `validator` не
  требуется; при проверке достаточно прогона композита.
- **Link-check** — пункт [T-18](../T-18-docs-journal-test/README.md)
  ([D64](../../decisions/D64-journal-integrity-test.md)); карточка T-18 не
  переписывается, связь фиксируется здесь.
- **Пилот без `--fix`** — правка канон-файлов линтером без ревью отдельным
  решением не вводится.
