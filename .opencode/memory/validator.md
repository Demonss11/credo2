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
- **02.10.2026 · r10-приёмка (T-15/C9/C12, `service-c9-c12`), до записи
  артефактов.** Адресная документная сверка (D50 + решение владельца
  02.10.2026 «тесты только при правках кода» — `cargo` НЕ запускается; `src/**`,
  `tests/**`, `Cargo.toml` не тронуты). Снимок: `develop` = `origin/develop`,
  HEAD `af53e23` + рабочее дерево (пакет не закоммичен). Проверено чтением/`rg`/
  `git diff` (по одному файлу через `./`): D88 ↔ факт — 6 пунктов (лимит
  `lead.md:6`=`AGENTS.md:132`; останов `dispatch-loop.md:49-51`; записи
  `lead.md:59-61`/`dispatch-loop.md:33-35`/`AGENTS.md:105-107`; глубокий план
  `analyst.md:67-68,:86-89`; resume `lead.md:50-51`/`analyst.md:90-92`; дробление
  `dispatch-loop.md:39-41`; сплит `analyst.md:93-94`/`dispatch-loop.md:169-171`).
  `agents-perms.mjs` — 11 из 18, `lead steps=24`. Журнал Q85↔D88 парны, каталоги
  `:107`/`:117`, TRACEABILITY:89 `in work`/T-15 🚧, карточка `C9`🚧 `:220`/`C12`🚧
  `:223`; ссылки живы, номеров строк нет. Границы: numstat по src/tests/Cargo
  пусто, `git diff --check` пусто. Статические гейты `docs_journal.rs` зелёные
  по чтению. Ориентир — принято, P1/P2/P3 нет.
  **После записи артефактов: принято (P1/P2/P3 нет).** Отчёт
  `docs/reviews/service-c9-c12-2026-10-02.md`; квитанция `service-c9-c12`
  iteration 1, accepted. Дальше — `migrator` (C9/C12 ✅, Q85 → done), гейт,
  `git` (develop), `complete`.
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
- **02.10.2026 · T-16 (iteration 2), до прогона.** Класс M; полный DoD (R2/D50).
  Снимок: ветка `feature/T-16-stale-check-test` (published, 0 коммитов),
  HEAD `2958bf5` + рабочее дерево (2 чужих M сервисной сессии не относятся к
  T-16). Проверено чтением: `src/lib.rs:811-816` `source_file_path` +
  `is_stale` на него (:819-825, поведение сохранено); `src/mcp.rs:224-246`
  `test()` при stale `parse_rule` текста файла, иначе `d.rule.clone()`,
  ошибки чтения/разбора → `ToolError::evaluation` (`evaluation_failed`, §4.5/
  D34, D40); юнит-тесты `src/mcp.rs:986,1022`; интеграционные `tests/mcp_draft.rs:212,262`.
  Границы: `git diff --numstat -- src tests Cargo.toml` = src/lib 7/4, src/mcp
  86/2, tests 75/0; `git diff --check` пусто; T-02/T-08 не тронуты; `Cargo.toml`
  не тронут. Статусы: карточка 🚧, `tasks/README.md:57` 🚧, TRACEABILITY:16
  `[T-16] 🚧` — синхронны. Фича `test_draft.feature` — 6 сценариев, T-16
  закрывает 1 («устаревший черновик»), сценарий (:63-70) зависит от T-02 ⬜ →
  фича остаётся 🟡. Запускаю: `cargo fmt --check`, `cargo clippy --all-targets
  -- -D warnings`, `cargo test --all`. Ориентир — 149 passed (144 + 2 юнит +
  2 интеграц.; прецедент 144 на T-15/C9-C12 + T-24 r2 = 141).
