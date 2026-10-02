# Память: validator (приёмка)

- **Канон:** `.opencode/rules/review.md`; `.opencode/rules/journal.md` §5.3/§7.
- **Правило:** чекпойнт **до** прогона тестов (что проверено, что запускаю) и
  **после** (результат, вердикт) — тесты прерывают сессию (R5). Кратко.
  Квитанция — append в `state/current/receipts.yaml`. Отчёт —
  `docs/reviews/<тип>-<id>-<дата>.md`.

## Чекпойнты

- **30.09.2026 · service-docs-lifecycle №1 (`research`)** — чекпойнт ДО прогона:
  база `develop` @ `9948ebf` + рабочее дерево; ожидал 12 `M` + ` D docs/research/
  doc-quality-checks-2026-09-29.md` + `?? .opencode/mail/service-docs-lifecycle.md`
  (подтверждено `git status --porcelain`). План: границы, ссылки, D65, каталоги,
  `agents-perms.mjs` ×2. `cargo` **не запускаю** (D50).
- **Итог (после):** **принято**, P1/P2/P3 нет. Границы чисты (`git diff --stat --
  src tests Cargo.toml` пусто; agents/rules/AGENTS/features пусто); ссылок канона
  на удаляемый файл нет (остались только в `docs/reviews/doc-tools-d66d68-…:17,69`
  — волна №2); D65:88–99 уточняет п.4, ссылки живые; каталоги/ID/статусы целы;
  perms ×2 = `11 из 18`; архив в git есть. Отчёт
  `docs/reviews/service-docs-lifecycle-2026-09-30.md`; квитанция записана.
  **Право `git rev-parse`** (D49, «Обновление» 30.09.2026): добавлено в allowlist —
  точная сверка базы: `git rev-parse develop origin/develop HEAD`; прежний обход
  (`git log --oneline -1` / `git show -s`) — резервный.
- **30.09.2026 · service-docs-lifecycle №2 (дельта allowlist
  `ls-files`/`check-ignore`)** — **принято**, P1/P2/P3 нет. База `develop` @
  `f28c8cb` (= `origin/develop` = `HEAD`) + рабочее дерево. Границы чисты
  (`src/tests/Cargo.toml/AGENTS.md` пусто); `.opencode/**` — 6 файлов дельты
  (agents/{validator,auditor}, rules/review.md, memory/{migrator,auditor}, лента).
  Права: `agents-perms.mjs` ×2 → `11 из 18`; `validator` +`git ls-files *`/
  +`git check-ignore *` (:31–32), `auditor` +`git check-ignore *` (:25) —
  совпадают с `review.md:89,96`; пробы `git ls-files` (путь) и `git check-ignore -v`
  (exit 1) работают; D49 :50–57 — 8 строк «Обновления №2». Отчёт
  `docs/reviews/service-docs-lifecycle-canon-2-2026-09-30.md`; квитанция
  записана. `cargo` не запускался (D50). **Новые права:** `git ls-files *`,
  `git check-ignore *` — сверка границ пакета / gitignore.
- **30.09.2026 · service-docs-lifecycle №2 (`канон агентов`)** — **принято**,
  P1/P2/P3 нет; оба замечания аудита закрыты (P2 — §8 `journal.md` оговорка
  реестра `findings-registry.md`/D48; P3 — две точки `Q54.md:12,15`). База
  `develop` @ `21c3c80` (= `origin/develop` = `HEAD`) + рабочее дерево. Границы
  чисты (`src/tests/Cargo.toml`, `AGENTS.md` пусто). Права: `agents-perms.mjs` ×2
  → `11 из 18`, `validator` ↔ `review.md` совпадают (`git branch --contains`,
  `git rev-parse`). Отчёт `docs/reviews/service-docs-lifecycle-canon-2026-09-30.md`;
  квитанция записана. `cargo` не запускался (D50).
  **Грабли:** `git diff` с несколькими dot-путями под `--` и `--numstat -- .opencode`
  движок отклоняет — одиночный `./`-путь или `git diff --numstat` без пути.
  `receipts.yaml` крупный (`grep -n "^- task:"` → 66 записей), хвост читать по
  `offset`.
- **30.09.2026 · service-statuses-review (приёмка)** — **возврат (rework)**,
  P1+P2. Отчёт `docs/reviews/service-statuses-review-2026-09-30.md`; квитанция
  `service-statuses-review` iteration 1, rework. Пакет сам по себе собран верно
  (диффы ±, границы чисты, права, записи append, P3 аудитора закрыт, DSH/D65 —
  ок; `cargo` не запускался, D50). **P1 — ветка/база:** дерево на `master`
  @ `9173fc4` (= `origin/master`), а лента (mail:34,148) называет коммит в
  `develop` @ `3d572f3`; ветки разошлись по тем же файлам (`git diff --stat
  develop HEAD`: migrator.md, validator.md, AGENTS.md, src/mcp.rs, tests/mcp_draft.rs)
  → `git switch develop` с грязными файлами откажет, коммит на текущей ветке
  уйдёт в `master` (git-workflow:26–28), на `develop` приехал бы лишний блок
  «Цикл задачи»; анкер D38 верен только для master. **P2** — validator.md:52–67,
  69–80 дубль «Цикл задачи». **Грабли:** `git branch --show-current` — нет в
  allowlist `validator` (смотреть `git status -sb` / `git branch --contains`);
  `git show -s --format=%ci <hash>` часть вызовов движок отклоняет (891 не прошёл,
  acaf3ec прошёл) — не полагаться; сравнивать ветки — `git diff --stat develop HEAD`
  (без путей). **Урок: перед приёмкой сервисной волны сверять ветку/базу
  (`git status -sb` + `git rev-parse HEAD develop`) с планом — расхождение
  master/develop ломает гейт.**
