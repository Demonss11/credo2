# tasks — задачи по коду CREDO

> **Назначение:** реестр задач по коду прототипа `credo2`, вытекающих из
> принятых решений. Задача — не решение: канон решений —
> журнал `../questions/` + `../decisions/`;
> требования —
> `../features/`. У каждой задачи есть поле «Источник»; задач «из воздуха»
> здесь нет.
>
> **Границы:** только следствия принятых решений для текущего прототипа.
> Отложенное (v0.2: БД, LSP `lsp-dar`, Notebook, batch, explain_client,
> импорт/экспорт, pre-release) — не здесь, а в фичах со статусом ⏸.
> **Исключение** ([D83](../decisions/D83-traceability-wave2.md)): v0.2-следствия
> принятых решений допускаются как задачи реестра — с пометой «v0.2» и
> зависимостью «приёмка v0.1»; детализация остаётся в фичах со статусом ⏸.
>
> **Роль в политике Q41:** рабочий реестр; статусы задач — здесь и в
> карточках, статусы требований — только в `../features/README.md`.

**Структура:** одна задача — одна папка `T-XX-<слаг>/README.md`
(карточка: источник → что сделать → критерий готовности → примечания).
Статус задачи — в сводке и в карточке.

**Статусы:** ⬜ открыта · 🚧 в работе · ✅ сделана.
**Приоритеты:** P0 — структурный фундамент: Фаза 0 (workspace, без изменения
поведения) · P1 — контракты и сквозной агентский цикл (до демо) ·
P2 — реестр и артефакты (до демо, если успеем) · P3 — после демо (v0.1.x).
Порядок работ: P0 → P1 → P2 → P3.

**Источник задачи:** поле «Источник» — решение журнала `Dn` и/или вопрос `Qn`
(оба ID стабильны). Задачи
появляются из сверки решения с кодом
([`../../.opencode/rules/journal.md`](../../.opencode/rules/journal.md) §5.3) —
при миграции журнала или при ревью; если решение не требует кода, задача не
заводится, и это фиксируется в D-файле («Задач не требуется»).

**Полнота:** каждая задача видна через [`../TRACEABILITY.md`](../TRACEABILITY.md)
(в колонке «Задачи» хотя бы одной строки) и имеет источник Q/D; задача без
журнальной пары — черновик, в работу не берётся; полноту и синхронность статусов
⬜/🚧/✅ проверяет тест `tests/docs_journal.rs` (план — `T-18` ⬜,
[D77](../decisions/D77-tasks-visibility-completeness.md)).

## Сводка

| ID | Задача | Источник | Приоритет | Зависит от | Статус |
|---|---|---|---|---|---|
| [T-10](T-10-workspace-phase-0/README.md) | Фаза 0: workspace `dar-core` + `credo-server` (бинарник пока `credo2.exe`) | D15 (Q1), SPEC §5/§8 | P0 | — | ⬜ |
| [T-01](T-01-draft-source-hash/README.md) | Черновик: `source`, `source_hash`, `stale`, `test_valid` | Q12, Q29 | P1 | — | ✅ |
| [T-02](T-02-test-gate/README.md) | Тест-гейт публикации: `last_test_checksum`/`tested_at` | Q16, Q34 | P1 | T-01 | ⬜ |
| [T-03](T-03-check-create/README.md) | `check.create`: `{name, source}` и ответ `{status, name}` | Q28 | P1 | — | ✅ |
| [T-04](T-04-mcp-errors/README.md) | MCP-ошибки: конверт и 10 стабильных кодов (§4.5) | Q29, Q11 | P1 | — | ✅ |
| [T-05](T-05-mcp-success-schemas/README.md) | MCP-схемы успеха инструментов (§4.5) | Q29 | P1 | T-01, T-02 | ⬜ |
| [T-11](T-11-agent-cycle/README.md) | Цикл агентов: Agile-петля, единый тестировщик, память и почта ролей | [D38](../decisions/D38-agent-cycle.md) (Q43) | P1 | — | ✅ |
| [T-12](T-12-agent-loop/README.md) | Разгрузка `lead`: loop-диспетчер, эфемерный `analyst`, состояние на диске | [D39](../decisions/D39-loop-dispatcher.md) (Q44) | P1 | — | ✅ |
| [T-13](T-13-agent-hardening/README.md) | W7: доработка агентов после Run 3 (доступы, дисциплина цикла, гигиена) | [D40](../decisions/D40-scope-threshold.md), [D41](../decisions/D41-dispatch-refinements.md) (Q45, Q46) | P1 | — | ✅ |
| [T-15](T-15-mcp-ready-process/README.md) | Процесс, готовый к MCP: схема состояния, операции, финализация, метрики | [D78](../decisions/D78-t15-mcp-ready-program.md) (Q74); записка [`mcp-ready-process.md`](T-15-mcp-ready-process/mcp-ready-process.md) | P1 | — | ⬜ |
| [T-16](T-16-stale-check-test/README.md) | `check.test` на stale-черновике: исполнение по тексту файла `rules/{name}.dar` | Q12 (Q29, T-01) | P1 | — | ⬜ |
| [T-06](T-06-registry-path-xyz/README.md) | Реестр: путь `checks/{name}/{X}/{Y}/{Z}/` | Q13, Q32 | P2 | — | ⬜ |
| [T-07](T-07-meta-fields/README.md) | `meta.json`: `display_name`, `source_hash`, `compiler_version` | Q13, Q7 | P2 | — | ⬜ |
| [T-08](T-08-materialize-source-file/README.md) | Публикация материализует `rules/{name}.dar` | Q12, Q33 | P2 | — | ⬜ |
| [T-17](T-17-merge-command/README.md) | `credo merge`: слияние ветки публикации в `main` (ancestor-проверка, CAS, удаление ветки) | [D56](../decisions/D56-merge-step.md) (Q15) | P2 | — | ⬜ |
| [T-14](T-14-grammar-message-sync/README.md) | GRAMMAR: синхронизация цитаты сообщения парсера | [D41](../decisions/D41-dispatch-refinements.md) (Q46) | P2 | T-03 | ⬜ |
| [T-09](T-09-check-run/README.md) | `check.run` — исполнение опубликованной версии | Q33 | P3 | T-06 | ⬜ |
| [T-18](T-18-docs-journal-test/README.md) | Тест целостности журнала `tests/docs_journal.rs` (ID, парность, таблицы, запреты) | [D64](../decisions/D64-journal-integrity-test.md) (Q60) | P3 | после D61–D63, D65 | ⬜ |
| [T-19](T-19-doc-quality-checks/README.md) | Doc-quality проверки: `doc-size` + `markdownlint-cli2`, композит `check` | [D66](../decisions/D66-doc-quality-checks.md) (Q62) | P3 | — | ⬜ |
| [T-20](T-20-traceability-wave2/README.md) | Волна 2 `TRACEABILITY`: разбор `open`-строк | [D83](../decisions/D83-traceability-wave2.md) (Q79) | P3 | — | ⬜ |

## DoD для любой задачи

`cargo fmt --check` + `cargo clippy --all-targets -- -D warnings` +
`cargo test --all` (все зелёные); полный прогон `cargo test --all` выполняет
только `validator`, а роли, работающие с кодом, ограничиваются компиляцией
(`fmt`/`check`/`clippy`). Соответствующие сценарии фич не противоречат
поведению. При закрытии задачи: обновить статус в карточке и в сводке, а
также статус требования в `../features/README.md`, если оно стало
выполняться.