- **02.10.2026 · T-16 — после прогона: ОТКЛОНЕНО (rework, P1 базовый).**
  `cargo fmt --check` pass; `cargo clippy --all-targets -- -D warnings` pass;
  `cargo test --all` — **lib 63 passed / 0 failed** (в т.ч. оба юнит-теста
  T-16), `docs_journal` — **13 passed / 1 FAILED**
  (`no_addresses_to_removable_or_session_data`, `docs_journal.rs:889`), прогон
  остановлен на `docs_journal` (интеграционные mcp_draft/publish/rest не
  дошли). **P1 — красный DoD (`review.md:55` — блокер), но дефект БАЗОВЫЙ, не
  T-16:** тест ловит ссылки в Q84:11, Q85:12, D87:22,74,100, D88:83,111 на
  `docs/analysis/T-24-run-2026-10-02-lead-session.md`; эти файлы не трогались
  T-16 (`git diff --numstat` пусто), последний коммит по ним — `2958bf5`
  (T-15 r10) / `af53e23` (T-15 r9) — предки базы ветки. Тест сканирует только
  `docs/questions`+`docs/decisions` (`journal_files` :677-681) — T-16-файлов
  там нет. T-16 сам по себе корректен: покрытие сценария
  `test_draft.feature` :55-61 подтверждено (интеграц. :212 + юнит :986;
  coverage condition `<18`, matched false, метки §4.5/D34), границы чисты
  (T-02/T-08/`Cargo.toml`/канон не тронуты), статусы 🚧 синхронны. Фича
  `test_draft.feature` остаётся 🟡 (6 сценариев, T-16 закрывает 1; сценарий
  :63-70 зависит от T-02 ⬜). Отчёт `docs/reviews/T-16-2026-10-02.md`;
  квитанция `T-16` iteration 2, rework. Рекомендация плану: базовый P1 —
  отдельная задача/сервисная правка D65-ссылок (зона migrator/канон), T-16
  перепринять (`-r2`) после зелёного `cargo test --all`. Урок: база ветки
  может быть красной — снимать снимок `cargo test --all` на базе до приёмки
  или явно отделять базовый P1 от дефекта задачи.
