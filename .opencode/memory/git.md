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
- 29.09.2026 · пакет `service-migration-q5q6` (прямо в `develop`, подтверждение
  владельца в `.opencode/mail/service-migration-q5q6.md` §«пакет подтверждён
  (коммит + push)» — дословно «Коммит + push»). Снимок до: 11 `M` + 5 `??` =
  16 путей, совпал; 17-й — чекпойнт `git.md` (этот). База HEAD `788aa4c`,
  `develop`, синхрон с `origin/develop`; коммита с целевым сообщением нет.
  17 путей: письмо `mail/service-migration-q5q6.md`,
  `memory/{docs-writer,git,migrator,service,validator}.md`,
  `state/current/receipts.yaml`, `docs/CHANGELOG.md`, `docs/OPEN_QUESTIONS.md`,
  `docs/SPECIFICATION.md`, `docs/TRACEABILITY.md`,
  `docs/decisions/D19-statuses-priorities-canon.md`, `docs/features/README.md`,
  `docs/questions/{Q5,Q6,README}.md`,
  `docs/reviews/migration-q5q6-2026-09-29.md`. Сообщение —
  `docs(D19): перенос Q5, Q6 — канон статусов и приоритетов`. Осталось: `add`
  17 → сверка staged (ровно 17) → коммит → проверки → `push origin develop`
  (таймаут ≥ 5 мин; публикует `788aa4c` и новый коммит, ahead 2) → `status -sb`
  (синхрон) → `log -3`. Хеш — не здесь (F43): вернуть `lead` ответом. Веток/
  merge/тегов нет.
- 29.09.2026 · пакет `service-dod-scope` (прямо в `develop`, подтверждение
  владельца в `.opencode/mail/service-dod-scope.md` §«пакет подтверждён
  (сужение: только коммит)» — дословно «Только коммит»; `push` отменён). Снимок
  до: фактически **12 `M` + 4 `??` = 16 путей**, а не 13 M + 4 ?? = 17, как в
  пакете (расхождение на 1); состав совпал с перечнем. 17-й — чекпойнт `git.md`
  (этот) → итого **17: 13 M + 4 ??**. База HEAD `0ee25f5`, `develop`, синхрон с
  `origin/develop`; коммита с целевым сообщением нет. 17 путей: письмо
  `mail/service-dod-scope.md`,
  `memory/{auditor,docs-writer,git,migrator,service,validator}.md`,
  `agents/validator.md`, `rules/review.md`, `state/current/receipts.yaml`,
  `docs/{BRIEF,SPECIFICATION,TRACEABILITY}.md`,
  `docs/decisions/D50-dod-by-package-scope.md`, `docs/questions/{Q55,README}.md`,
  `docs/reviews/service-dod-scope-2026-09-29.md`. Сообщение —
  `chore(agents): Q55/D50 — cargo-прогоны по составу пакета`. Осталось: `add`
  17 → сверка staged (ровно 17) → коммит локально, **без `push`**. Хеш — не
  здесь (F43): вернуть ответом. Веток/merge/тегов нет.
- 29.09.2026 · пакет `service-agent-tools` (прямо в `develop`, подтверждение
  владельца в `.opencode/mail/service-agent-tools.md` §«пакет подтверждён
  (коммит + push)» — дословно «Коммит + push»). Снимок до: 16 `M` + 5 `??` =
  21 путь, совпал; 22-й — чекпойнт `git.md` (этот). База HEAD `01a70fd`,
  `develop`, ahead 1; коммита с целевым сообщением нет. 22 пути: `AGENTS.md`,
  `agents/{auditor,validator}.md`, `rules/{review,workspace}.md`,
  `scripts/agents-perms.mjs`, письмо `mail/service-agent-tools.md`,
  `memory/{auditor,docs-writer,git,migrator,service,validator}.md`,
  `state/current/receipts.yaml`, `docs/{SPECIFICATION,TRACEABILITY}.md`,
  `docs/decisions/D51-agent-tools-token-hygiene.md`,
  `docs/features/{agents-audit,agents-cycle}.feature`,
  `docs/questions/{Q56,README}.md`,
  `docs/reviews/service-agent-tools-2026-09-29.md`. Сообщение —
  `chore(agents): Q56/D51 — agents-perms.mjs вместо сырого debug agents; rg -c для строк`.
  Осталось: `add` 22 → сверка staged (ровно 22) → коммит → проверки →
  `push origin develop` (таймаут ≥ 5 мин; публикует `01a70fd` и новый коммит) →
  `status -sb` → `log -3`. Хеш — не здесь (F43): вернуть `lead` ответом.
  Веток/merge/тегов нет.
