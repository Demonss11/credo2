# Приёмка `service-git-efficiency` (D75) — 2026-09-30

**Проверка:** исполнение D75 п.1–7 — канон роли `git` (`git.md`), правило
`git-workflow.md` (минимальный цикл, запреты, шаблон промпта `lead`,
идемпотентность), `review.md` §«Доступные команды», хелпер
`.opencode/scripts/git-check.mjs`, память `memory/git.md` (F35), журнал Q71/D75
(`TRACEABILITY`, каталоги, `findings-registry.md` F49–F55), лента/память ролей,
границы пакета.

**Версия:** `develop` @ `75b078c` + рабочее дерево (2026-09-30);
`git diff --shortstat` — 43 файла (42 M + 1 D), `??` — 31 (две незакоммиченные
волны: `service-doc-rework`, `service-git-efficiency`).

**Вердикт:** принято с замечаниями (один P3, не блокер).

**P1:** — критичных проблем нет.
**P2:** — нет.
**P3:** `docs/decisions/D75-git-lean-workflow.md:7-11` (`Affects`) перечисляет
`git.md`, `git-workflow.md`, `review.md`, `git-check.mjs`, `memory/git.md`, но не
`AGENTS.md`, тогда как `AGENTS.md:25` (карта `.opencode/scripts/`: добавлен
`git-check.mjs`) — прямое следствие исполнения D75 п.5 (закрытие P3-3 аудита,
лента `:318`). Последствие: трассировка «решение ↔ артефакты» неполна — при
аудите канона происхождение строки `AGENTS.md:25` не связано с D75/Q71
(прецедент — P3 приёмки `service-agent-tools`, `D51:13-14`, где `AGENTS.md`
внесён позже). Правка — дописать `AGENTS.md` в `Affects` (одна строка,
`migrator`); не блокер.

**Проверки:**

- `node .opencode/scripts/agents-perms.mjs` ×2 → прогоны идентичны:
  `agents: 11 из 18`; `git [subagent] steps=14` с `allow:node
  .opencode/scripts/git-check.mjs` (+ `*`) и `ask` на
  `add`/`commit`/`switch`/`checkout`/`merge`/`branch -d`/`tag`/`restore`/`push`/
  `fetch`/`pull`; `validator [subagent] steps=36` — права совпадают с
  `review.md:86-87` (`cargo fmt|clippy|test`, `agents-perms.mjs`, `rg`,
  `git status|diff|log|show|grep`, `git branch --contains *`) — эффективная
  конфигурация (не только текст `.md`) подтверждает D75 п.6.
- `node .opencode/scripts/git-check.mjs` → `permission.rejected` (вне allowlist
  `validator`; у `git` право есть — прогон выше). Принято **чтением кода**
  `git-check.mjs:1-105`: только read-only git — `status -sb` (`:51`),
  `log -1 --oneline` (`:59`), `status --porcelain` (`:64`),
  `diff --cached --name-status` (`:87`); импорты только `node:*` (`:14-16`);
  `--expect`-расхождение → `fail` + `exit 1` (`:99-104`); `head`-ветвь
  исправлена (`:59-61`); изменяющих команд нет.
- `git status --porcelain`, `git diff --stat -- src tests Cargo.toml` → последнее
  **пусто**; `src/**`, `tests/**`, `Cargo.toml` не тронуты.
- `rg "Минимальный цикл" .opencode` → в живом тексте 3 вхождения
  (`git-workflow.md:85` — заголовок-дом, `git.md:85` и `memory/git.md:11` —
  ссылки); дубля цикла нет (P2 аудита закрыт).
- `rg -c "^\| \[D" docs/decisions/README.md` → 75; `rg --files docs/decisions -g
  "D*.md"` → 75 файлов; `rg -c "^\| \[Q" docs/questions/README.md` → 71;
  `rg -c "^\| \[Q" docs/TRACEABILITY.md` → 71, Q71 — строка `:75` (D75 → Q71,
  `resolved`, задачи/реализация `—`).
- Журнал: `Q71.md:3` `resolved by [D75]`; `D75` `accepted`, `Resolves: [Q71]`,
  `Спека: —` (§10 упразднена D70 — строки D там нет и не требуется),
  `Tasks: —` + «Задач не требуется» (`:86-88`), «Сверка с кодом» ⚪ с уликами
  (`:69-81`); относительные ссылки Q71/D75 живые (цели существуют).
