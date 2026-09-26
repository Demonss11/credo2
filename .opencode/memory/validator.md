# Память: validator (приёмка)

- **Канон:** `.opencode/rules/review.md`; `docs/BRIEF.md` §5.3/§7.
- **Правило:** чекпойнт **до** прогона тестов (что проверено, что запускаю) и
  **после** (результат, вердикт) — тесты прерывают сессию (R5). Кратко.

## Чекпойнты

- **2026-09-26 · T-11 v2 · ДО прогона** (ветка `exp/agent-cycle-rerun`, база
  `e1b2550`, незакоммиченное дерево).
  - Проверено статически: журнал Q43→D38 (SPEC §10 №38, TRACEABILITY, Источник
    T-11 — все на месте, вердикт ⚪, ID уникальны, ссылки живые); 10 ролей ↔
    `AGENTS.md` ↔ `review.md`/`git-workflow.md` ↔ BRIEF §5.3/§5.7 — согласованы;
    права R2 (cargo test только validator, build ни у кого, tester check+fmt,
    auditor mode:all, steps у всех); 10 файлов памяти + лента в git (не
    игнорируются); 6 фич `agents-*` / 27 сценариев, счётчики 42/250 сходятся,
    обратные ссылки D38 есть.
  - Запускаю: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`,
    `cargo test --all`.
  - Находки-кандидаты (P3): `validator.md:53` «см. „Код“» и `agents-audit.feature:35`
    «Primary-агент» — неточные отсылки, не блокеры.

- **2026-09-26 · T-11 v2 · ПОСЛЕ прогона** — вердикт **принято** (P1/P2 нет; 2×P3).
  - `cargo fmt --check` — чисто; `cargo clippy --all-targets -- -D warnings` — ok;
    `cargo test --all` — 78 passed / 0 failed (incl. `features_inventory` 4/4).
  - Отчёт: `docs/reviews/T-11-2026-09-26.md`; итог — в ленту `.opencode/mail/T-11.md`.
  - P3: `validator.md:53` «см. „Код“» (в AGENTS нет раздела «Код»);
    `agents-audit.feature:35` «Primary-агент» (у auditor `mode: all`).

- **2026-09-26 · T-11 v2 · r2 (повторная)** — вердикт **принято** (P1/P2/P3 нет).
  - Правки после r1: `git.md`/`git-workflow.md` (подтверждение — одно, у `lead`;
    `question` у `git` убран, осталось `ask`), `validator.md` («см. чек-лист
    «Код» ниже»), `agents-audit.feature` («Агент доступен для приёмки»),
    `BRIEF.md §5.6` (`# Dn (Qx): …`, блока `Decisions:` нет).
  - `git status --porcelain -- src tests Cargo.toml Cargo.lock` — пусто; DoD
    полный не запускался (обоснованно: счётчики сценариев не менялись, r1 — 78/78).
  - Отчёт: `docs/reviews/T-11-2026-09-26-r2.md`; итог — в ленту.

## Чекпойнт · T-11 v2.1 · r3 · ДО прогона

- База: HEAD `2901001`; незакоммиченное дерево = 17 путей (13 файлов периметра
  + `mail/T-11.md`, `memory/git.md`). `git status --porcelain -- src tests
  Cargo.toml Cargo.lock` — пусто (код не менялся с r1).
- Машинно (`opencode debug agents`): 17 агентов (10 ролей со `steps`); `cargo
  test *` — только validator (1 вхождение ресурса); `opencode debug agents` —
  validator + auditor; edit у auditor — только своя память; у lead `question`
  есть, у git `question` нет (осталось `ask`).
- Находки-кандидаты (см. отчёт): P1 — схема A не синхронизирована с журналом
  (D38 §6, SPEC §10 №38, Q43: «вносит auditor» vs «сервисная сессия»);
  P3 — CHANGELOG «Primary-агент», agents-rework «полный повторный прогон» для
  канона/прав, agents-memory-mail «периодический чекпойнт K≈steps/3» без
  предписания в каноне/ролях.
- Запускаю (адресно): `cargo fmt --check`, `cargo test --test features_inventory`.
  Полный DoD не гоняю: src/tests не менялись с r1 (78/78).

## Чекпойнт · T-11 v2.1 · r3 · ПОСЛЕ прогона — вердикт **принято**

- P1/P2/P3 закрыты: журнал под схему A (D38 §6, SPEC §10 №38, Q43 — «сервисная
  сессия», пометка «уточнено 2026-09-26: схема A»); порог H5 в
  `agents-rework.feature` — три уровня, как в `review.md`; CHANGELOG «Primary-агент»
  помечен «пересмотрено»; `AGENTS.md` — периодический чекпойнт `K ≈ steps/3` ≤ 8 и
  headless `--auto`.
- `cargo fmt --check` — чисто; `cargo test --test features_inventory` — 4/4
  (42/253, процесс 6/30). Полный DoD не запускался (src/tests не менялись с r1).
- Отчёт: `docs/reviews/T-11-2026-09-26-r3.md`; итог — в ленту (append).
