# Память: migrator (журнал Q/D)

- **Канон:** `docs/BRIEF.md` (§2 ID, §4 шаблоны, §5.3 сверка, §7 перенос).
- **Правило:** чекпойнт — состояние записи (Q/D), что проверено, следующее
  действие, ссылки. Кратко.

## Чекпойнты

- Чекпойнтов ещё не было.
- 2026-09-28 · журнал W8 т. 2 (Run 5): созданы Q47–Q53/D42–D48 (файлы в
  `docs/questions/**`, `docs/decisions/**`), реестр `docs/analysis/findings-registry.md`
  дополнен F15–F43 (F15/F26/F27 — открыты, остальные закрыты), правки в
  `TRACEABILITY.md` (Q47–Q53) и `SPECIFICATION.md` §10 (№42–48). Сверка у всех D —
  ⚪; Tasks: — («канон внесён сервисной сессией; T-15 — docs-writer»). Право на
  реестр — в фронтматтере `migrator` (D48). F43 закрыт внесением C13 28.09.2026
  (аудит — расхождений нет, приёмка `T-15-c13`). Отчёты — лента
  `.opencode/mail/service-mcp-ready-r2.md`. Коммит — за ролью `git`; открытые
  находки: F15, F26, F27.
- 2026-09-28 · создан `docs/questions/README.md` — сводка живого журнала
  (12 строк: Q1, Q43–Q53; все `resolved`), по стилю `docs/tasks/README.md`;
  шапка — назначение/Q41/поддержка. Q-файлы не трогал. Коммит — позже.
- 2026-09-28 · адресная проверка `docs/questions/README.md` (P2/P3): Q47
  «Связано» → `—` (согласовано с `TRACEABILITY.md`; D42 — «задач не требуется»);
  правило поддержки в шапке сделано самодостаточным. Остальные строки сверены с
  `TRACEABILITY.md` — совпадают.
- 2026-09-28 · service-t11-closeout (H7): заведена карточка
  `docs/tasks/T-16-stale-check-test/README.md` (⬜, P1; `check.test` на
  stale-черновике → исполнение текста файла `rules/{name}.dar`; Источник Q12 +
  `test_draft.feature`; разграничено с T-08), строка в `docs/tasks/README.md`;
  реестр `findings-registry.md` — H7 «задача T-16 заведена»/связь `T-16`,
  F6 «частично закрыт…»/связь `T-15`; в `D38` §«Сверка с кодом» — пометка
  «(уточнено 2026-09-28: замечание закрыто)» про обратные ссылки. Q/D не
  заводились (Q12). Отчёт — лента `.opencode/mail/service-t11-closeout.md`.
  Коммит — за `git`.
- 2026-09-28 · service-t11-closeout (пробел прав `validator`): заведена
  `docs/questions/Q54.md` (🟡, resolved) → `docs/decisions/D49-validator-branch-contains.md`
  (accepted; одна read-only строка allowlist `git branch --contains *` +
  синхронизация `review.md` §«Доступные команды»). Строка №49 в
  `SPECIFICATION.md` §10, строка Q54 в `TRACEABILITY.md` и
  `questions/README.md`, F44 в `findings-registry.md` (закрыт D49/Q54).
  Разграничено: `git diff -- .opencode/...` — квик dot-пути после `--`, не
  пробел прав (уточнение владельца). Сверка D49 — ⚪ (права/процесс); ID
  свободны (последние Q53/D48). Правки канона — сервисная сессия; приёмка —
  смоук `git branch --contains`; коммит — за `git`. Отчёт — та же лента.
- 2026-09-28 · service-canon-hygiene (F45): в `docs/analysis/findings-registry.md`
  добавлена F45 (после F44) — `auditor`: `execute` (Code Mode) не запрещён,
  вызов `tools.shell` → «Unknown tool 'shell'»; статус «закрывается правкой
  канона 28.09.2026»; связь `service-canon-hygiene`. Q/D не заводились
  (образец F40–F42; сверка §5.3 не применима). Отчёт — лента
  `.opencode/mail/service-canon-hygiene.md`. `cargo`/git не запускались.
