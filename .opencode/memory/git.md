# Память: git (git-операции)

- **Канон:** `.opencode/rules/git-workflow.md`.
- **Правило:** чекпойнт — пакет, выполненные шаги и хеши, что осталось
  (идемпотентность при обрыве). Кратко.

## Чекпойнты

- 28.09.2026 · пакет `c13_records` (прямо в `develop`, подтверждение владельца в
  `.opencode/mail/service-mcp-ready-r3.md`). Снимок до: A(3)+B(26)=29 —
  совпал. Выполнено: A = `722a782`
  `chore(agents): C13 — записи пакета до коммита (F43)` (3 пути, staged=3).
  Осталось: записи до B (этот чекпойнт + отчёт в ленте `r3`) → B (26 путей) →
  `push origin develop` → `log -4`/`status -sb`/`show --stat HEAD`. Хеш B — не
  здесь (F43): вернуть `lead` ответом. Ветки/merge тегов нет.
- 28.09.2026 · пакет `wave0b_records` (прямо в `develop`, подтверждение владельца в
  `.opencode/mail/service-mcp-ready-r3.md`, запись сервисной сессии 28.09.2026).
  Снимок до: 6 ` M` (mail `r3`, `README.md`, `wave0b-plugins.md`, `wave0b-plan.md`,
  `wave0b-probes.md`, `wave0b-report.md`) + `?? .opencode/memory/service.md`; чекпойнт
  `git.md` — в составе пакета. 8 путей:
  `docs/tasks/T-15-mcp-ready-process/{wave0b-plugins,wave0b-plan,wave0b-probes,wave0b-report,README}.md`,
  `mail/service-mcp-ready-r3.md`, `memory/service.md`, `memory/git.md`. Сообщение —
  `chore(process): T-15 wave 0 фазы B (B0) — протоколы проб, вердикты, отчёт`; затем
  `push origin develop`. Хеш — не здесь (F43): возвращается `lead` ответом. Веток/merge/
  тегов нет.
- 28.09.2026 · пакет `wave0b_own_i1` (прямо в `develop`, подтверждение владельца в
  `.opencode/mail/service-mcp-ready-r4.md`, запись сервисной сессии 28.09.2026).
  Снимок до: 2 ` M` (`memory/service.md`, `README.md`) + 3 `??`
  (`mail/service-mcp-ready-r4.md`, `wave0b-own.md`, `wave0b-own-plugin-guide.md`);
  база `cafc4c4`, синхрон с `origin/develop`; чекпойнт `git.md` — в составе пакета.
  6 путей: `docs/tasks/T-15-mcp-ready-process/{wave0b-own,wave0b-own-plugin-guide,README}.md`,
  `mail/service-mcp-ready-r4.md`, `memory/service.md`, `memory/git.md`. Сообщение —
  `chore(process): T-15 B0-own BO-i1 — карточка мини-волны, разведка V2-API, шпаргалка, реестр`;
  затем `push origin develop`. Хеш — не здесь (F43): возвращается `lead` ответом.
  Веток/merge/тегов нет.
- 28.09.2026 · пакет `service-t11-closeout` (прямо в `develop`, подтверждение
  владельца в `.opencode/mail/service-t11-closeout.md` §«пакет подтверждён
  владельцем (сужение)» — «Только коммит, без push»). Снимок до: 13 `M` + 3 `??`
  = 16 путей, совпал; база HEAD `bed3019`, `develop`; чекпойнт `git.md` — в
  составе пакета (17-й). 17 путей: письмо `mail/service-t11-closeout.md`,
  `memory/{docs-writer,migrator,service,validator,git}.md`,
  `state/current/receipts.yaml`, `docs/CHANGELOG.md`,
  `docs/analysis/findings-registry.md`, `docs/decisions/D38-agent-cycle.md`,
  `docs/features/README.md`, `docs/reviews/T-11-closeout-2026-09-28.md`,
  `docs/tasks/README.md`, карточки
  `docs/tasks/{T-11-agent-cycle,T-15-mcp-ready-process,T-16-stale-check-test}/README.md`,
  `rustfmt.toml`. Сообщение — `chore(process): T-11 закрыта (критерий пилота —
  Run 4); T-16 (H7); правка rustfmt.toml`. Осталось: `add` 17 путей → сверка
  staged → коммит локально. **`push` не выполнять** (сужение владельца). Хеш —
  не здесь (F43): вернуть `lead` ответом. Веток/merge/тегов нет.
- 28.09.2026 · пакет `Q54/D49` (прямо в `develop`, подтверждение владельца в
  `.opencode/mail/service-t11-closeout.md` §«пакет Q54/D49 подтверждён (сужение)»,
  дословно «Только коммит» — `push` отменён). Снимок до: 12 ` M` + 3 `??` =
  15 путей, совпал; база HEAD `6d4c840`, `develop`, ahead 1; коммит с целевым
  сообщением отсутствует. 16 путей (чекпойнт `git.md` — 16-й): письмо
  `mail/service-t11-closeout.md`, `agents/validator.md`, `rules/review.md`,
  `memory/{auditor,migrator,service,validator,git}.md`,
  `state/current/receipts.yaml`, `docs/SPECIFICATION.md`,
  `docs/TRACEABILITY.md`, `docs/analysis/findings-registry.md`,
  `docs/questions/README.md`, `docs/decisions/D49-validator-branch-contains.md`,
  `docs/questions/Q54.md`, `docs/reviews/service-permissions-2026-09-28.md`.
  Сообщение — `chore(agents): Q54/D49 — validator: право git branch --contains
  (аудит и приёмка)`. Осталось: `add` 16 → сверка staged → коммит локально,
  **без `push`**. Хеш — не здесь (F43): вернуть ответом. Веток/merge/тегов нет.
