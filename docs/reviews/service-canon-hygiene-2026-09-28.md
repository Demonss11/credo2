# Приёмка: service-canon-hygiene (F45 + сокращение дублей `cargo`-правил)

**Проверка:** правки канона агентов сервисной операции `service-canon-hygiene`
(фикс F45 — `deny execute` у `auditor`; сокращение дублей правил
`cargo fmt|clippy|check|test` по Q41), ветка `develop`, рабочее дерево.
**Версия:** `develop`, HEAD `e1e90d4` + рабочее дерево (2026-09-28).
**Вердикт:** принято с замечаниями.

**P1:** — (критичных проблем нет).
**P2:** — (критичных проблем нет).
**P3:** `.opencode/memory/service.md` — нет чекпойнта операции
`service-canon-hygiene` (последняя запись `:94-105` — `service-t11-closeout`),
при том что лента содержит отчёт сервисной сессии
(`.opencode/mail/service-canon-hygiene.md:131-161`) → при resume сервисной
сессии контекст операции из памяти не восстанавливается (только из ленты).
Правка — одна запись-чекпойнт в `.opencode/memory/service.md` до `git add`;
не блокер (пакет `git` ещё не собран).

## Проверки

- **Права (машина):** `opencode debug agents` — **один прогон, `reload` не
  делался**. Вывод срезан token-guard: `…[token-guard] срез: опущено ~36912
  байт из 51378`, консольно показаны строки 1551–2616 (2616 всего) — это блоки
  `migrator` и `validator`. В видимой части:
  - `validator` → `allow`: `cargo fmt *`, `cargo clippy *`, `cargo test *`,
    `opencode debug agents`, `rg *`, `git status|diff|log|show|grep *`,
    **`git branch --contains *`** (D49), `edit` — `docs/reviews/**`,
    `receipts.yaml`, свои память/лента; `external_directory: deny`;
  - `migrator` → `shell` без `cargo` (совпадает с фронтматтером и
    `review.md:88`);
  - блок `auditor` попал в срез; полный файл —
    `%USERPROFILE%\.local\share\opencode\shell\…sh_0e9b1d89…out` — **вне зоны
    чтения роли** (`external_directory: deny`), не обходил. Машинное
    подтверждение `auditor.execute = deny` этим прогоном не получено;
    подтверждено статически `auditor.md:32` (`{ action: execute, resource: "*",
    effect: deny }`) и записью аудитора (`mail …:204-209` — полный файл, стр.
    378–380, блок `auditor`).
- **`cargo test` — только `validator`:** `rg -n "action: shell"
  .opencode/agents` → `cargo test *` ровно один — `validator.md:20`; у
  `coder`/`rust-expert` — `cargo check|fmt|clippy`, у `tester` —
  `cargo check|fmt`. Новых прав сверх D49/F45 нет.
- **Автосверка `review.md:80-91` ↔ фронтматтеры:** совпадают по всем 11 ролям
  (извлечение `action: shell`): `validator` ↔ `:17-28`, `coder`/`rust-expert` ↔
  `:17-23`/`:16-22`, `tester` ↔ `:16-21`, `analyst` ↔ `:21-27`, `lead` ↔
  `:12-18`, `auditor` ↔ `:16-26`, `docs-writer`/`migrator` ↔ `:25-30`/`:22-27`,
  `researcher` ↔ `:16-17`, `git` ↔ `:15-40`. Право без применения и команда без
  права — нет.
- **R2:** единственное определение — `dispatch-loop.md:43` («`cargo test` —
  только `validator` (R2: единственная точка прогона тестов; полный прогон —
  один)»). Все ссылки на R2 разрешаются в `dispatch-loop.md` §«Hard rules»
  (заголовок `:41`): `validator.md:40`, `coder.md:54-55`, `tester.md:51-52`,
  `rust-expert.md:62`, `auditor.md:85`; `AGENTS.md:83` — краткая метка без
  определения (допустимо). `rg -n "R2"` по канону — иных определений и
  висячих ссылок «(R2, `AGENTS.md`)» нет.