- **02.10.2026 · T-16 (участок базового P1), адресная проверка, до прогона.**
  Снимок: ветка `feature/T-16-stale-check-test`. Проверено чтением: гейт
  `no_addresses_to_removable_or_session_data` (`tests/docs_journal.rs:881-894`)
  сканирует `journal_files()` (:677-700 — questions/*.md, decisions/*.md,
  TRACEABILITY, tasks/README, features/README, SPECIFICATION) на
  `removable_addresses` (:840-877, whitelist `findings-registry.md` :866);
  `traceability_tasks_exist_and_match_registry` (:592-624, точное равенство
  статуса :610-613, полнота D77 :618-622);
  `traceability_lifecycle_matches_task_openness` (:564-585).
  `TRACEABILITY.md:65` — Q61/D65 `done`, `[T-25] ⬜`; `tasks/README.md:58` —
  T-25 ⬜. Запускаю ровно `cargo test --test docs_journal`. T-25 не закрываю,
  полный `--all` не запускаю. Ориентир — 14 passed / 0 failed.
- **02.10.2026 · T-16 (участок базового P1), адресная проверка — после прогона:
  ОТКЛОНЕНО (rework, P1).** `cargo test --test docs_journal` — **13 passed /
  1 FAILED**. `no_addresses_to_removable_or_session_data` (:881) — **зелёный**
  (D65-адреса Q84/Q85/D87/D88 сняты). `traceability_tasks_exist_and_match_registry`
  (:592) — **зелёный** (T-25 видна, ⬜ = реестр). Но
  `traceability_lifecycle_matches_task_openness` (:564) — **КРАСНЫЙ**:
  `tests/docs_journal.rs:577` — «TRACEABILITY Q61: `done` не допускает открытых
  задач». Причина: участок P1 заменил в `docs/TRACEABILITY.md:65` ячейку задач
  Q61/D65 `—` → `[T-25] ⬜`, но оставил lifecycle `done`. Это противоречит
  D82 §Следствия:55-56 («`done`/`open` ⇒ открытых задач нет», легенда — D77-гейт).
  `git diff docs/TRACEABILITY.md` подтверждает: ровно эта строка внесена
  участком P1. Либо Q61 → `in work` (как Q12 с T-16 🚧), либо ячейка `—` (T-25
  не привязывать к Q61). Тот же урок, что T-20 r2: помечая задачу открытой в
  строке, синхронизировать lifecycle. Базовый P1 **НЕ снят** — переприёмка
  T-16 `-r2` заблокирована. T-25 не тронута (⬜), полный DoD не запускался.
  P1/P2: P1 — lifecycle Q61 vs открытая T-25. Отчёт — лента T-16; квитанцию
  T-25 не оформлял (адресная проверка в рамках T-16).
- **02.10.2026 · T-16 (участок базового P1), повторная адресная проверка
  после правки migrator, до прогона.** Снимок: ветка
  `feature/T-16-stale-check-test`. Дельта migrator: `TRACEABILITY.md:65`
  lifecycle `done` → `in work`; ячейка `[T-25] ⬜` сохранена
  (`git diff -- docs/TRACEABILITY.md` — Q61/D65 строка `in work`,
  текст без номеров строк). Ожидаю: `no_addresses_to_removable_or_session_data`
  (:881) зелёный, `traceability_tasks_exist_and_match_registry` (:592) зелёный,
  `traceability_lifecycle_matches_task_openness` (:564) зелёный. Запускаю ровно
  `cargo test --test docs_journal`; `--all` НЕ запускаю (следующий шаг — приёмка
  T-16 `-r2`); T-25 не закрываю, статусы не меняю. Ориентир — 14 passed / 0 failed.
- **02.10.2026 · T-16 (участок базового P1), повторная адресная проверка —
  после прогона: P1 СНЯТ.** `cargo test --test docs_journal` — **14 passed /
  0 failed** (0.06s). Три целевых: `no_addresses_to_removable_or_session_data`
  (:881) — зелёный; `traceability_tasks_exist_and_match_registry` (:592) —
  зелёный; `traceability_lifecycle_matches_task_openness` (:564) — зелёный
  (migrator перевёл lifecycle Q61/D65 `done` → `in work`, ячейка `[T-25] ⬜`
  сохранена). Полный DoD (`--all`) не запускался — следующий шаг приёмка T-16
  `-r2`. T-25 не тронута (⬜ в карточке/реестре/TRACEABILITY), статусы не
  менялись, квитанция T-25 (приёмки) не оформлялась. Отчёт — лента T-16.
- **02.10.2026 · T-16 (`-r2`, iteration 2, rework=1), полный DoD, до прогона.**
  Класс M. Снимок: ветка `feature/T-16-stale-check-test` (= origin) @ 2958bf5
  (= develop = origin/develop, 0 коммитов) + рабочее дерево. Базовый P1 снят
  (адресный docs_journal 14/0, запись выше). Проверено чтением: реализация
  `src/lib.rs:811-816` `source_file_path` → `rules/{name}.dar`, `is_stale`
  на хелпере (:819-825); `src/mcp.rs:232-244` stale → `parse_rule(текст файла)`,
  иначе `d.rule.clone()`, ошибки чтения/разбора → `ToolError::evaluation`;
  метки :255-258 от черновика (§4.5/D34). Тесты: `tests/mcp_draft.rs:212,262`;
  юнит `src/mcp.rs:986,1022`. Сценарий `test_draft.feature:55-61` (3 «Тогда» +
  негатив) покрыт. Границы: `git diff --numstat -- src tests Cargo.toml` =
  lib 7/4, mcp 86/2, mcp_draft 75/0; Cargo.toml не тронут; `git diff --check`
  пусто; canon/права не тронуты; чужие `mail/service-mcp-ready-r10.md`,
  `memory/service.md` — вне пакета. Статусы T-16 🚧 = реестр :57 =
  TRACEABILITY :16; T-25 ⬜ (реестр :58, TRACEABILITY :65, Q61/D65 `in work`).
  Запускаю полный DoD: `cargo fmt --check`, `cargo clippy --all-targets --
  -D warnings`, `cargo test --all` (полная разбивка). Ориентир ≈149 passed /
  0 failed (lib 63, docs_journal 14, features_inventory 4, mcp_draft 25+2,
  mcp_errors, publish, rest, rest_cli).
- **02.10.2026 · T-16 (`-r2`, iteration 2) — после прогона: ОТКЛОНЕНО
  (rework, P1).** fmt pass; clippy pass; `cargo test --all` **FAIL** (exit 1)
  на `mcp_draft`: 26 passed / 1 FAILED. Разбивка: lib 63/0, main 0/0,
  docs_journal **14/0** (базовый P1 снят — жизненный цикл Q61/D65 `in work`
  ок), features_inventory 4/0; отдельно — mcp_errors 8/0, publish 12/0,
  rest 11/0, rest_cli 6/0. Всего 144 passed / 1 failed.
  **P1 — дефект ТЕСТА T-16, не кода:** `tests/mcp_draft.rs:244`
  `assert_eq!(tested["decision"], "пусто")`, факт `""`. Канон — D21:61-62
  («`""` в JSON объяснения»), Q10, `src/core.rs:585-594`
  (`unmatched_rule_has_empty_decision_q10` → `assert_eq!(e.decision, "")`).
  Литерал `"пусто"` единственный в репо и отсутствует в базе
  `HEAD:tests/mcp_draft.rs` (добавлен этой задачей); юнит `src/mcp.rs:1011-1018`
  `decision` не утверждает. Реализация `src/mcp.rs:232-244` корректна.
  Правка (tester, одна строка): `"пусто"` → `""`. Урок: r1 не поймал — прогон
  стоял на docs_journal и `mcp_draft` не дошёл; «зелёный по чтению» тест
  надо прогонять целиком.
  Границы чисты: Cargo.toml/core.rs/канон/T-02/T-08 не тронуты; чужие сервисные
  вне T-16. Сценарий `test_draft.feature:55-61` покрыт; фича 🟡.
  Отчёт `docs/reviews/T-16-2026-10-02-r2.md`; квитанция `T-16` iteration 2,
  verdict rework. Дальше: `tester` (правка) → `validator -r3`.
- **02.10.2026 · T-16 (`-r3`, iteration 2, rework=1), полный DoD, до прогона.**
  Класс M. Снимок: ветка `feature/T-16-stale-check-test` (= origin) @ `2958bf5`
  (= develop = origin/develop, 0 коммитов) + рабочее дерево. Правка `-r2` принята
  чтением: `tests/mcp_draft.rs:244` = `assert_eq!(tested["decision"], "", "{tested}")`
  (`""`, не `"пусто"`); `rg '"пусто"' *.rs` — нет совпадений (литерал удалён).
  Проверено ранее: реализация `src/lib.rs:811-816`/`src/mcp.rs:232-244`, метки
  :254-258 от черновика (§4.5/D34); сценарий `test_draft.feature:55-61` покрыт,
  фича 🟡; базовый P1 снят (docs_journal 14/0). Границы по снимку статуса:
  `src/lib.rs`, `src/mcp.rs`, `tests/mcp_draft.rs`; `Cargo.toml`/`core.rs` не
  тронуты; канон/права нет; чужие `mail/service-mcp-ready-r10.md`,
  `memory/service.md` и S-файлы T-25 вне T-16. Запускаю полный DoD:
  `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`,
  `cargo test --all` (полная разбивка). Ориентир 145 passed / 0 failed
  (mcp_draft 27/0, docs_journal 14/0).
- **02.10.2026 · T-16 (`-r3`, iteration 2) — после прогона: ПРИНЯТО.** fmt pass;
  clippy pass; `cargo test --all` **pass** (exit 0), **145 passed / 0 failed**
  (lib 63/0, main 0/0, docs_journal 14/0, features_inventory 4/0, mcp_draft
  **27/0**, mcp_errors 8/0, publish 12/0, rest 11/0, rest_cli 6/0, doc 0/0).
  P1 `-r2` снят: `tests/mcp_draft.rs:244` = `""`, литерал `"пусто"` удалён
  (`rg` — нет), тест зелёный. Сценарий `test_draft.feature:55-61` покрыт;
  фича 🟡 (6 сценариев, :63-70 ← T-02 ⬜). Границы чисты (Cargo.toml/core/канон
  не тронуты; T-02/T-08 ⬜; чужие сервисные и S-файлы T-25 вне T-16). P1/P2/P3
  нет. Отчёт `docs/reviews/T-16-2026-10-02-r3.md`; квитанция `T-16` iteration 2,
  verdict accepted. Дальше — closeout (docs-writer → migrator → пакет → git →
  complete); T-25 не трогать.
- **02.10.2026 · T-25 (S), адресная документная приёмка, до прогона.** Снимок:
  ветка `feature/T-25-d65-analysis-addresses` (= origin) @ `2958bf5`
  (= develop = origin/develop, 0 коммитов) + рабочее дерево (`mixed_worktree`).
  Проверено чтением/rg: D65-адрес
  `docs/analysis/T-24-run-2026-10-02-lead-session.md` снят из Q84/Q85/D87/D88
  (факты — реестр); в живом журнале (`tests/docs_journal.rs:677` journal_files:
  questions/decisions/TRACEABILITY/tasks README/features README/SPECIFICATION)
  адресов на `docs/analysis/**` нет, кроме whitelist `findings-registry.md`
  и шаблонов (`**`, `<T-XX>`); тест `docs_journal.rs:881` (`removable_addresses`
  :820, whitelist :846). TRACEABILITY:65 = `[T-25] ⬜`, Q61 `in work`; реестр
  `docs/tasks/README.md:58` T-25 ⬜ — синхронны (тест :592, точное равенство).
  F73 в реестре; карточка T-25 ⬜. Границы: `git diff --numstat -- src tests
  Cargo.toml` — T-16-пути (lib 7/4, mcp 86/2, mcp_draft 75/0) вне T-25;
  T-16 closeout (`CHANGELOG.md`, карточка T-16, TRACEABILITY:16, registry:57) —
  не T-25. `git diff --check` пусто. Запускаю `cargo test --test docs_journal`.
  Ориентир 14 passed / 0 failed.
- **02.10.2026 · T-25 — после прогона: ПРИНЯТО.** `cargo test --test docs_journal`
  — **14 passed / 0 failed** (0.09s), в т.ч. `no_addresses_to_removable_or_session_data`
  и `traceability_tasks_exist_and_match_registry`. P1/P2/P3 нет. Отчёт
  `docs/reviews/T-25-2026-10-02.md`; квитанция `T-25` iteration 1, accepted.
  Дальше — closeout (docs-writer ✅/🚧, migrator Q61 → done), затем пакет `git`
  (T-16-closeout отделить; `docs/tasks/README.md` — hunk-разбор) → complete.
- **02.10.2026 · Контроль совмещённого дерева T-16+T-25 перед коммитом
  (не новая приёмка), до прогона.** Снимок: ветка
  `feature/T-25-d65-analysis-addresses` (= origin/…, 0 коммитов) @ HEAD `2958bf5`
  (= develop) + рабочее дерево (`mixed_worktree`). Проверено чтением/git:
  `git diff --numstat -- src tests Cargo.toml` = `src/lib.rs` 7/4, `src/mcp.rs`
  86/2, `tests/mcp_draft.rs` 75/0 (Cargo.toml пусто) — совпадает с принятым T-16
  `-r3`; `git diff --check` пусто; `git status --porcelain` — чужие сервисные
  `.opencode/mail/service-mcp-ready-r10.md` и `.opencode/memory/service.md` в
  `next_action.yaml` `package.add_paths` отсутствуют (исключены). Closeout-правки:
  `TRACEABILITY.md:65` Q61/D65 `done` + `[T-25] ✅` (диff `-` уже `done|—`, менялась
  только колонка задач), `tasks/README.md:57` T-16 ✅ + `:58` T-25 ✅,
  карточка T-16 `:3` ✅. Запускаю `cargo test --test docs_journal` (R2/D50),
  ориентир 14 passed / 0 failed.
- **02.10.2026 · Контроль совмещённого дерева T-16+T-25 — после прогона:
  принято с замечанием (P2).** `cargo test --test docs_journal` — **14 passed /
  0 failed** (0.11s), в т.ч. ключевые `traceability_tasks_exist_and_match_registry`
  и `traceability_lifecycle_matches_task_openness` — ok. Четыре требуемых пункта
  зелёные (numstat = -r3, `git diff --check` пусто, чужое сервисное исключено).
  **P2 (пакет, не код):** `next_action.yaml` `package.add_paths` (:87-117) и
  process-коммит (note :124-125) не содержат `.opencode/mail/T-16-r2.md` —
  файл `??` в `git status`, создан этим re-plan; при этом `T-25.md:69` (в пакете)
  ссылается на `[T-16-r2.md](T-16-r2.md)`. Следствие: том ленты не попадёт в
  историю, относительная ссылка в коммите повиснет. Правило самого плана
  (`next_action.yaml:128-129`): «перед гейтом — сверка add_paths со СВЕЖИМ
  `git status`… расхождение — re-plan до гейта». Правка (analyst/lead, 1 строка):
  добавить `.opencode/mail/T-16-r2.md` в add_paths (process-коммит). Прочие 34
  пути `git status` ↔ add_paths сходятся; чужие `.opencode/mail/service-mcp-ready-r10.md`
  и `.opencode/memory/service.md` — вне пакета, как заявлено. P1/P3 нет. Рекомендую
  дополнить план ДО `git branch_end`; при подтверждении владельцем — коммит.
