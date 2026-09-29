# D66: Doc-quality проверки — size + lint локально, link-check — в T-18

- **Статус:** accepted
- **Дата:** 2026-09-29
- **Resolves:** [Q62](../questions/Q62.md)
- **Спека:** [`SPECIFICATION.md`](../SPECIFICATION.md) §10, решение №66
- **Affects:** `.opencode/scripts/doc-checks/**` (новое: `check-doc-size.mjs`,
  ruleset `markdownlint-cli2`, композит `check`),
  `.opencode/package.json` (поле `scripts`);
  [`BRIEF.md`](../BRIEF.md) §7/§8 (после исполнения);
  `.github/workflows/ci.yml` (отдельный job `docs`, позже);
  `tests/docs_journal.rs` — link-check
  ([T-18](../tasks/T-18-docs-journal-test/README.md),
  [D64](D64-journal-integrity-test.md)); обзор
  [`doc-quality-checks-2026-09-29.md`](../research/doc-quality-checks-2026-09-29.md)
- **Tasks:** [T-19](../tasks/T-19-doc-quality-checks/README.md) (см. «Сверка с
  кодом»)

## Контекст

Соседний проект применяет композит doc-проверок (`markdownlint-cli2`, лимит
строк, `cspell`, `markdown-link-check`; порядок «быстрые раньше») как единый
`npm run check`. У нас проверка ссылок — вручную ([`BRIEF.md`](../BRIEF.md) §7),
а link-check запланирован в Rust-тесте `tests/docs_journal.rs`
([T-18](../tasks/T-18-docs-journal-test/README.md),
[D64](D64-journal-integrity-test.md)) — один владелец уже назначен. Node-
инфраструктура в `.opencode/` есть (`npm ci`, `clean-logs.mjs`,
`agents-perms.mjs`). Опыт 29.09.2026: ссылочная гниль и ручное сжатие документов
([`BRIEF.md`](../BRIEF.md) 395→304 строки). Обзор-основание —
[`doc-quality-checks-2026-09-29.md`](../research/doc-quality-checks-2026-09-29.md).
Полный контекст — [Q62](../questions/Q62.md).

## Решение

1. **Node-инструменты в `.opencode/scripts/doc-checks/`:**
   - **doc-size** — лимит по умолчанию **300 строк**; фиксированные
     исключения-«книги»: `SPECIFICATION.md`, `GRAMMAR.md`, `CHANGELOG.md`,
     `BRIEF.md`, `features/README.md`; охват — живые зоны журнала/ролей/правил/
     карточек; история `reviews/**`/`analysis/**` и рабочие `mail/**`/`state/**` —
     вне;
   - **markdownlint-cli2** — ruleset: `MD001`, `MD025`, `MD040`, `MD047`,
     `MD024` siblings-only; `MD013`/`MD033`/`MD041`/`MD060` — off;
   - **композит `check`** — порядок «быстрые раньше».
2. **Link-check — единый владелец:** Rust-тест `tests/docs_journal.rs`
   ([T-18](../tasks/T-18-docs-journal-test/README.md),
   [D64](D64-journal-integrity-test.md)) дополняется проверкой относительных
   ссылок; Node-`markdown-link-check` **отклонён** — не плодить два инструмента
   с одной зоной ответственности («один факт — один канон», Q41/[D60](D60-docs-ownership-sync.md)).
3. **Режим включения:** пилот в temp (замер шума) → локальный прогон перед
   коммитами документов → отдельный CI-job `docs` (позже; [D50](D50-dod-by-package-scope.md)
   не затрагивается — Node-инструменты не требуют `cargo`).

## Следствия

- Лимит строк и узкий markdown-lint доступны локально у ролей-писателей
  документов; link-check остаётся за [T-18](../tasks/T-18-docs-journal-test/README.md)
  (единый владелец).
- Инструменты живут вне Rust-кода → [D50](D50-dod-by-package-scope.md) (cargo
  только при изменениях кода) не задет; зависимости фиксируются lock-файлом.
- Принят «быстрый сперва»: size (мс, без зависимостей) → lint.
- Пилот до включения — сухой прогон без `--fix`, отчёт-счётчики, утверждение
  таблицы лимитов/исключений.