- **DoD:** буквальный список — ровно один, `AGENTS.md:304-306`; ссылки
  `review.md:49` и `validator.md:79-80` ведут в §«Сборка, тесты и пересборка»
  (`AGENTS.md:298`) — якорь резолвится; блокер «Красный DoD» смысл сохранил.
- **Дубли `cargo`:** `rg -n "cargo (fmt|clippy|check|test)" .opencode AGENTS.md`
  — в каноне остались: права (4 фронтматтера ×3–4 строки), сводка
  `review.md:81-83`, DoD `AGENTS.md:304-306`, R2/expect `dispatch-loop.md:43,46`,
  бюджет `rust-expert.md:45`; полных дублей нет. Прочие вхождения — `skills/`,
  `memory/`, `mail/`, `state/` (не канон).
- **Формулировки на месте:** «сузь … и повтори» — `workspace.md:33`,
  `review.md:107`, `validator.md:45`, `tester.md:36`; «тесты не запускай» —
  `coder.md:54`, `tester.md:51`, `rust-expert.md:61`; шаблон и правила отчёта
  (`review.md` §Отчёт) не тронуты.
- **Реестр:** `docs/analysis/findings-registry.md:53` F45 — «закрыт правкой
  канона 28.09.2026: `deny execute` у `auditor` + причина в теле; аудит/приёмка
  — пакет `service-canon-hygiene`» (факт соответствует `auditor.md:32,121-125`);
  `:54` F46 — «закрыт сведением к одному канону…» (R2 → `dispatch-loop.md`; DoD →
  `AGENTS.md` §Сборка; матрица → фронтматтеры + `review.md`) — соответствует
  фактам; экранирование `\|` в ячейке F46 корректно; diff реестра = +2 строки,
  F40–F44 не задеты.
- **Границы:** `git status --porcelain` → 11 `M` + 1 `??` (лента
  `service-canon-hygiene.md`); из `docs/**` изменён только
  `docs/analysis/findings-registry.md` (по протоколу); `docs/tasks|reviews|
  questions|features|decisions` — без правок; `git diff --stat -- src tests
  opencode.json` — пусто; `AGENTS.md`/`.opencode/rules`/`.opencode/agents`
  изменены только как в составе.

## DoD

Полный `cargo test --all` **не перезапускался**: пакет не трогает Rust-код
(`git diff -- src tests opencode.json` пусто; `src/**`/`tests/**` неизменны с
`W8-config` — ср. квитанции `W8-canon`/`T-15-c13`/`service-permissions`), правки
— канон/процесс. `cargo fmt --check`/`clippy` для того же не требовались.
Последний полный DoD — `107 passed / 0 failed`, `features_inventory 4/4`
(47 files / 278 scenarios), квитанция `W8-canon`/`T-04`.

## Что проверено и ок

- Права ролей: `validator` — тесты и полный DoD (R2), `git branch --contains`
  (D49); `auditor` — `deny execute` (F45); матрица `review.md:80-91` совпадает
  с фронтматтерами; лишних прав нет.
- Канон не разорван: R2 — одно определение + разрешающиеся ссылки; DoD — один
  буквальный список + ссылки; «сузь и повтори», «тесты не запускай»,
  формулировки отчётов — на месте.
- Реестр F45/F46 — соответствие фактам, экранирование, прецеденты F40–F44.
- Границы: `src/**`, `tests/**`, `opencode.json`, `docs/tasks|reviews|questions|
  features|decisions` не тронуты; правки — только правовое/канонное ядро,
  реестр, лента и две памяти ролей.
- Аудит `auditor` (свежая сессия) — «Инструкция ↔ права: расхождений нет»,
  P1/P2/P3 нет — согласуется с моей проверкой.
