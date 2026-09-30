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
  **Ловушка роли:** движок прав блокирует `git rev-parse` (нет в allow-списке
  `validator` — `review.md` даёт только `git branch --contains`); для проверки
  базы использовать `git log --oneline -1` / `git show -s`.