- 2026-09-28 · service-canon-hygiene (F45 закрыт, F46 заведена): в
  `docs/analysis/findings-registry.md` F45 переведён в «закрыт правкой канона
  28.09.2026» по факту внесения (`auditor.md:32` deny `execute`, `:124-125`
  причина), после F45 добавлена F46 — дубли `cargo fmt|clippy|check|test`
  в каноне (Q41-дрейф), статус «закрыт сведением к одному канону (R2 →
  `dispatch-loop.md`; DoD → `AGENTS.md` §Сборка; матрица → фронтматтеры +
  `review.md`)», связь `service-canon-hygiene`. Сверка §5.3 не применима
  (права/канон), Q/D не заводились. `cargo`/git не запускались. Коммит — за
  `git`; отчёт — та же лента. Открытых находок реестра не добавилось.
- 2026-09-28 · service-migration-q2q3: перенос связки **Q2+Q3 → D16**
  (`docs/decisions/D16-dsl-canon-regex-mvp.md`, `Resolves: Q2, Q3`). Созданы
  `docs/questions/Q2.md`, `Q3.md` (resolved by D16), указатели в
  `OPEN_QUESTIONS.md` (блок Q2–Q3), строки в `TRACEABILITY.md` и
  `questions/README.md`, ссылка в `SPECIFICATION.md` §10 №16. Сверка — ✅:
  `parse_rule` regex-минимум, `GRAMMAR.md` канон v0.1, `SPEC §1.3` без `Тогда`;
  Tasks: — (дрейф канона языка F3/F11 покрыт T-14; фактически цитата в `GRAMMAR.md`
  §2, не §7 — отметить при исполнении T-14). `features/**`/`CHANGELOG` не трогал
  (docs-writer). Отчёт — `.opencode/mail/service-migration-q2q3.md`. Коммит — за
  `git`.
- 2026-09-28 · service-migration-q2q3 (сопутствующая правка T-14, зона
  `docs/tasks/**`): карточка `T-14-grammar-message-sync/README.md` — заголовок
  без `§7`, в «Что сделать»/«Критерий готовности» ссылка `§7` → `§2 (таблица
  «Ограничения v0.1», строка 7)`, в «Примечания» — уточнение про смену
  нумерации и непереписывание исторических „§7“ (D41/§10 №41); сводка
  `docs/tasks/README.md` — строка T-14 `GRAMMAR §7:` → `GRAMMAR:`. Источник/
  даты не трогались. Иные вхождения `§7` в карточке (Контекст `:14`,
  «Что сделать» `:21`) — не расширял, факт в отчёт. `cargo`/git не запускались;
  отчёт — та же лента.
- 2026-09-28 · service-migration-q2q3 (микро-правка, продолжение): уточнение
  §7→§2 в карточке T-14 **завершено** — сняты последние два вхождения
  (Контекст `:14`, «Что сделать» `:21`; оба → `§2`). В карточке `§7` остаётся
  лишь в примечании как намеренная историческая цитата „§7“ (D41/§10 №41).
  Больше ничего не менялось. `cargo`/git не запускались; отчёт — та же лента.
- 2026-09-29 · service-migration-q4: перенос **Q4 → D17**
  (`docs/decisions/D17-priority-out-of-mvp.md`, `Resolves: Q4`). Создан
  `docs/questions/Q4.md` (resolved by D17; 🔴 блокер; дата 2026-09-24), указатель
  в `OPEN_QUESTIONS.md` (блок Q4), строки в `TRACEABILITY.md` и
  `questions/README.md`, ссылка в `SPECIFICATION.md` §10 №17. Сверка — ✅:
  `parse_rule` без `Приоритет`, `GRAMMAR.md` §4/§5 ❌/v0.2, фичи — целевое v0.2;
  Tasks: —. Факт: архивный список фич устарел (`explain.feature` без
  `Приоритет`; актуально `parser`/`execution`/`editor`/`lsp`/`client_explanation`;
  `semver` — MAJOR, иное) — на `docs-writer` (нота + шапки фич), задачи нет.
  `features/**`/`CHANGELOG` не трогал. Отчёт — `.opencode/mail/service-migration-q4.md`.
  Коммит — за `git`.
- 2026-09-29 · service-migration-q5q6: перенос **Q5 → D19**
  (`docs/decisions/D19-statuses-priorities-canon.md`, `Resolves: Q5`) и
  попутного **Q6** (отдельного D нет, BRIEF §7). Созданы `docs/questions/Q5.md`,
  `Q6.md` (resolved by D19; у Q6 — «закрыт попутно при Q5»; приоритет обоих —
  ⚪ оформление), указатели в `OPEN_QUESTIONS.md` (блок Q5–Q6), строки в
  `TRACEABILITY.md` (Q6 — пометка «закрыт попутно при Q5») и
  `questions/README.md` (Q6 — «попутно (D19)»), ссылка/пометка «Q6 — попутно»
  в `SPECIFICATION.md` §10 №19 (текст строки не переписывался). Сверка — ✅:
  SPEC §6.1/§6.2 — ссылки на README без дублей, `features/README.md` — две
  колонки (статус + приоритет), `tests/features_inventory.rs` проверяет
  структуру/полноту/счётчики/«Итого»; Tasks: —. `features/**`/`CHANGELOG` не
  трогал (docs-writer). Отчёт — `.opencode/mail/service-migration-q5q6.md`.
  Коммит — за `git`.
