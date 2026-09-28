# questions — журнал вопросов Q/D

> **Назначение:** сводка живого журнала вопросов для быстрого обзора. Источник
> истины — сами `Qn.md`; полные решения — `../decisions/Dn-*.md`; краткий канон —
> `../SPECIFICATION.md` §10; связи Q → D → feature → задача — `../TRACEABILITY.md`;
> до завершения миграции оставшиеся вопросы — в `../OPEN_QUESTIONS.md` (архив).
>
> **Роль в политике Q41:** рабочая сводка, **не канон**: текст вопроса и решения
> живёт в `Qn.md` и `Dn-*.md`, формулировка решения — в §10; статусы и
> приоритеты требований — только в `../features/README.md`.
>
> **Поддержка:** при заведении новой `Qn` `migrator` добавляет строку в эту
> сводку тем же изменением, что и файл; статус — при закрытии решением (процесс
> журнала — [`../BRIEF.md`](../BRIEF.md) §5.1–§5.2).

**Каталог:** `docs/questions/Qn.md` без слага. Сквозной номер в порядке
появления (`Q1`, `Q2`, …); номера не переиспользуются.

**Статусы:** `open` · `resolved by Dn` · `dropped`.

## Сводка

| Q | Тема (1 строка) | Решение | Статус | Связано |
|---|---|---|---|---|
| [Q1](Q1.md) | SPEC — план «с нуля» или доработка `credo2`? | [D15](../decisions/D15-evolution-credo2.md) — эволюция `credo2`, не greenfield | resolved | [T-10](../tasks/T-10-workspace-phase-0/README.md) |
| [Q2](Q2.md) | где канонический DSL: `concept.md`, SPEC или features? | [D16](../decisions/D16-dsl-canon-regex-mvp.md) — канон v0.1 — `GRAMMAR.md` (EBNF включён) | resolved | [Q3](Q3.md) |
| [Q3](Q3.md) | минимальный DSL (regex, одно сравнение) или полноценный лексер/AST? | [D16](../decisions/D16-dsl-canon-regex-mvp.md) — regex-минимум; лексер+AST — v0.2 | resolved | [Q2](Q2.md) |
| [Q43](Q43.md) | каким должен быть цикл работы команды агентов? | [D38](../decisions/D38-agent-cycle.md) — Agile-петля, единый тестировщик, память и почта | resolved | [T-11](../tasks/T-11-agent-cycle/README.md) |
| [Q44](Q44.md) | как разгрузить `lead` и сделать цикл durable? | [D39](../decisions/D39-loop-dispatcher.md) — loop-диспетчер, эфемерный `analyst`, состояние на диске | resolved | [T-12](../tasks/T-12-agent-loop/README.md) |
| [Q45](Q45.md) | какой порог scope-решения в цикле диспетчера? | [D40](../decisions/D40-scope-threshold.md) — узкий порог + триггер частичного покрытия | resolved | [T-13](../tasks/T-13-agent-hardening/README.md) |
| [Q46](Q46.md) | какие уточнения канона цикла нужны по итогам Run 3? | [D41](../decisions/D41-dispatch-refinements.md) — уточнения `dispatch-loop` после Run 3 | resolved | [T-13](../tasks/T-13-agent-hardening/README.md), [T-14](../tasks/T-14-grammar-message-sync/README.md) |
| [Q47](Q47.md) | как формулировать `expect` и нумеровать участки, если `iteration` — rework-цикл? | [D42](../decisions/D42-expect-iteration.md) — `expect` в терминах hard rules; `iteration` — только rework | resolved | — |
| [Q48](Q48.md) | как `auditor` фиксирует отчёт в ленте при правах только на память? | [D43](../decisions/D43-auditor-mail.md) — право `edit .opencode/mail/**` (отчёт в ленту) | resolved | — |
| [Q49](Q49.md) | что делать при расхождении брифа с каноном и при затыке, каких запасов `steps` требуют роли и как работать со срезом и лентами? | [D44](../decisions/D44-run5-refinements.md) — бриф ↔ канон, затык, `steps`, allowlist, срез B1, ленты | resolved | — |
| [Q50](Q50.md) | что закрепляем из волны 0 и каким должен быть конфиг-пакет качества? | [D45](../decisions/D45-wave0-quality-config.md) — A/B1 (норма), B2 (отключить), deny `execute`, конфиг качества | resolved | — |
| [Q51](Q51.md) | как разделять продуктовые и процессные коммиты и как коммитить состояние? | [D46](../decisions/D46-product-process-commits.md) — разделение составов, `chore(process): …`, `state/**` | resolved | — |
| [Q52](Q52.md) | как завершать ветку при устаревшем upstream, что с `git -C` и где хеши closeout? | [D47](../decisions/D47-git-refinements-run5.md) — `push --delete` → `branch -d`; `workdir` полем; хеши в отчёте `git`/`progress` | resolved | — |
| [Q53](Q53.md) | кто ведёт реестр находок, если инструкции адресуют его `migrator`, а права не покрывают `docs/analysis/**`? | [D48](../decisions/D48-findings-registry-owner.md) — реестр ведёт `migrator` (право + синхронизация) | resolved | — |
| [Q54](Q54.md) | чем `validator` подтверждает вхождение коммита в историю ветки при приёмке? | [D49](../decisions/D49-validator-branch-contains.md) — право `git branch --contains` (read-only) | resolved | — |
