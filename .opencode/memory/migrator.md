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