- 28.09.2026 · пакет `service-canon-hygiene` (прямо в `develop`, подтверждение
  владельца в `.opencode/mail/service-canon-hygiene.md` §«пакет подтверждён
  (коммит + push)» — «Коммит + push», 28.09.2026). Снимок до: 14 `M` + 2 `??` =
  16 путей, совпал; база HEAD `e1e90d4`, `develop`, ahead 2 (`6d4c840`, `e1e90d4`);
  коммита с целевым сообщением нет (grep по `--all` пусто); чекпойнт `git.md` —
  17-й путь (в составе пакета). 17 путей: `AGENTS.md`,
  `agents/{auditor,coder,rust-expert,tester,validator}.md`,
  `rules/{dispatch-loop,review}.md`, `mail/service-canon-hygiene.md`,
  `memory/{auditor,git,migrator,service,validator}.md`,
  `state/current/receipts.yaml`, `docs/analysis/findings-registry.md`,
  `docs/reviews/service-canon-hygiene-2026-09-28.md`. Сообщение —
  `chore(agents): F45/F46 — deny execute у auditor; дубли cargo-правил сведены
  (Q41)`. Осталось: `add` 17 → сверка staged (17) → коммит → проверки → `push
  origin develop` (таймаут ≥ 5 мин; публикует `6d4c840`, `e1e90d4`, новый
  коммит) → `status -sb` (синхрон) → `log -3`. Хеш — не здесь (F43): вернуть
  `lead` ответом. Веток/merge/тегов нет.
- 29.09.2026 · пакет `service-migration-q2q3` (прямо в `develop`, подтверждение
  владельца в `.opencode/mail/service-migration-q2q3.md` §«пакет подтверждён
  (сужение: только коммит)» — дословно «Только коммит»; `push` отменён). Снимок
  до: 16 `M` + 5 `??` = 21 путь, совпал; 22-й — чекпойнт `git.md` (этот). База
  HEAD `491e153`, `develop`, синхрон с `origin/develop` (`491e153`); коммита с
  целевым сообщением нет. 22 пути: письмо `mail/service-migration-q2q3.md`,
  `memory/{docs-writer,git,migrator,service,validator}.md`,
  `state/current/receipts.yaml`, `docs/CHANGELOG.md`, `docs/OPEN_QUESTIONS.md`,
  `docs/SPECIFICATION.md`, `docs/TRACEABILITY.md`,
  `docs/decisions/D16-dsl-canon-regex-mvp.md`, `docs/features/README.md`,
  `docs/features/{execution,lexer,parser}.feature`, `docs/questions/Q2.md`,
  `docs/questions/Q3.md`, `docs/questions/README.md`,
  `docs/reviews/migration-q2q3-2026-09-28.md`, `docs/tasks/README.md`,
  `docs/tasks/T-14-grammar-message-sync/README.md`. Сообщение —
  `docs(D16): перенос Q2, Q3 — канон языка v0.1 и парсер MVP`. Осталось: `add`
  22 → сверка staged (ровно 22) → коммит локально, **без `push`**. Хеш — не
  здесь (F43): вернуть ответом. Веток/merge/тегов нет.
- 29.09.2026 · пакет `service-migration-q4` (прямо в `develop`, подтверждение
  владельца в `.opencode/mail/service-migration-q4.md` §«пакет подтверждён
  (коммит + push)» — дословно «Коммит + push»). Снимок до: 16 `M` + 4 `??` =
  20 путей, совпал; 21-й — чекпойнт `git.md` (этот). База HEAD `f488085`,
  `develop`, ahead 1; коммита с целевым сообщением нет. 21 путь: письмо
  `mail/service-migration-q4.md`,
  `memory/{docs-writer,git,migrator,service,validator}.md`,
  `state/current/receipts.yaml`, `docs/CHANGELOG.md`, `docs/OPEN_QUESTIONS.md`,
  `docs/SPECIFICATION.md`, `docs/TRACEABILITY.md`,
  `docs/decisions/D17-priority-out-of-mvp.md`, `docs/features/README.md`,
  `docs/features/{client_explanation,editor,execution,lsp,parser}.feature`,
  `docs/questions/Q4.md`, `docs/questions/README.md`,
  `docs/reviews/migration-q4-2026-09-29.md`. Сообщение —
  `docs(D17): перенос Q4 — Приоритет вне MVP`. Осталось: `add` 21 → сверка
  staged (ровно 21) → коммит → проверки → `push origin develop` (таймаут ≥ 5
  мин; публикует `f488085` и новый коммит, ahead 2) → `status -sb` (синхрон) →
  `log -3`. Хеш — не здесь (F43): вернуть `lead` ответом. Веток/merge/тегов нет.
