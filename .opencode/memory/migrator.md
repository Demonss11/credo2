# Память: migrator (журнал Q/D)

- **Канон:** `.opencode/rules/journal.md` (§2 ID, §4 шаблоны, §5.3 сверка).
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
- 2026-09-29 · service-migration-q12q15 (вызов 1 из 2): перенос **Q12 → D54**
  (`docs/decisions/D54-source-of-truth-flow.md`, `Resolves: Q12`, §10 №54) и
  **Q13 → D14** (`docs/decisions/D14-published-artifact-canon.md`,
  `Resolves: Q13`, §10 **№14** — строка была без D-ссылки, ссылка добавлена).
  Созданы `docs/questions/Q12.md`, `Q13.md` (resolved; даты 2026-09-25;
  приоритеты: Q12 🔴 блокер, Q13 🟡 важно). Указатели в `OPEN_QUESTIONS.md`
  (блок Q12–Q13), строки в `TRACEABILITY.md` (после Q11) и
  `questions/README.md` (между Q11 и Q42). Сняты устаревшие пометки
  «Q13 (ожидает переноса)» в `questions/README.md` (строки Q7/Q11),
  `Q7.md:11`, `Q11.md:10-11` → `[Q13](Q13.md)`. Сверка §5.3 — 🟡/🟡:
  Q12 — `src/lib.rs` `source_hash`/`stale` ✅ (T-01), `check.test` по тексту
  черновика `src/mcp.rs:235` → **T-16**, нет материализации `.dar`
  `src/lib.rs:389-393` → **T-08**; Q13 — плоский путь `src/lib.rs:342/383/462`
  → **T-06**, `CheckMeta` `src/core.rs:70-81`/запись `src/lib.rs:373-381` без
  `display_name`/`source_hash`/`compiler_version` → **T-07**; артефакт-тройка/
  неизменяемость ✅; `publish.feature`/`storage_paths.feature` обновлены по
  решению (читал, не правил). Новых расхождений MVP нет, новых задач нет.
  «Уточнение Q32 (ожидает переноса)» и «Следствие для кода (Q7)» — в Следствиях
  D14. `features/**`/`CHANGELOG`/`src`/`tests` не трогал (docs-writer/чужие
  зоны); `cargo` не запускался (D50); shell одиночные. ID: заняты D14, D54;
  свободны — Q57, §10 №55 (под Q14/D55). Отчёт — лента
  `.opencode/mail/service-migration-q12q15.md`; коммит — за `git`.
- 2026-09-29 · service-migration-q12q15 (вызов 1, верификация): `rg -n
  "^### Q1[23]" docs/OPEN_QUESTIONS.md` → 104/109 (указатели, полных текстов
  нет); `rg "Q13 \(ожидает переноса\)" docs` → только `docs/reviews/**`
  (историческое, не правится). Остаток вызова 1 закрыт; впереди вызов 2 —
  Q14 → D55, Q15 → D56.
- 2026-09-29 · service-migration-q12q15 (вызов 2 из 2): перенос **Q14 → D55**
  (`docs/decisions/D55-publish-branch-name.md`, `Resolves: Q14`, новая §10
  №55) и **Q15 → D56** (`docs/decisions/D56-merge-step.md`, `Resolves: Q15`,
  новая §10 №56). Созданы `docs/questions/Q14.md` (⚪ оформление),
  `Q15.md` (🟡 важно); указатели в `OPEN_QUESTIONS.md` (блок Q14–Q15),
  строки в `TRACEABILITY.md` (после Q13) и `questions/README.md` (между Q13 и
  Q42). Заведена задача **T-17** (`docs/tasks/T-17-merge-command/README.md`,
  Источник: D56 (Q15), P2 — **на подтверждение владельца**) + строка в
  `docs/tasks/README.md`. Сверка §5.3 — Q14 ✅ (`src/lib.rs:397`,
  `src/mcp.rs:326/332`, `tests/publish.rs:27`, `publish_rules.feature:10`,
  `publish.feature:25`; задач не требуется); Q15 🟡 (ветка создаётся ✅
  `src/lib.rs:321-409`; шага слияния нет ⬜: нет `credo merge`,
  `merge-base --is-ancestor`, CAS-`update-ref` (`update_ref` 3-арг
  `src/lib.rs:161-164`; `create_ref` zero-oid `:169-179`), удаления ветки;
  `src/mcp.rs:335-340` — ручной un-CAS `git update-ref`;
  `git_integration.feature:42-48` → T-17). Новых расхождений вне T-17 нет.
  Блок **Q12–Q15 закрыт**. `features/**`/`CHANGELOG`/`src`/`tests` не трогал;
  `cargo` не запускался (D50); shell одиночные. ID: заняты D14/D54/D55/D56;
  свободны — Q57, §10 №57. Отчёт — лента
  `.opencode/mail/service-migration-q12q15.md`; коммит — за `git`.
- 2026-09-29 · service-migration-q12q15 (сверка связей, P3-аналог):
  `docs/TRACEABILITY.md` — строка Q12 (:19) дополнена `notebook_ui.feature`,
  `editor.feature`, `file_management.feature` (3 → 6, как `D54:9-14`); строка
  Q13 (:20) дополнена `mcp_tools.feature` (4 → 5, как `D14:10-14`). Q14 (:21) и
  Q15 (:22) сверены — `.feature`-состав совпадает с `Affects` `D55`/`D56`
  (`D55` дополнительно ссылается на `features/README.md` — сводка, не
  `.feature`). Больше ничего не менял; фичи не правил; `cargo` не запускался.
  Отчёт — та же лента; коммит — за `git`.
- 2026-09-29 · service-migration-q12q15 (rework P2/P3, после приёмки iteration 1):
  P2 — Q15 выведен из «ожидают переноса» (`Q12.md:13-14`,
  `D54:129-132`, `questions/README.md:36`) и стал живой ссылкой; в группе
  оставлены только не перенесённые (Q30/Q33/Q32/Q29 и т. п.). P3-1 — единая
  метка блока **«Q12–Q15»**: `OPEN_QUESTIONS.md:114,119` и поля «Перенос»
  `Q12.md:12`/`Q13.md:11`/`Q14.md:9`/`Q15.md:10`. P3-2 — `TRACEABILITY.md:19`
  (Q12, «Задачи») добавлен `T-01` → `T-16, T-08, T-01`; Q13/Q14/Q15 состав
  задач = `Tasks` их D. Свип `rg "ожида(ет|ют) переноса"` — пометки только у
  реально открытых Q; форма `Q1[2-5] \(ожидает переноса\)` — только
  `docs/reviews/**`. Фичи/`CHANGELOG`/`src`/`tests` не трогал; `cargo` не
  запускался (D50). Повторная приёмка — `validator` `-r2`; отчёт — та же лента;
  коммит — за `git`.
- 2026-09-29 · service-migration-q16q19 (вызов 1 из 2): перенос **Q16 → D32**
  (`docs/decisions/D32-test-gate-mvp.md`, `Resolves: Q16`; строка §10 №32 уже
  была — маркер `(Q34 (ожидает переноса), Q16; …[D32]…)`) и **Q17 → D57**
  (`docs/decisions/D57-bare-git-immutability.md`, `Resolves: Q17`, новая §10
  №57). Созданы `docs/questions/Q16.md` (🟡), `Q17.md` (🟡) — resolved, дата
  2026-09-26; указатели в `OPEN_QUESTIONS.md` (блок **Q16–Q19**, :124/:129);
  строки в `TRACEABILITY.md` (:23/:24) и `questions/README.md` (:40/:41).
  Сверка §5.3 — Q16 🟡: метка теста ✅ (`src/lib.rs` 746–750/850–874;
  `src/mcp.rs` 238–248, `draft_json` 439–453), гейт publish ⬜ (нет чтения
  `last_test_checksum` в `src/mcp.rs:282-342`/`src/lib.rs:321-409`) → T-02
  (новых задач нет; T-05 зависит); кэш `.dar-notebook/results-cache.json` — зона
  Notebook v0.2. Q17 🟡: слой 2 ✅ (`src/lib.rs` 344–351, `create_ref` 169–180,
  397–400), слой 1 плоский путь `checks/{name}/{vstr}` (342/383/462) → T-06,
  слой 3 CAS-merge ⬜ → T-17. `features/**`/`src`/`tests`/`CHANGELOG` не трогал;
  `cargo` не запускался (D50); shell одиночные. ID: заняты D32/D57; следующая
  свободная §10 — **№58** (под Q19/D58 в вызове 2). Отчёт — лента
  `.opencode/mail/service-migration-q16q19.md`; коммит — за `git`.
- 2026-09-29 · service-migration-q16q19 (вызов 2 из 2): перенос **Q18 → D35**
  (`docs/decisions/D35-semver-v01.md`, `Resolves: Q18`, строка §10 №35 уже
  была — маркер `(Q18; …[D35]…)`) и **Q19 → D58**
  (`docs/decisions/D58-workspace-data-dirs.md`, `Resolves: Q19`, новая §10
  **№58**). Созданы `docs/questions/Q18.md`, `Q19.md` (⚪ оформление, resolved,
  дата 2026-09-26); указатели в `OPEN_QUESTIONS.md` (блок **Q16–Q19**,
  :134/:139); строки в `TRACEABILITY.md` (:25/:26) и `questions/README.md`
  (:42/:43). Сверка §5.3 — Q18 ✅ (`src/core.rs` `compare_pre`/`compare_ident`
  381–412, `Ord` 415–426, `PartialEq` без build 272–279, `as_storage` 343–351;
  тесты 630–634/651–656/658–663/699–708; дубль build → `src/lib.rs` 344–351 +
  `src/mcp.rs` 59/103); Q19 ⚪ (документная: `.credo/` — `src/lib.rs` 784–787;
  SPEC §4.3 368–372; `notebook_ui.feature:40`). Задач не требуется (known —
  T-05). Свип «ожидает переноса»: сняты пометки у Q18 в `Q13.md:14`,
  `questions/README.md:37`, `D14:55,146-148` → живая ссылка; форма
  `Q1[89] \(ожидает переноса\)` — пусто (reviews не трогал). Блок **Q16–Q19
  закрыт**. `features/**`/`CHANGELOG`/`src`/`tests` не трогал; `cargo` не
  запускался (D50); shell одиночные. ID: заняты D35/D58; следующий свободный —
  §10 **№59** (D59), Q-номер **Q57**. Отчёт — та же лента; коммит — за `git`.
- 2026-09-29 · service-decisions-readme: создан `docs/decisions/README.md` —
  рабочая сводка решений, зеркально `questions/README.md` (не канон, прецедент
  Q41). Шапка (Назначение / Q41 / Поддержка), «Каталог», «Статусы», таблица
  **29 строк** (`D | Тема | Решает | Дата | Задачи | Статус`, порядок по номеру;
  D14…D58 — сверено с glob, 29 файлов). Данные — из шапок `Dn-*.md`
  (`Статус`/`Дата`/`Resolves`/`Tasks`); нюанс D32: `Q16` + `Q34 (ожидает
  переноса)`; D54: `T-16, T-08 (T-01 — сделана)`. **С этого момента сводка
  ведётся при заведении каждого нового `Dn`** (шапка «Поддержка», процесс
  §5.2) — добавлено в правило поддержки. Проверки: 29 строк, каждый D один раз,
  цели ссылок существуют; `cargo` не запускался (D50); shell одиночные.
  Q/D не заводились. `questions/README.md`/`docs/README.md`/`BRIEF.md`/
  `SPECIFICATION.md`/`TRACEABILITY.md` не трогал (карта и CHANGELOG — за
  `docs-writer`). Отчёт — лента `.opencode/mail/service-decisions-readme.md`;
  коммит — за `git`.
