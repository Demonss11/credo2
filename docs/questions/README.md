# questions — журнал вопросов Q/D

> **Назначение:** сводка живого журнала вопросов для быстрого обзора. Источник
> истины — сами `Qn.md`; полные решения — `../decisions/Dn-*.md`; связи
> Q → D → feature → задача — `../TRACEABILITY.md`;
> архив `OPEN_QUESTIONS.md` удалён после завершения миграции, источник — журнал.
>
> **Роль в политике Q41:** рабочая сводка, **не канон**: текст вопроса и решения
> живёт в `Qn.md` и `Dn-*.md`; статусы и
> приоритеты требований — только в `../features/README.md`.
>
> **Поддержка:** при заведении новой `Qn` `migrator` добавляет строку в эту
> сводку тем же изменением, что и файл (процесс журнала —
> [`../../.opencode/rules/journal.md`](../../.opencode/rules/journal.md) §5.1–§5.2).

**Каталог:** `docs/questions/Qn.md` без слага. Сквозной номер в порядке
появления (`Q1`, `Q2`, …); номера не переиспользуются.

## Сводка

| Q | Тема | D |
|---|---|---|
| [Q1](Q1.md) | SPEC — план «с нуля» или доработка `credo2`? | [D15](../decisions/D15-evolution-credo2.md) |
| [Q2](Q2.md) | где канонический DSL: `concept.md`, SPEC или features? | [D16](../decisions/D16-dsl-canon-regex-mvp.md) |
| [Q3](Q3.md) | минимальный DSL (regex, одно сравнение) или полноценный лексер/AST? | [D16](../decisions/D16-dsl-canon-regex-mvp.md) |
| [Q4](Q4.md) | `Приоритет` входит в MVP? | [D17](../decisions/D17-priority-out-of-mvp.md) |
| [Q5](Q5.md) | единая легенда статусов и приоритетов? | [D19](../decisions/D19-statuses-priorities-canon.md) |
| [Q6](Q6.md) | пересчитать сводную таблицу SPEC §6.1? | [D19](../decisions/D19-statuses-priorities-canon.md) |
| [Q7](Q7.md) | §11 не содержит терминов, которыми оперируют фичи (`check`, `active`, `service_hash`, …)? | [D52](../decisions/D52-glossary-terms-canon.md) |
| [Q8](Q8.md) | отсутствующее поле: ошибка или `0`? | [D21](../decisions/D21-core-semantics-v01.md) |
| [Q9](Q9.md) | типизация и сравнение чисел | [D21](../decisions/D21-core-semantics-v01.md) |
| [Q10](Q10.md) | словарь решений и «не сработало» | [D21](../decisions/D21-core-semantics-v01.md) |
| [Q11](Q11.md) | язык сообщений об ошибках: русский для человекочитаемого, латиница для машинного? | [D53](../decisions/D53-error-messages-language.md) |
| [Q12](Q12.md) | единственный источник истины: `.dar`-файлы или песочница+JSON? | [D54](../decisions/D54-source-of-truth-flow.md) |
| [Q13](Q13.md) | что именно публикуется и по какому пути? | [D14](../decisions/D14-published-artifact-canon.md) |
| [Q14](Q14.md) | имя ветки публикации | [D55](../decisions/D55-publish-branch-name.md) |
| [Q15](Q15.md) | кто и как мержит ветку в `main`? | [D56](../decisions/D56-merge-step.md) |
| [Q16](Q16.md) | публикация без теста и контрольная сумма | [D32](../decisions/D32-test-gate-mvp.md) |
| [Q17](Q17.md) | иммутабельность: файловая система или bare-git? | [D57](../decisions/D57-bare-git-immutability.md) |
| [Q18](Q18.md) | сортировка pre-release и build-метаданные | [D35](../decisions/D35-semver-v01.md) |
| [Q19](Q19.md) | где живут данные: `.credo/` и `.dar-notebook/`? | [D58](../decisions/D58-workspace-data-dirs.md) |
| [Q20](Q20.md) | канонические пути REST | [D22](../decisions/D22-rest-paths-canon.md) |
| [Q21](Q21.md) | схема ответа `GET /checks` | [D23](../decisions/D23-get-checks-manifest.md) |
| [Q22](Q22.md) | аутентификация REST — заголовок | [D26](../decisions/D26-rest-auth-x-api-key.md) |
| [Q23](Q23.md) | формат ошибок REST | [D25](../decisions/D25-rest-error-envelope.md) |
| [Q24](Q24.md) | массовый прогон (batch): отложен | [D36](../decisions/D36-batch-deferred.md) |
| [Q25](Q25.md) | объяснение для клиента: отложено | [D37](../decisions/D37-client-explanation-deferred.md) |
| [Q26](Q26.md) | импорт/экспорт `.dar`: контракт | [D24](../decisions/D24-import-export-deferred.md) |
| [Q27](Q27.md) | порт REST и запуск сервера | [D27](../decisions/D27-rest-launch-address.md) |
| [Q28](Q28.md) | параметры `check.create` | [D31](../decisions/D31-check-create-contract.md) |
| [Q29](Q29.md) | ответы `check.test`/`check.list_published`/`check.deprecate`: контракты MCP-инструментов | [D34](../decisions/D34-mcp-tool-contracts.md) |
| [Q30](Q30.md) | транспорт MCP в Notebook: локальный sidecar или общий сервер | [D29](../decisions/D29-notebook-mcp-transport.md) |
| [Q31](Q31.md) | где живёт чат агента? | [D12](../decisions/D12-agent-chat-panel.md) |
| [Q32](Q32.md) | два git-репозитория и мультиверсионность | [D28](../decisions/D28-two-git-contours.md) |
| [Q33](Q33.md) | чем исполняется правило в редакторе? | [D30](../decisions/D30-execution-mechanism.md) |
| [Q34](Q34.md) | тесты: внутри `.dar` или в `.dar-notebook/`? | [D32](../decisions/D32-test-gate-mvp.md) |
| [Q35](Q35.md) | что создаёт новый workspace и какой шаблон получает новое правило? | [D59](../decisions/D59-workspace-templates.md) |
| [Q36](Q36.md) | конвейеры: несколько правил в одном файле? | [D18](../decisions/D18-pipelines-out-lsp-mvp.md) |
| [Q37](Q37.md) | реестр полей и типов для автодополнения | [D33](../decisions/D33-fields-registry-source.md) |
| [Q38](Q38.md) | состав LSP сверх SPEC | [D18](../decisions/D18-pipelines-out-lsp-mvp.md) |
| [Q39](Q39.md) | поведение при падении LSP | [D6](../decisions/D6-lsp-degradation.md) |
| [Q40](Q40.md) | исполняемые ли Gherkin-сценарии | [D20](../decisions/D20-features-docs-dod.md) |
| [Q41](Q41.md) | владелец и синхронизация документов | [D60](../decisions/D60-docs-ownership-sync.md) |
| [Q42](Q42.md) | каноническая схема объяснения и язык полей | [D21](../decisions/D21-core-semantics-v01.md) |
| [Q43](Q43.md) | каким должен быть цикл работы команды агентов? | [D38](../decisions/D38-agent-cycle.md) |
| [Q44](Q44.md) | как разгрузить `lead` и сделать цикл durable? | [D39](../decisions/D39-loop-dispatcher.md) |
| [Q45](Q45.md) | какой порог scope-решения в цикле диспетчера? | [D40](../decisions/D40-scope-threshold.md) |
| [Q46](Q46.md) | какие уточнения канона цикла нужны по итогам Run 3? | [D41](../decisions/D41-dispatch-refinements.md) |
| [Q47](Q47.md) | как формулировать `expect` и нумеровать участки, если `iteration` — rework-цикл? | [D42](../decisions/D42-expect-iteration.md) |
| [Q48](Q48.md) | как `auditor` фиксирует отчёт в ленте при правах только на память? | [D43](../decisions/D43-auditor-mail.md) |
| [Q49](Q49.md) | что делать при расхождении брифа с каноном и при затыке, каких запасов `steps` требуют роли и как работать со срезом и лентами? | [D44](../decisions/D44-run5-refinements.md) |
| [Q50](Q50.md) | что закрепляем из волны 0 и каким должен быть конфиг-пакет качества? | [D45](../decisions/D45-wave0-quality-config.md) |
| [Q51](Q51.md) | как разделять продуктовые и процессные коммиты и как коммитить состояние? | [D46](../decisions/D46-product-process-commits.md) |
| [Q52](Q52.md) | как завершать ветку при устаревшем upstream, что с `git -C` и где хеши closeout? | [D47](../decisions/D47-git-refinements-run5.md) |
| [Q53](Q53.md) | кто ведёт реестр находок, если инструкции адресуют его `migrator`, а права не покрывают `docs/analysis/**`? | [D48](../decisions/D48-findings-registry-owner.md) |
| [Q54](Q54.md) | чем `validator` подтверждает вхождение коммита в историю ветки при приёмке? | [D49](../decisions/D49-validator-branch-contains.md) |
| [Q55](Q55.md) | обязателен ли cargo-прогон `validator` для пакетов без изменений кода? | [D50](../decisions/D50-dod-by-package-scope.md) |
| [Q56](Q56.md) | какими инструментами роли экономят токены при проверках (подсчёт строк; машинная сверка прав)? | [D51](../decisions/D51-agent-tools-token-hygiene.md) |
| [Q57](Q57.md) | судьба архива `OPEN_QUESTIONS.md` после завершения миграции | [D61](../decisions/D61-archive-removal.md) |
| [Q58](Q58.md) | роль и структура `BRIEF.md` после завершения миграции | [D62](../decisions/D62-brief-journal-rules.md) |
| [Q59](Q59.md) | индекс журнала: три таблицы и жизненный цикл `TRACEABILITY` | [D63](../decisions/D63-journal-index-lifecycle.md) |
| [Q60](Q60.md) | тест целостности журнала | [D64](../decisions/D64-journal-integrity-test.md) |
| [Q61](Q61.md) | политика ссылок и допустимых дублей в документации | [D65](../decisions/D65-reference-policy.md) |
| [Q62](Q62.md) | doc-quality проверки: состав, место, режим включения | [D66](../decisions/D66-doc-quality-checks.md) |
| [Q63](Q63.md) | спелл-чек документации (`cspell` en+ru) | [D67](../decisions/D67-cspell-deferred.md) |
| [Q64](Q64.md) | `CHANGELOG`: генератор из коммитов или рукописный? | [D68](../decisions/D68-changelog-handwritten.md) |
| [Q65](Q65.md) | ретро-D для до-журнальных решений §10 (№ 1–5, 7–11, 13) | [D69](../decisions/D69-retro-decisions.md) |
| [Q66](Q66.md) | `SPECIFICATION.md`: существенное сокращение и §10-индекс | [D70](../decisions/D70-spec-reduction.md) |
| [Q67](Q67.md) | правила журнала → `.opencode/rules/journal.md`, `BRIEF.md` удалить | [D71](../decisions/D71-journal-rules-relocation.md) |
| [Q68](Q68.md) | `CHANGELOG`: очистка и новое правило ведения | [D72](../decisions/D72-changelog-full-cleanup.md) |
| [Q69](Q69.md) | `README`: корневой создать, `docs/README.md` отрефакторить | [D73](../decisions/D73-readme-entrypoints.md) |
| [Q70](Q70.md) | `GRAMMAR`: нормативный фокус | [D74](../decisions/D74-grammar-normative-focus.md) |
| [Q71](Q71.md) | работа роли `git`: сокращение шагов и токенов | [D75](../decisions/D75-git-lean-workflow.md) |
| [Q72](Q72.md) | TRACEABILITY: колонки Q и D — темы или только ссылки? | [D76](../decisions/D76-traceability-links-only.md) |
| [Q73](Q73.md) | полнота реестра задач: видимость всех T-XX через TRACEABILITY | [D77](../decisions/D77-tasks-visibility-completeness.md) |
| [Q74](Q74.md) | T-15: как оформляем программу «процесс, готовый к MCP»? | [D78](../decisions/D78-t15-mcp-ready-program.md) |
| [Q75](Q75.md) | внесение полноты задач в канон `journal.md` §7 | [D79](../decisions/D79-journal-canon-completeness.md) |
| [Q76](Q76.md) | полнота фич: видимость всех `*.feature` через TRACEABILITY | [D80](../decisions/D80-features-visibility-completeness.md) |
| [Q77](Q77.md) | process-mining инструмент `pm`: состав, источники, место | [D81](../decisions/D81-pm-process-mining.md) |
| [Q78](Q78.md) | жизненный цикл `TRACEABILITY`: как разгрести массу `resolved` | [D82](../decisions/D82-traceability-lifecycle-waves.md) |
| [Q79](Q79.md) | волна 2 `TRACEABILITY`: как разобрать оставшиеся `open`-строки | [D83](../decisions/D83-traceability-wave2.md) |
| [Q80](Q80.md) | шум в `.opencode/rules/**`: ссылки на решения и исторические приписки | [D84](../decisions/D84-rules-revision.md) |
| [Q81](Q81.md) | применимость внешнего материала KodaSkills (`skills`) к CREDO | — |
