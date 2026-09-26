# Отчёт приёмки — служебный проход «агенты + правила»

**Проверка:** служебный проход: хранимые отчёты приёмки (`docs/reviews/`), новая
роль `researcher`, абсолютный путь внешнего ADR у `docs-writer`; согласование
`AGENTS.md`, `docs/README.md`, `docs/CHANGELOG.md`.
**Версия:** рабочее дерево + HEAD `c824599` (изменения не закоммичены).
**Вердикт:** принято с замечаниями

**P1:** —
**P2:** `.opencode/agents/lead.md:89` — полезный вызов `@researcher` добавлен,
но во frontmatter `lead` (строки 17–23) нет `subagent: researcher allow`
(разрешены только migrator, docs-writer, coder, tester, validator, git).
Последствие: `@researcher` будет отклонён системой прав → маршрут «исследование
→ researcher → lead» (`AGENTS.md:46–47`) оркестратор запустить не сможет.
Правка: добавить `- { action: subagent, resource: "researcher", effect: allow }`.
**P3:** —

**Проверки:**
- `git status --short` → 7 изменённых файлов + 1 untracked (`researcher.md`).
- `git log -1 --oneline` → `c824599 chore: ревизия и усиление рабочей группы агентов`.
- `cargo test --all` → 60/60 passed (33 unit + 0 main + 4 inventory + 12 publish + 11 rest).
- `cargo fmt --check` → красный в `src/mcp.rs:389` (предсуществующее, вне периметра).

**Что проверено и ок:**
- `researcher.md` (новая роль): `edit` только `docs/research/**`; `webfetch`/`websearch`
  allow; `shell` только `rg`; `subagent`/`question`/`external_directory` deny; формат
  обзора содержит «Источники» и «Что я не знаю». Утверждение CHANGELOG
  «webfetch/websearch разрешены только ей» верно — у всех прочих ролей они deny.
- `validator.md`: `deny edit *` → `allow edit docs/reviews/**`; код и журнал остаются
  read-only; имя `docs/reviews/<тип>-<id>-<дата>.md` согласовано с `review.md`
  §«Хранение отчётов» и не противоречит шаблону «Отчёт».
- `docs-writer.md`: абсолютный путь `D:/pyTechNotes/dar/dar7/dar/dar/DECISIONS.md`
  соответствует `../../DECISIONS.md` из корня репо (два уровня вверх) и тексту «при
  согласовании»; отсылка к `docs/reviews/` из карточки не противоречит
  `docs/tasks/README.md`.
- `review.md` §«Хранение отчётов»: имя/автор/формат/«не канон» согласованы с
  промптами `validator` и `docs-writer`.
- `AGENTS.md`: таблица и маршруты соответствуют фактическим 8 ролям (lead, migrator,
  docs-writer, coder, tester, validator, researcher, git = 8 файлов в
  `.opencode/agents/`). Примечание: в брифе заявлено «9 ролей» — опечатка, фактически 8.
- Q41: `reviews/` и `research/` помечены «не канон» (`docs/README.md:22–23,39–40`,
  `review.md:53`) и не дублируют статусы (`review.md:59–60`). Относительные
  markdown-ссылки в затронутых файлах живые; `reviews/`/`research/` — будущие зоны
  записи, упоминаются инлайн-кодом, а не ссылками.

**Не проверено и почему:** существование внешнего `D:/pyTechNotes/dar/dar7/dar/dar/DECISIONS.md`
прямо не читал (у `validator` `external_directory: deny`); путь сверен арифметикой и
со ссылкой `../../DECISIONS.md` в `AGENTS.md` (предсуществующая ссылка, вне периметра).

## Результат

P2 закрыт: `lead` получил `subagent: researcher allow`; маршрут исследования работоспособен. Повторная проверка — вердикт «принято».