- **30.09.2026 · service-statuses-review (повторная приёмка, r2)** — **принято**,
  P1/P2 закрыты, P1/P2/P3 нет. База — `master` @ `9173fc4` (= `origin/master`) +
  рабочее дерево; `develop` @ `3d572f3` (= `origin/develop`). Отчёт
  `docs/reviews/service-statuses-review-2026-09-30-r2.md`; квитанция iteration 2
  `accepted`. **Итог:** `git diff develop -- ./.opencode/agents/validator.md` →
  только §10-ханк (блок-дубль на `develop` не приедет); `rg '^## Цикл задачи'` →
  :52; validator.md `+1/−15`; migrator.md `+2/−7`; D38 `+7` (оба дубля), D70 `+8`;
  границы `src/tests/Cargo.toml/AGENTS.md` пусты; `agents-perms` ×2 → `11 из 18`;
  память append (F35); P1 закрыт решением владельца (база `master`, коммит в
  `master` по гейту — исключение из git-workflow:26–28, санкция только в ленте).
  **Урок:** при повторной приёмке канона доказывать закрытие через
  `git diff <новая-база> -- <файл>` — `develop`-дифф validator.md был решающим.
  `cargo` не запускался (D50).
- **01.10.2026 · service-lifecycle-w2-prep (чекпойнт ДО прогона):** база —
  `develop` (ожидаю `## develop...origin/develop`); пакет документно-служебный
  (без T-XX). Что проверяю: (1) `git diff` TRACEABILITY (Q78/D82 → `in work`
  +`T-20 ⬜`, новая строка Q79/D83), `tasks/README.md` (строка T-20 + абзац
  «Исключение»/D83), `questions|decisions/README.md` (+1); (2) Q79/D83/T-20 —
  формы, `Resolves`, «Сверка с кодом» ⚪, `Tasks: T-20`, критерий; D65; ссылки;
  (3) счётчики: `open`=26, `in work`=21, `done`=32; словарь {open,in work,done};
  (4) границы: `git status --porcelain` — 5 `M` + 4 `??` (до моих записей);
  `git diff --stat -- src tests Cargo.toml AGENTS.md` пусто; (5)
  `agents-perms.mjs` ×2 → 11 из 18; (6) `cargo` **не запускаю** (D50). Итог —
  после прогона.
  **Итог (после):** **принято**, P1/P2/P3 нет. База `develop` @ `b04a77a`
  (= `origin/develop` = `HEAD`); линейно после w1 (`cb7d159` ∈ `develop`,
  `git log -3` → `b04a77a` поверх `cb7d159`). Счётчики `open`=26, `in work`=21,
  `done`=32; `rg -c "^| \[Q"` = 79 (арифметика точна); `resolved`/`dropped` —
  только легенда (:87–88); `| in work | —` пусто (каждое `in work` с задачей);
  `done` с задачами — только ✅. Диффы: TRACEABILITY Q78/D82 `open`→`in work`
  +`T-20 ⬜` + новая Q79/D83 `in work` +`T-20 ⬜`; `tasks/README.md` +T-20 и
  абзац «Исключение» (D83); каталоги +1/+1; D82/D63/D64/T-18/`journal.md` пусто
  (тела не переписаны). Q79/D83/T-20 — формы, ⚪, «Задач не требуется сверх
  T-20»; ссылки живые (`git ls-files`); D65 чисто. Границы: 5 `M` + 4 `??` (до
  моих записей) — ровно план; `src/tests/Cargo.toml/AGENTS.md` пусто; perms ×2 =
  `11 из 18`; `cargo` не запускался (D50). Отчёт
  `docs/reviews/service-lifecycle-w2-prep-2026-10-01.md`; квитанция записана.
  **Грабли:** многопутевой `git diff -- ...` с dot-путями отклонён (как и
  раньше) — одиночные вызовы; голый `./.opencode/memory/migrator.md` под `--`
  прошёл после `./`-формы (numstat 19/0).