- 2026-09-29 · service-decisions-readme (P3, микроправка): `README.md:17`
  уточнён абзац про пропуски номеров §10 — «до-журнальные решения **и решения
  ещё не перенесённых вопросов (Q20–Q41)**; при переносе записи получают
  D-файлы с этими номерами (прецедент — D32/D35)» (абзац :17-19). Таблица и
  шапка не тронуты; `cargo` не запускался (D50); не коммитил. Отчёт — та же
  лента.
- 2026-09-29 · service-migration-q20q23 (вызов 1 из 2): перенос **Q20 → D22**
  (`docs/decisions/D22-rest-paths-canon.md`, `Resolves: Q20`, строка §10 №22
  уже была — маркер `(§1.3.1, Q20; …[D22]…)`) и **Q21 → D23**
  (`docs/decisions/D23-get-checks-manifest.md`, `Resolves: Q21`, §10 №23 —
  `(Q21; …[D23]…)`). Созданы `docs/questions/Q20.md`, `Q21.md` (дата
  2026-09-25, resolved, «Перенос 2026-09-29, блок Q20–Q23»). Сводки: строки
  D22/D23 в `decisions/README.md` (после D21; шапка «Q20–Q41» → «Q22–Q41»),
  Q20/Q21 в `questions/README.md` и `TRACEABILITY.md` (после Q19; Feature —
  ровно `.feature` из `Affects`). Свип: §1.3.1 (:50) и §4.4 (:406) —
  `OPEN_QUESTIONS.md` заменён на D22/D23; `D53` «Связанные» — Q20 живой
  ссылкой (Q22/Q23 не трогал). Архив: блоки Q20 (:148–218) и Q21 (:220–256) →
  указатели «→ перенесён» (формат Q16–Q19, «блок Q20–Q23»); Q22 (:158)/Q23 не
  трогались. Сверка §5.3 — обе ✅ (Q20: `src/rest.rs` маршруты 20–29, OpenAPI
  235–360, фичи `rest_api/evaluate/batch/import_export`, `tests/rest.rs`;
  Q21: `src/lib.rs` `ManifestEntry` 525–531/`build_manifest` 541–608,
  `src/rest.rs` `manifest_response` 96–111/409 161–168, unit-тесты
  927/938/948 и 421–450, `tests/rest.rs` 126–144). Задачи: — («код уже
  соответствует канону»). `features/**`/`CHANGELOG`/`src`/`tests` не трогал;
  `cargo` не запускался (D50); архив — одним PowerShell-rewrite (numstat
  9/109). ID: заняты D22/D23; далее — **D26 (Q22), D25 (Q23)** (§10 №26/№25
  уже есть), затем §10 **№59**, Q-номер **Q57**. Отчёт — лента
  `.opencode/mail/service-migration-q20q23.md`; коммит — за `git`.
- 2026-09-29 · service-migration-q20q23 (вызов 2 из 2): перенос **Q22 → D26**
  (`docs/decisions/D26-rest-auth-x-api-key.md`, `Resolves: Q22`, строка §10
  №26 уже была — маркер `(Q22; …[D26]…)`) и **Q23 → D25**
  (`docs/decisions/D25-rest-error-envelope.md`, `Resolves: Q23`, §10 №25 —
  `(Q23; …[D25]…)`). Созданы `docs/questions/Q22.md`, `Q23.md` (дата
  2026-09-25, resolved, «Перенос 2026-09-29, блок Q20–Q23»). Сводки: строки
  D25/D26 в `decisions/README.md` (между D23 и D32; parenthetical «Q22–Q41» →
  «Q24–Q41»), Q22/Q23 в `questions/README.md` и `TRACEABILITY.md` (Feature —
  ровно `.feature` из `Affects`). Свип: `Q11.md:12` и `D53` «Связанные» —
  Q22/Q23 живыми ссылками. Архив: блоки Q22 (:158–202) и Q23 (:204–254) →
  указатели «→ перенесён» (формат Q20/Q21, «блок Q20–Q23»). Сверка §5.3 — обе
  ✅ (Q22: `src/rest.rs` `auth` 63–85, `src/main.rs` `--api-key`/
  `CREDO_API_KEY` 34–36, `tests/rest.rs` 293–310; Q23: `src/rest.rs` `err()`
  52–61 и коды, `details` нет, `tests/common/mod.rs` `assert_error_envelope`
  184–225, `tests/rest.rs` 376+). Задачи: — («код уже соответствует»).
  `features/**`/`CHANGELOG`/`src`/`tests` не трогал; `cargo` не запускался
  (D50). **Блок Q20–Q23 закрыт** (D22/Q20, D23/Q21, D26/Q22, D25/Q23). ID:
  заняты D22/D23/D25/D26; далее §10 **№59**, Q-номер **Q57**. Отчёт — та же
  лента; коммит — за `git`.
- 2026-09-29 · service-migration-q24q26 (вызов 1 из 2): перенос **Q24 → D36**
  (`docs/decisions/D36-batch-deferred.md`, `Resolves: Q24`, §10 №36 — добавлена
  D-ссылка) и **Q25 → D37** (`docs/decisions/D37-client-explanation-deferred.md`,
  `Resolves: Q25`, §10 №37). Созданы `docs/questions/Q24.md`, `Q25.md` (дата
  2026-09-26, ⚪ оформление, «Перенос: 2026-09-29, блок Q24–Q26»). Сводки:
  D36/D37 в `decisions/README.md` (после D35, перед D38; **35 строк**,
  parenthetical «Q24–Q41» → «Q26–Q41»); Q24/Q25 в `TRACEABILITY.md` и
  `questions/README.md` (Feature — ровно `.feature` из `Affects`). Архив:
  Q24/Q25 → указатели (метка «блок Q24–Q26»), Q26 полный — ждёт вызова 2.
  Сверка §5.3 — обе ⚪ (отсрочка): `src/rest.rs` 19–31 без `.../batch`, поиск
  по `src/` 0; фичи-шапки «ОТЛОЖЕНО», `features/README.md` ⏸/⏳ (275/276);
  Tasks: —. Свип «ожидает переноса»: Q20/Q21/Q23 + `questions/README.md` —
  Q24 живой ссылкой. SPEC §10 №36/№37 — D-ссылки (маркеры целы); OPEN_QUESTIONS
  в SPEC — :507 Q29 (не трогать) и шапка (:816). `features/**`/`CHANGELOG`/
  `src`/`tests` не трогал; `cargo` не запускался (D50). **Q26 → D24 (§10 №24)**
  — вызов 2. Далее §10 **№59**, Q57. Отчёт — лента
  `.opencode/mail/service-migration-q24q26.md`; коммит — за `git`.
- 2026-09-29 · service-migration-q24q26 (вызов 2 из 2): перенос **Q26 → D24**
  (`docs/decisions/D24-import-export-deferred.md`, `Resolves: Q26`, §10 №24 —
  добавлена D-ссылка). Создан `docs/questions/Q26.md` (дата 2026-09-25,
  ⚪ оформление, «Перенос: 2026-09-29, блок Q24–Q26»). Сводки: D24 в
  `decisions/README.md` (между D23 и D25; **36 строк**; parenthetical
  «Q26–Q41» → «Q27–Q41»); Q26 в `TRACEABILITY.md` и `questions/README.md`
  (Feature — `import_export.feature`). Архив: Q26 → указатель, полного текста
  нет. Сверка §5.3 — ⚪ (отсрочка): в `src/` нет `check.export_draft`/
  `check.import`/`text/dar` (только «сторож»-комментарий `mcp.rs:597` + тест
  `no_import_export_tools_q26`), `rest.rs` 19–31 без экспорта/импорта; шапка
  фичи «ОТЛОЖЕНО», `features/README.md:274` ⏸/⏳; Tasks: —. Свип: Q26 — живой
  ссылкой в Q20/Q24/D36/D22(Связанные)/`questions/README.md`. **Связка
  Q24–Q26 закрыта** (D36/D37/D24). Дальше 4б-2 (Q27+Q30); §10 **№59**, Q57.
  `features/**`/`CHANGELOG`/`src`/`tests` не трогал; `cargo` не запускался
  (D50). Отчёт — лента `.opencode/mail/service-migration-q24q26.md`; коммит —
  за `git`.
- 2026-09-29 · service-migration-q28q29 (вызов 1 из 2): перенос **Q28 → D31**
  (`docs/decisions/D31-check-create-contract.md`, `Resolves: Q28`). Создан
  `docs/questions/Q28.md` (дата 2026-09-26, resolved, «Перенос: 2026-09-29,
  блок Q28–Q29»). Сводки: D31 в `decisions/README.md` (между D26 и D32;
  **37 строк**; parenthetical «(Q27–Q41)» → «(Q27, Q30–Q41)»); Q28 в
  `TRACEABILITY.md` (Feature — `draft.feature`, `agent_minimal.feature`) и
  `questions/README.md`. §10 №31 (:854) — добавлено «полный контекст —
  [D31](decisions/D31-check-create-contract.md)». Архив: Q28 (:215–252) →
  указатель, метка «блок Q28–Q29». Свип Q28: `Q45.md:18`,
  `D40-scope-threshold.md:19` — живой ссылкой; Q29 не трогался. Сверка §5.3 —
  ✅: `src/mcp.rs` `create` 155–184 (`{name,source}`, `parse_rule` 167, сверка
  заголовка 168–173, ответ `{status,name}` 183, `upsert_draft` 180), tool spec
  472–477, юнит-тесты 676–733; `tests/mcp_draft.rs` 63–99/104–141/369–402/
  408–481/487–499; `tests/mcp_errors.rs`; фичи `draft.feature` 2–4/16–25/58–71,
  `agent_minimal.feature` 40–41; SPEC §4.5/§7. Tasks: T-03 — **сделана**;
  новых нет. Вызов 2 — **Q29 → D34** (`D34-mcp-tool-contracts.md`; §4.5, не
  копировать; `SPECIFICATION.md:507` → D34; свип Q29). Q27/Q30 — 4б-2.
  `features/**`/`CHANGELOG`/`src`/`tests` не трогал; `cargo` не запускался
  (D50). Отчёт — лента `.opencode/mail/service-migration-q28q29.md`; коммит —
  за `git`.
