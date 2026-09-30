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
- **30.09.2026 · service-docs-lifecycle №2 (дельта allowlist
  `ls-files`/`check-ignore`)** — **принято**, P1/P2/P3 нет. База `develop` @
  `f28c8cb` (= `origin/develop` = `HEAD`) + рабочее дерево. Границы чисты
  (`src/tests/Cargo.toml/AGENTS.md` пусто); `.opencode/**` — 6 файлов дельты
  (agents/{validator,auditor}, rules/review.md, memory/{migrator,auditor}, лента).
  Права: `agents-perms.mjs` ×2 → `11 из 18`; `validator` +`git ls-files *`/
  +`git check-ignore *` (:31–32), `auditor` +`git check-ignore *` (:25) —
  совпадают с `review.md:89,96`; пробы `git ls-files` (путь) и `git check-ignore -v`
  (exit 1) работают; D49 :50–57 — 8 строк «Обновления №2». Отчёт
  `docs/reviews/service-docs-lifecycle-canon-2-2026-09-30.md`; квитанция
  записана. `cargo` не запускался (D50). **Новые права:** `git ls-files *`,
  `git check-ignore *` — сверка границ пакета / gitignore.
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
  `receipts.yaml` крупный (`grep -n "^- task:"` → 66 записей), хвост читать по
  `offset`.
- **30.09.2026 · service-statuses-review (приёмка)** — **возврат (rework)**,
  P1+P2. Отчёт `docs/reviews/service-statuses-review-2026-09-30.md`; квитанция
  `service-statuses-review` iteration 1, rework. Пакет сам по себе собран верно
  (диффы ±, границы чисты, права, записи append, P3 аудитора закрыт, DSH/D65 —
  ок; `cargo` не запускался, D50). **P1 — ветка/база:** дерево на `master`
  @ `9173fc4` (= `origin/master`), а лента (mail:34,148) называет коммит в
  `develop` @ `3d572f3`; ветки разошлись по тем же файлам (`git diff --stat
  develop HEAD`: migrator.md, validator.md, AGENTS.md, src/mcp.rs, tests/mcp_draft.rs)
  → `git switch develop` с грязными файлами откажет, коммит на текущей ветке
  уйдёт в `master` (git-workflow:26–28), на `develop` приехал бы лишний блок
  «Цикл задачи»; анкер D38 верен только для master. **P2** — validator.md:52–67,
  69–80 дубль «Цикл задачи». **Грабли:** `git branch --show-current` — нет в
  allowlist `validator` (смотреть `git status -sb` / `git branch --contains`);
  `git show -s --format=%ci <hash>` часть вызовов движок отклоняет (891 не прошёл,
  acaf3ec прошёл) — не полагаться; сравнивать ветки — `git diff --stat develop HEAD`
  (без путей). **Урок: перед приёмкой сервисной волны сверять ветку/базу
  (`git status -sb` + `git rev-parse HEAD develop`) с планом — расхождение
  master/develop ломает гейт.**
- **30.09.2026 · service-statuses-review (повторная приёмка, r2)** — **принято**,
  P1/P2 закрыты, P1/P2/P3 нет. База — `master` @ `9173fc4` (= `origin/master`) +
  рабочее дерево; `develop` @ `3d572f3` (= `origin/develop`). Отчёт
  `docs/reviews/service-statuses-review-2026-09-30-r2.md`; квитанция iteration 2
  `accepted`. **Итог:** `git diff develop -- ./.opencode/agents/validator.md` →
  только §10-ханк (блок-дубль на `develop` не приедет); `rg '^## Цикл задачи'` →
  :52; validator.md `+1/−15`; migrator.md `+2/−7`; D38 `+7` (оба дубля), D70 `+8`;
  границы `src/tests/Cargo.toml/AGENTS.md` пусты; `agents-perms` ×2 → `11 из 18`;
  память append (F35); P1 закрыт решением владельца (база `master`, коммит в
  `master` по гейту — исключение из git-workflow:26–28, санкция только в ленте).
  **Урок:** при повторной приёмке канона доказывать закрытие через
  `git diff <новая-база> -- <файл>` — `develop`-дифф validator.md был решающим.
  `cargo` не запускался (D50).