- **01.10.2026 · service-review-links №1 (чекпойнт ДО прогона):** база —
  `develop` (ожидаю `## develop...origin/develop`, `HEAD` = `b9fd791`); правка
  — удаление строки «Отчёт приёмки: …» из `docs/tasks/T-03-check-create/README.md`
  (D65, `review.md` §«Хранение отчётов»). План: `git diff -- …` (1 удаление,
  0 добавлений; «Источник» цел — D70); `git grep -n "reviews/" -- docs/tasks`
  (только зоны T-18/T-19); адресные ссылки на `docs/reviews/**` по
  questions/decisions/features/tasks/`docs/*.md` — пусто; `git status --porcelain`
  (3 пути: T-03, memory/migrator, лента); границы `git diff --stat -- src tests
  Cargo.toml AGENTS.md .opencode/agents .opencode/rules` — пусто; `cargo`
  **не запускаю** (D50, пакет документный). Итог — после прогона.
  **Итог (после):** **принято**, P1/P2/P3 нет. `git diff -- …` карточки →
  1 удаление/0 добавлений, «Источник: Q28 …» (:6) цел (D70); `git grep
  "reviews/" -- docs/tasks` → только зоны T-18 (47/64/75)/T-19 (34);
  questions/features пусто, decisions — зоны (D64/D65/D66), `docs/*.md` —
  только `README.md:21,25`; `reviews/T-` — только внутри `docs/reviews/**`;
  `Отчёт приёмки` в `docs/tasks` нет; улики `T-03-2026-09-26{-r2}.md` на месте.
  Ветка/база `develop...origin/develop`, `HEAD`/`develop`/`origin/develop` =
  `b9fd791` (= план). Границы `src/tests/Cargo.toml/AGENTS.md/.opencode/agents/
  rules` пусты (многоточечный `--stat` отклонён движком — одиночные вызовы).
  Отчёт `docs/reviews/service-review-links-2026-10-01.md`; квитанция записана.
  `cargo` не запускался (D50).
  **Грабли:** `git diff --stat` с 6 путями (в т.ч. dot-paths) отклонён — уже
  дважды; рабочий приём — одиночный путь на вызов. `receipts.yaml` — 1152
  строки; хвост читать по `offset` (конец = запись `service-statuses-review`
  iteration 2), `edit`-анкер — строка `report:` нужной итерации (дважды
  повторяющиеся `snapshot`/`at` не уникальны).
- **01.10.2026 · service-lifecycle-w1 (чекпойнт ДО прогона):** база —
  `develop` (ожидаю `## develop...origin/develop`, `HEAD`/`develop` =
  `b9fd791`); пакет документно-служебный (без T-XX). Что проверяю: (1)
  `TRACEABILITY.md` — словарь {open,in work,done}, нет ячеек `resolved`/
  `dropped`, счётчики 27/19/32, поимённая сверка раскладки с лентой, легенда;
  (2) `git diff` по `.opencode/rules/journal.md` §3 (только пункт сводного
  цикла), D63/D64 (append-«Обновления»), T-18, каталоги Q/D; тела решений не
  переписаны; сессионных адресов нет (D65); (3) Q78/D82 — формы журнала,
  `Resolves`, «Сверка с кодом» ⚪, «Задач не требуется», строка = `open`;
  (4) границы `git status --porcelain` (8 `M` + 3 `??` до моих записей),
  `git diff --stat -- src tests Cargo.toml AGENTS.md` пусто; (5)
  `agents-perms.mjs` ×2 → 11 из 18; (6) DoD: `cargo` **не запускаю** (D50 —
  пакет без `src/**`/`tests/**`/`Cargo.toml`/`features`-счётчиков). Итог —
  после прогона.
  **Итог (после):** **принято**, P1/P2/P3 нет. База `develop` @ `cb7d159`
  (= `origin/develop` = `HEAD`) + рабочее дерево. Счётчики `open` = 27,
  `in work` = 19, `done` = 32; ячеек `resolved`/`dropped` нет; раскладка
  поимённо = ленте (done-28 + Q28/Q43–Q45; open-26 + Q78); легенда точна.
  `journal.md` §3 — один хунк 3/2 (+D82), строки :28–30 целы; D63/D64 —
  append-«Обновления» 5/0 и 4/0, тела целы; T-18 3/1; каталоги +1/+1.
  Q78/D82 — формы, ⚪, «Задач не требуется», строка = `open`, D65 чисто.
  Границы: 8 `M` + 3 `??`; `src/tests/Cargo.toml/AGENTS.md`, `docs/features`
  пусто; perms ×2 = `11 из 18`; `cargo` не запускался (D50). Отчёт
  `docs/reviews/service-lifecycle-w1-2026-10-01.md`; квитанция записана.
  **Грабли:** голый dot-путь `.opencode/**` под `--` отклонён — форма
  `./.opencode/...` проходит (подтверждено на journal.md, numstat 3/2).
- **01.10.2026 · service-rules-revision (волна 1, чекпойнт ДО прогона):** база —
  `develop...origin/develop` (сверю `HEAD`/`develop`); пакет документно-канонный
  (без T-XX). Что проверяю: (1) `git diff` по 4 rules + `AGENTS.md` §«Служебная
  зона и аудит» — только целевые правки, нормы/якоря целы; (2) `R2`/`R7`
  (`dispatch-loop.md`), `F43` ровно один (`git-workflow.md`), `T-18` без `⬜`
  (`journal.md` §7), `./`-формы и `git -C`, решения только `D82`/`D65`/`D48`;
  (3) закрытие P2-1/P2-2/P3 аудита (journal §7; D84 R2/R7, R5-примечание,
  код-форма, п.6, Следствия, Сверка); (4) Q80/D84 — формы, строка TRACEABILITY =
  `done`, каталоги; (5) `agents-perms.mjs` ×2 → `11 из 18`; (6) границы:
  10 `M` + 3 `??` (до моих записей), `git diff --stat -- src tests Cargo.toml`
  пусто; `cargo` **не запускаю** (D50). Итог — после прогона.
  **Итог (после):** **принято**, P1/P2/P3 нет. База `develop` @ `3821811`
  (= `origin/develop` = `HEAD`). Диффы 4 rules + `AGENTS.md` — только целевые
  (снят декор `Dn/Qn`, история/даты/пробы/Run/BRIEF, F-декор, реальные ID в
  примерах → шаблоны; дубли → ссылки). Якоря цели: R2 `dispatch-loop.md:39`,
  R7 `:59`, `F43` ровно один (`git-workflow.md:77`), `T-18` без `⬜`
  (`journal.md:103`), `./`-формы и `git -C` — дом `dispatch-loop.md:68–75`;
  решения — только `D82`/`D65`/`D48` в формате «основание — `Dn`».
  Находки аудита закрыты: P2-1 (нет `⬜`), P2-2/P3 (D84 :39–41,57–60,72,91–96,
  :26–27,51). Q80/D84 — формы, `Resolves` взаимны, `TRACEABILITY.md:84` =
  `done`, каталоги +1; ссылки живые (`git ls-files`). `agents-perms.mjs` ×2 =
  `11 из 18` (идентично); `.opencode/agents`/`workspace.md`/`scripts` не
  тронуты. Границы: 10 `M` + 3 `??` (ровно план), `git diff --check` пусто;
  `cargo` не запускался (D50). Отчёт
  `docs/reviews/service-rules-revision-2026-10-01.md`; квитанция записана.
  **Грабли:** `git diff --stat` с 6 dot-путями отклонён (как и раньше) —
  одиночные вызовы. `R5` — предсуществующий висячий якорь (метка ролей,
  долг волны 2) — не блокер приёмки, зафиксировано в отчёте.
  **Урок:** для канон-волн сверять «дом» дедуп-ссылок с фактическими
  `##`-заголовками файлов (`rg "^## "`) — быстрая проверка резолвимости.
