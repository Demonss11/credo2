# Приёмка: операция `service-statuses-review`

**Проверка:** сервисная операция `service-statuses-review` (служебная зона, без
`T-XX`): правка канона ролей (`migrator.md`, `validator.md`), журнальные анкеры
`D38`/`D70`, записи (память `migrator`/`auditor`, лента). Решение владельца:
статусы `TRACEABILITY` (`hold`) не меняются.

**Версия:** рабочее дерево на ветке `master` @ `9173fc4` (= `origin/master`),
`develop` @ `3d572f3` (= `origin/develop`) + незакоммиченный пакет `6 M + 1 ??`
(снимок 30.09.2026).

**Вердикт:** **возврат (rework) — не принято**; пакет к гейту не готов.

**P1:** ветка/база операции. Дерево на `master` (`git status -sb` →
`## master...origin/master`, `git rev-parse HEAD develop` → `9173fc4` /
`3d572f3`), а лента называет целевой веткой коммита `develop`
(`.opencode/mail/service-statuses-review.md:34,148`). Ветки разошлись по этим же
файлам: `git diff --stat develop HEAD` → `.opencode/agents/migrator.md` 7,
`.opencode/agents/validator.md` 13, `AGENTS.md` 2, `src/mcp.rs` 143,
`tests/mcp_draft.rs` 193 (+ `docs/reviews/T-03*/T-11*`, `docs/tasks/T-03…`).
Последствия: (а) `git switch develop` с грязными `migrator.md`/`validator.md`
(файлы есть в обоих коммитах и локально изменены) будет отклонён — гейт
остановится; (б) коммит без переключения уйдёт прямым коммитом в `master`, что
запрещено (`.opencode/rules/git-workflow.md:26–28`); (в) диффы пакета сняты
против `master`, а `develop` получил бы чужие строки — прежде всего лишний блок
`## Цикл задачи` в `validator.md` (`git diff develop HEAD -- ./.opencode/agents/validator.md`
→ +13). Анкер `D38` («удалён дублирующийся абзац») верен только для канона
`master`: в `develop` этого дубля нет (`git diff develop HEAD -- ./.opencode/agents/migrator.md`
показывает его как добавляемый в `master`). → правка: re-plan — владелец
определяет базу/ветку (`master` или `develop`); при базе `develop` дифф и записи
перепроверяются относительно `develop`, при базе `master` — исправляется
формулировка ленты о `develop` и учитывается дубль из P2.

**P2:** `.opencode/agents/validator.md:52–67` и `:69–80` — два раздела
`## Цикл задачи` (две версии одной нормы, тот же класс, что снятый дубль
«Память и почта» в `migrator.md`); файл в пакете, но дубль не снят. Последствие:
канон принимающей роли содержит две расходящиеся версии цикла — неоднозначная
инструкция (различия в п.3–5). → правка: снять одну версию (единым коммитом с
пакетом либо отдельной волной).

**P3:** — (P3 прошлого аудита про нумерацию закрыт: список «Порядок работы» —
`1–6` без разрывов, `migrator.md:60–76`).

**Проверки:**
- `git status --porcelain` → 6 `M` + 1 `??`: `.opencode/agents/migrator.md`,
  `.opencode/agents/validator.md`, `.opencode/memory/{migrator,auditor}.md`,
  `docs/decisions/{D38-agent-cycle,D70-spec-reduction}.md`,
  `?? .opencode/mail/service-statuses-review.md`. Состав пакета совпал; «7 `M`»
  брифа — описка (два канона + две памяти + два решения = 6 `M`, путей всего 7).
- `git diff --numstat` → migrator.md `+2/−7`, validator.md `+1/−2`,
  memory/auditor.md `+27/−0`, memory/migrator.md `+30/−0`, D38 `+4/−0`,
  D70 `+8/−0`.
- `git diff -- ./.opencode/agents/migrator.md` → ровно: снят абзац-дубль
  «Память и почта» (вариант «свой файл», −3 строки), снят пункт «Добавь строку
  решения в §10» (−2), перенумерация `6→5`, `7→6`; ханки только в теле
  (`@@ -55`, `@@ -70`), фронтматтер `permissions` не тронут.
- `git diff -- ./.opencode/agents/validator.md` → снят пункт «строка `Dn` есть в
  `docs/SPECIFICATION.md` §10», `;`→`.`; ханк `@@ -87`, фронтматтер не тронут.
- `git diff -- ./docs/decisions/D38-agent-cycle.md` → ровно один пункт
  «**Обновление 30.09.2026**» (+4) в конце «Следствий», перед «Сверка с кодом»;
  тело решения не переписано, только стабильный путь `.opencode/agents/migrator.md`
  (D65).
- `git diff -- ./docs/decisions/D70-spec-reduction.md` → ровно один пункт
  «**Обновление 30.09.2026**» (+8), формулировка «Найдено при закрытии P3
  аудита…»; ссылка `../tasks/T-18-docs-journal-test/README.md` жива; сессионных
  адресов нет.
- `docs/TRACEABILITY.md` — не в диффе (статусы не тронуты).
- `git diff --stat -- src tests Cargo.toml AGENTS.md` → пусто.
- `node .opencode/scripts/agents-perms.mjs` ×2 → идентично, `agents: 11 из 18`,
  расхождений нет; права `validator` совпадают с `review.md:88–90`.
- `rg -n "§10" .opencode/agents .opencode/rules` → только исторические пояснения:
  `migrator.md:47,87`, `journal.md:22`; инструкций создавать/проверять §10 нет.
- DoD: `cargo fmt/clippy/test` **не запускались** — пакет без изменений
  `src/**`, `tests/**`, `Cargo.toml` (D50; `.opencode/rules/review.md:36–39`);
  исключение для счётчиков `docs/features/**` не сработало.
- Технические проблемы: `git branch --show-current` правом отклонён (нет в
  allowlist `validator`) — ветку определял `git status -sb` и
  `git branch --contains HEAD`. `git show -s --format=%ci 9173fc4` отклонён
  правом, при этом тот же шаблон для другого хеша прошёл — не полагался,
  использовал `git log -1 --format=%ci`.

**Что проверено и ок:** диффы четырёх файлов соответствуют описанию операции
(объём, места вставок, отсутствие переписывания тел решений); фронтматтеры обоих
ролей не тронуты (права/steps `28`/`36` на месте); `docs/TRACEABILITY.md` не
изменён; границы `src/tests/Cargo.toml/AGENTS.md` — чисто; ссылки стабильные
(D65), относительная ссылка `T-18` жива; записи памяти — append-only
(`+30`/`+27`, удалений нет), канону ролей не противоречат (F35); лента — по
порядку, факты сходятся (D38 `+4`, D70 `+8`, migrator.md `+2/−7`); P3 прошлого
аудита закрыт; машинная сверка прав — чисто.