- [`BRIEF.md`](../BRIEF.md) §7/§8 после исполнения получает ссылку на
  инструменты; CI-job `docs` заводится отдельно от job `test`.

## Сверка с кодом

Вердикт: ⬜ **не реализовано** — Node-инструментов doc-quality нет; Rust-тест
`tests/docs_journal.rs` (владелец link-check) ещё не написан
([D64](D64-journal-integrity-test.md)/[T-18](../tasks/T-18-docs-journal-test/README.md)).
Решение порождает задачу [T-19](../tasks/T-19-doc-quality-checks/README.md) и
дополняет [T-18](../tasks/T-18-docs-journal-test/README.md) пунктом link-check.

Что проверено (чтением, 29.09.2026), чем подтверждено:

- **Скриптов нет:** обзор `glob .opencode/scripts/**` → `clean-logs.mjs`,
  `agents-perms.mjs`, `session-analysis/**`; папки `doc-checks/` и скриптов
  `check-doc-size.mjs`/`check-doc-quality.mjs` нет, ruleset отсутствует.
- **Место есть:** `.opencode/scripts/` существует; `.opencode/package.json`
  содержит только `dependencies` (`@opencode/plugin`) — поля `scripts` пока нет;
  `npm ci` в `.opencode/` — уже норма ([D45](D45-wave0-quality-config.md)).
- **Link-check владелец:** `glob tests/*.rs` → `rest.rs`, `publish.rs`,
  `mcp_errors.rs`, `mcp_draft.rs`, `features_inventory.rs`; `docs_journal.rs`
  отсутствует; карточка [T-18](../tasks/T-18-docs-journal-test/README.md) уже
  фиксирует link-check как v0.2-пункт.
- **CI:** `.github/workflows/ci.yml` — единственный job `test` (Rust DoD);
  job `docs` не заведён.
- **Опора:** обзор [`doc-quality-checks-2026-09-29.md`](../research/doc-quality-checks-2026-09-29.md)
  (разделы «Рекомендации», «Архитектура внедрения», «Пилот»), проверки 29.09.2026.

`cargo` не запускался (§5.3, [D50](D50-dod-by-package-scope.md)): решение
документно-инструментальное, кода прототипа не меняет; адресный прогон не
требуется.

**Задачи:** [T-19](../tasks/T-19-doc-quality-checks/README.md) — реализация
инструментов (исполнитель — сервисная сессия, служебная зона `.opencode/**`,
Node). Дополнение [T-18](../tasks/T-18-docs-journal-test/README.md) пунктом
link-check фиксируется здесь и в карточке T-19 (карточка T-18 не
переписывается).

## Альтернативы

- **Полный Node-набор, включая links** — отклонено: дубль владельца link-check
  ([T-18](../tasks/T-18-docs-journal-test/README.md)), два инструмента на одну
  зону ответственности (Q41/[D60](D60-docs-ownership-sync.md)).
- **Только size** — отклонено: markdown-lint дёшев и адресует markdown-форму,
  которую T-18 не проверяет.
- **Не брать** — отклонено: ссылочная гниль и неограниченный рост документов
  (опыт 29.09: [`BRIEF.md`](../BRIEF.md) 395→304).

## Ссылки

- Вопрос: [Q62](../questions/Q62.md)
- Связанные: [D64](D64-journal-integrity-test.md)/[T-18](../tasks/T-18-docs-journal-test/README.md)
  (единый владелец link-check); [D65](D65-reference-policy.md) («ссылка, не
  копия», время жизни адреса); [D45](D45-wave0-quality-config.md) (конфиг-пакет
  качества, `npm ci`); [D50](D50-dod-by-package-scope.md) (DoD по составу
  пакета); [Q63](../questions/Q63.md) ([D67](D67-cspell-deferred.md), спелл-чек
  отложен); [Q64](../questions/Q64.md) ([D68](D68-changelog-handwritten.md),
  CHANGELOG рукописный)
- Обзор: [`doc-quality-checks-2026-09-29.md`](../research/doc-quality-checks-2026-09-29.md)
- Задача: [T-19](../tasks/T-19-doc-quality-checks/README.md)
- Краткий канон: [`SPECIFICATION.md`](../SPECIFICATION.md) §10, решение №66
- Связи: [`TRACEABILITY.md`](../TRACEABILITY.md)