- 2026-09-29 · service-migration-q28q29 (вызов 2 из 2): перенос **Q29 → D34**
  (`docs/decisions/D34-mcp-tool-contracts.md`, `Resolves: Q29`; `Tasks:` T-04
  (сделана), T-05 (открыта)). Создан `docs/questions/Q29.md` (дата
  2026-09-26, resolved, «Перенос: 2026-09-29, блок Q28–Q29»). Канон: §4.5 —
  общие правила/коды-перечень/инварианты/сводная таблица; в D34 — таблица
  «когда используется» для 10 кодов, полные JSON-схемы всех 10 инструментов,
  вне-MVP (ничего не потеряно). Сводки: D34 в `decisions/README.md` (между D32
  и D35; **38 строк**; parenthetical «(Q27, Q30–Q41)» остаётся); Q29 в
  `TRACEABILITY.md` (Feature — `mcp_tools`/`draft`/`test_draft`/`publish`/
  `deprecation`) и `questions/README.md`. §10 №34 (:857) — «полный контекст —
  [D34]». Архив: блок Q29 (после сдвига :220–526) → указатель, метка «блок
  Q28–Q29»; Q30 :225 цел. Свип Q29: `Q45.md:18`, `D40-scope-threshold.md:19`,
  `Q12/Q13/Q16/Q17/Q18.md`, `questions/README.md`, `D32/D35/D54/D14/D57` —
  живой ссылкой; «ожидает переноса» у Q29 не осталось. Сверка §5.3 — **🟡**:
  ✅ конверт+10 кодов (`src/mcp.rs` 38–67, 72–131, 533–546), инварианты 1–5
  (`create` 155–184, `draft_json` 438–455, `test_valid` 439–440), `list_drafts`
  186–199 / `get_draft` 438–455 / `test` 250–261; 🟡 схемы успеха —
  `delete_draft` 277–279, `publish` 328–341, `deprecate` 391–396,
  `list_published` 399–411, `rebuild_manifest` 413–419 → T-05 (открыта). Снят
  дрейф «полные схемы»: SPEC :525 и §10 №34 → D34; `SPECIFICATION.md:507` →
  D34 (OPEN_QUESTIONS в SPEC осталась только в шапке §10 :816); D34 :314,
  OPEN_QUESTIONS :714. **Связка «MCP-контракты» закрыта** (D31/Q28, D34/Q29).
  Далее — 4б-2 (Q27+Q30); свободный §10 — **№59**, следующий Q — **Q57**.
  `features/**`/`CHANGELOG`/`src`/`tests` не трогал; `cargo` не запускался
  (D50). Отчёт — та же лента; коммит — за `git`.
- 2026-09-29 · service-migration-q28q29 (P2 rework): приёмка `validator` дала P2 —
  `D34-mcp-tool-contracts.md:420` держал `Q18 (semver)` в перечне «ожидают
  переноса», хотя Q18 мигрирован (D35). Правка одной строки: `:420–421` →
  `[Q18](../questions/Q18.md) / [D35](D35-semver-v01.md)` (semver),
  `Q33 (draft-first) — ожидает переноса`. Свип
  `rg "ожида[ею]т переноса" docs/decisions/D34-mcp-tool-contracts.md` → только
  Q33. `cargo` не запускался (D50). Отчёт — лента (секция `P2`); далее приёмка
  `-r2`; коммит — за `git`.
- 2026-09-29 · service-migration-q27q30 (вызов 1 из 2): перенос **Q27 → D27**
  (`docs/decisions/D27-rest-launch-address.md`, `Resolves: [Q27]`; слаг
  `D27-rest-launch-address`). Создан `docs/questions/Q27.md` (дата 2026-09-25,
  🟡 важно, «Перенос: 2026-09-29, блок 4б-2 "Транспорт и запуск", Q27+Q30»).
  Кросс-ссылки: D27/Q27 → **живые** `[Q30](../questions/Q30.md)` +
  `[D29](D29-notebook-mcp-transport.md)` (появятся вызовом 2); «ожидает
  переноса» про Q30/Q29 в D27 нет. Сводки: D27 в `decisions/README.md` (между
  D26 и D31; **39 строк**); parenthetical `:18` «(Q27, Q30–Q41)» → «(Q30–Q41)»;
  §10 №27 (`:850`) — `(§4.2, Q27; полный контекст — [D27]…)`, второй столбец и
  маркер `(Q30)` в №29 целы; Q27 в `TRACEABILITY.md` (`:34`, Feature — `—`) и
  `questions/README.md` (`:51`). Архив: Q27 (`:183`) → указатель, метка
  «блок 4б-2…, Q27+Q30» (НЕ «Q27–Q30»: Q28/Q29 — другой блок, не смежные).
  Свип Q27: `D31:150` — `Q27, Q30+` → `[Q27]… (запуск и адрес REST), Q30+`;
  прочие — SPEC `:185/:270/:303` (bare-упоминания канона) и `features/README.md:216`
  (docs-writer). Сверка §5.3 — ✅: `src/main.rs` (`:9` адрес `127.0.0.1:8080`,
  `:19–28` флаги, `:56–65` приоритет `--no-rest`>`--addr`>`--rest`, `:76–92`
  REST по `Some(addr)` / иначе только `run_stdio`); SPEC §4.2 `:303–317`, §10
  `:850`; карта зон `features/README.md:328` (CLI → `main.rs`). Tasks: — (код
  соответствует; расхождение было в SPEC; фичи — инфраструктура, D27 прямо
  фиксирует). `cargo` не запускался (D50); `src`/`tests`/`features`/`CHANGELOG`
  не трогал. Следующий §10 — №59, Q — Q57. Отчёт — лента
  `.opencode/mail/service-migration-q27q30.md`; коммит — за `git`.
  **Вызов 2 (тем же sessionID):** Q30 → D29 (`D29-notebook-mcp-transport.md`),
  оживить ссылки на D27/Q27, свип Q30 (`decisions/README.md:18` → «(Q31–Q41)»,
  `Q12:14`, `Q15:13`, `questions/README.md:36,39`, `D54:132`, `D56:120`).
- 2026-09-29 · service-migration-q27q30 (вызов 2 из 2; блок 4б-2 закрыт): перенос
  **Q30 → D29** (`docs/decisions/D29-notebook-mcp-transport.md`, `Resolves: [Q30]`;
  слаг `D29-notebook-mcp-transport`). Создан `docs/questions/Q30.md` (2026-09-26,
  🟡 важно, «Перенос: 2026-09-29, блок 4б-2 "Транспорт и запуск", Q27+Q30»).
  Кросс-ссылки Q27↔Q30 живые в обе стороны (D27↔D29, Q27↔Q30). Сводки: §10 №29
  (`:852`) — `(Q30; полный контекст — [D29]…)`, второй столбец/маркер `(Q27)`
  целы; D29 в `decisions/README.md` (D27 `:38` → **D29 `:39`** → D31 `:40`;
  **40 строк**); parenthetical `:18` → «(Q31–Q41)»; Q30 в `TRACEABILITY.md`
  (`:37`, Feature — `agent_minimal.feature`) и `questions/README.md` (`:54`).
  Архив: Q30 (`:203`) → указатель, метка «блок 4б-2…, Q27+Q30»; Q31 цел. Свип Q30
  (полный): `D31:150–151`, `Q12.md:14`, `Q15.md:13`, `questions/README.md:36,39`,
  `D54:131–133`, `D56:119–121` — Q30 живой ссылкой, остальные Q31+/Q32/Q33/Q34
  остаются «ожидают переноса». Пометок «ожидает переноса» про Q27/Q30 больше нет
  (вне `docs/reviews/**`). Сверка §5.3 — ✅: `src/main.rs` без флагов →
  `mcp::run_stdio` (`:92`), REST не поднимается (`:85–90`); `src/mcp.rs`
  `run_stdio` (`:18`), `rmcp::transport::stdio()` (`:23`); SPEC §2.2 `:178/:182–187`,
  §2.3 `:267–270`, §4.1 `:306–307`, §10 №29 `:852`. Tasks: — (правок в коде
  прототипа не требует; интеграция Notebook — вне периметра credo2).
  `cargo` не запускался (D50); `src`/`tests`/`features`/`CHANGELOG` не трогал.
  **Итог блока 4б-2:** Q27+Q30 закрыты (D27/D29); свободные §10 — **№59**,
  Q — **Q57**; `docs-writer`: шапка `agent_minimal.feature` (`# Q30…` → `# D29 (Q30)…`),
  `features/README.md:207/213/216`, CHANGELOG. Отчёт — лента
  `.opencode/mail/service-migration-q27q30.md`; коммит — за `git`.
- 2026-09-29 · service-migration-q27q30 (пост-проверка сервисной сессии):
  выведен Q30 из перечней-сказуемых «ожидают переноса» — `D54-source-of-truth-flow.md:132–133`
  (`[Q30] … (транспорт MCP в Notebook); Q33 …, Q32 … — ожидают`) и
  `D56-merge-step.md:120–121` (`[Q30] … , Q32 … — ожидает`). Тексты решений не
  тронуты. Перепроверка `rg "ожида[ею]т переноса" docs` (вне `reviews/**`):
  субъекты — только Q31–Q39/Q31+ (Q1–Q30 + Q42 мигрированы; `Q31*.md` нет).
  Ссылки Q30/D29 живые. `cargo` не запускался (D50). Отчёт — та же лента
  (секция «пост-проверка»); коммит — за `git`.
- 2026-09-29 · service-migration-q27q30 (rework iteration 1): закрыты P2/P3
  приёмки (`docs/reviews/migration-q27q30-2026-09-29.md`). P2 — в
  `D29-notebook-mcp-transport.md` пары «раздел ↔ строки SPEC» приведены к карте
  (§2.2 `:127–142`, §2.3 `:143–190`, §4.1 `:253–280`, §4.2 `:281–340`):
  `Affects` `:7`, «Следствия» `:41`, «Сверка» `:70`, «Краткий канон» `:109` →
  `§2.2 (:134), §2.3 (:178, :182–187), §4.1 (:267–270), §4.2 (:303–317)`
  (вариант «б» приёмки: разделы §2.2/§2.3/§4.1 — намеренные по Q30, строки к
  ним; §4.2 — канон «только MCP»/запуска, ссылка на Q30 `:307`). Лента:
  Q30-отчёт `:125/:131` исправлен. P3 — `:170` «34 указателя» → «31» (проверено
  `rg -c "→ перенесён"` = 31, `rg -c "Мигрирован"` = 31). `cargo` не запускался
  (D50). Дальше — приёмка `-r2`. Отчёт — та же лента; коммит — за `git`.
- 2026-09-29 · service-migration-q31q33 (вызов 1 из 2): перенос **Q31 → D12**
  (`docs/decisions/D12-agent-chat-panel.md`, `Resolves: [Q31]`; слаг
  `D12-agent-chat-panel`). Создан `docs/questions/Q31.md` (2026-09-26,
  ⚪ оформление, UI-решение; «Перенос: 2026-09-29, блок «Notebook-функции»,
  Q31+Q33»). Кросс-ссылки Q31↔Q33 живые в D12/Q31 (`[Q33](../questions/Q33.md)`,
  `[D30](D30-execution-mechanism.md)` — появятся вызовом 2); пометок «ожидает
  переноса» про Q33 в D12 нет. §10 №12 (`:835`): после `(Q31)` добавлено
  `; полный контекст — [D12](decisions/D12-agent-chat-panel.md)` (второй столбец
  и текст строки целы). Сводки: D12 в `decisions/README.md` (`:27`, первым до
  D14; parenthetical `:18` «(Q31–Q41)» → «(Q32–Q41)»); Q31 в `TRACEABILITY.md`
  (`:38`, после Q30) и `questions/README.md` (`:55`). Архив: Q31 (`:213`) →
  указатель, метка «блок «Notebook-функции» (Q31+Q33)»; блок Q32 цел. Свип Q31:
  `D31:151` («Q31+» → «Q32+»), `D58:103`, `D37:83-84/:112-113`, `Q19.md:12`,
  `Q25.md:15`, `questions/README.md:43,:49` — Q31 живой ссылкой, прочие
  Q34/Q35/Q36/Q37 остаются «ожидают переноса»; `features/README.md`/SPEC
  bare-упоминания и `reviews/**` не трогал. Сверка §5.3 — ⚪: `src/` только
  main/mcp/rest/core/lib (UI/чата нет); карта зон `features/README.md:322–329`
  без UI-зоны; `check.test`/`Explanation` — предмет Q33/D30. Tasks: — (UI
  Notebook вне `credo2`). `cargo` не запускался (D50); `features/**`/`CHANGELOG`/
  `src`/`tests` не трогал. **Вызов 2 (тем же sessionID):** Q33 → D30
  (`D30-execution-mechanism.md`), оживить ссылки на D12/Q31, свип Q33 (`D31:151`
  → «Q32, Q34+», `decisions/README.md:18` → «(Q32, Q34–Q41)», `D54:133`,
  `D34:421`, `Q12:14`, `questions/README.md:36,39`). Отчёт — лента
  `.opencode/mail/service-migration-q31q33.md`; коммит — за `git`.
