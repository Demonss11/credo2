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
- **02.10.2026 · T-24 (S-пилот), до прогона.** Класс S; полный DoD (R2/D50).
  Снимок: ветка `feature/T-24-rest-cli-contour-test` (= origin/…), HEAD `caac10d`
  (= develop = origin/develop) + рабочее дерево; коммитов нет. Проверено
  чтением/`rg`: `tests/rest_cli.rs` — 6 `#[test]` (запуск/`GET /health`
  поллинг; `--api-key` 401 без/с неверным, 200 с верным; env `CREDO_API_KEY`;
  открытый режим; открытые пути `/health`,`/docs`,`/openapi.json`;
  `--no-rest`+`--rest`/`--addr` → clap); аргументы/контракт совпадают с
  `src/main.rs:17-37` (`conflicts_with_all`), `src/rest.rs:63-85` (auth, открытые
  пути, текст 401), `:96-106` (`schema_version`,`count`); dev-deps
  (`tempfile`,`serde_json`) на месте, `Cargo.toml` не менялся. Границы: `git diff
  --numstat -- src tests Cargo.toml features docs` пусто; `git diff --check`
  пусто; `tests/rest_cli.rs` — untracked (новый). Запускаю: `cargo fmt --check`,
  `cargo clippy --all-targets -- -D warnings`, `cargo test --all`. Ориентир —
  принято, P1/P2/P3 нет (прецедент 135 passed + 6 новых ≈ 141).
- **02.10.2026 · T-24 (`-r2`, iteration 2), до прогона.** Класс S; полный DoD
  (R2/D50). Снимок: ветка `feature/T-24-rest-cli-contour-test` (= origin), HEAD
  `caac10d` (= develop = origin/develop) + рабочее дерево; коммитов нет;
  `tests/rest_cli.rs` — untracked. Проверено чтением: P1 закрыт —
  `tests/rest_cli.rs:47` схлопнут в let-chain
  `if let Ok((200, body)) = request(...) && body.contains("ok") { return; }`
  (rustfmt-перенос :47-48); логика та же; иных правок нет. Квитанция T-24
  iteration 1 (rework) на месте (:1552-1565). Запускаю: `cargo fmt --check`,
  `cargo clippy --all-targets -- -D warnings` (подтвердить закрытие P1),
  `cargo test --all` (регресс). Ориентир — 141 passed / 0 failed (rest_cli 6).
  Границы — `git diff --numstat -- src tests Cargo.toml features docs` пусто.
- **02.10.2026 · T-24 (`-r2`) — после прогона: принято.** `cargo fmt --check`
  pass; `cargo clippy --all-targets -- -D warnings` **pass** (P1 закрыт,
  let-chain `:47`); `cargo test --all` — **141 passed / 0 failed** (rest_cli 6/6,
  features_inventory 4/4). Границы чисты (numstat пусто, `git diff --check`
  пусто). Покрытие карточки 6/6. P1/P2/P3 нет. Отчёт
  `docs/reviews/T-24-2026-10-02-r2.md`; квитанция `T-24` iteration 2, accepted.
  Дальше — `docs-writer` (T-24 ✅) + `migrator` (Q22 → done, видимая `[T-24] ✅`)
  → гейт пакета → `git` branch_end → `complete`.
- **02.10.2026 · T-24 — после прогона: rework (P1).** `cargo fmt --check` pass;
  `cargo clippy --all-targets -- -D warnings` **fail** —
  `tests/rest_cli.rs:47` `clippy::collapsible_if` (rust-1.96.0, `-D warnings`:
  «could not compile test rest_cli»); `cargo test --all` — **141 passed / 0
  failed** (lib 61, docs_journal 14, features_inventory 4, mcp_draft 25,
  mcp_errors 8, publish 12, rest 11, rest_cli 6). Покрытие карточки — 6/6
  пунктов, контракт совпал с `src/main.rs:17-37`/`src/rest.rs:63-85,:96-106`;
  границы чисты (`git diff --numstat -- src tests Cargo.toml features docs`
  пусто, `git diff --check` пусто, трекнутые тесты не тронуты). P2/P3 нет.
  Правка P1 — схлопнуть `if` в let-chain (`tests/**`, зона `tester`), затем
  `-r2`. Отчёт `docs/reviews/T-24-2026-10-02.md`; квитанция `T-24` iteration 1,
  rework. Урок: `tester` clippy не запускает (не в правах) — единственный
  страж `clippy --all-targets` на тестовом таргете это `validator`; тест обязан
  быть clippy-чистым заранее (лет-чейны, `collapsible_if`).
- **02.10.2026 · r9-приёмка (T-15/фаза C, `service-t24-fixes`), до записи
  артефактов.** Адресная документная сверка (D50 + решение владельца
  02.10.2026 «тесты только при правках кода» — `cargo` НЕ запускается;
  `src/**`, `tests/**`, `Cargo.toml` не тронуты). Снимок: `develop` =
  `origin/develop`, HEAD `6c28e51` + рабочее дерево (пакет не закоммичен).
  Проверено чтением/`rg`/`git diff` (по одному файлу; `git diff -- <path>`
  отклоняется — рабочая форма `git diff <path>`): D87 ↔ факт — 4 пункта
  (lead.md:19 rev-parse + :43-44 `#default`; git.md:22-23 branch --list/
  ls-remote; analyst.md заметка :49-50; tester.md:19 clippy + шаг 4 + шаблон);
  CCSN `git-workflow.md:117-121`; stale `dispatch-loop.md:110-113`; review.md
  §«Доступные команды» синхронен фронтматтерам (lead/git/analyst/tester).
  `agents-perms.mjs` — 11 из 18, новые права видны (reload применён). Журнал
  Q84↔D87 парны, каталоги + TRACEABILITY Q84 `in work`/T-15 🚧, карточка C14 🚧.
  Границы: `git diff --numstat -- src tests Cargo.toml` пусто; `git diff
  --check` пусто; изменены только разрешённые пути. Статические гейты
  `docs_journal.rs` (Q↔D, каталоги, TRACEABILITY lifecycle/tasks) зелёные по
  чтению. Ориентир — принято, P1/P2 нет; P3 — C14 источник разбора по имени
  (не блокер, зафиксирован аудитом).
  **После записи артефактов: принято (P1/P2 нет).** `cargo` не запускался
  (D50). Все 4 пункта D87 подтверждены `git diff` по файлам; `agents-perms` —
  11 из 18; гейты `docs_journal.rs` зелёные по чтению; границы чисты. P3
  оставлена открытой (не блокер). Отчёт
  `docs/reviews/service-t24-fixes-2026-10-02.md`; квитанция
  `service-t24-fixes` iteration 1, accepted. Дальше — гейт пакета → `git`
  (develop, сервисный пакет без ветки) → `complete`. Урок: `git diff -- <path>`
  отклоняется движком — рабочая форма `git diff <path>` (без `--`); несколько
  путей в одном вызове тоже не матчатся — по одному файлу.
