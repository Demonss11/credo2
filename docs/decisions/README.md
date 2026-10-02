# decisions — журнал решений Q/D

> **Назначение:** сводка принятых решений журнала Q/D для быстрого обзора. Источник
> истины и формулировки — сами `Dn-*.md`; связи D → Q → feature → задача —
> `../TRACEABILITY.md`; вопросы — `../questions/README.md`;
> архив `OPEN_QUESTIONS.md` удалён после завершения миграции.
>
> **Роль в политике Q41:** рабочая сводка, **не канон**: текст решения живёт в
> `Dn-*.md`, связи — в `TRACEABILITY.md`.
>
> **Поддержка:** при заведении нового `Dn` `migrator` добавляет строку в эту сводку
> тем же изменением, что и файл (процесс журнала —
> [`../../.opencode/rules/journal.md`](../../.opencode/rules/journal.md) §5.2).

**Каталог:** `docs/decisions/Dn-<слаг>.md`. `Dn` — сквозной номер решения
(исторически = № строки прежней таблицы `SPECIFICATION.md` §10; таблица
упразднена решением [D70](D70-spec-reduction.md)); номера не переиспользуются,
слаг не переименовывается. Пропуски номеров — исторические §10-строки без
отдельного D-файла: до-журнальные решения; при переносе записи получают D-файлы
с этими номерами (прецеденты заполнения пропусков — D32/D35, D20/D60).
Неперенесённых записей не осталось: все вопросы архива оформлены журналом
(блок «Процесс», Q40+Q41, — последний).

**Статусы:** `accepted` · `superseded by Dm` · `rejected`.

## Сводка

