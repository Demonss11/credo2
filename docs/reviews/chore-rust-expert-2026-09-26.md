# Отчёт приёмки — проход «роль rust-expert»

**Проверка:** новая роль `rust-expert`, разгрузка `coder`/`tester` от skill
`rust-skills`, обязательный шаг идиоматики в маршруте кода.
**Версия:** рабочее дерево + HEAD `c0b301d` (изменения не закоммичены).
**Вердикт:** принято с замечаниями

**P1:** —
**P2:** `AGENTS.md:25` — карта репозитория всё ещё описывает skill как
«skill по Rust для ролей `coder`/`tester`», но в этом проходе у `coder`
(`.opencode/agents/coder.md:26`) и `tester` (`.opencode/agents/tester.md:24`)
`skill rust-skills` — `deny`, а носителем навыка стал `rust-expert`
(`AGENTS.md:39`). Последствие: карта противоречит таблице ролей и фактическим
правам — читающий карту решит, что `coder`/`tester` применяют навык. Правка:
`skill по Rust для роли \`rust-expert\``.
**P3:** `docs/CHANGELOG.md:40-44` — запись озаглавлена «…и разгрузка `coder`»
и упоминает отключение навыка только у `coder`, хотя проход разгружает и
`tester`. Правка (одна строка): «у `coder` и `tester`».

**Проверки:**
- `git status` → изменены 5 файлов (`coder.md`, `lead.md`, `tester.md`,
  `AGENTS.md`, `docs/CHANGELOG.md`) + 1 untracked (`rust-expert.md`).
- `git log -1 --oneline` → `c0b301d chore: отчёты приёмки, роль researcher и внешний ADR`.
- `rg -n "rust-skills|rust-expert" .opencode AGENTS.md docs/CHANGELOG.md docs` →
  ссылки согласованы; единственное противоречие — `AGENTS.md:25` (P2).
- `cargo test --all` → 60/60 passed (33 unit + 0 main + 4 inventory + 12 publish + 11 rest).
- `cargo fmt --check` — не гонял повторно: известно красное `src/mcp.rs` (предсуществующее, вне периметра).

**Что проверено и ок:**
- `rust-expert.md` (новая роль): `edit` deny `*` + allow только `src/**`/`tests/**`;
  `shell` только `cargo check|clippy|fmt|test`, `rg`, `git status|diff`; `skill` deny `*`
  + allow `rust-skills`; `webfetch`/`websearch`/`subagent`/`question`/`external_directory`
  deny; модель `opencode-go/deepseek-v4-pro`. Соответствует acceptance целиком.
- Промпт эксперта не конфликтует с каноном: «правки без изменения поведения»
  (`:54`), контракты — `SPECIFICATION.md` §4.5, язык — `GRAMMAR.md` (`:61-62`);
  эскалации — к `lead`/`tester` (`:71-76`). Ссылки живые: `docs/SPECIFICATION.md`
  §4.5 (строка 483), `docs/GRAMMAR.md`, `.opencode/rules/workspace.md` существуют.
- `coder.md`/`tester.md`: `skill`-правило ровно одно — `deny rust-skills` (последнее
  и единственное для этого навыка); остальные навыки не заблокированы (`deny *` для
  `skill` отсутствует); в промптах нет утверждения о доступности навыка — наоборот,
  «тебе недоступен» (`coder.md:49`, `tester.md:56-57`).
- Шаг `rust-expert` обязателен и согласован в трёх местах: маршрут
  `coder → rust-expert → tester → validator → docs-writer → git` (`AGENTS.md:46-48`),
  цикл приёмки (`lead.md:54-56`), «Полезные вызовы» (`lead.md:88`).
- Q41: роль описана один раз в каноне (`AGENTS.md:39`); промпт её не дублирует,
  а уточняет операционное поведение со ссылками на канон. Дополнительных упоминаний
  `rust-expert`/`rust-skills` в `docs/README.md`, `docs/tasks/README.md`,
  `docs/features/README.md` нет.

## Результат

P2 и P3 закрыты: карта репозитория и CHANGELOG согласованы с правами ролей. Вердикт — принято.