- `findings-registry.md:57-63` — **F49–F55** (7 записей) со статусом
  «закрывается D75 (…)» и привязкой Q71/[D75]; закрытие — за `migrator` после
  приёмки.
- Канон роли: `git.md:6` `steps: 14`; `:30-31` allow `git-check.mjs` (+ `*`);
  `:82-87` §«Минимальный цикл пакета» — **ссылка** на правило + хелпер;
  `:79-80` пост-push запрет. `git-workflow.md:85-95` — 7 шагов (`status -sb` →
  `log -1` → записи до `add` (F43) → `add` → `diff --cached --name-status` →
  `commit` → `push`) + список запретов + «post-push git-команд нет»;
  `:126-134` — «минимальная проверка», серии `--grep` запрещены (шаблон в
  кавычках); `:68-70` — шаблон промпта `lead` (операция; ссылка на запись пакета
  в ленте без дубля состава — Q41; база `HEAD`; режим; сообщение).
  `review.md:96-98` — у `git` `clean-logs.mjs` + `git-check.mjs`;
  автопроверка `:99-100`.
- `AGENTS.md:25` — карта `.opencode/scripts/` называет `git-check.mjs` (P3-3
  аудита закрыт); `:24` — `journal.md` в списке правил.
- `memory/git.md` — 34 строки: ссылка на правило (`:10-12`) + уроки (хвостовые
  чтения, quoting `--grep`, отказы прав, порядок записей, идемпотентность,
  дисциплина путей); сверено с `git-workflow.md:85-95,126-146` и
  `review.md:103-110` — расхождений (F35) нет.
- Лента/память ролей: `memory/auditor.md:190-219` (аудит D75 + закрытие P2/P3) и
  `memory/migrator.md:1108-1127` (Q71→D75) согласуются с фактическим состоянием
  (`steps: 14`, 34 строки памяти, `agents-perms` ×2, состав правок).

**DoD (cargo не запускался — D50):** правило `review.md:36-39` +
`D50-dod-by-package-scope.md`. Состав пакета — `.opencode/**` + журнальные
записи `docs/**`; `src/**`, `tests/**`, `Cargo.toml` не менялись
(`git diff --stat -- src tests Cargo.toml` пусто), `docs/features/**` не
затрагивались (состав сценариев и счётчики 47/278 не менялись) — следовательно,
ни cargo-прогоны, ни адресный `features_inventory` не требуются. Проверки —
чтением, адресными `rg` и `agents-perms.mjs` ×2.

**Технические проблемы:** `node .opencode/scripts/git-check.mjs` отклонён
движком прав (`permission.rejected`) — права `validator` его не покрывают (по
замыслу D75 хелпер адресован роли `git`; `agents-perms` это подтверждает);
read-only-свойства подтверждены чтением исходника (см. «Проверки»), как и в
аудите `auditor`. Коды выхода скриптов инструментально не различаю (харнесс
отдаёт «Exited with code 1» на любую неуспешную команду) — для `git-check.mjs`
ветка `--expect` → `fail` → `exit 1` проверена по исходнику `:99-104`.

**Наблюдения (не находки):**

1. `git-workflow.md:68-70` шаблон промпта `lead` дан перечнем в одну физическую
   строку, а не буквальным 5–8-строчным блоком; «5–8 строк» в D75 п.4 — бюджет
   промпта, все пять элементов (операция, ссылка на ленту, база `HEAD`, режим,
   сообщение) названы — требование покрыто по смыслу.
2. `memory/git.md:15-16` и `:22-23` — два пункта об идемпотентности (лёгкая
   избыточность внутри 34 строк, канону не противоречит; ср. остаток аудита про
   `git.md:73-77`).

**Что проверено и ок:** `git.md` (тело и фронтматтер), `git-workflow.md`,
`review.md` §«Доступные команды», `git-check.mjs` (исходник), `memory/git.md`,
`AGENTS.md:24-25`, лента `service-git-efficiency.md` (факты, аудит D75,
закрытие P2/P3), памяти `auditor`/`migrator`/`git`, `Q71`, `D75`, `TRACEABILITY`,
`questions/README`, `decisions/README`, `findings-registry.md`, границы пакета,
живость относительных ссылок. Вердикт аудита `auditor` («инструкция ↔ права»: без
расхождений; P2/P3 закрыты) подтверждён независимо.