- 2026-09-29 · service-dod-scope: **новая запись** (не из архива) —
  `docs/questions/Q55.md` (🟡, resolved) → `docs/decisions/D50-dod-by-package-scope.md`
  (accepted; пакет без изменений `src/**`/`tests/**` — cargo у `validator` не
  запускается; исключение — правки счётчиков/состава сценариев
  `docs/features/**` → адресный `cargo test --test features_inventory`).
  Строка №50 в `SPECIFICATION.md` §10, строка Q55 в `TRACEABILITY.md` и
  `questions/README.md`. `OPEN_QUESTIONS.md` не трогал (новый Q, не архив);
  Q55 — первый «живой» Q после завершения переносов. Сверка D50 — ⚪
  (процесс/документы): расхождение `review.md:28-33` ↔ `validator.md:87` ↔
  `BRIEF.md` §5.7 зафиксировано. Tasks: — (правки канона — сервисная сессия и
  `docs-writer`). Реестр находок не трогал (вне явного скоупа задачи). ID
  свободны (последние Q54/D49). `cargo`/git не запускались. Отчёт — лента
  `.opencode/mail/service-dod-scope.md`; коммит — за `git`.
- 2026-09-29 · service-dod-scope (P3 аудита, `Cargo.toml`): дополнен (не
  переписан) `docs/decisions/D50-dod-by-package-scope.md` — перечень «пакет без
  изменений кода» → `src/**`, `tests/**`, `Cargo.toml` (п. 1, п. 3, альтернативы
  (а)/(б)), в п. 1 — пометка «уточнено 29.09.2026 при аудите: перечень дополнен
  `Cargo.toml` — зона `coder`; `review.md`/`BRIEF.md` синхронизированы»;
  «Следствия»/«Сверка с кодом» — факт синхронизации. Та же правка — в строке
  №50 `SPECIFICATION.md` §10 (зона `migrator`). `Q55.md` **не менял** (вопрос
  как заданный; перечень — в D50/§10; правка «задним числом» противоречит
  BRIEF §11). Канон `review.md`/`validator.md`/`BRIEF.md` уже синхронизирован
  сервисной сессией/`docs-writer`. `cargo`/git не запускались. Отчёт — та же
  лента; коммит — за `git`.
- 2026-09-29 · service-agent-tools: **новая запись** (не из архива) —
  `docs/questions/Q56.md` (🟡, resolved) → `docs/decisions/D51-agent-tools-token-hygiene.md`
  (accepted; инструменты экономии токенов ролей: скрипт-обёртка
  `.opencode/scripts/agents-perms.mjs` — основной инструмент машинной сверки прав
  (~1.9 КБ вместо 122 КБ JSON); сырой `opencode debug agents` убрать у
  `validator`/`auditor` (при сбое — эскалация); каноничная форма подсчёта строк
  `rg -c '^' <файл>`). Строка №51 в `SPECIFICATION.md` §10, строка Q56 в
  `TRACEABILITY.md` и `questions/README.md`. `OPEN_QUESTIONS.md` не трогал (новый
  Q). Сверка D51 — ⚪ (процесс/инструменты) + факты-замеры (122 511 Б / 2617
  строк; прототип 1 883 Б; `rg -c '^'` = 272). Tasks: — (скрипт и правки канона —
  сервисная сессия; приёмка — `validator` смоук). Ревизия канона подтверждена:
  `review.md:32,86,92`, `validator.md:21`, `auditor.md:25,151-152,178`.
  Реестр находок не трогал (вне скоупа). ID свободны (последние Q56/D51). `cargo`/git
  не запускались; shell одиночные. Отчёт — лента
  `.opencode/mail/service-agent-tools.md`; коммит — за `git`.
