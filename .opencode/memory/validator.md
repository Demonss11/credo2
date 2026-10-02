# Память: validator (приёмка)

- **Канон:** `.opencode/rules/review.md`; `.opencode/rules/journal.md` §5.3/§7.
- **Правило:** чекпойнт **до** прогона тестов (что проверено, что запускаю) и
  **после** (результат, вердикт) — тесты прерывают сессию (R5). Кратко.
  Квитанция — append в `state/current/receipts.yaml`. Отчёт —
  `docs/reviews/<тип>-<id>-<дата>.md`.

## Чекпойнты

- **02.10.2026 · T-15/C1 (сервисная операция r8), до прогона.** Проверено
  чтением: состав пакета (`git status` — 10 `M` + 4 `??`, `src/tests/Cargo.toml`
  не тронуты); `state-schema.md` (4 артефакта, поля/типы/обязательность,
  писатель/читатель, инварианты, `session_index`, `owner_response`);
  `dispatch-loop.md` ссылается на схему; D86 `Affects` включает analyst/lead;
  Q83↔D86 парны; каталоги/TRACEABILITY синхронны; карточка C1 🚧; роль
  `lead.md:88` = `idle` (P3 закрыт). Запускаю: `cargo fmt --check`,
  `cargo clippy --all-targets -- -D warnings`, `cargo test --all` (R2/D50).
  Ориентир 135 passed / 0 failed.
- **02.10.2026 · T-15/C1 — после прогона: принято.** fmt pass; clippy pass;
  `cargo test --all` — 135 passed / 0 failed (docs_journal 14/14,
  features_inventory 4/4). P1/P2/P3 нет. Отчёт
  `docs/reviews/T-15-c1-2026-10-02.md`; квитанция `T-15-c1` (iteration 1,
  accepted) в `state/current/receipts.yaml`. F15 не закрыт — `migrator`.
- **02.10.2026 · T-20 (волна 2 TRACEABILITY, L), адресная документная
  приёмка (D50 — cargo не запускается: `src/**`, `tests/**`, `Cargo.toml` не
  менялись).** Снимок: ветка `feature/T-20-traceability-wave2`, HEAD `d414998`
  + рабочее дерево. Проверено чтением/`rg`/glob: TRACEABILITY — 9 `done`
  (Q8–Q11,Q14,Q18,Q20,Q21,Q23) без задач; Q22 `in work` + T-24 ⬜; 16 `open`
  (Q2–Q4,Q19,Q24–Q26,Q30,Q31,Q35–Q39,Q63,Q65); Q81 `open` (—/—); Q78/Q79
  `in work` + T-20 🚧 (→ done на шаге закрытия, D82): `rg -c "open \|"` = 17 =
  16+Q81. T-24: карточка + строка сводки, источник Q22/D26, `tester`, P3, ⬜,
  без номеров строк. T-20 🚧 в сводке и карточке. Границы: `git diff --numstat
  -- src tests Cargo.toml` пусто; `git diff --check` пусто. **Находка P2:**
  `docs/tasks/T-24-rest-cli-contour-test/README.md:53` — `[D50](../D50-…)`
  резолвится в `docs/tasks/D50-…` (нет файла), верно `../../decisions/D50-…`
  (T-20 того же уровня использует `../../decisions/`; строка 54 антипаттерна
  требует живых ссылок). `review.md` §«Что блокер»: битые ссылки — блокер.
  Вердикт: **rework**. Отчёт `docs/reviews/T-20-2026-10-02.md`; квитанция
  `T-20` iteration 1, verdict rework.
- **02.10.2026 · T-15/C1 адресная переприёмка (r2), без cargo (D50).**
  Пакет не закоммичен (`deferred_by_owner`); после приёмки владелец вернул на
  доработку канон (порог `review.md` §«Возврат на доработку»: канон/права →
  новый отчёт + машинная сверка прав + адресные проверки). Дельта: сужено
  «Когда читать» `state-schema.md` (analyst+validator, auditor по потребности;
  lead/git не читают); `lead.md` — inline `session_index` + `channel`/
  `owner_response`/`result`, отсылка к схеме снята; в силе `wait_for_user`
  (:98), `idle` (:49), D86 `Affects` с ролями (:11), шаблон `lead.md` `idle`.
  Проверки: `agents-perms.mjs` — 11 из 18, команды = `review.md`
  §«Доступные команды»; `git status` — пакет + `memory/service.md` + r8;
  `git diff --numstat -- src tests Cargo.toml` пусто; `git diff --check` пусто.
  `cargo` не запускался (D50). Вердикт: принято, P1/P2/P3 нет. Отчёт
  `docs/reviews/T-15-c1-2026-10-02-r2.md`; квитанция `T-15-c1` iteration 1,
  `report -r2`.