- 2026-09-29 · service-migration-q31q33 (вызов 2 из 2; **блок «Notebook-функции»
  закрыт**): перенос **Q33 → D30** (`docs/decisions/D30-execution-mechanism.md`,
  `Resolves: [Q33]`; слаг `D30-execution-mechanism`; `Tasks: T-08 (открыта),
  T-09 (открыта)`). Создан `docs/questions/Q33.md` (2026-09-26, 🟡 важно,
  «Перенос: 2026-09-29, блок «Notebook-функции», Q31+Q33»). §10 №30 (`:853`) —
  `(Q33; полный контекст — [D30](decisions/D30-execution-mechanism.md))`;
  сводки: D30 в `decisions/README.md:41` (между D29 и D31), `:18` →
  «(Q32, Q34–Q41)»; Q33 в `TRACEABILITY.md:39` (после Q31) и
  `questions/README.md:56`. Архив `OPEN_QUESTIONS.md:284` → указатель (та же
  метка). Свип Q33: `D54:133` → `[Q33]…; Q32 — ожидает переноса`,
  `D34:421` → `[Q33] (draft-first)` (маркер снят целиком — иных ожидающих нет),
  `D31:151` → «Q32, Q34+», `Q12.md:14` → `[Q33]; Q32 (ожидает)`,
  `questions/README.md:36` → `[Q33]`. Сверка §5.3 — 🟡: `check.test` реализован
  (`src/mcp.rs:143/:214–262`, `src/core.rs` `Explanation`/`evaluate_rule`) и
  покрыт (`tests/mcp_draft.rs`, юнит-тесты `src/mcp.rs:817–870`); draft-first
  (`src/mcp.rs:771–781`, `tests/mcp_draft.rs:367`); `check.run` отсутствует
  (`tool_specs` `:470–530` без него; `VersionDeprecated` `:43–46` dead_code) → T-09;
  материализация `.dar` при публикации отсутствует (`publish` `:282–341`) → T-08.
  `cargo` не запускался (D50); `features/**`/`CHANGELOG`/`src`/`tests` не трогал.
  **Итог блока:** Q31+Q33 закрыты (D12/D30); свободные ID — §10 **№59**, Q **Q57**;
  `docs-writer`: шапки фич (`# D12 (Q31)` в `notebook_ui`/`agent_minimal`/
  `inline_execution`; `# D30 (Q33)` в `inline_execution`/`agent_minimal`/
  `draft`/`mcp_tools`), заметки `features/README.md:229/:240`, CHANGELOG, статусы
  задач. Отчёт — лента; коммит — за `git`.
- 2026-09-29 · service-migration-q31q33 (пост-проверка сервисной сессии, до
  приёмки): в 4 местах отделён перенесённый `[Q31]` от группы-перечня
  «ожидают переноса» разделителем `;` — `D58:103`, `Q19.md:12`,
  `questions/README.md:43` (Q19), `:49` (Q25). Контроль
  `rg -n "Q3[13].{0,60}ожида" docs/questions docs/decisions docs/SPECIFICATION.md
  docs/TRACEABILITY.md` — 5 совпадений, во всех `[Q31]`/`[Q33]` стоят отдельно
  (`;`) перед маркером, маркер только у Q32/Q34+/Q36/Q37. `reviews/**`/
  `analysis/**` не трогал; `cargo` не запускался (D50). Отчёт — та же лента
  (секция «пост-проверка»); коммит — за `git`.
- 2026-09-29 · service-migration-q31q33 (P3 приёмки закрыт адресно): в
  `D30-execution-mechanism.md:7` `Affects` дополнен разделами SPEC —
  `§2.2, §4.1, §4.2, §7 и §10` (было только §10), в согласовании с «Кратким
  каноном» `:142-143` и «Следствиями» `:47-49`. Контроль `rg` по D30: `Affects`
  `:7` и `Краткий канон` `:142-143` покрывают один набор разделов; `Спека` `:6` —
  «§10, решение №30» (краткий канон, не менялся). `cargo` не запускался (D50);
  границы — D30 + лента + память. Отчёт — та же лента (секция «P3»); коммит — за
  `git`.
- 2026-09-29 · service-migration-q32 (блок «Git-контур», **Q32 → D28**, готово):
  перенос `docs/decisions/D28-two-git-contours.md` (`Resolves: [Q32]`; слаг
  `D28-two-git-contours`; `Tasks: T-06`). Создан `docs/questions/Q32.md`
  (2026-09-26, 🟡 важно; «Перенос: 2026-09-29, блок «Git-контур», Q32»;
  «Связано» — живые Q13/Q15/Q16/Q17/Q18/Q20/Q22/Q33 + D28 + T-06 + 8 фич).
  §10 №28 (`:851`) — `Q32 (полный контекст — [D28](decisions/D28-two-git-contours.md)):`;
  сводки: D28 в `decisions/README.md:40` (между D27 и D29), parenthetical `:18`
  → «(Q34–Q41)»; Q32 в `TRACEABILITY.md:39` и `questions/README.md:56`.
  Архив `OPEN_QUESTIONS.md:219` → указатель («блок «Git-контур» (Q32)»). Свип
  Q32: `D31:151` → «Q34+ — ожидают переноса»; `D24:84–85/:115–116`,
  `D29:107–108`, `D30:138–139`, `D35:113`, `D54:133`, `D55:88–89`, `D56:121`,
  `D57:126`; Q12/Q13/Q14/Q15/Q17/Q18/Q26/Q30/Q33; `questions/README.md`
  `:37,:39,:41,:42,:50,:54`. **Сверх списка (по контрольному `rg`) поправлены
  `D14:50` и `D14:148`** — Q32 там был под маркером. Сверка §5.3 — 🟡: плоский
  путь `src/lib.rs:342/:383/:462/:317`, `list_from_ref :241–248`; MCP
  `src/mcp.rs` publish/deprecate/list_published; REST-шаблон `src/rest.rs:25–29`
  (канон Q20); тесты пути нет → [T-06]; UI — целевое Notebook. `cargo` не
  запускался (D50); `features/**`/`CHANGELOG`/`src`/`tests`/`tasks`/`reviews`/
  `analysis` не трогал. Отчёт — `.opencode/mail/service-migration-q32.md`;
  коммит — за `git`. Далее: `docs-writer` (шапки 8 фич + `features/README.md` +
  CHANGELOG) → `validator`.
- 2026-09-29 · service-migration-q34 (блок «Тесты», **особый случай —
  расширение D32**, готово): перенос Q34 → **D32** (`Resolves: Q16, Q34`,
  нового D-файла нет, линия §10 №32 общая). Создан `docs/questions/Q34.md`
  (2026-09-26, 🟡 важно; «Перенос: 2026-09-29, блок «Тесты», в рамках D32»;
  все 5 пунктов решения + следствие для кода; «Связано» — Q16/Q19/Q33 + D32/D58
  + T-02 + SPEC §1.2/§4.3/§10 + 4 фичи). D32: `Resolves` → `[Q16], [Q34]`;
  `Affects` — §1.2 (тесты вне `.dar`), `draft`/`inline_execution`/`notebook_ui`/
  `features/README.md` (полнота к «Следствиям», урок q31q33 P3); `Tasks` →
  `T-02 (открыта)`; в «Следствиях» новый пункт — тесты **только** в кэше,
  «быстрые прогоны» — черновики тестов, `.dar-notebook/` в `.gitignore`
  (закрывает п.1/2/5 Q34); «Ссылки» → живой `[Q34]`; «Вопрос:» → «Вопросы:».
  §10 №32 (`:855`) → `Q34, Q16;`; `decisions/README.md:18` → «(Q35–Q41)»,
  строка D32 `:44`; Q34 в `TRACEABILITY.md:41` (после Q33) и
  `questions/README.md:58` + свип `:40` (Q16)/`:43` (Q19). Архив
  `OPEN_QUESTIONS.md:231` → указатель (метка «блок «Тесты», Q16+Q34»). Свип Q34:
  `D14:148`, `D31:151` (`Q34+`→`Q35+`), `D58:103`, `Q16.md:13`, `Q13.md:14`,
  `Q19.md:12` — ссылки живые. Контроль `rg "Q34[^\n]{0,60}ожида"` / `rg
  "ожида[ею]т переноса"` — Q34 не под маркером, маркеры только у Q35–Q41
  (совпадения в `docs/reviews/**` — исторические). Сверка §5.3 — 🟡 (метка ✅,
  гейт ⬜ — T-02); `cargo` не запускался (D50); `features/**`/`CHANGELOG`/`src`/
  `tests` не трогал. Отчёт — `.opencode/mail/service-migration-q34.md`; коммит —
  за `git`. Далее: `docs-writer` (шапки `# D32 (Q34)` в `notebook_ui`/
  `inline_execution`/`test_draft`/`publish` + ноты `features/README.md:130/:236`)
  → `validator`.
- 2026-09-29 · service-migration-q35: перенос **Q35 → D59**
  (`docs/decisions/D59-workspace-templates.md`, `Resolves: [Q35]`; слаг
  `D59-workspace-templates`). Созданы `docs/questions/Q35.md` (resolved by D59;
  2026-09-26, ⚪ оформление, «Перенос: 2026-09-29, блок «Создание workspace и
  шаблоны»») и D59 (`Tasks: —`). §10 — **новая строка №59**
  (`SPECIFICATION.md:882`, после №58 `:881`); `decisions/README.md` — строка D59
  (`:70`; **44 строки**) + `:18` «(Q35–Q41)» → «(Q36–Q41)»; Q35 в
  `TRACEABILITY.md:42` (после Q34, перед Q42; Feature — `notebook_ui`/
  `file_management`) и `questions/README.md:59`; архив `:238–278` → указатель
  (метка «блок «Создание workspace и шаблоны»», Q36 цел). Свип: `D31:151`
  (`Q35+`→`Q36+`), `D58:103`, `Q19.md:12`, `questions/README.md:43` — живые
  ссылки. Контроль `rg "ожида[ею]т переноса"` — субъекты только Q36–Q39/Q37
  (вне `docs/reviews/**`). Сверка §5.3 — ⚪ не применимо (Notebook — Фаза 2–3;
  workspace в `src/` только как путь `AppState`; логики создания
  `rules/`/`Пример.dar`/шаблона нет); «задач не требуется» (T-10 — не про это).
  `cargo` не запускался (D50); `src`/`tests`/`features`/`CHANGELOG` не трогал.
  Лимит шагов в первом вызове — отчёт дописан вторым. Дальше: `docs-writer`
  (шапки `# D59 (Q35)`, ноты `features/README.md:203/:208`, CHANGELOG) →
  `validator` → `git`.