- 2026-09-29 · service-agent-tools (P3 аудита, закрыты минимальным диффом):
  (1) `D51` «Решение» п. 2 — ссылки на `auditor.md` без номеров строк
  («allowlist `validator`/`auditor`, шаг 4 чек-листа `auditor` и шаблон его
  отчёта»), в `Affects` снят указатель «строка `opencode debug agents`»;
  (2) число сводки «~1.9 КБ» → «≈3–4 КБ (11 ролей)» в `D51` («Решение»,
  «Следствия»), `SPECIFICATION.md` §10 №51 и `Q56` (вариант (б),
  «Рекомендация») — замер ленты ≈3.4 КБ; (3) `Q56` «Связано» — без «строка
  `opencode debug agents`». Проверки: `rg -n "1\.9 КБ" docs` → пусто;
  `rg -nw "строка" Q56/D51` → пусто. Блок «Сверка с кодом» D51 (стр. 21/25/
  151–152, 178) оставлен как датированная улика (аудит его не адресовал).
  Вне зоны: `agents-perms.mjs:7` (сервисная сессия). `cargo`/git не запускались.
  Отчёт — та же лента; коммит — за `git`.
- 2026-09-29 · service-agent-tools (остаточная P3, помёта снимка): `D51`
  §«Сверка с кодом» — после «ревизия совпадает с лентой» добавлено «(номера
  строк — снимок на момент ревизии 29.09.2026, до правок D51)». Формулировки и
  факты не переписывались. `cargo`/git не запускались. Отчёт — та же лента;
  коммит — за `git`.
- 2026-09-29 · service-agent-tools (P3 приёмки, `AGENTS.md` в D51): в
  `docs/decisions/D51-agent-tools-token-hygiene.md` добавлен `AGENTS.md`
  §«Служебная зона» в `Affects` и в «Решение» п. 4 (норма после `reload` —
  машинная сверка через `node .opencode/scripts/agents-perms.mjs`, ожидание
  `agents: 11 из 18`) — следствие закрытия P2-1 аудита. Больше ничего не
  трогал. `cargo`/git не запускались. Отчёт — та же лента; коммит — за `git`.
- 2026-09-29 · service-migration-q7: перенос **Q7 → D52**
  (`docs/decisions/D52-glossary-terms-canon.md`, `Resolves: Q7`). Создан
  `docs/questions/Q7.md` (resolved by D52; приоритет ⚪ оформление — правит
  противоречие документации, кода не меняет; дата 2026-09-26), указатель в
  `OPEN_QUESTIONS.md` (блок Q7), строки в `TRACEABILITY.md` (Задачи: T-07) и
  `questions/README.md`, **новая** строка №52 в `SPECIFICATION.md` §10
  (решения Q7 в §10 не было — BRIEF §7). Сверка — ⚪ (документы): статьи §11
  на месте (14 новых + уточнения `Черновик`/`Публикация`); пометка
  `publish.feature:7-9` на месте; `CheckMeta` (`src/core.rs`) без
  `display_name`/`source_hash`/`compiler_version` — расхождение покрыто
  **T-07** (Источник: Q13, Q7), новой задачи нет (`Tasks: —`); `ManifestEntry`/
  `service_hash` (`src/lib.rs`) сверены с §11. `features/**`/`CHANGELOG` не
  трогал (docs-writer); в блоке Q13 архива осталась фраза «открытый Q7»
  (`OPEN_QUESTIONS.md:443`) — чужая запись, снимется при переносе Q13.
  ID свободны (последние Q56/D51; теперь — Q57/D53). `cargo`/git не
  запускались; shell одиночные. Отчёт — лента
  `.opencode/mail/service-migration-q7.md`; коммит — за `git`.
- 2026-09-29 · P3 приёмки service-migration-q7: заголовок указателя Q7 в
  `OPEN_QUESTIONS.md` приведён к формату соседей — `### Q7. ✅ Термины в
  глоссарии (решено 2026-09-26) → перенесён`; тело указателя («Мигрирован …»,
  ссылки на `questions/Q7.md` и `decisions/D52-glossary-terms-canon.md`) не
  трогалось. `cargo`/git не запускались; отчёт — та же лента.
