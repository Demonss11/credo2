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
  **Право `git rev-parse`** (D49, «Обновление» 30.09.2026): добавлено в allowlist —
  точная сверка базы: `git rev-parse develop origin/develop HEAD`; прежний обход
  (`git log --oneline -1` / `git show -s`) — резервный.
- **30.09.2026 · service-docs-lifecycle №2 (`канон агентов`)** — **принято**,
  P1/P2/P3 нет; оба замечания аудита закрыты (P2 — §8 `journal.md` оговорка
  реестра `findings-registry.md`/D48; P3 — две точки `Q54.md:12,15`). База
  `develop` @ `21c3c80` (= `origin/develop` = `HEAD`) + рабочее дерево. Границы
  чисты (`src/tests/Cargo.toml`, `AGENTS.md` пусто). Права: `agents-perms.mjs` ×2
  → `11 из 18`, `validator` ↔ `review.md` совпадают (`git branch --contains`,
  `git rev-parse`). Отчёт `docs/reviews/service-docs-lifecycle-canon-2026-09-30.md`;
  квитанция записана. `cargo` не запускался (D50).
  **Грабли:** `git diff` с несколькими dot-путями под `--` и `--numstat -- .opencode`
  движок отклоняет — одиночный `./`-путь или `git diff --numstat` без пути.
  `receipts.yaml` крупный (`grep -n "^- task:"` → 64 записи), хвост читать по
  `offset`.
