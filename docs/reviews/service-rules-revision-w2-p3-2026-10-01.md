# Приёмка: service-rules-revision-w2 — закрытие P3 (быстрая правка)

**Проверка:** быстрая правка P3 волны 2 канона агентов — добавление служебного
глагола «см.» в ссылку на `R2` в 4 карточках `.opencode/agents/**` (по
инструкции владельца «сделай быструю правку»). Тип — канон (`.opencode/**`);
порог существенности — новый отчёт + машинная сверка прав
(`review.md` §«Порог существенности»).

**Версия:** `develop` @ `c8ffb0f` (= `origin/develop` = `HEAD`) + рабочее дерево
(2026-10-01). База — коммит волны 2 `c8ffb0f` «docs(Q80/D84): волна 2 канона
агентов — якорь R5, чистка agents/** (service-rules-revision-w2)».

**Вердикт:** принято

**P1:** —
**P2:** —
**P3:** —

## Проверки

- `git diff -- ./.opencode/agents/{coder,tester,rust-expert,validator}.md` —
  каждый файл **ровно один ханк `+1/−1`**: в ссылке `(R2 — `.opencode/rules/
  dispatch-loop.md` §«Hard rules»)` вставлено «см.» → `(R2 — см. …)`.
  Иных правок нет. Строки: `coder.md:54`, `tester.md:51`,
  `rust-expert.md:62`, `validator.md:44` (четвёртая — тот же дефект, вне
  P3 аудитора, закрыта в этой же правке).
- `git diff --numstat` → `1 1` для каждой из 4 карточек; пятая запись —
  `13 0 .opencode/mail/service-rules-revision-w2.md` (append ленты, не правка
  канона).
- `rg -n "\(R2 — `\.opencode/rules/dispatch-loop\.md` §«Hard rules»"
  .opencode/agents` → **пусто** (exit 1): ссылок на `R2` без «см.» не осталось.
- `rg -n "\(R2 — " .opencode/agents` → 4 совпадения, все с «см.»:
  `validator.md:44`, `coder.md:54`, `rust-expert.md:62`, `tester.md:51`.
- `node .opencode/scripts/agents-perms.mjs` ×2 → идентично,
  `agents: 11 из 18`; расхождений нет; фронтматтеры не менялись.
- Дом ссылки резолвится: `.opencode/rules/dispatch-loop.md:37` — `## Hard rules`
  (правка `R5` волны 2 внутри §).
- Границы: `git status --porcelain` = 4 `M .opencode/agents/*` +
  `M .opencode/mail/service-rules-revision-w2.md` (лента) — ровно ожидаемый
  состав, посторонних артефактов нет.
- `git diff --stat -- src tests Cargo.toml AGENTS.md` → **пусто**.
- `git diff --check` → пусто (пробельных дефектов нет).
- `cargo` не запускался (D50; `src/**`, `tests/**`, `Cargo.toml` не менялись —
  правка канонная).

## Что проверено и ок

- `.opencode/agents/coder.md`, `tester.md`, `rust-expert.md`, `validator.md` —
  правка минимальна и единообразна (только «см.»), нормы не затронуты.
- Ссылочная целостность `R2` → `.opencode/rules/dispatch-loop.md` §«Hard rules»
  (дом есть, глагол при ссылке — как требует P3 аудитора).
- Права ролей (машинная сверка ×2) — без изменений.
- Границы пакета (канон-файлы agents + лента), отсутствие кодовых изменений.
- Лента `.opencode/mail/service-rules-revision-w2.md` — append-запись
  «P3 закрыт (быстрая правка) — готово» на месте.

## Границы

`src/**`, `tests/**`, `Cargo.toml`, `AGENTS.md`, `.opencode/rules/**` — не
менялись; иных `.opencode/agents/**`, кроме 4 перечисленных карточек, — нет.
