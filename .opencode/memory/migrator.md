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

