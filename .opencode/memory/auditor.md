# Память: auditor (аудит системы агентов)

- **Канон:** `AGENTS.md` §Рабочая группа агентов; чек-лист — в промпте роли.
- **Правило:** чекпойнт — что аудировано, находки P1–P3, что осталось. Кратко.

## Чекпойнты

- **30.09.2026, волна `service-docs-lifecycle`, операция №1 `research` — аудит:**
  пакет (D65 «Обновление», снятие 18 ссылок в Q62–Q64/D66–D68/T-19, `docs/README.md`,
  память `researcher`, удаление `docs/research/doc-quality-checks-2026-09-29.md`).
  Итог: P1/P2/P3 нет. Память `researcher` не противоречит канону роли (F35);
  границы целы (`src/tests/Cargo.toml`, `.opencode/agents/**`, `.opencode/rules/**`,
  `AGENTS.md`, `docs/features/**` не тронуты). Грабли: лимит 5 «findings» в чек-листе —
  при чистом аудите записываю ещё и статус «Находки: нет» (иначе чекпойнт читается
  как «аудит не выполнен»); `agents-perms.mjs` дал 18/18 через `--all` (двойной
  прогон); `opencode reload` не требовался (агенты/правила не менялись).
- **30.09.2026, волна `service-docs-lifecycle`, операция №2 `reviews`/служебная
  зона — аудит:** пакет (review.md §«Хранение отчётов» + `git rev-parse`;
  journal.md §8 — `research/reviews/analysis` в «время жизни адреса»;
  docs-writer.md «ссылка не ставится»; validator.md строка `git rev-parse *`;
  память validator/git/researcher; D49 «Обновление»). Инструкция ↔ права:
  расхождений нет (`agents-perms.mjs` ×2 → `11 из 18`, у validator — `git
  rev-parse *`). Находки: **P2** — journal.md §8 объявляет `docs/analysis/**`
  антипаттерном безусловно, а D65 п.4 (:50) держит ссылки на analysis
  «допустимыми» + `findings-registry.md` живым реестром (D65:53–55); «Обновление»
  (:88–99) заявляет уточнение п.4, но текст п.4 не тронут — нормы
  сосуществуют. **P3** — Q54.md:12,15 потеряли терминальную пунктуацию при
  снятии ссылок. Грабли: `git diff -- ./.opencode/...` и `git diff -- docs/...`
  с несколькими dot-путями под `--` движок отклоняет (нужен `./`-префикс,
  одиночный путь за вызов); сверка ленты — чтением хвостами `offset`/`limit`
  (полное чтение дало срез ~32 КБ).
- **30.09.2026, волна `service-docs-lifecycle`, операция №2 — allowlist
  `ls-files`/`check-ignore` — аудит:** дельта (.opencode/agents/validator.md
  +`git ls-files *` +`git check-ignore *`; auditor.md +`git check-ignore *`;
  review.md §«Доступные команды» — обе строки; D49 — 8 строк «Обновления»).
  Инструкция ↔ права: расхождений нет (`agents-perms.mjs` ×2 → `11 из 18`;
  у validator `allow git ls-files *`/`check-ignore *`, у auditor — `check-ignore *`).
  Находки: **нет**. Проба: `git check-ignore -v` у auditor работает (exit 1),
  `git ls-files` отклонён (права нет — так и задумано). Границы целы
  (`src/tests/Cargo.toml/AGENTS.md` пусто), `cargo` не запускался (D50).
  Грабли: `git diff -- <дот-пути>` движок отклоняет — только `./`-префикс
  (подтверждено снова); `git ls-files` в аудите недоступен (ожидаемо).
