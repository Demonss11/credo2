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
