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