- 2026-09-29 · service-migration-q36q38: перенос **Q36+Q38 → D18**
  (`docs/decisions/D18-pipelines-out-lsp-mvp.md`, `Resolves: [Q36], [Q38]`;
  слаг `D18-pipelines-out-lsp-mvp`; **D18 закрывает пропуск D17→D19**). Созданы
  `docs/questions/Q36.md`, `Q38.md` (2026-09-24, 🟡 важно, «Перенос: 2026-09-29,
  блок «Конвейеры + LSP-состав», Q36+Q38»). §10 №18 (`:841`) — D-ссылка (diff
  ровно 1 строка); `decisions/README.md` — строка D18 (`:32`, между D17/D19;
  **45 табличных строк**), `:18` «(Q36–Q41)» → «(Q37, Q39–Q41)»; Q36/Q38 в
  `TRACEABILITY.md:43–44` и `questions/README.md:60–61`. Архив:
  Q36 (`:244`) и Q38 (`:296`) → указатели (метка «блок «Конвейеры + LSP-состав»,
  Q36+Q38»); Q37 цел. Свип: `D31:151` «Q36+»→«Q37+»; `D37:84/:113`, `D36:100`,
  `Q20:13`, `Q21:13`, `Q24:14`, `Q25:15`, `questions/README:44,45,48,49` —
  Q36 живой ссылкой; `Q10:11/:62` «Q36–Q39» → «Q37, Q39»; `D17:107`/`D22:148` —
  Q36/Q38 живыми ссылками (по принципу, маркеров не было). Контроль
  `rg "ожида[ею]т переноса"` — субъекты только Q37, Q39–Q41 (вне
  `docs/reviews/**`); про Q36/Q38 маркеров нет. Сверка §5.3 — ⚪ не применимо
  (LSP — `lsp-dar`, Фаза 2, вне `credo2`; `rg` по `src/**` lsp/graph/pipeline —
  пусто; конвейеров нет, язык v0.1); «задач не требуется» (T-14 — не об этом).
  `cargo` не запускался (D50); `src`/`tests`/`features`/`CHANGELOG` не трогал.
  **Лимит шагов в первом вызове — отчёт/чекпойнт дописаны вторым (тем же
  sessionID).** Дальше: `docs-writer` (шапки `lsp.feature`/`graph_view.feature`,
  нота `features/README.md:83`, CHANGELOG) → `validator` → `git`.
- 2026-09-29 · service-migration-q36q38 (P3 приёмки закрыт адресно): в
  `docs/questions/Q25.md:15` восстановлена пометка «ожидает переноса» у Q37
  (при снятии Q36 из группового маркера `(ожидают переноса)` адрес Q37 потерялся).
  Стало: `Q37 (БД/справочники) — ожидает переноса; [Q36](Q36.md) (конвейеры);`.
  Контроль `rg -n "Q37|ожида[ею]т переноса" docs/questions/Q25.md` — маркер ровно
  один (`:15`), вхождения `:51/:62` — текст тела. Прочие файлы не трогал;
  `cargo` не запускался (D50). Отчёт — та же лента (секция `P3 закрыт`).
- 2026-09-29 · service-migration-q37 (блок «Реестр полей»): перенос **Q37 → D33**
  (`docs/decisions/D33-fields-registry-source.md`, `Resolves: [Q37]`; слаг
  `D33-fields-registry-source`; закрыт пропуск нумерации D32 → D34). Создан
  `docs/questions/Q37.md` (2026-09-26, 🟡 важно, «Перенос: 2026-09-29, блок
  «Реестр полей»»; 6 пунктов решения + «Следствие для кода»; «Связано» —
  Q9/Q10/Q34/Q36/Q38 + D18/D21/D32 + SPEC §3.3/§4.3/§10 + 2 фичи). §10 №33
  (`:856`) — `(Q37; полный контекст — [D33]…)` (diff ровно 1 строка). Сводки:
  D33 в `decisions/README.md` (между D32 `:45` и D34 `:47`; **46 табличных
  строк**), `:18` «(Q37, Q39–Q41)» → «(Q39–Q41)»; Q37 в `TRACEABILITY.md:44` и
  `questions/README.md:61`. Архив `OPEN_QUESTIONS.md:250` → указатель (метка
  «блок «Реестр полей»»); указатели Q36/Q38 и блок Q39 целы. Свип: `D31:151`
  («Q37+»→«Q39+»), `Q25:15` (маркер → живая `[Q37]`), `Q10:11–12/:62`
  (→ `[Q37]`, маркер только у Q39), `D37:112–113` (→ живая `[Q37]` + «закрыт
  D33»), `D18:135` (→ `[Q37]` (закрыт D33), маркер у Q39), `Q38:12–13`
  (→ живая `[Q37]`, «ожидает переноса» — Q39). Контроль
  `rg "ожида[ею]т переноса" docs --glob "!docs/reviews/**"` — субъекты только
  Q39 (Q40/Q41 маркеров не имели); про Q37 маркеров не осталось. Сверка §5.3 —
  ⚪ не применимо (БД/completion — `lsp-dar`, Фаза 2): `rg` по `src/**`
  `tables|completion|автодополн|подсказ` — 0; `src/core.rs` `HashMap` (`:5`,
  `:155`), решения из текста правила (`:218`), тест
  `contract_decisions_follow_rule_vocabulary_q10` (`:598–602`) — хардкод-словаря
  нет; SPEC §3.3 `:234`, §4.3 `:399–403`, §10 `:856` ✅; `GRAMMAR.md` таблиц не
  содержит (правок не требуется); фичи `lsp.feature`/`notebook_ui.feature` ✅.
  Tasks: — (`rg "Q37" docs/tasks` — 0; «переход на БД» — пост-MVP). `cargo`/git
  не запускались (D50); `src`/`tests`/`features`/`CHANGELOG` не трогал.
  Примечание: `.opencode/memory/service.md` был modified до моей работы — не
  трогал. Отчёт — `.opencode/mail/service-migration-q37.md`; коммит — за `git`.
- 2026-09-29 · service-migration-q39: перенос **Q39 → D6**
  (`docs/decisions/D6-lsp-degradation.md`, слаг `D6-lsp-degradation`,
  `Resolves: [Q39]`). Создан `docs/questions/Q39.md` (2026-09-26, 🟡 важно,
  «Перенос: 2026-09-29, блок «LSP-поведение»»). §10 №6 (`:829`) — хвост `(Q39)`
  → `(Q39; полный контекст — [D6]…)`, diff 1 строка (нового номера §10 нет).
  Сводка `decisions/README.md` — D6 **первой строкой** перед D12, `:18`
  «(Q39–Q41)» → «(Q40–Q41)», **47 строк**; `TRACEABILITY:46` и
  `questions/README:63` — Q39 между Q38 и Q42. Архив Q39 (`:262–304`) →
  указатель (метка «блок «LSP-поведение»»), Q40 цел. Сверка §5.3 — ⚪
  (LSP/Notebook — Фаза 2–3, вне `credo2`; `rg src/**` — нет); Tasks: —. Особый
  свип: ложная привязка OAuth→Q39 снята (`D26:25/:107/:115`, `Q22.md:11–12`, и
  `questions/README:46` — узел убран, осталось `[Q23]`; сознательное отклонение
  от буквального п.6 в пользу «Особого пункта»; тексты решений не переписаны).
  Свипы: `D31:151` «Q39+»→«Q40+»; `Q10:12/:62`/`Q38:13`/`D18:135` — живые
  `[Q39]`. Контрольный `rg "ожида[ею]т переноса"` — только «Q40+». `cargo`/git
  не запускались (D50); `src`/`tests`/`features`/`CHANGELOG` не трогал. Обрыв по
  лимиту шагов после правок; отчёт+чекпойнт записаны вторым вызовом. Отчёт —
  `.opencode/mail/service-migration-q39.md`; коммит/push — за `git` (пакет
  подтверждён владельцем). Далее `docs-writer` (шапка `lsp_notebook`, нота
  `features/README.md:256`, CHANGELOG) → `validator`.
- 2026-09-29 · service-migration-q39 (P3 закрыт адресно): `validator` дал один P3 —
  в `docs/decisions/D6-lsp-degradation.md:91–93` ссылки на строки сценариев
  `docs/features/lsp_notebook.feature` устарели на +1 после вставки `# D6 (Q39)`
  в `:6` (`docs-writer`). Правка только чисел: «Перезапуск LSP при падении»
  `:38–45`→`:39–46`, «Деградация без сервера языка» `:47–53`→`:48–54`,
  «Повторные падения прекращают автоперезапуск» `:55–60`→`:56–61`; пометка
  `Q39` `:2–5` цела. Проверено чтением фичи (границы `:39–46/:48–54/:56–61`) и
  `rg -n "lsp_notebook" D6` — иных ссылок с номерами нет. Формулировки не
  переписывались. `cargo`/git не запускались (D50). P3 закрыт без новой
  итерации (прецедент q31q33/q36q38); других P нет — пакет готов к `git`.
  Отчёт — та же лента.
- 2026-09-29 · service-migration-q40q41 (**блок «Процесс» закрыт**, последние
  записи архива): перенос **Q40 → D20**
  (`docs/decisions/D20-features-docs-dod.md`, `Resolves: [Q40]`; слаг
  `D20-features-docs-dod`) и **Q41 → D60**
  (`docs/decisions/D60-docs-ownership-sync.md`, `Resolves: [Q41]`; слаг
  `D60-docs-ownership-sync`). Созданы `docs/questions/Q40.md`, `Q41.md`
  (resolved; приоритет ⚪ оформление; «Перенос: 2026-09-29, блок «Процесс»,
  Q40+Q41»). §10 №20 (`:843`) — `(Q40; полный контекст — [D20]…)` (diff ровно
  1 строка); шапка §10 (`:811`) — `[D60]`; новая строка **№60** (`:883`).
  Архив: Q40 (`:272`) и Q41 (`:278`) → указатели (стиль Q39); порядок
  Q39→Q40→Q41 цел; неперенесённых полных блоков не осталось. Сводка
  `decisions/README.md` — D20 `:36` (между D19/D21), D60 `:76` (после D59),
  **49 строк**; parenthetical `:18–20` переформулирован. `TRACEABILITY:47–48`,
  `questions/README:64–65`, `Q5.md:11`, `D19:58/:76–77/:118–119`,
  `D31:151–153` — живые ссылки Q40/Q41; тексты решений не переписаны.
  Сверка обоих D — ⚪ (процесс/документы): D20 — `tests/features_inventory.rs`
  (4 теста, doc-комментарий Q40) + `ci.yml` (`:10–15`, не менялся); D60 —
  SPEC §6.1/шапка §10, `AGENTS.md`, `docs/README.md`; ADR-003 (корневой
  `DECISIONS.md`) внешний, чтение отклонено (`external_directory`).
  Tasks: —. Свип `ожида\s*[ею]т\s+переноса` — вне `reviews/**` чисто.
  `cargo`/git не запускались (D50). Свободные ID: §10 — **№61**, Q — **Q57**.
  Далее `docs-writer` (шапки/ноты фич, CHANGELOG, финиш серии — отдельным
  блоком) → `validator` → `git`. Отчёт — `.opencode/mail/service-migration-q40q41.md`;
  коммит — за `git`.