- **02.10.2026 · T-20 повторная приёмка (`-r2`, участок №2), до прогона.** Класс
  L, адресная документная сверка (D50 — cargo не запускается: `src/**`,
  `tests/**`, `Cargo.toml` не менялись). Снимок: ветка
  `feature/T-20-traceability-wave2`, HEAD `d414998` + рабочее дерево. Заявлено
  `migrator`: `docs/tasks/T-24-rest-cli-contour-test/README.md:53` →
  `[D50](../../decisions/D50-dod-by-package-scope.md)` (ровно одна правка),
  карточка **untracked** — сверка чтением. Проверяю: резолв всех относительных
  ссылок T-24; регресс TRACEABILITY (9 done без задач, Q22 in work + T-24 ⬜,
  16 open, Q81 open, Q78/Q79 in work, T-20 🚧); границы; артефакты отчёт -r2 +
  квитанция iteration 2. Ориентир — принято, P1/P2/P3 нет.
- **02.10.2026 · T-20 (`-r2`) — после проверки: rework (P1).** Фикс P2 ок
  (`T-24:53` D50 резолвится, прочие ссылки живые). Найдено:
  `TRACEABILITY.md:82,:83` Q78/Q79 `[T-20] ⬜` vs `tasks/README.md:66` T-20 🚧 →
  тест `tests/docs_journal.rs:592 traceability_tasks_exist_and_match_registry`
  (`assert :610–613`, точное равенство) красный. Внесено участком №1, пропущено
  итерацией 1 (квитанция :1509). Доказано чтением (D50 — cargo не запускался).
  Все 24 пары реестр↔TRACEABILITY сверены — расхождение одно (T-20). Правка:
  `⬜`→`🚧` в двух строках (migrator). Границы чисты. Отчёт
  `docs/reviews/T-20-2026-10-02-r2.md`; квитанция `T-20` iteration 2, rework.
  Примечание к закрытию: Q78/Q79 → `done` + видимая `[T-20] ✅` (не `—`), иначе
  D77-часть теста. Урок: при пометке задачи 🚧 синхронизировать **обе** колонки —
  реестр и TRACEABILITY (легенда :93, тест D77); «остаточный риск» итерации
  материализуется — проверять гейты статически по чтению теста, даже без cargo.
- **02.10.2026 · T-20 (`-r3`), до прогона.** Класс L, адресная документная
  сверка (D50 — cargo не запускается: `src/**`, `tests/**`, `Cargo.toml` не
  менялись). Снимок: ветка `feature/T-20-traceability-wave2`, HEAD `d414998` +
  рабочее дерево. Чтением подтверждено: `TRACEABILITY.md:82,:83` Q78/Q79
  `[T-20] 🚧` = реестр `tasks/README.md:66` 🚧 (фикс P1 внесён); T-24:53 →
  `../../decisions/D50-dod-by-package-scope.md` (P2 в силе); T-20 🚧 в сводке
  (:66) и карточке (`README.md:3`). Осталось: сверка всех пар
  «реестр↔TRACEABILITY», регресс (9 done без задач, Q22 in work+T-24 ⬜, 16 open,
  Q81 open), границы, артефакты -r3 + квитанция iteration 3. Ориентир — принято,
  P1/P2/P3 нет.
- **02.10.2026 · T-20 (`-r3`) — после проверки: принято.** Фикс P1 подтверждён:
  `TRACEABILITY.md:82,:83` `[T-20] 🚧` = реестр `:66` = карточка `:3`; гейт
  `docs_journal.rs:592` (assert `:610–613`) статически зелёный. Все 23 пары
  «реестр↔TRACEABILITY» — 0 расхождений. Фикс P2 (`T-24:53` →
  `../../decisions/D50-…md`) в силе, цель существует. Регресс: 9 `done` без
  открытых задач, Q22 `in work`+T-24 ⬜, 16 `open`, Q81 `open`,
  `rg -c "open \|"` = 17; инварианты `done`/`in work` не нарушены. Границы чисты
  (`git diff --numstat -- src tests Cargo.toml` пусто). P1/P2/P3 нет. Отчёт
  `docs/reviews/T-20-2026-10-02-r3.md`; квитанция `T-20` iteration 3, accepted.
  Дальше — `migrator` (закрытие: T-20 ✅ + Q78/Q79 `done` с видимой `[T-20] ✅`,
  D82/D77), затем `auditor` (L), гейт, `git`, `complete`.