- **01.10.2026 · service-rules-revision-w2 (волна 2: якорь R5 + agents/**,
  чекпойнт ДО прогона):** база — `develop` = `origin/develop` = `HEAD` @
  `52d8989` (сверено). Пакет канонно-документный (без T-XX). Что проверяю:
  (1) `git diff ./.opencode/agents/<7>.md` — только целевые снятия (Q/D-декор,
  F-декор, история, примеры), нормы и фронтматтеры целы; `git.md` — `F43`
  (`:79`); (2) якорь `R5` — `dispatch-loop.md:41–42` в §«Hard rules»; резолв
  ссылок `coder.md:52`, `tester.md:50`, `validator.md:58`, `AGENTS.md:135`,
  память `validator`; (3) D84 — append-only «Обновление 01.10.2026», тело
  цело, D65 (нет сессионных адресов); (4) `agents-perms.mjs` ×2 → `11 из 18`;
  (5) границы: `git status --porcelain` = 9 `M` + 7 agents `M` + `M D84` +
  `??` лента (до моих записей); `git diff --stat -- src tests Cargo.toml
  AGENTS.md` пусто; `cargo` **не запускаю** (D50). Итог — после прогона.
  **Итог (после):** **принято**, P1/P2/P3 нет. База `develop` = `origin/develop`
  = `HEAD` @ `52d8989`. Якорь `R5` — `dispatch-loop.md:41–42` §«Hard rules»
  (паттерн R2 :39/R7 :61); ссылки `coder:52`, `tester:50`, `validator:58`,
  `AGENTS.md:135`, память — резолвятся. 7 карточек — только целевые снятия,
  нормы целы; `F43` — `git.md:79` (единственный `[DQF]\d`); шум-контроль по
  Q/D-меткам, истории/пробам/Run/BRIEF — пусто; D84 +4 строки append-only,
  тело цело, D65 чисто; perms ×2 = `11 из 18`; фронтматтеры по существу не
  менялись (`analyst` — только комментарий :10). Границы ровно план; `cargo`
  не запускался (D50). Отчёт `docs/reviews/service-rules-revision-w2-2026-10-01.md`;
  квитанция записана. P3 (предсуществующее, вне диффа): `coder:54`/`tester:51`/
  `rust-expert:62` — ссылки на R2 без глагола. **Грабли:** `git diff` с ≥2
  dot-путями под `--` — «Permission denied» (третий раз); `git grep` без `-n`
  правом отклонён — заменил `rg`.
- **01.10.2026 · service-rules-revision-w2, iteration 2 (закрытие P3 — быстрая
  правка):** **принято**, P1/P2/P3 нет. База `develop` @ `c8ffb0f`
  (= `origin/develop` = `HEAD` — коммит волны 2 уже сделан `git`). Правка — канон
  (`.opencode/agents/**`): в 4 карточках `(R2 — …)` → `(R2 — см. …)`
  (`coder:54`, `tester:51`, `rust-expert:62`, `validator:44`). Каждый дифф ровно
  `+1/−1` (numstat `1/1` ×4), нормы целы; `rg` без «см.» → пусто (exit 1); дом
  `dispatch-loop.md:37` §«Hard rules» резолвится; perms ×2 = `11 из 18`; границы
  4 `M` agents + `M` лента; `git diff --stat -- src tests Cargo.toml AGENTS.md`
  пусто; `git diff --check` пусто; `cargo` не запускался (D50). Отчёт
  `docs/reviews/service-rules-revision-w2-p3-2026-10-01.md`; квитанция iteration 2
  `accepted`. **Грабли:** голый dot-путь `.opencode/...` под `--` отклонён —
  `./`-форма работает; 4-путевой `git diff` под `--` отклонён — диффы по одному
  пути; `node -e` с кириллицей/`«»` в строке в shell — синтаксическая ошибка
  (использовать `edit` для append-записей). **Урок:** для быстрых docs/canon-правок
  порог достаточен: per-wave отчёт + `agents-perms` ×2 + адресные диффы; полный
  DoD (`cargo`) не нужен при неизменных `src/tests/Cargo.toml` (D50).
- **02.10.2026 · T-21 (приёмка, класс S) — чекпойнт ДО прогона:** база-ветка
  `feature/T-21-mcp-test-struct-api` от `develop` @ `5786875` (коммитов нет);
  дерево смешанное (пути T-18 некоммитнуты + `src/mcp.rs` T-21). Что проверяю:
  (1) `git status -sb`/`git diff --stat -- tests Cargo.toml`; (2) `src/mcp.rs` —
  struct-API (`err.code`/`err.message`), `to_json`, `required_str` в `create`;
  (3) поведение T-04/Q29 (прежде `''` принималось) + `create_requires_name_param`;
  (4) полный DoD: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`,
  `cargo test --all` — тяжёлый, отчёт в ленту ДО; (5) точный остаток ошибок
  `tests/**` (файл+код) при красном. Отчёт `docs/reviews/T-21-2026-10-02.md`,
  квитанция append. Итог — после прогона.
  **Итог (после):** **rework (iteration 1)** — красный DoD (review.md:55), но
  дифф T-21 **корректен и в границах**; правок `src/mcp.rs` не требуется.
  Ветка `feature/T-21-mcp-test-struct-api` @ `5786875` (= `develop` =
  `origin/develop`, коммитов нет). `git diff --numstat -- src/mcp.rs` = `12/18`;
  `git status --porcelain -- src Cargo.toml` = только `M src/mcp.rs`;
  `git diff --stat -- tests Cargo.toml` и `git diff HEAD -- tests` пусто;
  `Cargo.toml`/`Cargo.lock` не тронуты. `required_str` (:158–159, :431–445) —
  контракт T-04/Q29 сохранён (код `validation_failed` во всех ветках; отказ на
  `''` — намерение Q28/D31). **`cargo test --lib` → 61/0 (unit-тесты
  `src/mcp.rs` зелёные)**; `cargo clippy --lib -- -D warnings` → ok.
  **Красное — только `tests/**`, вне scope T-21: 5 ошибок** —
  `tests/mcp_draft.rs:543:41` E0425 + `:603/624/647:9` E0061 (tracked, `git diff
  HEAD -- tests` пусто ⇒ предсуществующие, база `develop` @ `5786875`, последний
  коммит файла `9173fc4`; файл внутренне противоречив: часть вызовов
  `mcp.create(SRC)` 1-arg против `common/mod.rs:148` 2-arg) Первая причина
  E0425: `use serde_json::json;` без `Value`. `tests/docs_journal.rs:168:5`
  dead_code `d` — файл T-18 (untracked, шапка :8,:25). Отчёт
  `docs/reviews/T-21-2026-10-02.md`; квитанция `rework`. **Урок:** при смешанном
  дереве (T-18+T-21) красный `tests/**` атрибутируется по `git diff HEAD --
  tests` (пусто ⇒ предсуществующее) + шапке untracked-файла (T-18); вердикт по
  сквозному DoD — rework с P2 «вне scope», не правка чужого `src/**`.
- **02.10.2026 · T-21 (-r2, повторная приёмка) — чекпойнт ДО прогона:** база
  `5786875` (= `develop` = `origin/develop`); все ветки (T-18/T-21/T-22) на этом
  коммите, коммитов нет. Внимание: текущая рабочая ветка —
  `feature/T-22-mcp-draft-test-fix` (создана под T-22), `feature/T-21-...`
  содержится в HEAD; дифф T-21 (`src/mcp.rs`) — тот же. Что проверяю:
  (1) `git diff -- src/mcp.rs` = `12/18`, байт-в-байт как в -r1 (не изменён);
  (2) `git diff -- tests/mcp_draft.rs` = T-22-фикс (`Value`-импорт, 3×1-arg
  `mcp.create(SRC)` → `create(NAME, SRC)`, rustfmt); тесты не ослаблены;
  (3) `tests/docs_journal.rs` (T-18) не тронут; (4) полный DoD:
  `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`,
  `cargo test --all`. Отчёт `docs/reviews/T-21-2026-10-02-r2.md`, квитанция
  append. Итог — после прогона.
  **Итог (после):** **rework (iteration 1 -r2)** — красный DoD, **единственный
  остаток `tests/docs_journal.rs:168:5` dead_code `d`** (файл T-18, untracked,
  шапка :8,:25), вне scope T-21. Ошибки `tests/mcp_draft.rs` из -r1
  (E0425 + E0061×3) **устранены T-22-фиксом** (`git diff -- tests/mcp_draft.rs`
  = `12/8`: `use serde_json::{Value, json}`, 3×`create(NAME,SRC)`, rustfmt;
  `cargo test --test mcp_draft` 25/0). Дифф T-21 `src/mcp.rs` = `12/18`,
  **байт-в-байт как -r1**; `cargo fmt --check` exit 0; `test --lib` 61/0;
  `publish` 12/0; `rest` 11/0; `features_inventory` 4/0. Поведение T-04/Q29
  сохранено. Отчёт `docs/reviews/T-21-2026-10-02-r2.md`; квитанция `rework`.
  **Грабли/факт:** текущая рабочая ветка была `feature/T-22-...` (не T-21) —
  все ветки на `5786875`, дерево общее, дифф T-21 идентичен; вердикт по
  сквозному DoD снова `rework` (T-21 нельзя принять, пока T-18 красный), фикс
  чужой зоны — следующим шагом очереди (T-18). **Урок:** «-r2 после чужого
  фикса» ≠ автоматический accepted: перепроверять ВЕСЬ остаток `tests/**`
  (одна чужая ошибка = rework + точная атрибуция).
- **02.10.2026 · T-21 (-r3, повторная приёмка) — чекпойнт ДО прогона:** база
  `5786875` (= `develop` = `origin/develop`); рабочие ветки T-18/T-21/T-22 на этом
  коммите, текущая — `feature/T-22-mcp-draft-test-fix`; дерево содержит все фиксы
  (T-18/T-21/T-22 некоммитнуты). Что проверяю: (1) `git diff -- src/mcp.rs` =
  `12/18`, байт-в-байт как -r1/-r2 (blob `43f752e..f560749`); (2) T-18-фикс: в
  `tests/docs_journal.rs` `TraceRow` +`lifecycle/tasks/realization`, `row.d`
  читается (:549) — dead_code снят; (3) полный DoD: `cargo fmt --check`,
  `cargo clippy --all-targets -- -D warnings`, `cargo test --all`. Отчёт
  `docs/reviews/T-21-2026-10-02-r3.md`, квитанция append. Итог — после прогона.
  **Итог (после):** **принято (iteration 1 accepted)**, P1/P2/P3 нет.
  `cargo fmt --check` exit 0; `cargo clippy --all-targets -- -D warnings` exit 0;
  `cargo test --all` — все цели ok, **135 passed/0 failed** (lib 61/0,
  docs_journal 14/0, features_inventory 4/0, mcp_draft 25/0, mcp_errors 8/0,
  publish 12/0, rest 11/0). Дифф T-21 `src/mcp.rs` = `12/18`, **байт-в-байт как
  -r1/-r2** (blob `43f752e..f560749`); поведение T-04/Q29 сохранено. Отчёт
  `docs/reviews/T-21-2026-10-02-r3.md`; квитанция `accepted`. **Грабли (важно):**
  PowerShell при `cargo test --all 2>&1` печатает `NativeCommandError` и exit 1
  из-за записи cargo в stderr — это НЕ провал; проверять результат по
  `test result: ok` или повтором `cargo test --all --quiet` (там чисто).
  **Факт:** серия -r1/-r2/-r3 показала модель приёмки «сквозной DoD»: чужая
  зона (`tests/**`) держала rework, пока T-18/T-22 не закрылись; `src/mcp.rs`
  ни разу не переправлялся.
- **02.10.2026 · service-question-kodaskills (приёмка Q81, документный пакет)** —
  **принято**, P1/P2/P3 нет. База `develop` @ `c2f905f` (= `origin/develop` = `HEAD`).
  Q81 — следующий свободный после Q80 (Q82 нет, `git ls-files docs/questions`);
  форма §4 (поля Статус/Дата/Приоритет/Связано; разделы Контекст/Вопрос/Варианты/
  Рекомендация), `open`; строки `questions/README.md:103` и
  `TRACEABILITY.md:85` согласованы (`Q81 | — | open | — | —`), D = `—`, задач нет.
  Тело Q81 ссылается на `D82`/`Q79` как трактовку `open` — это нормативная ссылка,
  не `Resolves`; §8 «Q без D» снят обоснованием «решение не принято» (§3/D82).
  D65 — сессионных адресов нет; `:\d+` в Q81 нет; `Resolves`/`T-` нет;
  `rg Q81 docs/decisions|tasks|features` пусто. Границы: `src/tests/Cargo.toml`
  не тронуты; docs-диффы ровно +1/+1; `git diff --check` пусто; `git status` —
  целевые + штатные state analyst/lead. `cargo` не запускался (D50). Отчёт
  `docs/reviews/service-question-kodaskills-2026-10-02.md`; квитанция iteration 1
  `accepted`. **Грабли:** `git ls-files docs/questions` показывает только
  tracked-файлы — новый Q81 там отсутствует, это ожидаемо (номер проверяется по
  максимуму tracked + наличие/отсутствие Q82).
- **02.10.2026 · T-18 (приёмка, M) — чекпойнт ДО прогона:** база `5786875`
  (= `develop` = `origin/develop`); дерево смешанное (пути T-18 + T-21 +
  T-22), коммитов нет; рабочая ветка `feature/T-22-...`, но приёмка T-18 —
  по артефактам дерева. Внимание: тест T-18 (`tests/docs_journal.rs`,
  untracked) — первый прогон; `d`-фикс участка №5 присутствует (:511 `match
  row.d`). Проверяю: (1) `git diff --numstat -- src/mcp.rs` = `12/18`,
  `-- tests/mcp_draft.rs` = `12/8` (не переправлены); (2) тест не ослаблен
  (D64/D77/D80/D82); канон вычищен (D85/F56); (3) полный DoD:
  `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`,
  `cargo test --all`. Отчёт `docs/reviews/T-18-2026-10-02.md`; квитанция
  append (T-18). Итог — после прогона.
  **Итог (после):** **rework.** `cargo test --all` exit 1: fmt/clippy exit 0;
  lib 61/0; `tests/docs_journal.rs` 8 passed / **6 failed**. Отдельные цели:
  mcp_draft 25/0, publish 12/0, rest 11/0, features_inventory 4/0 (зелёные).
  **5 из 6 красных — дефекты самого теста T-18** (ложные): (1)
  `field_line` (tests/docs_journal.rs:109-113) не срезает ведущий пробел →
  `starts_with`-гейт `:349-354` падает на Q1 (`resolved by …` с пробелом);
  (2) `cell_task_statuses` (:206-234) не понимает ID `T-XX` (дефис после `T`)
  → пустой набор задач → красные `:581` (T-01/D77) и `:535` (Q1 `in work`);
  (3) `section()` (:146-152) ловит inline `` `## Сверка с кодом` `` в ноте
  D64:83 → секция обрезана → `:472` (D64 без «Вердикт:»); (4) `feature_tokens`/
  `feature_registry` (:237-249/:285-297) не нормализуют префикс `features/` →
  `:613` (`testing.feature`, TRACEABILITY:44 зовёт `features/testing.feature`).
  **1 реальный канон-дефект:** D39 (`:44-45,56-57,61`) адресует
  `.opencode/state/current/*.yaml` — нарушение D65 п.4, вне вычистки T-18 →
  через `migrator`. Тест не ослаблен (dead_code снят усилением :506-523).
  Диффы T-21 (`src/mcp.rs` 12/18) и T-22 (`tests/mcp_draft.rs` 12/8) не
  переправлены. База `5786875` ×3. Отчёт `docs/reviews/T-18-2026-10-02.md`;
  квитанция `rework` записана. **Урок:** первый прогон нового машинного гейта
  проверять на ложные срабатывания разбора (trim полей, `T-` дефис, `find`
  заголовка vs inline, нормализация префиксов) — «тест не ослаблен» ≠ «тест
  корректен»; красный DoD с 5/6 ложными = rework в зоне `tester`, не правка
  канона «под тест».
- **02.10.2026 · T-23 (синхронизация TRACEABILITY, класс L, вариант Б) —
  чекпойнт ДО прогона:** база `develop` @ `71ece40` (= `origin/develop` =
  `HEAD`), ветки нет (прямая правка). Дерево по брифу: `M docs/TRACEABILITY.md`,
  `M docs/analysis/findings-registry.md`, F43-остаток (`M .opencode/mail/T-18.md`,
  `M .opencode/state/current/progress.yaml`), записи ролей/state, `??` лента
  `service-traceability-closeout.md` и досье `T-23-2026-10-02.md`; `src/**`/
  `tests/**`/`Cargo.toml` диффом пусто. Адресно: 4 строки TRACEABILITY
  (Q29:33, Q60:64, Q73:77, Q76:80) против реестра `docs/tasks/README.md` и
  легенды; F57; затем **полный DoD** (`fmt --check`, `clippy --all-targets
  -- -D warnings`, `test --all`) — главное `docs_journal` 14/0 в составе 135/0.
  Отчёт `docs/reviews/T-23-2026-10-02.md`; квитанция append (accepted/rework).
  Итог — после прогона.
  **Итог (после):** **принято**, P1/P2/P3 нет. Полный DoD зелёный:
  `fmt --check` exit 0, `clippy --all-targets -- -D warnings` exit 0,
  `test --all` exit 0 — **135/0** (lib 61/0, `docs_journal` **14/14**,
  features_inventory 4/0, mcp_draft 25/0, mcp_errors 8/0, publish 12/0,
  rest 11/0, doc 0/0); 0 ignored. Ранее красные
  `traceability_tasks_exist_and_match_registry` (:592) и
  `traceability_lifecycle_matches_task_openness` (:564) — зелёные; проверки
  (1)–(8) целы (не ослаблены). 4 строки TRACEABILITY сверены с реестром
  (`T-04 ✅:51, T-05 ⬜:52, T-18 ✅:64, T-21 ✅:67, T-22 ✅:68`): Q29:33
  (T-21/T-22 ✅, `in work` — T-05 ⬜), Q60:64/Q73:77/Q76:80 (T-18 ✅, `done`).
  F57 :69 — ID уникален, связи Q60/D64, Q73/D77, Q76/D80, Q29/D34. Границы:
  `git diff --stat -- src tests Cargo.toml` пусто, канон агентов/rules пусто,
  `git diff --check` пусто. База `develop` = `origin/develop` = `HEAD` =
  `71ece40`; ветки нет (вариант Б; `progress.yaml:798` — «Вариант Б: прямая
  правка»). Квитанция `accepted` (iteration 1) записана.
  **Урок:** при прямой правке канона «вариант Б» ветку/базу подтверждать
  `git rev-parse HEAD develop origin/develop` ×3 (все равны) — отсутствие
  ветки не означает отсутствие базы; адресную приёмку строить таблицей
  «строка ↔ реестр ↔ легенда» с номерами строк.
  **Итог (после):** **принято**, P1/P2/P3 нет. Полный DoD зелёный:
  `fmt --check` exit 0; `clippy --all-targets -- -D warnings` exit 0;
  `test --all` exit 0 — lib 61/0, **docs_journal 14/14**, features_inventory
  4/0, mcp_draft 25/0, mcp_errors 8/0, publish 12/0, rest 11/0, doc 0/0.
  Четыре P1-дефекта разбора закрыты (правки в парсерах, проверки целы,
  42 `assert`); канон-дефект D39 снят (D39 = 4/4, зона `.opencode/state/**`).
  Тест не ослаблен (dead_code снят усилением :506-523). Границы: `src/mcp.rs`
  12/18 (= -r1, байт-в-байт), `tests/mcp_draft.rs` 12/8 (= -r1);
  `Cargo.toml`/`Cargo.lock` не тронуты. База `5786875` ×3. Отчёт
  `docs/reviews/T-18-2026-10-02-r2.md`; квитанция `accepted` (iteration 1)
  записана. **Урок:** повторная приёмка нового гейта подтверждает не только
  «зелёный DoD», но и неослабленность — сверять, что правки локализованы в
  разборе, а число/тела проверок сохранены; при accepted фиксировать
  неизменность чужих диффов (T-21/T-22).
- **02.10.2026 · T-23 (синхронизация TRACEABILITY, класс L, вариант Б) —
  чекпойнт ДО прогона:** база `develop` @ `71ece40` (= `origin/develop` =
  `HEAD`), ветки нет (прямая правка). Дерево по брифу: `M docs/TRACEABILITY.md`,
  `M docs/analysis/findings-registry.md`, F43-остаток (`M .opencode/mail/T-18.md`,
  `M .opencode/state/current/progress.yaml`), записи ролей/state, `??` лента
  `service-traceability-closeout.md` и досье `T-23-2026-10-02.md`; `src/**`/
  `tests/**`/`Cargo.toml` диффом пусто. Адресно: 4 строки TRACEABILITY
  (Q29:33, Q60:64, Q73:77, Q76:80) против реестра `docs/tasks/README.md` и
  легенды; F57; затем **полный DoD** (`fmt --check`, `clippy --all-targets
  -- -D warnings`, `test --all`) — главное `docs_journal` 14/0 в составе 135/0.
  Отчёт `docs/reviews/T-23-2026-10-02.md`; квитанция append (accepted/rework).
  Итог — после прогона.
  **Итог (после):** **принято**, P1/P2/P3 нет. Полный DoD зелёный:
  `fmt --check` exit 0, `clippy --all-targets -- -D warnings` exit 0,
  `test --all` exit 0 — **135/0** (lib 61/0, `docs_journal` **14/14**,
  features_inventory 4/0, mcp_draft 25/0, mcp_errors 8/0, publish 12/0,
  rest 11/0, doc 0/0); 0 ignored. Ранее красные
  `traceability_tasks_exist_and_match_registry` (:592) и
  `traceability_lifecycle_matches_task_openness` (:564) — зелёные; проверки
  (1)–(8) целы (не ослаблены). 4 строки TRACEABILITY сверены с реестром
  (`T-04 ✅:51, T-05 ⬜:52, T-18 ✅:64, T-21 ✅:67, T-22 ✅:68`): Q29:33
  (T-21/T-22 ✅, `in work` — T-05 ⬜), Q60:64/Q73:77/Q76:80 (T-18 ✅, `done`).
  F57 :69 — ID уникален, связи Q60/D64, Q73/D77, Q76/D80, Q29/D34. Границы:
  `git diff --stat -- src tests Cargo.toml` пусто, канон агентов/rules пусто,
  `git diff --check` пусто. База `develop` = `origin/develop` = `HEAD` =
  `71ece40`; ветки нет (вариант Б; `progress.yaml:798` — «Вариант Б: прямая
  правка»). Квитанция `accepted` (iteration 1) записана.
- **02.10.2026 · service-t15-run-review (приёмка, L-пакет документов) —
  чекпойнт ДО прогона:** база `develop` = `origin/develop` = `HEAD` = `98f7225`
  (ветки нет). Адресно: TRACEABILITY 3 ячейки (:54/:55/:78) T-15 ⬜→🚧 = реестр
  🚧; карточка T-15 🚧, F26/F27/F15 «⏸ · данные 02.10» (чекбоксы не менялись),
  6 отчётов; F58–F61 после F57, F58 +`validator` 1×36; границы чисты
  (`src/tests/Cargo.toml` пусто, канон агентов/rules/AGENTS/opencode.json пусто,
  `git diff --check` пусто). Запускаю полный DoD (135/0, docs_journal 14/14).
  **Итог (после):** **принято**, P1/P2/P3 нет. Полный DoD зелёный: `fmt --check`
  exit 0, `clippy --all-targets -- -D warnings` exit 0, `test --all` exit 0 —
  **135/0** (lib 61/0, `docs_journal` **14/14**, features_inventory 4/0,
  mcp_draft 25/0, mcp_errors 8/0, publish 12/0, rest 11/0, doc 0/0); 0 ignored.
  TRACEABILITY ровно 3 ячейки (:54/:55/:78) ⬜→🚧 = реестр = карточка; `in work`
  законен; F58–F61 после F57, F58 +`validator` 1×36; старые строки целы; границы
  (src/tests/Cargo.toml, канон, check) чисты. Отчёт
  `docs/reviews/service-t15-run-review-2026-10-02.md`; квитанция `accepted`
  (iteration 1) записана. **Урок:** при ревизии прогона целевой пакет может
  включать F43-остаток чужих сервисных записей (`service-traceability-closeout.md`,
  `progress.yaml`) — фиксировать их как «не мои» в отчёте, но не исключать из
  пакета, если так решено владельцем (лента :22–24).
  **Урок:** при прямой правке канона «вариант Б» ветку/базу подтверждать
  `git rev-parse HEAD develop origin/develop` ×3 (все равны) — отсутствие
  ветки не означает отсутствие базы; адресную приёмку строить таблицей
  «строка ↔ реестр ↔ легенда» с номерами строк.
