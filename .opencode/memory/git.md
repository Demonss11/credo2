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
- Чекпойнтов ещё не было (до 28.09.2026).
