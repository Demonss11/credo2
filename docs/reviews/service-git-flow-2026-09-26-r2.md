# Приёмка (повторная, r2): git-flow и модели агентов (service-git-flow)

**Проверка:** адресная повторная приёмка после r1 — устранение единственной P2
(`auditor.md` держал `deepseek-v4-pro` в headless-команде) и сопутствующие
правки в ролях:
- `.opencode/agents/auditor.md` — headless-команда на модель роли
  `opencode-go/deepseek-v4.1-flash`; маска `git branch *` сужена до read-only;
- `.opencode/agents/lead.md` — та же суженая маска веток; правило про DoD
  заменено ссылкой на `AGENTS.md` §Рабочая группа (P3 аудита);
- `.opencode/agents/git.md` — добавлен `{ action: question, resource: "*",
  effect: deny }` (не наследует общий `allow`);
- лента `.opencode/mail/service-git-flow.md` — порядок «аудит → приёмка»
  приведён к канону `AGENTS.md` §Служебная зона и аудит.
**Версия:** HEAD `5b6f978` + рабочее дерево (изменения не закоммичены);
снимок 2026-09-26 (хеш тот же, что в r1 — новых коммитов нет).
**Вердикт:** принято.

## Находки

**P1:** — критичных проблем нет.

**P2:** — нет. P2 r1 устранена: `auditor.md:142` теперь задаёт
`--model opencode-go/deepseek-v4.1-flash`, совпадая с фронтматтером
(`auditor.md:4`) и текстом строки. Прежняя P2 закрыта.

**P3:** — нет.

## Проверки

- `git status --porcelain -- src tests Cargo.toml Cargo.lock` → пусто (код,
  тесты и зависимости не тронуты).
- `git status --porcelain` → периметр сервисного изменения: 10 изменённых
  файлов (`auditor.md`, `git.md`, `lead.md`, `rust-expert.md`, `validator.md`,
  `git-workflow.md`, `AGENTS.md`, `features/README.md`, `agents-cycle.feature`,
  `agents-git-approval.feature`) + новые `?? .opencode/mail/service-git-flow.md`
  и `?? docs/reviews/service-git-flow-2026-09-26.md`. Секретных/чужих правок
  вне зоны нет.
- `git log -1 --oneline` → `5b6f978 chore: очистка ленты задач T-11` — тот же
  HEAD, что в r1; правки r2 остаются в рабочем дереве (коммит — за `git`).
- `rg -n "^model:" .opencode/agents` → 10 вхождений, все
  `opencode-go/deepseek-v4.1-flash` (lead, coder, validator, tester, auditor,
  docs-writer, researcher, migrator, rust-expert, git) — 10/10 flash.
- `rg -n "deepseek-v4-pro" .` → в каноне и ролях нет. Совпадения только:
  `opencode.json` (whitelist доступных моделей владельца, не модель роли),
  память `validator.md`, лента `service-git-flow.md`, исторические
  `docs/reviews/service-git-flow-2026-09-26.md`,
  `chore-rust-expert-2026-09-26.md`, `chore-auditor-2026-09-26.md` — улики,
  не канон.
- `rg -n "git branch" .opencode/agents` → у `lead` (`lead.md:18–20`) и `auditor`
  (`auditor.md:21–23`) только read-only: `git branch -l *`, `git branch -a *`,
  `git branch --show-current`; широкой маски `git branch *` нет. У `git`
  (`git.md:20–22`) те же read-only + `git branch -d *` под `ask` (`git.md:32`).
- `rg -n "question" .opencode/agents` → у `git` `question` = `deny`
  (`git.md:41`); `allow` — только у `lead` и `auditor` (кому по канону
  положено спрашивать/подтверждать).
- `rg -n "cargo test" .opencode` → разрешение `cargo test *` (shell `allow`)
  есть ровно у `validator` (`validator.md:19`); у остальных ролей — упоминания
  в прозе («только validator») или в skill `rust-skills` (не права). Утечки
  прав нет.
- `opencode debug agents` (два прогона) → конфиг читается из текущих файлов,
  применённые права совпадают с YAML: у `validator` shell `cargo fmt *`,
  `cargo clippy *`, `cargo test *`, `opencode debug agents`, `rg *`,
  `git status/diff/log/show/grep *`; `question: deny`; `edit` только
  `docs/reviews/**`, память и почта. Модели видимых в выводе ролей —
  `deepseek-v4.1-flash` (migrator, researcher, rust-expert, tester, validator);
  формат вывода и порядок ролей идентичны прогону r1 (2355 строк), что
  подтверждает применимость файлов без `reload`.
- Счётчики (проверка без прогона теста): `rg -c "Сценарий:" docs/features` →
  42 файла, сумма 253 + 2 «Структура сценария» = **255** сценариев; процесс
  агентов 6+3+6+5+6+6 = **32**. Совпадает с `docs/features/README.md`
  (строки 284–286: 42 файла / 255, процесс 6 / 32) — как в r1.

## Что проверено и ок

- **P2 r1 закрыта:** `auditor.md:142` — headless-команда на flash; расхождения
  с фронтматтером и текстом строки нет.
- **Маски веток read-only:** у `lead` и `auditor` только `-l`/`-a`/
  `--show-current`; широкий `git branch *` не оставлен.
- **Права `git`:** `git branch -d *` под `ask`, `question` = `deny`
  (`git.md:32,41`) — согласовано с каноном «у `git` права `question` нет»
  (`AGENTS.md` §Служебная зона; `git.md` §Пакет и подтверждение).
- **Правило про DoD:** ссылка `lead.md:87–89` на `AGENTS.md` §Рабочая группа
  без пересказа — дубль канона не воспроизведён.
- **Порядок «аудит → приёмка»:** лента (`service-git-flow.md:20`) и
  `AGENTS.md:127–129` согласованы («независимый аудит `auditor` → приёмка
  `validator` → коммит»).
- **Периметр:** изменения только в служебной зоне (роли + `git-workflow.md` +
  `AGENTS.md`) и документации процесса (`features/**`, лента, отчёт); `src`,
  `tests`, `Cargo.toml`, `Cargo.lock` чист.
- **Фичи/README не менялись с r1:** счётчики 42/255 и процесс 32 совпадают с
  зафиксированными в r1; `features_inventory` был зелёным, повторный прогон не
  требуется (правки r2 фич не касались).
