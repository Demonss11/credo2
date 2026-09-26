# Память: validator (приёмка)

- **Канон:** `.opencode/rules/review.md`; `docs/BRIEF.md` §5.3/§7.
- **Правило:** чекпойнт **до** прогона тестов (что проверено, что запускаю) и
  **после** (результат, вердикт) — тесты прерывают сессию (R5). Кратко.

## Чекпойнты

- **2026-09-26 · service-git-flow.** Проверено читаньем: HEAD `5b6f978`;
  `git status --porcelain -- src tests Cargo.toml Cargo.lock` пусто; изменены
  ровно периметр (10 файлов M + новая лента). Модели всех 10 ролей —
  `opencode-go/deepseek-v4.1-flash` (`rg -n "^model:"`); `git branch -d *` →
  `ask` (git.md:32); `cargo test *` — только validator.md:19; `opencode debug
  agents` — validator.md:20 и auditor.md:22. README: 42/255; процесс 32
  (cycle 6, rework 3, memory-mail 6, git-approval 5, sized-routes 6, audit 6).
  **Запускаю:** `cargo fmt --check`, `cargo test --test features_inventory`,
  `opencode debug agents`. До результата — без вердикта.
- **2026-09-26 · service-git-flow r3 (итог).** Свёртка §«Ветки задачи» в
  `git.md` до ссылки на `git-workflow.md` — Q41, дублей команд нет, якоря
  `git-workflow.md:45,66` существуют; права `git` без изменений; периметр и
  debug agents без отклонений; код/тесты/фичи не тронуты (DoD не нужен).
  **Вердикт: принято.** Отчёт: `docs/reviews/service-git-flow-2026-09-26-r3.md`.
  Дальше: коммит пакета `git` после подтверждения `lead` (перед коммитом
  системы агентов — свежий аудит).
- **2026-09-26 · service-git-flow r2 (итог).** `opencode debug agents` ×2 —
  права совпадают с YAML, flash (видимые роли), формат идентичен r1;
  `git status --porcelain -- src tests Cargo.toml Cargo.lock` пусто;
  `git log -1 --oneline` → `5b6f978` (тот же HEAD). P2 r1 закрыта
  (`auditor.md:142` flash); маски веток read-only; `git.md:41` question deny;
  счётчики 42/255 (процесс 32). **Вердикт: принято.** Отчёт:
  `docs/reviews/service-git-flow-2026-09-26-r2.md`. Дальше: коммит пакета `git`
  после подтверждения `lead` (перед коммитом системы агентов — свежий аудит).
- **2026-09-26 · service-git-flow r2 (чекпойнт до прогона).** Проверено чтеньем:
  `git status --porcelain -- src tests Cargo.toml Cargo.lock` пусто; периметр =
  роли (auditor/lead/git/rust-expert/validator) + rules/git-workflow + AGENTS +
  features×3 + новая лента + отчёт r1. `auditor.md:142` — теперь flash;
  маски веток read-only (`-l`/`-a`/`--show-current`) у lead и auditor;
  `git.md:32` `git branch -d *` ask, `git.md:41` question deny; `lead.md:88` —
  ссылка на AGENTS.md без пересказа. `rg ^model:` — 10/10 flash. `cargo test *`
  — только validator.md:19. `deepseek-v4-pro` — лишь opencode.json whitelist
  (не роль), историч. reviews/память/лента. **Запускаю:**
  `opencode debug agents`; фичи/README не менялись — features_inventory не гоняю.
  До результата — без вердикта.
- **2026-09-26 · service-git-flow (итог).** `cargo fmt --check` — ok;
  `cargo test --test features_inventory` — 4 passed; `opencode debug agents` ×2 —
  17 агентов, 10/10 flash, `git branch -d *` ask, `cargo test *` только
  validator, `opencode debug agents` — validator+auditor. Вердикт: **отклонено**
  (одна P2): `.opencode/agents/auditor.md:140` держит `deepseek-v4-pro` в
  headless-команде при фронтматтере flash. Отчёт:
  `docs/reviews/service-git-flow-2026-09-26.md`. Дальше: после правки — адресная
  приёмка `-r2`.