- 2026-09-29 · service-migration-q40q41 (**P3 закрыт адресно**, прецедент Q39):
  `validator` дал один P3 (адресные ссылки после вставок `docs-writer`).
  (1) `D20-features-docs-dod.md:81–82` — три адреса `features/README.md`:
  `:12–20`→`:12–21`, `:331`→`:333`, `:284`→`:285–286` (проверено чтением
  `features/README.md`). (2) `D60-docs-ownership-sync.md:107` — улика
  `OPEN_QUESTIONS.md:341` → живая ссылка `[Q41](../questions/Q41.md) (`:51–55`)`
  (указатель Q41 в архиве — `:278–282`). `Q41.md:51–55` правки не требовал.
  Формулировки/структура не тронуты; `rg ":12–20|:331|:284|OPEN_QUESTIONS.md:341"`
  — старых адресов вне `reviews/**` нет. `cargo`/git не запускались (D50).
  Отчёт — та же лента (шапка «P3 закрыт»); далее подтверждение P3 у `validator`
  → `git`.
- 2026-09-29 · service-docs-hygiene (шаг 1 — фиксация Q57–Q61, без решений):
  созданы `docs/questions/Q57.md`…`Q61.md` (все `open`, ⚪ оформление, дата
  2026-09-29; без поля «Перенос» — новые записи). Строки добавлены в
  `docs/TRACEABILITY.md` (после Q56 `:63`: `rg -c '^\| \[Q[0-9]+'` = **61**)
  и `docs/questions/README.md` (+5; = **61**); «Связано» перекрёстно
  (Q57↔Q58/Q59, Q58↔Q57/Q59/Q60, Q59↔Q60/Q58, Q60↔Q59/Q57, Q61↔Q59/Q58).
  D/§10/архив/`decisions/**`/`features/**`/`BRIEF.md`/`README.md`/`AGENTS.md`
  не трогались. **Затык (права):** `edit docs/CHANGELOG.md` → `Permission denied`
  (rights `migrator` покрывают журнал, но не CHANGELOG) — запись «заведены
  Q57–Q61» НЕ внесена; нужно либо расширение прав служебной сессией, либо
  `docs-writer`. `cargo`/git не запускались (D50). Отчёт — лента
  `.opencode/mail/service-docs-hygiene.md`; коммит — за `git`.
  Свободные ID: §10 — №61, Q — Q62.
- 2026-09-29 · service-docs-hygiene (шаг 2а — дополнение записей по разведке
  `auditor`, до решений): **Q57** — расширены адреса (служебная зона
  `migrator`/`docs-writer`: право/шаг/строки архива; карточка `T-16`; правки
  `.opencode/agents/**` — по протоколу служебной зоны, входят в объём решения);
  **Q61** — правило «время жизни адреса» (вариант (а) + «Рекомендация»: канон
  ссылается только на неудаляемые артефакты; `mail/**`/`state/**`/`analysis/**`
  — нельзя; Q60 учитывает; существующие адреса `mail/**` — объект исполнения);
  реестр — **F47/F48** («кандидат», связи Q57/Q61 и Q57/Q58). Номера строк в
  Q57 не выносил (`BRIEF.md` §11); literal `.opencode/mail/**` — только code-span
  в F47. Проверки: `rg -c "\| F[0-9]+ \|"` = 48; `Q12.md`/`T-16` существуют;
  `cargo`/git не запускались (D50). Отчёт — лента
  `.opencode/mail/service-docs-hygiene.md`; коммит — за `git`.
- 2026-09-29 · service-docs-hygiene (шаг 3, вызов 1 из 2 — решения **D61–D63**):
  оформлены принятые решения (владелец, 29.09.2026). Созданы
  `docs/decisions/D61-archive-removal.md` (`Resolves: [Q57]`),
  `D62-brief-journal-rules.md` (`Resolves: [Q58]`),
  `D63-journal-index-lifecycle.md` (`Resolves: [Q59]`); Статус `accepted`, дата
  2026-09-29, Спека `SPECIFICATION.md` §10 №61/62/63, `Tasks: —`; «Сверка с
  кодом» у всех ⚪ (процесс/документы) с фактами чтения. Правки: §10 — строки
  **№61–63** после №60 (`:884–886`; №60 `:883` цел); Q57–Q59 — `:3`
  `resolved by [D6x]`; `TRACEABILITY.md:64–66` — D-ссылки + `resolved`,
  Feature/Задачи `—`; `questions/README.md:81–83` — D-ссылки + `resolved`;
  `decisions/README.md` — 3 строки (`:77–79`), **49 → 52** (цель 54 — вызов 2).
  Кросс-ссылки D61↔D62/D63 живые; на D64/D65 — **без ссылок** (только
  `[Q60]`/`[Q61]`), чтобы не было битых адресов до вызова 2. Архив/D64/D65/Q60/
  Q61/CHANGELOG/`features/**`/`src`/`tests` не трогал. `cargo` не запускался
  (D50); git только `status --short`. **Затык:** `edit` памяти по `oldString` не
  совпал (пробелы) — чекпойнт вызова 1 дописан при resume по уникальному анкору.
  **Вызов 2 (тем же sessionID):** D64 (Q60 — тест целостности журнала) и D65
  (Q61 — политика ссылок/дублей, «время жизни адреса»), §10 №64–65, статусы
  Q60/Q61, `TRACEABILITY:67–68`, `questions/README:84–85`, `decisions/README`
  → 54 строки, задача на тест целостности, F47/F48. Отчёт — лента; коммит — за
  `git`.
- 2026-09-29 · service-docs-hygiene (шаг 3, **вызов 2 из 2 — D64–D65, готово**):
  служебные записи вызова 1 дописаны (память — по уникальному анкору; лента —
  секция `## migrator · D61–D63`). Созданы
  `docs/decisions/D64-journal-integrity-test.md` (`Resolves: [Q60]`; слаг
  `D64-journal-integrity-test`; вердикт **⬜** — `tests/docs_journal.rs` нет) и
  `D65-reference-policy.md` (`Resolves: [Q61]`; **⚪** процесс/документы). §10 —
  строки **№64–65** (`:887–888`; №63 `:886` цел). Q60/Q61 (`:3`) —
  `resolved by [D64]`/`[D65]`. `TRACEABILITY.md:67–68` — D-ссылки + `resolved`
  (у Q60 Задачи `[T-18]`). `questions/README.md:84–85` — D-ссылки + `resolved`.
  `decisions/README.md:80–81` — D64/D65, **54 строки** (49 → 52 → 54, цель
  достигнута). **Задача:** `docs/tasks/T-18-docs-journal-test/README.md` (⬜, P3,
  Исполнитель `tester`, Источник `D64 (Q60)`, зависит «после D61–D63/D65») +
  строка `tasks/README.md:54`. Правка D65: снят мёртвый адрес
  `tests/docs_journal.rs` (файла нет) → code-span + ссылка `T-18`. `cargo` не
  запускался (D50); git только `status`. Архив/`BRIEF.md`/`.opencode/agents/**`/
  `features/**`/`src`/`tests`/CHANGELOG не трогал. **Замечание (lead):** правило
  «время жизни адреса» (D65 п.4) буквально запрещает ссылки канона на
  `docs/analysis/**`, но журнал (Q59/Q60/Q61, D48/D61/D63) и `TRACEABILITY`
  (Q53) адресуют `analysis/findings-registry.md`; нужно решение — исключение для
  реестра находок (артефакт владения по D48) либо чистка при исполнении; тест
  D64 (п.8) должен это учесть. **Остаток:** исполнение D61–D65 (`docs-writer`/
  `migrator`/сервисная сессия), `validator`, `git` (пакет). Отчёт — лента.
- 2026-09-29 · service-docs-hygiene (шаг 3, **уточнение D64/D65 — исключение
  реестра**): по решению сервисной сессии (по существу D48) в
  `D65-reference-policy.md` п.4 добавлена оговорка: `docs/analysis/findings-registry.md`
  — живой реестр находок (владелец `migrator`, D48), штатно не удаляется, ссылки
  на него из канона/журнала допустимы; прочие `docs/analysis/**` — рабочие данные.
  В `D64-journal-integrity-test.md` п.8 реестр помечен как whitelisted-исключение.
  Правки точечные, смысл пунктов сохранён; проверено чтением и glob (ссылки
  живые), прочих правок нет. Архив/исполнение D61–D63 не трогались; `cargo`/git
  не запускались (D50). Отчёт — лента (секция `D64/D65 — исключение реестра`).
- 2026-09-29 · service-docs-hygiene (шаг 3, **rework P2/P3 после приёмки -r1**):
  **P2** — исчерпывающий свип живых адресов `mail/**` в каноне выявил больше, чем
  перечень валидатора: Q54–Q61 (ленты `service-t11-closeout`/`service-dod-scope`/
  `service-agent-tools`/`service-docs-hygiene`), `questions/README:80` (Q56),
  **D49–D51** (ранее не названы), строка D43 в `decisions/README:59` (упоминание
  права, не адрес), `CHANGELOG` (запись Q57–Q61). `D65-reference-policy.md` п.4 —
  неполный перечень заменён фактическим (по файлам, без номеров строк), формула
  «снимаются исполнением» сохранена; `Q61.md` синхронизирован (Q54–Q61, D49–D51,
  CHANGELOG). `D64` п.8 — перечней/чисел не было, правка не требовалась. **P3** —
  `docs/tasks/T-18-docs-journal-test/README.md`: добавлено whitelist-исключение
  `docs/analysis/findings-registry.md` (живой реестр, владелец `migrator`, D48),
  согласовано с D64/D65. Проверки: свип-таблица в ленте; Q61/D64/T-18 связны;
  других правок нет; `cargo`/git не запускались (D50). Дальше — приёмка `-r2`.
  Отчёт — лента (секция `rework P2/P3 (D65, T-18)`).
- 2026-09-29 · service-docs-hygiene (исполнение **D61** — снятие markdown-ссылок
  на архив): правки исходно по списку (SPEC §10 шапка, D17, D60, D61, Q57,
  `tasks/README`, `T-16`) + **сверх списка** `docs/TRACEABILITY.md:75–76`
  (легаси-футер). Везде ссылка `[…](…OPEN_QUESTIONS.md)` → code-span/текст, имя
  сохранено; формулировки решений не переписаны. Свип
  `rg -n "\]\([^)]*OPEN_QUESTIONS" docs -g "!docs/reviews/**" -g "!docs/analysis/**"`
  → пусто, кроме `docs/features/README.md:26` (зона `docs-writer` — остаток).
  `rg OPEN_QUESTIONS` — только упоминания-имена/контексты без ссылок. `cargo`/git
  не запускались (D50). Остаток: `features/README.md:26`, удаление файла архива,
  служебная зона `.opencode/agents/**`. Отчёт — лента (секция `D61 — снятие
  ссылок`).