- 29.09.2026 · пакет `service-migration-q7` (прямо в `develop`, подтверждение
  владельца в `.opencode/mail/service-migration-q7.md` §«пакет подтверждён
  (сужение: только коммит)» — дословно «Только коммит»; `push` отменён). Снимок
  до: 11 `M` + 4 `??` = 15 путей, совпал; 16-й — чекпойнт `git.md` (этот).
  База HEAD `5883a17`, `develop`, синхрон с `origin/develop`; коммита с целевым
  сообщением нет. 16 путей: письмо `mail/service-migration-q7.md`,
  `memory/{docs-writer,git,migrator,service,validator}.md`,
  `state/current/receipts.yaml`, `docs/CHANGELOG.md`, `docs/OPEN_QUESTIONS.md`,
  `docs/SPECIFICATION.md`, `docs/TRACEABILITY.md`,
  `docs/decisions/D52-glossary-terms-canon.md`, `docs/features/publish.feature`,
  `docs/questions/Q7.md`, `docs/questions/README.md`,
  `docs/reviews/migration-q7-2026-09-29.md`. Сообщение —
  `docs(D52): перенос Q7 — терминологический канон §11`. Осталось: `add` 16 →
  сверка staged (ровно 16) → коммит локально, **без `push`**. Хеш — не здесь
  (F43): вернуть ответом. Веток/merge/тегов нет.
- 29.09.2026 · пакет `service-migration-q8q10q42` (прямо в `develop`,
  подтверждение владельца в `.opencode/mail/service-migration-q8q10q42.md`
  §«пакет подтверждён (сужение: только коммит)» — дословно «Только коммит»;
  `push` отменён). Снимок до: 15 `M` + 8 `??` = 23 пути, совпал; 24-й —
  чекпойнт `git.md` (этот). База HEAD `8f0c9d9`, `develop`, ahead 1; коммита с
  целевым сообщением нет. 24 пути: письмо `mail/service-migration-q8q10q42.md`,
  `memory/{docs-writer,git,migrator,service,validator}.md`,
  `state/current/receipts.yaml`, `docs/CHANGELOG.md`, `docs/OPEN_QUESTIONS.md`,
  `docs/SPECIFICATION.md`, `docs/TRACEABILITY.md`,
  `docs/decisions/D21-core-semantics-v01.md`,
  `docs/features/{errors,execution,explain,explain_full,test_draft}.feature`,
  `docs/questions/{Q8,Q9,Q10,Q42,README}.md`,
  `docs/reviews/migration-q8q10q42-2026-09-29.md`,
  `docs/reviews/migration-q8q10q42-2026-09-29-r2.md`. Сообщение —
  `docs(D21): перенос Q8–Q10, Q42 — семантика ядра v0.1`. Осталось: `add` 24 →
  сверка staged (ровно 24) → коммит локально, **без `push`**. Хеш — не здесь
  (F43): вернуть ответом. Веток/merge/тегов нет.
- 29.09.2026 · пакет `service-migration-q11` (прямо в `develop`, подтверждение
  владельца в `.opencode/mail/service-migration-q11.md` §«пакет подтверждён
  (сужение: только коммит)» — дословно «Только коммит»; `push` отменён). Снимок
  до: 16 `M` + 4 `??` = 20 путей, совпал; 21-й — чекпойнт `git.md` (этот). База
  HEAD `c9193da`, `develop`, ahead 2; коммита с целевым сообщением нет. 21 путь:
  письмо `mail/service-migration-q11.md`,
  `memory/{docs-writer,git,migrator,service,validator}.md`,
  `state/current/receipts.yaml`, `docs/CHANGELOG.md`, `docs/OPEN_QUESTIONS.md`,
  `docs/SPECIFICATION.md`, `docs/TRACEABILITY.md`,
  `docs/decisions/D53-error-messages-language.md`,
  `docs/features/{errors,evaluate,rest_api,rest_auth}.feature`,
  `docs/questions/{Q8,Q11,Q42,README}.md`,
  `docs/reviews/migration-q11-2026-09-29.md`. Сообщение —
  `docs(D53): перенос Q11 — язык сообщений об ошибках`. Осталось: `add` 21 →
  сверка staged (ровно 21) → коммит локально, **без `push`**. Хеш — не здесь
  (F43): вернуть ответом. Веток/merge/тегов нет.
- 29.09.2026 · пакет `service-migration-q11-microfix-d53` (прямо в `develop`,
  решение владельца 29.09.2026 дословно «давай сделаем микроправку D53 и делаем
  push в develop»). Снимок до: 5 `M` (`mail/service-migration-q11.md`,
  `memory/{migrator,service,validator}.md`,
  `docs/decisions/D53-error-messages-language.md`) + 1 `M` `memory/git.md`
  (этот) = 6, совпал. База HEAD `746f985`, `develop`, ahead 3 (`8f0c9d9`,
  `c9193da`, `746f985`); `origin/develop` = `5883a17`; `pull` не выполнять;
  коммита с целевым сообщением нет. Пакет (6): `mail/service-migration-q11.md`,
  `memory/{git,migrator,service,validator}.md`,
  `docs/decisions/D53-error-messages-language.md`. Сообщение —
  `docs(D53): уточнение «Следствий» — errors.feature в списке пометок Q11`;
  затем `push origin develop` (таймаут ≥ 5 мин; публикует `8f0c9d9`, `c9193da`,
  `746f985` и новый коммит). Осталось: `add` 6 → сверка staged (6) → коммит →
  проверки → `push`. Хеш — не здесь (F43): вернуть `lead` ответом.
  Веток/merge/тегов нет.