| D | Краткая тема | Решает | Дата | Статус |
|---|---|---|---|---|
| [D1](D1-comment-syntax.md) | `//`-комментарии вместо `:::`-блоков | — | 2026-09-29 | accepted |
| [D2](D2-file-first-source-of-truth.md) | Файл = источник истины (file-first) | — | 2026-09-29 | accepted |
| [D3](D3-lsp-sidecar-process.md) | LSP как sidecar-процесс | — | 2026-09-29 | accepted |
| [D4](D4-lsp-transport-stdio.md) | Транспорт LSP — stdio | — | 2026-09-29 | accepted |
| [D5](D5-lsp-client-codemirror.md) | Клиент LSP — `codemirror-languageserver` | — | 2026-09-29 | accepted |
| [D6](D6-lsp-degradation.md) | Падение LSP — нет второго движка; автоперезапуск ≤ 3, ручной перезапуск | [Q39](../questions/Q39.md) | 2026-09-26 | accepted |
| [D7](D7-tauri-ipc-notebook.md) | Tauri IPC для Notebook | — | 2026-09-29 | accepted |
| [D8](D8-mcp-for-agent.md) | MCP для агента | — | 2026-09-29 | accepted |
| [D9](D9-nextjs-removed.md) | Next.js убран | — | 2026-09-29 | accepted |
| [D10](D10-graph-deferred.md) | Граф связей отложен | — | 2026-09-29 | accepted |
| [D11](D11-wasm-native-first.md) | WASM: сначала нативный, потом браузер | — | 2026-09-29 | accepted |
| [D12](D12-agent-chat-panel.md) | Чат агента — правая панель окна Notebook; результаты `check.test` — инлайн | [Q31](../questions/Q31.md) | 2026-09-26 | accepted |
| [D13](D13-f64-mvp-decimal-v02.md) | Семантика чисел: `f64` в MVP, `Decimal` в v0.2 | — | 2026-09-29 | accepted |
| [D14](D14-published-artifact-canon.md) | Артефакт публикации — неизменяемая JSON-тройка, каталог версии | [Q13](../questions/Q13.md) | 2026-09-25 | accepted |
| [D15](D15-evolution-credo2.md) | Эволюция `credo2`, а не greenfield | [Q1](../questions/Q1.md) | 2026-09-24 | accepted |
| [D16](D16-dsl-canon-regex-mvp.md) | Канон языка v0.1 — `GRAMMAR.md`; парсер MVP — regex-минимум | [Q2](../questions/Q2.md), [Q3](../questions/Q3.md) | 2026-09-24 | accepted |
| [D17](D17-priority-out-of-mvp.md) | `Приоритет` вне MVP; вернётся в v0.2 | [Q4](../questions/Q4.md) | 2026-09-24 | accepted |
| [D18](D18-pipelines-out-lsp-mvp.md) | Конвейеры/скоринги/таблицы вне MVP; состав LSP MVP | [Q36](../questions/Q36.md), [Q38](../questions/Q38.md) | 2026-09-24 | accepted |
| [D19](D19-statuses-priorities-canon.md) | Две оси меток; единственный канон — `features/README.md` | [Q5](../questions/Q5.md), [Q6](../questions/Q6.md) | 2026-09-24 | accepted |
| [D20](D20-features-docs-dod.md) | `.feature` — документация; DoD и инвентаризация фич | [Q40](../questions/Q40.md) | 2026-09-24 | accepted |
| [D21](D21-core-semantics-v01.md) | Семантика ядра v0.1 — строгие ошибки, словарь решений, объяснение | [Q8](../questions/Q8.md), [Q9](../questions/Q9.md), [Q10](../questions/Q10.md), [Q42](../questions/Q42.md) | 2026-09-24 | accepted |
| [D22](D22-rest-paths-canon.md) | Канон REST-путей — `/checks/{name}/versions/{version}/...` | [Q20](../questions/Q20.md) | 2026-09-25 | accepted |
| [D23](D23-get-checks-manifest.md) | Схема `GET /checks` — манифест `{schema_version, count, service_hash, checks[]}` | [Q21](../questions/Q21.md) | 2026-09-25 | accepted |
| [D24](D24-import-export-deferred.md) | Импорт/экспорт `.dar` — вне MVP/v0.2 | [Q26](../questions/Q26.md) | 2026-09-25 | accepted |
| [D25](D25-rest-error-envelope.md) | Формат ошибок REST — конверт `{"error": {"code", "message"}}` | [Q23](../questions/Q23.md) | 2026-09-25 | accepted |
| [D26](D26-rest-auth-x-api-key.md) | Аутентификация REST — заголовок `x-api-key` | [Q22](../questions/Q22.md) | 2026-09-25 | accepted |
| [D27](D27-rest-launch-address.md) | Запуск и адрес REST — `127.0.0.1:8080`; флаги `--rest`/`--addr`/`--no-rest` | [Q27](../questions/Q27.md) | 2026-09-25 | accepted |
| [D28](D28-two-git-contours.md) | Два git-контура — workspace-репо и read-only реестр публикаций | [Q32](../questions/Q32.md) | 2026-09-26 | accepted |
| [D29](D29-notebook-mcp-transport.md) | Транспорт MCP в Notebook — локальный sidecar по stdio | [Q30](../questions/Q30.md) | 2026-09-26 | accepted |
| [D30](D30-execution-mechanism.md) | Единый механизм исполнения из редактора — MCP `check.test` (draft-first) | [Q33](../questions/Q33.md) | 2026-09-26 | accepted |
| [D31](D31-check-create-contract.md) | Контракт `check.create` — `{name, source}`; «сохранить = обновить» | [Q28](../questions/Q28.md) | 2026-09-26 | accepted |
| [D32](D32-test-gate-mvp.md) | Тест-гейт публикации — метка теста в черновике | [Q16](../questions/Q16.md), [Q34](../questions/Q34.md) | 2026-09-26 | accepted |
| [D33](D33-fields-registry-source.md) | Источник схемы полей и словарей — БД (в MVP `HashMap`/демо-конфиг) | [Q37](../questions/Q37.md) | 2026-09-26 | accepted |
| [D34](D34-mcp-tool-contracts.md) | Контракты MCP-инструментов — правила, коды, инварианты, схемы | [Q29](../questions/Q29.md) | 2026-09-26 | accepted |
| [D35](D35-semver-v01.md) | Semver v0.1 — числовое сравнение pre-release; build вне идентичности | [Q18](../questions/Q18.md) | 2026-09-26 | accepted |
| [D36](D36-batch-deferred.md) | Массовый прогон (batch) — вне MVP/v0.2 | [Q24](../questions/Q24.md) | 2026-09-26 | accepted |
| [D37](D37-client-explanation-deferred.md) | Объяснение для клиента — вне MVP/v0.2 | [Q25](../questions/Q25.md) | 2026-09-26 | accepted |
| [D38](D38-agent-cycle.md) | Цикл агентов: Agile-петля, единый тестировщик, память и почта | [Q43](../questions/Q43.md) | 2026-09-26 | accepted |
| [D39](D39-loop-dispatcher.md) | Loop-диспетчер, эфемерный `analyst`, состояние на диске | [Q44](../questions/Q44.md) | 2026-09-27 | accepted |
| [D40](D40-scope-threshold.md) | Scope-порог — узкий + триггер частичного покрытия | [Q45](../questions/Q45.md) | 2026-09-27 | accepted |
| [D41](D41-dispatch-refinements.md) | Уточнения `dispatch-loop` после пилота Run 3 | [Q46](../questions/Q46.md) | 2026-09-27 | accepted |
| [D42](D42-expect-iteration.md) | `expect` в терминах hard rules; `iteration` — только rework | [Q47](../questions/Q47.md) | 2026-09-28 | accepted |
| [D43](D43-auditor-mail.md) | Отчёт `auditor` — в ленту задачи (право `edit .opencode/mail/**`) | [Q48](../questions/Q48.md) | 2026-09-28 | accepted |
| [D44](D44-run5-refinements.md) | Уточнения Run 5 — бриф ↔ канон, затык, `steps`, allowlist, срез B1 | [Q49](../questions/Q49.md) | 2026-09-28 | accepted |
| [D45](D45-wave0-quality-config.md) | Судьба волны 0 и конфиг-пакет качества | [Q50](../questions/Q50.md) | 2026-09-28 | accepted |
| [D46](D46-product-process-commits.md) | Продуктовые и процессные коммиты; `state/**` | [Q51](../questions/Q51.md) | 2026-09-28 | accepted |
| [D47](D47-git-refinements-run5.md) | Git-уточнения Run 5 — удаление ветки, `git -C`, хеши closeout | [Q52](../questions/Q52.md) | 2026-09-28 | accepted |
| [D48](D48-findings-registry-owner.md) | Реестр находок — владелец и зона записи | [Q53](../questions/Q53.md) | 2026-09-28 | accepted |
| [D49](D49-validator-branch-contains.md) | Право `git branch --contains` для `validator` | [Q54](../questions/Q54.md) | 2026-09-28 | accepted |
| [D50](D50-dod-by-package-scope.md) | DoD `validator` по составу пакета — cargo только при изменениях кода | [Q55](../questions/Q55.md) | 2026-09-29 | accepted |
| [D51](D51-agent-tools-token-hygiene.md) | Инструменты экономии токенов — сводка прав и подсчёт строк | [Q56](../questions/Q56.md) | 2026-09-29 | accepted |
| [D52](D52-glossary-terms-canon.md) | Терминологический канон глоссария (`SPECIFICATION.md` §11) | [Q7](../questions/Q7.md) | 2026-09-26 | accepted |
| [D53](D53-error-messages-language.md) | Язык сообщений об ошибках — принцип «машина / человек» | [Q11](../questions/Q11.md) | 2026-09-25 | accepted |
| [D54](D54-source-of-truth-flow.md) | Источник истины — `.dar`-файл; поток `файл → черновик → публикация` | [Q12](../questions/Q12.md) | 2026-09-25 | accepted |
| [D55](D55-publish-branch-name.md) | Имя ветки публикации — `publish/{name}-{version}` | [Q14](../questions/Q14.md) | 2026-09-25 | accepted |
| [D56](D56-merge-step.md) | Минимальный цикл публикации — явный шаг `credo merge` | [Q15](../questions/Q15.md) | 2026-09-25 | accepted |
| [D57](D57-bare-git-immutability.md) | Иммутабельность версии — bare-git, без отдельного механизма | [Q17](../questions/Q17.md) | 2026-09-26 | accepted |
| [D58](D58-workspace-data-dirs.md) | Каталоги данных workspace — `.credo/` и `.dar-notebook/` | [Q19](../questions/Q19.md) | 2026-09-26 | accepted |
| [D59](D59-workspace-templates.md) | Создание workspace и шаблоны — `rules/Пример.dar`, `README.md`, `.gitignore` | [Q35](../questions/Q35.md) | 2026-09-26 | accepted |
| [D60](D60-docs-ownership-sync.md) | Владение документами и синхронизация — «один факт — один канон» | [Q41](../questions/Q41.md) | 2026-09-26 | accepted |
| [D61](D61-archive-removal.md) | Завершение миграции — архив `OPEN_QUESTIONS.md` удаляется | [Q57](../questions/Q57.md) | 2026-09-29 | accepted |
| [D62](D62-brief-journal-rules.md) | `BRIEF.md` после миграции — компактные правила ведения журнала | [Q58](../questions/Q58.md) | 2026-09-29 | superseded by [D71](D71-journal-rules-relocation.md) |
| [D63](D63-journal-index-lifecycle.md) | Индекс журнала — единая таблица жизненного цикла | [Q59](../questions/Q59.md) | 2026-09-29 | accepted |
| [D64](D64-journal-integrity-test.md) | Тест целостности журнала — `tests/docs_journal.rs` | [Q60](../questions/Q60.md) | 2026-09-29 | accepted |
| [D65](D65-reference-policy.md) | Политика ссылок и дублей — «ссылка, не копия»; «время жизни адреса» | [Q61](../questions/Q61.md) | 2026-09-29 | accepted |
| [D66](D66-doc-quality-checks.md) | Doc-quality проверки: size+lint, link-check в T-18 | [Q62](../questions/Q62.md) | 2026-09-29 | accepted |
| [D67](D67-cspell-deferred.md) | Cspell — отложен (v0.2) | [Q63](../questions/Q63.md) | 2026-09-29 | accepted |
| [D68](D68-changelog-handwritten.md) | CHANGELOG — рукописный, генератор отклонён | [Q64](../questions/Q64.md) | 2026-09-29 | accepted |
| [D69](D69-retro-decisions.md) | Ретро-D §10 (№ 1–5, 7–11, 13) — 11 D-файлов | [Q65](../questions/Q65.md) | 2026-09-29 | accepted |
| [D70](D70-spec-reduction.md) | SPEC — сокращение; таблица §10 упраздняется | [Q66](../questions/Q66.md) | 2026-09-29 | accepted |
| [D71](D71-journal-rules-relocation.md) | Правила журнала → `.opencode/rules/journal.md`; `BRIEF.md` удаляется | [Q67](../questions/Q67.md) | 2026-09-29 | accepted |
| [D72](D72-changelog-full-cleanup.md) | `CHANGELOG` — полная очистка; далее только кодовые изменения | [Q68](../questions/Q68.md) | 2026-09-29 | accepted |
| [D73](D73-readme-entrypoints.md) | README — корневой создаётся; `docs/README.md` — лёгкая карта | [Q69](../questions/Q69.md) | 2026-09-29 | accepted |
| [D74](D74-grammar-normative-focus.md) | `GRAMMAR` — нормативный минимум; rationale — ссылками | [Q70](../questions/Q70.md) | 2026-09-29 | accepted |
| [D75](D75-git-lean-workflow.md) | Роль `git` — минимальный цикл, хелпер + allowlist, `steps` 28→14 | [Q71](../questions/Q71.md) | 2026-09-30 | accepted |
| [D76](D76-traceability-links-only.md) | TRACEABILITY — только ссылки Qn/Dn; темы — в каталогах | [Q72](../questions/Q72.md) | 2026-09-30 | accepted |
| [D77](D77-tasks-visibility-completeness.md) | Полнота задач — каждая T-XX видна в TRACEABILITY через пару Q/D | [Q73](../questions/Q73.md) | 2026-09-30 | accepted |
| [D78](D78-t15-mcp-ready-program.md) | T-15 — программа «процесс, готовый к MCP» (фазы A–G) | [Q74](../questions/Q74.md) | 2026-09-30 | accepted |
| [D79](D79-journal-canon-completeness.md) | Полнота задач — в канон журнала §7 | [Q75](../questions/Q75.md) | 2026-09-30 | accepted |
| [D80](D80-features-visibility-completeness.md) | Полнота фич — каждая `*.feature` поимённо видна в TRACEABILITY | [Q76](../questions/Q76.md) | 2026-09-30 | accepted |
| [D81](D81-pm-process-mining.md) | Инструмент `pm` — uv-проект process mining агентского процесса | [Q77](../questions/Q77.md) | 2026-09-30 | accepted |
| [D82](D82-traceability-lifecycle-waves.md) | Жизненный цикл `TRACEABILITY` — `open` · `in work` · `done`, волны | [Q78](../questions/Q78.md) | 2026-10-01 | accepted |
| [D83](D83-traceability-wave2.md) | Волна 2 `TRACEABILITY` — разбор `open`-строк отдельной задачей | [Q79](../questions/Q79.md) | 2026-10-01 | accepted |
| [D84](D84-rules-revision.md) | Ревизия `.opencode/rules/**` — актуальная норма, ссылка на решение, без истории | [Q80](../questions/Q80.md) | 2026-10-01 | accepted |
| [D85](D85-sverka-snapshot-scope.md) | «Сверка с кодом» `D`-файла — датированный снимок для номеров строк, адреса строги | [Q82](../questions/Q82.md) | 2026-10-02 | accepted |
| [D86](D86-state-schema.md) | Схема состояния процесса — отдельный канон, модель D42 | [Q83](../questions/Q83.md) | 2026-10-02 | accepted |