- 2026-09-29 · service-docs-hygiene (исполнение **D65 п.4** — mail-ссылки):
  сняты markdown-ссылки канона на `.opencode/mail/**` (ссылка → code-span имени
  ленты + помета «(`.opencode/mail/**`)»). Правки: `Q54` (2: `:12`,`:18`), `Q55`,
  `Q56`, `Q57`–`Q61` (по 1, `service-docs-hygiene`), `questions/README:80`
  (строка Q56), `D49:69`, `D50` (3: `:31`,`:87`,`:115`), `D51` (2: `:32`,`:107`).
  Проверка `rg -n "\]\([^)]*\.opencode/mail" docs -g "!docs/reviews/**"
  -g "!docs/analysis/**"` → в `docs/questions/**`/`docs/decisions/**` **пусто**;
  остаются `CHANGELOG:353,365` (зона `docs-writer`), `T-15/**` (историч.),
  `reviews/**`/`analysis/**` (историч.). Формулировки решений не переписаны;
  `cargo`/git не запускались (D50). **Остаток:** CHANGELOG (docs-writer),
  `T-15/**`. Отчёт — лента (секция `D65 — mail-ссылки`).
- 2026-09-29 · service-docs-hygiene (исполнение **D63 п.2–3** — каталоги):
  перезаписаны `docs/questions/README.md` (`| Q | Тема | D |`; без «Статус»/
  «Связано»; D — только ссылка; шапка без архива; **61** строка Q1→Q61) и
  `docs/decisions/README.md` (`| D | Краткая тема | Решает | Дата | Статус |`;
  без «Задачи»; темы сокращены до первой смысловой части; шапка без архива;
  **54** строки D6→D65). Порядок/ссылки сохранены, формулировки решений не
  переписаны. Проверки: `rg -c "^\| \[Q"` = 61, `rg -c "^\| \[D"` = 54; нет
  колонок «Статус»/«Связано»/«Задачи»; `OPEN_QUESTIONS.md` — только
  упоминания-имена; `cargo`/git не запускались (D50). Отчёт — лента (секция
  `D63 — каталоги Q/D`).
- 2026-09-29 · service-docs-hygiene (хвосты **BRIEF-ссылки + F47/F48**): живые
  указатели `BRIEF.md §11` → `§8` в `D65` (3), `Q61` (2), `Q57` (2: «Связано» —
  критерий-§7 убран, `§11`→`§8`; `:60`), `Q58` («Связано» `:7` → `§1,§4,§5,§6,§8`
  + историч. помета). Исторические описания «что было» (D62, Q58 контекст, D64,
  Q60, D61, Q57 `:19/:66`, SPEC §10 №62, CHANGELOG, T-18) **не трогал** — это не
  живые указатели. Проверка `rg "BRIEF.md §11"` вне истории → пусто. Реестр:
  `F47`/`F48` из «кандидат» → «закрыт 2026-09-29» (F47 — D65; F48 — D61/D62;
  связи Q57/Q61, формат как F45/F46). `cargo`/git не запускались (D50). Отчёт —
  лента (секция `BRIEF-ссылки и F47/F48`).
- 2026-09-29 · service-docs-hygiene (P3 аудита — **§9→§7**): `Q60` (`:15` §9→§7;
  критерий-§7 → «историч.; выполнен D61»; `:7` «Связано» → `§7 (целостность;
  исторически §9), §8`), `D64` (`:19` §9→§7; `:21`/`:107` критерий → «§7, историч.;
  выполнен D61»), `T-18` (`:17` §9→§7; `:18` критерий → пометка). Проверка
  `rg "BRIEF.md §9"` по трём файлам → пусто (единств. `§9` — историч. `reviews/**`).
  Смысл не менялся; `cargo`/git не запускались (D50). Отчёт — лента (секция
  `P3 — §9→§7`).
- 2026-09-29 · service-docs-hygiene (rework **P2/P3 — analysis-ссылки**): уточнено
  «время жизни адреса» — `clean-logs.mjs` чистит только `memory`/`mail`, поэтому
  `docs/analysis/**` штатно не удаляется и **допустим** как провенанс; зона
  запрета — `mail`/`state`. Правки: `D65` п.4 (переформулировка + перечень
  mail как есть) и «Сверка с кодом» (`:104` — фактический список Q54–Q61/D49–D51
  вместо «пяти Q57–Q61»; снят счётчик «8 адресов» в «Следствиях»); `D64` п.8
  (`analysis` — не проверяется); `Q61` «Рекомендация»; `T-18` (запреты+whitelist).
  P3-b: `Q57:19/:66`, `D61:27/:67` — критерий `§7` → «историч.; выполнен D61».
  Проверки: D64/D65 согласованы (зона = mail/state, analysis — допустим); других
  правок нет; `cargo`/git не запускались (D50). Дальше — приёмка `-r2`.
- 2026-09-29 · service-docs-hygiene (rework **P2-r2 — SPEC §10 №64/№65**):
  синхронизировал канон §10 с уточнённым правилом. №64 (`:887`) — из запретов теста
  убран `docs/analysis/**`, оставлены `mail`/`state` + whitelist
  `findings-registry`; №65 (`:888`) — «analysis не допускается» → «допустим как
  провенанс; запрет — mail/state; whitelist — реестр (D48)». Согласовано с D64 п.8
  (`:50–55`) и D65 п.4 (`:46–55`). Прочие строки §10 и файлы не трогал (`BRIEF.md`
  §8 `:292-295` — остаток за `docs-writer`). `cargo`/git не запускались (D50).
  Отчёт — лента (секция `P2-r2 — SPEC §10 №64/№65`). Дальше — приёмка `-r3`.
- 2026-09-29 · service-doc-tools (шаг 1 — **Q62–Q64 заведены**, open): созданы
  `docs/questions/Q62.md` (doc-quality проверки: состав/место/режим;
  рекомендация (а) `size`+`markdownlint`, link-check — T-18),
  `Q63.md` (`cspell` en+ru — отложить),
  `Q64.md` (`CHANGELOG` — рукописный, генератор отклонить). Все `open`,
  ⚪ оформление, дата 2026-09-29; опора — обзор
  `docs/research/doc-quality-checks-2026-09-29.md`. Строки: `TRACEABILITY.md`
  (3 после Q61, D/Задачи/Реализация `—`, lifecycle `open`) и
  `questions/README.md` (3 после Q61, D `—`). D/§10 (`decisions/**`) **не
  трогал** — решения D66–D68 и строки №66–68 только после согласования
  владельцем; `CHANGELOG.md` не трогал (docs-writer, финальный пакет).
  Проверки: `rg -c "^\| \[Q"` → **64/64**; все 15 относительных ссылок в
  Q62–Q64 живы; `cargo`/git не запускались (D50). Сверка с кодом не
  применялась (⚪ оформление, вопроса без решения). Отчёт — лента
  `.opencode/mail/service-doc-tools.md`; коммит — за `git`.
  **Следующее:** шаг 2 — D66–D68 к согласованию владельцем (свободные §10 —
  №66; Q — Q65).
- 2026-09-29 · service-doc-tools (шаг 2 — **D66–D68 + T-19**, шаг готов к
  приёмке): перенос **Q62 → D66** (`docs/decisions/D66-doc-quality-checks.md`,
  «Сверка» ⬜ не реализовано), **Q63 → D67** (`D67-cspell-deferred.md`, ⚪),
  **Q64 → D68** (`D68-changelog-handwritten.md`, ⚪); все `accepted`. Создан
  `docs/tasks/T-19-doc-quality-checks/README.md` (Источник D66 (Q62), P3, ⬜;
  link-check — в T-18; пилот до включения) + строка в `docs/tasks/README.md`.
  Правки: `SPECIFICATION.md` §10 **№66–68** (`:889–891`); `decisions/README.md`
  +3 (**57** = 57 D-файлов); `questions/README.md` Q62–Q64 (D-ссылки);
  `TRACEABILITY.md` Q62–Q64 — Q62 `in work` (**T-19** ⬜), Q63/Q64 `resolved`;
  Q62–Q64 статус `resolved by D6x`. **D65-свип («время жизни адреса»):** в
  Q62–Q64 снят провенанс-адрес ленты (`(\`.opencode/mail/**\`)` → имя
  `service-doc-tools`); в `D66`/`T-19` `.opencode/mail/**` — только
  зона-исключение охвата (оставлено осознанно; ср. D65/§10 №64/T-18); сняты
  2 битых линка `D66`/`T-19` на ещё не существующий `tests/docs_journal.rs` →
  code-span (как в D64). Проверки: ссылки живы; «ожида(ет|ют) переноса» — без
  субъектов (только описания в T-18/Q60 и `reviews/**`); 64 Q-строки в
  TRACEABILITY. `cargo`/git не запускались (D50); `CHANGELOG` не трогал
  (docs-writer). **Свободные:** §10 — **№69**, Q — **Q65**. Отчёт — лента
  `.opencode/mail/service-doc-tools.md` (секция «хвосты»); коммит — за `git`.
- 2026-09-29 · service-doc-rework (шаг 2 — **Q65–Q70 заведены**, open): созданы
  `docs/questions/Q65.md`–`Q70.md` (все `open`, ⚪ оформление, 2026-09-29) —
  ретро-D §10 №1–5/7–11/13 (11 файлов, без новых §10-строк), сокращение SPEC +
  §10-индекс, BRIEF → `.opencode/rules/journal.md`, очистка CHANGELOG,
  README×2, нормативный фокус GRAMMAR. Рекомендации — решения владельца
  29.09.2026; открытые развилки: Q68 (потеря записи о ломающих при полной
  очистке), Q66 (провенанс §1.3.1: Q20 ↔ D22). Строки: `TRACEABILITY.md` +6
  после Q64 (D/Задачи/Реализация `—`, `open`), `questions/README.md` +6
  (D `—`) → **70/70** (`rg -c "^\| \[Q"`). Ссылки Q65–Q70 живы; на `mail/**`
  не ссылался (D65) — разведка пересказана текстом; `tests/docs_journal.rs` и
  `.opencode/rules/journal.md` — code-span (не существуют). D/§10/
  `decisions/**`/`CHANGELOG.md`/`BRIEF.md` не трогал (D69–D74 — после
  согласования). `cargo` не запускался (D50); git — только read-only
  `status --porcelain`. **Дальше:** шаг 3 — D69–D74 к согласованию владельцем
  (свободные §10 — **№69**, Q — **Q71**). Отчёт — лента
  `.opencode/mail/service-doc-rework.md`; коммит — за `git`.
