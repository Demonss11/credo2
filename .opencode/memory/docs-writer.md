# Память: docs-writer (документация и статусы)

- **Канон:** `docs/tasks/README.md`, `docs/features/README.md`,
  `.opencode/rules/journal.md` §5.6/§7.
- **Правило:** чекпойнт — какие файлы/статусы менялись, что осталось, ссылки.
  Кратко.

## Чекпойнты

- 02.10.2026, T-24 (закрытие после accepted -r2): статусы ✅ в карточке
  (`docs/tasks/T-24-rest-cli-contour-test/README.md`) и сводке
  (`docs/tasks/README.md`); `features/`/CHANGELOG не трогались (D72). Маркер
  закрытия — `✅ сделана` в карточке, `✅` в последней ячейке строки реестра.
- 02.10.2026, T-16 (взятие в работу): ⬜ → 🚧 в карточке
  (`docs/tasks/T-16-stale-check-test/README.md:3`, «🚧 в работе») и сводке
  (`docs/tasks/README.md:57`); прочие статусы не тронуты. Закрытие (✅,
  `features/test_draft.feature`, CHANGELOG) — после приёмки `validator`.
- 02.10.2026, T-16 (closeout, приёмка accepted -r3): 🚧 → ✅ в карточке
  (`docs/tasks/T-16-stale-check-test/README.md:3`) и сводке
  (`docs/tasks/README.md:57`); CHANGELOG `docs/CHANGELOG.md:9-16` — запись T-16
  (D72, ссылка на `docs/reviews/T-16-2026-10-02-r3.md`). `features/` **не
  тронут**: `test_draft.feature` остаётся 🟡 (6 сценариев; T-02 ⬜). TRACEABILITY
  (migrator) и T-25 не трогались. `git diff --numstat -- docs/features` пусто.
- 02.10.2026, T-25 (closeout, адресная приёмка accepted, iteration 1): ⬜→✅ в
  карточке (`docs/tasks/T-25-d65-analysis-addresses/README.md`:3, «✅ сделана»)
  и сводке (`docs/tasks/README.md`:58); `docs/CHANGELOG.md`:9-14 — запись T-25
  (D72, ссылка на `docs/reviews/T-25-2026-10-02.md`). Строка :57 (T-16 ✅) — не
  тронута (пакет T-16); `features/` и `TRACEABILITY` (migrator) не трогались.
  Риск: CHANGELOG несёт правки двух пакетов (T-25 :9-14 / T-16 :15-22) —
  hunk-разбор на гейте T-25.
