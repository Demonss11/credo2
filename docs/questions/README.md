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
| [Q4](Q4.md) | `Приоритет` входит в MVP? | [D17](../decisions/D17-priority-out-of-mvp.md) — вне MVP; вернётся в v0.2 с конвейерами | resolved | [Q3](Q3.md) |
| [Q5](Q5.md) | единая легенда статусов и приоритетов? | [D19](../decisions/D19-statuses-priorities-canon.md) — две независимые оси меток; канон — `features/README.md` | resolved | [Q6](Q6.md) (попутно) |
| [Q6](Q6.md) | пересчитать сводную таблицу SPEC §6.1? | [D19](../decisions/D19-statuses-priorities-canon.md) — **попутно (D19)** при Q5: таблица §6.1 удалена, канон — README | resolved | [Q5](Q5.md) |
| [Q7](Q7.md) | §11 не содержит терминов, которыми оперируют фичи (`check`, `active`, `service_hash`, …)? | [D52](../decisions/D52-glossary-terms-canon.md) — терминологический канон §11: 14 новых статей + уточнение двух | resolved | [T-07](../tasks/T-07-meta-fields/README.md); [Q13](Q13.md) |
| [Q8](Q8.md) | отсутствующее поле: ошибка или `0`? | [D21](../decisions/D21-core-semantics-v01.md) — строгая ошибка `UnknownField` (REST 422 / MCP); `0.0` не подставляется | resolved | [Q9](Q9.md) |
| [Q9](Q9.md) | типизация и сравнение чисел | [D21](../decisions/D21-core-semantics-v01.md) — проверка типов на исполнении (`TypeMismatch`); строки только `==`/`!=`; `f64` точно | resolved | [Q8](Q8.md) |
| [Q10](Q10.md) | словарь решений и «не сработало» | [D21](../decisions/D21-core-semantics-v01.md) — словарь задаёт банк, без `"Pass"`; «не сработало» — пустые `decision`/`reason` | resolved | [Q8](Q8.md) |
| [Q11](Q11.md) | язык сообщений об ошибках: русский для человекочитаемого, латиница для машинного? | [D53](../decisions/D53-error-messages-language.md) — «машина / человек»: русский `message`, латиница в кодах/ключах/`name` | resolved | [Q13](Q13.md); [Q22](Q22.md), [Q23](Q23.md); [Q42](Q42.md) |
| [Q12](Q12.md) | единственный источник истины: `.dar`-файлы или песочница+JSON? | [D54](../decisions/D54-source-of-truth-flow.md) — источник истины `.dar`; поток `файл → черновик → публикация`, `stale` | resolved | [Q13](Q13.md), [Q15](Q15.md); Q30, Q33 (ожидают переноса); [T-16](../tasks/T-16-stale-check-test/README.md), [T-08](../tasks/T-08-materialize-source-file/README.md) |
| [Q13](Q13.md) | что именно публикуется и по какому пути? | [D14](../decisions/D14-published-artifact-canon.md) — артефакт — неизменяемая JSON-тройка `checks/{name}/{X}/{Y}/{Z}/` | resolved | [Q7](Q7.md), [Q12](Q12.md); [Q18](Q18.md); Q29, Q32 (ожидают переноса); [T-06](../tasks/T-06-registry-path-xyz/README.md), [T-07](../tasks/T-07-meta-fields/README.md) |
| [Q14](Q14.md) | имя ветки публикации | [D55](../decisions/D55-publish-branch-name.md) — `publish/{name}-{version}`; `checks/...` — путь артефакта, не имя ветки | resolved | [Q13](Q13.md), [Q15](Q15.md) |
| [Q15](Q15.md) | кто и как мержит ветку в `main`? | [D56](../decisions/D56-merge-step.md) — двухшаговый цикл: `check.publish` → ветка, `credo merge` → `main` (ancestor-проверка, CAS, удаление ветки) | resolved | [Q14](Q14.md); Q30, Q32 (ожидают переноса); [T-17](../tasks/T-17-merge-command/README.md) |
| [Q16](Q16.md) | публикация без теста и контрольная сумма | [D32](../decisions/D32-test-gate-mvp.md) — тест-гейт: метка теста в черновике (`last_test_checksum`/`tested_at`); отказ без теста и при расхождении | resolved | [Q12](Q12.md), [Q13](Q13.md); Q34, Q29 (ожидают переноса); [T-02](../tasks/T-02-test-gate/README.md) |
| [Q17](Q17.md) | иммутабельность: файловая система или bare-git? | [D57](../decisions/D57-bare-git-immutability.md) — bare-git, три слоя защиты (каталог версии, отказ на publish, CAS при merge) | resolved | [Q13](Q13.md), [Q15](Q15.md); Q32, Q29 (ожидают переноса); [T-06](../tasks/T-06-registry-path-xyz/README.md), [T-17](../tasks/T-17-merge-command/README.md) |
| [Q18](Q18.md) | сортировка pre-release и build-метаданные | [D35](../decisions/D35-semver-v01.md) — semver v0.1: числовое сравнение pre-release; build вне идентичности и отбрасывается при записи | resolved | [Q13](Q13.md), [Q16](Q16.md); Q32, Q29 (ожидают переноса) |
| [Q19](Q19.md) | где живут данные: `.credo/` и `.dar-notebook/`? | [D58](../decisions/D58-workspace-data-dirs.md) — оба каталога в корне workspace, вне git; серверный и клиентский домены | resolved | [Q12](Q12.md), [Q17](Q17.md); Q31, Q34, Q35 (ожидают переноса) |
| [Q20](Q20.md) | канонические пути REST | [D22](../decisions/D22-rest-paths-canon.md) — канон `/checks/{name}/versions/{version}/...`; сегмент `/versions/` обязателен; конвейеры — отдельный ресурс v0.2 | resolved | [Q21](Q21.md); Q24, Q26, Q36 (ожидают переноса) |
| [Q21](Q21.md) | схема ответа `GET /checks` | [D23](../decisions/D23-get-checks-manifest.md) — манифест `{schema_version, count, service_hash, checks[]}`; `ManifestEntry = name/active/supported/deprecated`; `kind` не вводится | resolved | [Q20](Q20.md), [Q13](Q13.md); Q24, Q36 (ожидают переноса) |
| [Q22](Q22.md) | аутентификация REST — заголовок | [D26](../decisions/D26-rest-auth-x-api-key.md) — `x-api-key`; `CREDO_API_KEY`/`--api-key`; открытые `/health`, `/docs`, `/openapi.json`; единый текст 401 | resolved | [Q23](Q23.md); Q39 (ожидает переноса) |
| [Q23](Q23.md) | формат ошибок REST | [D25](../decisions/D25-rest-error-envelope.md) — конверт `{"error": {"code", "message"}}`; коды `snake_case`, тексты русские; `details` не вводится | resolved | [Q11](Q11.md), [Q22](Q22.md); Q24 (ожидает переноса) |
| [Q42](Q42.md) | каноническая схема объяснения и язык полей | [D21](../decisions/D21-core-semantics-v01.md) — ключи `snake_case` + поле `condition`, тексты русские, без `priority` | resolved | [Q4](Q4.md) (`priority` вне MVP) |
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
| [Q55](Q55.md) | обязателен ли cargo-прогон `validator` для пакетов без изменений кода? | [D50](../decisions/D50-dod-by-package-scope.md) — DoD по составу пакета: cargo только при изменениях кода | resolved | — |
| [Q56](Q56.md) | какими инструментами роли экономят токены при проверках (подсчёт строк; машинная сверка прав)? | [D51](../decisions/D51-agent-tools-token-hygiene.md) — скрипт-сводка `agents-perms.mjs` + `rg -c '^'`; сырой `opencode debug agents` убран | resolved | [service-agent-tools](../../.opencode/mail/service-agent-tools.md) |
