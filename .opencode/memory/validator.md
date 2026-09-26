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