- 2026-09-29 · service-doc-rework (**шаг 3 — D69–D74 оформлены**): создано
  **17 D-файлов** в `docs/decisions/` — 11 ретро-D (`D1-comment-syntax`,
  `D2-file-first-source-of-truth`, `D3-lsp-sidecar-process`,
  `D4-lsp-transport-stdio`, `D5-lsp-client-codemirror`, `D7-tauri-ipc-notebook`,
  `D8-mcp-for-agent`, `D9-nextjs-removed`, `D10-graph-deferred`,
  `D11-wasm-native-first`, `D13-f64-mvp-decimal-v02`; все `accepted`, `Resolves: —`,
  `Спека: SPEC §10, №N (историч.; упраздняется D70)`) и 6 решений —
  `D69-retro-decisions` (Q65), `D70-spec-reduction` (Q66; **таблица §10
  упраздняется**), `D71-journal-rules-relocation` (Q67), `D72-changelog-full-cleanup`
  (Q68), `D73-readme-entrypoints` (Q69), `D74-grammar-normative-focus` (Q70);
  `Спека: —`. **Строки §10 для D69–D74 не заводились** (правка владельца);
  `CHANGELOG`/`BRIEF`/`SPEC`/`GRAMMAR`/`README`/`.opencode/**` не трогал
  (исполнение). Правки: Q65–Q70 `open` → `resolved by [D69]…[D74]`;
  `TRACEABILITY.md` +6 строк (D-ссылки, `resolved`, Задачи/Реализация `—`);
  `questions/README.md` +6 (D-ссылки); `decisions/README.md` +17 строк (ретро-D
  по номерам: D1–D5 перед D6, D7–D11 между D6/D12, D13 между D12/D14; D69–D74
  после D68). Проверки: **74 = 74** (README-строки D и `glob docs/decisions/D*.md`);
  Q-строки **70/70** (TRACEABILITY и questions/README); статусы и `Спека: —`
  верны; ссылки живые (T-01/T-08/T-16/T-18/T-19, `features/*`, `src/*`,
  `AGENTS.md`, `.opencode/rules/**`); §10 не тронут; историч. `reviews/analysis/mail`
  не тронуты. Сверка §5.3 — чтением, без `cargo`/git (D50): ретро-D ✅/🟡/⬜/⚪ по
  карте (№1 ✅ `core.rs`/`GRAMMAR.md`; №13 ✅ `core.rs:13/:453`; №8 ✅ `mcp.rs`;
  №2 🟡 `mcp.rs:239–258/:440`; №3–5/7 ⬜/⚪; №9 ⚪; №10/11 ⬜). D69 фиксирует
  исключение T-18 «ретро-D без Q — допустимо» (карточку правит исполнение D70).
  **Замечание:** «63» в задании — промежуточный ориентир (57 + 6 D69–D74),
  итог **74 = 74** (согласовано). Отчёт — лента `.opencode/mail/service-doc-rework.md`
  (секция `## migrator · 29.09.2026 · D69–D74`); коммит — за `git`. **Дальше:**
  шаг 3 завершён → согласование/приёмка пакета; затем исполнение D70–D74
  (SPEC, BRIEF→rules, CHANGELOG, README×2, GRAMMAR, карточка T-18) — канон
  агентов по протоколу (аудит `auditor`).
- 2026-09-29 · service-doc-rework (D70/D71, часть migrator): исполнены D70 (SPEC)
  и D71 (ссылки на `BRIEF` в зонах migrator). **SPEC §10: таблица (№1–68,
  «Обоснование», шапка-Q41) удалена** — 70 строк, 6 правок `edit` (преамбула +
  5 чанков строк, контент-анкеры); раздел = «## 10. Решения — журнал» + абзац
  (журнал `decisions/`+`questions/`, `TRACEABILITY.md`,
  `.opencode/rules/journal.md`, корневой `DECISIONS.md`; статусы —
  `features/README.md`, Q5/Q40/Q41). CRLF сохранён: `rg -c "\r$"` 924 → **844**.
  **§1.3.1 (P3): провенанс Q20 ↔ D22 верен** — `D22` `Resolves: Q20`, прямо
  закрепляет принцип атомарной композиции (§1.3.1); минимальная правка
  «решением Q20» → «решением [`D22`] (вопрос [`Q20`])». `BRIEF`→`journal.md`:
  `decisions/README.md` (шапка + «Каталог»: `Dn` сквозной, историч. §10, D70),
  `questions/README.md` (§5.1–§5.2; снят «краткий канон §10»), `tasks/README.md`
  (§5.3; источник — журнал, без «§10 напрямую»), `T-18` (§7; у `D` — запись в
  `decisions/README.md`, §10-адреса историч. — D70 п.4), `T-19` (§7/§8; `BRIEF.md`
  убран из исключений doc-size — D71 п.4; «395→304» — историч. code-span + D71).
  Пути: `../../.opencode/rules/journal.md` (docs/*), `../../../…` (карточки). `BRIEF`
  в SPEC — 0; в зонах — только историч. имена (Q58/Q67/D62/D71, TRACEABILITY,
  каталоги) и `docs/BRIEF.md` в `T-19:14`. `cargo`/git не запускались (D50).
  Отчёт — `service-doc-rework`. **Остаток (след. вызов):** D-файлы с живыми
  `[`BRIEF.md`](../BRIEF.md)` (D38, D50, D60, D61, D62, D64, D65, D66, D71, D73),
  вкл. D66 (D71 п.4) и D60 (D70 п.3 — таблица канонов «SPEC — архитектура и §10»);
  Q-файлы — историч., D71 п.3 «не трогать»; `docs/README.md`/`CHANGELOG.md` —
  `docs-writer`.
- 2026-09-29 · service-doc-rework (D70/D71, вызов 2 — D-файлы): де-линкованы
  живые `[BRIEF.md](../BRIEF.md)` в `docs/decisions` (D50 ×4, D60 Affects,
  D61 ×4, D62 ×4, D64 ×5, D65 ×8, D71 ×3) → code-span `BRIEF.md` + «(историч.)».
  **D66**: ссылки → `journal.md` §7/§8; `BRIEF.md` убран из исключений doc-size
  (D71 п.4; `journal.md` в исключения не нужен — <300 строк, охват
  `.opencode/rules/**`). **D60**: таблица канонов, строка SPEC → «целевая
  архитектура; решения — журнал `decisions/` (D70)». **D62**: статус →
  `superseded by D71` + note в начале; статус в `decisions/README.md`
  синхронизирован. **T-18**: «Зависит от» + D71. Проверки: `rg "\]\([^)]*BRIEF"
  docs/decisions` **пусто**; `rg BRIEF` — только code-span/plain-история;
  `journal.md` ссылки живые; `D60:37` без §10; `D62:7` superseded. `cargo`/git не
  запускались (D50). Отчёт — `service-doc-rework` (секция D-файлы). **Остаток:**
  живые BRIEF-ссылки вне зоны — `docs/README.md`/`CHANGELOG.md` (`docs-writer`);
  Q-файлы (Q43/Q55/Q57/Q58/Q60/Q61/Q62/Q67) — историч. (D71 п.3); тело D60
  (§10-canon строки 53/76/93–97) — историч. текст решения, не переписан.
- 2026-09-29 · service-doc-rework (D72 п.4 → D68): в
  `docs/decisions/D68-changelog-handwritten.md` в «Следствия» добавлен пункт
  «Обновление 29.09.2026» — область ведения `CHANGELOG`: только кодовые
  изменения продукта; документация/процесс — нет (история — git/журнал Q/D);
  ссылка на `[D72](D72-changelog-full-cleanup.md)`. Историч. текст D68 не
  переписан. Проверки: чтением — связность, ссылка живая
  (`docs/decisions/D72-changelog-full-cleanup.md`). `cargo`/git не запускались
  (D50). Отчёт — `service-doc-rework`. Дальше — пакет D70/D71(+D72) к
  аудиту/приёмке; коммит — за `git`.
- 2026-09-29 · service-doc-rework (P2/P3 аудита): **P2** — в Q-файлах
  `[`BRIEF.md`](../BRIEF.md)` → code-span `BRIEF.md` + «историч.; правила —
  `.opencode/rules/journal.md`, [D71]» (Q43:9, Q55:11/36, Q57:7, Q58:7/13,
  Q60:7/15, Q61:7/17/42, Q62:10/19, Q67:10). **P3** — T-18 (проверка парности):
  названо исключение «ретро-D без Q (D69: `D1`–`D5`, `D7`–`D11`, `D13`)».
  Проверки: `rg "\]\([^)]*BRIEF" docs/questions docs/tasks` **пусто**;
  `rg BRIEF docs/questions` — только code-span/plain-история. `cargo`/git не
  запускались (D50). Отчёт — `service-doc-rework` (секция P2/P3). Дальше — пакет
  к аудиту/приёмке; коммит — за `git`.
- 2026-09-29 · service-doc-rework (P2-2 приёмки): в
  `docs/decisions/D60-docs-ownership-sync.md:44` (таблица канонов) строка
  `CHANGELOG.md`: «хронология прототипа» → «кодовые изменения продукта
  ([D72])». Согласовано с `AGENTS.md:172`, `docs/README.md:15`. Других правок
  D60 нет. `cargo`/git не запускались (D50). Отчёт — `service-doc-rework` (P2-2).
  Дальше — пакет к аудиту/приёмке; коммит — за `git`.
- 2026-09-30 · service-git-efficiency (оформление, **Q71 → D75**, готово):
  созданы `docs/questions/Q71.md` (🟡 важно; сразу `resolved by [D75]` —
  решение владельца 30.09.2026) и
  `docs/decisions/D75-git-lean-workflow.md` (`accepted`, `Resolves: [Q71]`,
  `Спека: —` (§10 упразднена D70), `Affects` — `.opencode/agents/git.md`,
  `rules/git-workflow.md`, `rules/review.md`, `scripts/git-check.mjs` (новый),
  `memory/git.md`; `Tasks: —`). Строки: `TRACEABILITY.md` (+1, Q71/D75,
  `resolved`, задачи/реализация `—`), `questions/README.md` (+1),
  `decisions/README.md` (+1, после D74); F49–F55 в
  `docs/analysis/findings-registry.md` (статус «закрывается D75»; post-push,
  серии `--grep`, полные чтения ленты/памяти, промпт-дубль состава, отказы прав,
  eol/newline-пробы, `memory/git.md` 602 строки). Сверка §5.3 — ⚪
  (процесс/инструменты): `git.md` `steps: 28`/allowlist без `git-check.mjs`,
  `git-workflow` §«Пакет и подтверждение»/§«Идемпотентность», `review.md`
  §«Доступные команды», `memory/git.md` 602 строки, хелпер отсутствует.
  Проверки: **75 = 75** (README-строки D = файлы `D*.md`); Q-строки **71/71**;
  ссылки Q71/D75 живые (`rg -o`); ID не переиспользованы (свободные — Q72/D76).
  `cargo`/git не запускались (D50); `.opencode/**` (кроме ленты/памяти) не
  трогал — исполнение D75 за сервисной сессией. Отчёт — лента
  `.opencode/mail/service-git-efficiency.md`; коммит — за `git`.
- 2026-09-30 · service-git-efficiency (хвосты приёмки D75): **P3** — в
  `docs/decisions/D75-git-lean-workflow.md` `Affects` дополнен
  [`AGENTS.md`](../../AGENTS.md) (карта `.opencode/rules/`+`.opencode/scripts/`;
  прецедент D51); **F49–F55** в `docs/analysis/findings-registry.md` —
  «закрывается D75» → «закрыт 30.09.2026 (D75; …; принято `validator` — отчёт
  `docs/reviews/git-efficiency-2026-09-30.md`)», суть/связи не переписаны.
  Проверки: `rg` — `AGENTS.md` в `Affects` (:12), 7 закрытых F-строк, 0
  «закрывается D75», ссылка на отчёт живая. Прочих правок нет; `cargo`/git не
  запускались (D50). Отчёт — та же лента (`## migrator · 30.09.2026 · P3 и
  F49–F55`); коммит — за `git` (сейчас — «не коммитим»).


