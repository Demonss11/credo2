# Память: docs-writer (документация и статусы)

- **Канон:** `docs/tasks/README.md`, `docs/features/README.md`,
  `.opencode/rules/journal.md` §5.6/§7.
- **Правило:** чекпойнт — какие файлы/статусы менялись, что осталось, ссылки.
  Кратко.

## Чекпойнты

- 30.09.2026, `service-docs-lifecycle` №1: `docs/README.md:25–27` — формулировка
  «рабочие артефакты» (`analysis/reviews/research`) по D65; `git grep "на них
  ссылаются" -- docs/README.md` пусто; затронут ровно этот блок.
- 02.10.2026, `closeout_t21_t18_t22` (участок №9, T-21): закрытие трёх задач после
  принятых приёмок (T-21 -r3, T-18 -r2; T-22 подтверждён в -r3).
  Правки: карточки `docs/tasks/T-21-mcp-test-struct-api/README.md`,
  `T-18-docs-journal-test/README.md`, `T-22-mcp-draft-test-fix/README.md`
  (⬜/🚧 → ✅); строки T-18/T-21/T-22 в `docs/tasks/README.md` → ✅;
  `docs/CHANGELOG.md` — три записи (T-21/T-18/T-22) со ссылками на
  `docs/reviews/T-21-2026-10-02-r3.md` и `docs/reviews/T-18-2026-10-02-r2.md`
  (T-22 подтверждён в -r3). Порядок T-21→T-18→T-22. `cargo` не запускался (D50).
  Других файлов не трогал. Осталось: гейт пакета (owner-gate) → `git` (три
  пакета F43).