- 2026-09-29 · service-migration-q8q10q42: перенос связки **Q8, Q9, Q10, Q42 →
  D21** (`docs/decisions/D21-core-semantics-v01.md`, `Resolves: Q8, Q9, Q10,
  Q42`, `Спека: §10 №21`). Созданы `docs/questions/Q8.md`, `Q9.md`, `Q10.md`,
  `Q42.md` (resolved by D21; даты 2026-09-24; приоритеты: Q8/Q9 — 🔴 блокер,
  Q10/Q42 — 🟡 важно, обоснованы). Указатели в `OPEN_QUESTIONS.md` (блок
  Q8–Q10, Q42; Q11 не тронут), строки в `TRACEABILITY.md` (4) и
  `questions/README.md` (4), ссылка «(полный контекст — [D21]…)» в
  `SPECIFICATION.md` §10 №21 (текст не переписывался; строка №52 не тронута).
  Сверка — ✅: `src/core.rs` (`EvalError::UnknownField`/`TypeMismatch`,
  `evaluate_rule -> Result`, `Explanation` c `condition`/без `priority`,
  `contract_from_rule` без `"Pass"`; тесты `*_q8/*_q9/*_q10/*_q42`),
  `src/rest.rs` (422 `evaluation_failed`), `src/mcp.rs` (ошибка инструмента),
  фичи `errors/explain/explain_full/execution/test_draft`, `features/README.md`.
  Нюанс: сценарий «Несовместимые типы» в `errors.feature` описан на парсере —
  целевое v0.2 (уже 🟡 в README), задач не заводилось (`Tasks: —`).
  `features/**`/`CHANGELOG` не трогал (docs-writer). `cargo`/git не
  запускались; shell одиночные. ID свободны (последние Q56/D52). Отчёт — лента
  `.opencode/mail/service-migration-q8q10q42.md`; коммит — за `git`.
- 2026-09-29 · service-migration-q8q10q42 (возврат P1, iteration 1): сняты
  битые markdown-ссылки на непересённые вопросы в новых файлах — `Q8.md:57`
  `[Q11](Q11.md)` → `Q11 (ожидает переноса)`; `Q42.md:54` → `(Q11 (ожидает
  переноса))`; `Q10.md:11` и `:62` `[Q36](Q36.md)–[Q39](Q39.md)` → `Q36–Q39
  (ожидают переноса)`. Конвенция соседей — `Q7.md:11` (ID текстом). Проверка:
  `rg "Q(11|36|39)\.md\)" docs/questions` пусто (в `docs` остались только
  цитаты в отчёте приёмки `docs/reviews/migration-q8q10q42-2026-09-29.md`).
  Решения не переписывались; `cargo`/git не запускались. Отчёт — та же лента;
  повторная приёмка — `validator` `-r2`.
- 2026-09-29 · service-migration-q11: перенос **Q11 → D53**
  (`docs/decisions/D53-error-messages-language.md`, `Resolves: Q11`). Создан
  `docs/questions/Q11.md` (resolved by D53; приоритет 🟡 важно — канон языка
  сообщений, демо не блокирует; дата 2026-09-25), указатель в `OPEN_QUESTIONS.md`
  (блок Q11), строки в `TRACEABILITY.md` (Задачи: —) и `questions/README.md`,
  **новая** строка №53 в `SPECIFICATION.md` §10 (решения Q11 в §10 не было —
  BRIEF §7); сняты устаревшие «Q11 (ожидает переноса)» в `Q8.md:57`/`Q42.md:54`
  → живые ссылки `[Q11](Q11.md)`. Сверка — ✅: `src/rest.rs` (тексты 404/410/
  409/401, коды латиницей), `src/mcp.rs` (русские `message`, 10 `snake_case`-
  кодов), тесты `tests/rest.rs`/`tests/common/mod.rs`/`tests/mcp_errors.rs`;
  резерв `version_deprecated` в MCP — известный (T-09/Q33); `Tasks: —`.
  `features/**`/`CHANGELOG` не трогал (docs-writer). ID свободны (последние
  Q56/D52; теперь — Q57/D54). `cargo`/git не запускались; shell одиночные.
  Отчёт — лента `.opencode/mail/service-migration-q11.md`; коммит — за `git`.
- 2026-09-29 · service-migration-q11 (P3 приёмки): в `docs/TRACEABILITY.md` строка
  Q11 (`:18`) в колонке «Feature» дополнена [`errors.feature`](features/errors.feature)
  (согласовано с `Affects` D53 и обратной ссылкой `errors.feature:5`). Больше
  ничего не трогал. `cargo`/git не запускались; отчёт — та же лента.
- 2026-09-29 · service-migration-q11 (микроправка D53): в §«Следствия» D53
  (`:63-64`) в перечень шапочных пометок Q11 добавлен `errors.feature`
  (`evaluate` / `rest_api` / `rest_auth` / `errors`). Больше ничего не менялось.
  `cargo`/git не запускались; отчёт — та же лента.

