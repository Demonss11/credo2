# Сервисная лента: service-mcp-ready (том 3)

**Назначение:** продолжение линии `service-mcp-ready` — сервисные операции без
задачи (MCP-ready, процесс агентов). Тома 1–2 и ленты прошлых прогонов очищены
28.09.2026 (`node .opencode/scripts/clean-logs.mjs`), содержимое — в истории git.

Формат записей — `AGENTS.md` §«Память и почта».

---

## auditor · 28.09.2026 · аудит правки канона C13 (F43)

- Сделано: аудит C13 «Хвостовые записи пакета» (F43). Сверены
  `.opencode/rules/git-workflow.md` §«Пакет и подтверждение» п. 4 и
  §«Завершение задачи» (строки 74–80, 94–95),
  `.opencode/agents/git.md` §«Пакет и подтверждение» п. 5 (строки 76–78),
  `AGENTS.md` §«Память и почта» (строки 116–120). Согласованность трёх текстов —
  ok; с F37/D47, D46/F22 — ok; `review.md` команды не менялись.
- Проверки: чтение 6 файлов, 2 `rg`; машинная сверка `debug agents` не требуется
  (права/фронтматтеры правкой не затронуты).
- Вердикт: «Инструкция ↔ права: расхождений нет»; P1/P2 нет.
- P3 (кандидаты в правку): `AGENTS.md:120` — якорь `§«Пакет»` (нет такого
  заголовка; точный — «Пакет и подтверждение»); `git-workflow.md:78–79` —
  критерий «`git status --porcelain` пуст» соседствует с допущением
  «остаточной служебной грязи» (уточнить область критерия).
- Дальше / риски: P3 — сервисной сессии владельца; затем свежий аудит и приёмка.

---

## auditor · 28.09.2026 · аудит закрытия P3 (C13)

- Сделано: свежий аудит двух правок после C13. (1) `AGENTS.md:120` — якорь
  заменён на `§«Пакет и подтверждение»`; заголовок существует
  (`git-workflow.md:59`) — ссылка разрешается. (2) `git-workflow.md:74–81`
  §«Пакет и подтверждение» п. 4 — критерий приёмки сужен до «новых правок
  **от роли `git`** после `push` нет (`git status` их не показывает)»; «грязь»
  других ролей и остатки вне пакета — не «догоняющим» коммитом, а следующим
  плановым пакетом. Противоречие «пусто vs остаток допустим» снято; прочие
  части п. 4 (записи до `git add` входят в пакет; после `push` роль не пишет в
  отслеживаемые файлы; хеши — в ответе `lead`/`progress.yaml`) по существу не
  изменились и согласуются с `AGENTS.md:116–120`.
- Проверки: чтение 4 файлов, 3 `rg` (на 1 больше бюджета — доп. поиск дубля
  критерия `porcelain`: иных формулировок критерия в каноне и `git.md` нет,
  новых конфликтов нет; `docs/tasks/T-15-mcp-ready-process/README.md:124` —
  «подхватом следующим плановым пакетом» — согласовано). `opencode debug
  agents` не запускался: фронтматтеры/права правкой не затронуты.
- Вердикт: «Инструкция ↔ права: расхождений нет»; P1/P2/P3 нет — оба P3 C13
  закрыты.
- Дальше / риски: коммит — сервисной сессии владельца. Аудит без задачи;
  `git.md` п. 5 не перечитывался (критерий там отсутствует по `rg`).

---

## validator · 28.09.2026 · приёмка (чекпойнт, в работе)

- Сделано (чекпойнт до машинной проверки): прочитаны канон C13
  (`git-workflow.md` §«Пакет и подтверждение» п. 4, §«Завершение задачи»;
  `git.md` п. 5; `AGENTS.md:116–120`), карточка T-15 (фазы B1/C/G + реестр),
  `findings-registry.md` (F43, F15/F26/F27), `receipts.yaml`, эта лента.
  Снимок `git status --porcelain`: 9 удалений `mail/**`, 11 `memory/*.md`,
  канон (`AGENTS.md`, `git.md`, `git-workflow.md`), доки (реестр, T-15),
  новый `?? service-mcp-ready-r3.md`; HEAD `9006ca1`. Полный DoD не требуется:
  `src/**`/`tests/**` с принятого `W8-canon` не менялись (подтвержу diff).
- Дальше / риски: `opencode debug agents` ×2 (11 ролей; steps/права), сверка
  diff канона, живые ссылки, F43↔C13↔F22/D46. Вердикт и квитанция — после.
- Отчёт: `docs/reviews/T-15-c13-2026-09-28.md`.

---

## validator · 28.09.2026 · приёмка (результат)

- Сделано: приёмка пакета «C13 (F43) + T-15 + очистка логов». `opencode debug
  agents` ×2 — 11 ролей, steps совпадают с фронтматтерами; `git diff` — правки
  только в теле (`git.md` п. 5, `AGENTS.md` 116–120, `git-workflow.md` п. 4 +
  closeout), фронтматтеры/права не тронуты; якорь §«Пакет и подтверждение»
  резолвится (`git-workflow.md:59`); `review.md` не менялся; F43/F15/F27 и C13
  согласованы; `mail` = `.gitkeep` + `r3`; 11 `memory/*.md` сброшены
  (`auditor.md` — 2 свежих чекпойнта после очистки); `src/**`/`tests/**` тождественны
  `W8-canon` (`22f7683..9006ca1` — пусто) — DoD не перезапускался.
- Вердикт: **принято с замечаниями** — P1/P2 нет; P3 `docs/tasks/T-15-mcp-ready-process/README.md:126`
  («`git status` пуст» vs суженный канон «от роли `git`», `git-workflow.md:78–81`).
- Проверки: `opencode debug agents` ×2, `git status|diff|log|show`, `rg`; бэкап
  `<temp>/opencode/logs-backup-20260928-154456` напрямую не проверен
  (`external_directory: deny`) — принят по строке вывода.
- Дальше / риски: квитанция `T-15-c13` (iteration 1, accepted) в `receipts.yaml`;
  P3 — в сервисную сессию владельца (одна строка), не блокер. Коммит — по пакету.

---

## validator · 28.09.2026 · адресная проверка `docs/questions/README.md`

- Сделано: независимая приёмка новой сводки журнала вопросов
  (`docs/questions/README.md`, `migrator`, 28.09.2026; untracked, вне пакета
  `T-15-c13`). Сверены 12 строк «Сводки» ↔ 12 Q-файлов (`Q1`, `Q43–Q53`);
  статусы ↔ `Q47.md` / `docs/TRACEABILITY.md`; ссылки `Qn.md`, `Dn-*.md`,
  `T-XX` (glob — все резолвятся); шапка Q41; правило поддержки ↔ `BRIEF.md`
  §5.1–§5.2; стиль ↔ `docs/tasks/README.md`.
- Вердикт: **принято с замечаниями**. Полнота, шапка Q41, структура и стиль —
  ok. P2: `docs/questions/README.md:30` — строка Q47 в «Связано» ведёт на
  `T-15`, тогда как `docs/TRACEABILITY.md:13` даёт `—`, а
  `docs/decisions/D42-expect-iteration.md:10,54` прямо фиксирует «Задач не
  требуется»; в `Q47.md` (поле «Связано») `T-15` также нет. Правка — одна
  строка: заменить `[T-15]` на `—`. P3: `README.md:13–14` — ссылка «правила —
  `BRIEF.md` §5.1–§5.2»: §5.1 п. 3 / §5.2 п. 7 требуют строку в
  `TRACEABILITY.md`, про сводку `questions/README.md` BRIEF не говорит —
  указатель косвенный (риск дрейфа сводки при новой `Qn`).
- Проверки: чтение 7 файлов (`validator.md`, эта лента, `questions/README.md`,
  `TRACEABILITY.md`, `Q47.md`, `tasks/README.md`, `BRIEF.md` §5.1–§5.2),
  2 `glob`, 3 `rg`; `git log -1` → HEAD `9006ca1`; `git status --porcelain`
  (снимок). `git rev-parse HEAD` отклонён правами — заменён `git log -1`.
- Дальше / риски: замечание — в сервисную сессию владельца (`migrator`/канон),
  не блокер. Полный DoD (R2) не запускался: `src/**`/`tests/**` правкой не
  затронуты, сводка — процессный документ. Не коммичено.

---

## сервисная сессия · 28.09.2026 · подтверждение владельца: пакет `c13_records` (в develop)

- Владелец подтвердил (2026-09-28): коммиты **A** `chore(agents): C13 …` (3 пути)
  и **B** `chore(process): T-15 реестр задач и C13-приёмка; очистка логов; сводка
  вопросов` (26 путей) — **прямо в `develop`** + `push origin develop`.
- Состав: A — `AGENTS.md`, `.opencode/agents/git.md`,
  `.opencode/rules/git-workflow.md`; B — удаления 9 лент, новый
  `service-mcp-ready-r3.md`, 11 `memory/*.md`, `receipts.yaml`,
  `findings-registry.md`, карточка T-15, отчёт `T-15-c13`,
  `docs/questions/README.md`.
- Основание: приёмка `T-15-c13` (accepted; P3 закрыт) и адресная проверка
  `docs/questions/README.md` (P2/P3 закрыты).
- **Первое применение C13:** отчёт роли `git` и чекпойнт памяти — **до** `git add`
  пакета B и входят в него; после `push` роль в отслеживаемые файлы не пишет;
  хеши A/B — ответом `lead`.

---

## git · 28.09.2026 · чекпойнт до коммита B (F43)

- Сделано: пакет `c13_records` — подтверждение сверено (лента `r3`, строки
  117–131). Снимок `git status --porcelain` до: A(3)+B(26)=29 — совпал с пакетом.
  Коммит **A** `722a782` `chore(agents): C13 — записи пакета до коммита (F43)`
  (3 пути, `git diff --cached --name-only` = 3). Чекпойнт памяти дописан.
- Проверки: `git status --porcelain` (29 = 3+26), `git status -sb`
  (`develop...origin/develop [ahead 1]`, HEAD `9006ca1`), `git log --oneline -4`;
  после `add` — `git diff --cached --name-only` = 3.
- Дальше / риски: коммит **B** (26 путей) → `git push origin develop` → `log`/
  `status`/`show --stat`. Хеш B здесь не приводится (F43). После `push` записей
  в отслеживаемые файлы нет.

---

## сервисная сессия · 28.09.2026 · B0-i1: изоляция и формы установки V2 (готово)

- Сделано: temp-проект `%TEMP%\opencode\wave0b-probe` (`git init`, dummy `.env`,
  `hello.txt`); зафиксированы обе формы установки V2: локальный
  `.opencode/plugins/*.ts` + `npm install @opencode/plugin` (283 пакета, 60 с) и
  config `plugins[]` (каталог); startup-критерий чистый; протокол —
  `wave0b-probes.md`; улики — `target/wave0b-i1/`.
- Проверки: `opencode run --auto --model opencode-go/deepseek-v4.1-flash` ×3 →
  `ок`; маркеры setup `.opencode/wave0b-probe-loaded.json` (13:53:02.507Z) и
  `.opencode/wave0b-probe-config-loaded.json` (13:53:48.073Z); в логе
  `loading plugin` без `failed to load plugin`.
- Дальше / риски: прогон 1 не завершился в CLI (сессия `succeeded`, `/wait` — 499
  после kill) — повторы норма; для i4/i7 контроль таймаута. Далее **B0-i2**: Shell
  Strategy — инструкция, не плагин; `instructions` в V2 не загружается (доки +
  проба), контент — кандидат в C.

---

## сервисная сессия · 28.09.2026 · B0-i2: Shell Strategy (готово; 🔴 установка / 🟢 контент)

- Сделано: паспорт (v1.1.0, MIT, инструкция без кода); проба механизма в temp:
  A — `instructions` + маркер-правило → не загрузилось (`ок`); B — контроль через
  `AGENTS.md` → маркер применён (`ПАНТЕРА-9137`); сверка с дисциплиной ролей
  (allowlist'ы; Linux-only части; конфликт `git --no-pager` с префиксными
  правилами; `GIT_TERMINAL_PROMPT=0` — учесть).
- Проверки: `opencode run --auto --model opencode-go/deepseek-v4.1-flash` ×2;
  V2-доки config#instructions («accepts but does not load»).
- Вердикт: 🔴 как V2-артефакт (README-способ установки не работает; кандидат
  V1-эпохи); 🟢 содержание — рекомендация C (раздел «non-interactive shell» под
  pwsh/allowlist). Улики — `target/wave0b-i2/`.
- Дальше / риски: **B0-i3** (Opencode Telemetry) — установка пакета, SQLite/CLI,
  сверка с `opencode stats`/`session export`.

---

## сервисная сессия · 28.09.2026 · B0-i3: Opencode Telemetry (готово; 🔴)

- Сделано: паспорт (v0.2.0, MIT, V1 Plugin API); пробы форм V2 в temp: config-пути
  (абс/отн/файл) не подхватываются; `.opencode/plugins/<dir>/` — подхват только с
  корневым `index.ts`, затем WARN `failed to load plugin` («Plugin must export a
  default definition with an id and an effect or setup function»); CLI `octm help`
  работает (Bun 1.4.2); сверка — `opencode stats --days 1` и `opencode session list`.
- Проверки: `opencode run --auto --model opencode-go/deepseek-v4.1-flash` ×3 (ок);
  `bun run bin/cli.ts help`; `opencode stats --days 1` (19 сессий, 59 промптов,
  1.2k шагов, 207.7m токенов, 97.1% tool success — без расхода токенов).
- Вердикт: 🔴 для OpenCode 2.0.18 (V1-only; данных не даёт); метрики D — нативные
  `stats`/`session export`. Улики — `target/wave0b-i3/`.
- Дальше / риски: **B0-i4** (Subagent Reporter + Agent Identity) — headless-прогон
  с субагентом; следить за stdout-стримом и атрибуцией роль/модель.

---

## сервисная сессия · 28.09.2026 · B0-i4: Subagent Reporter + Agent Identity (готово; 🔴×2)

- Сделано: оба кандидата — V1 Plugin API; пробы в temp дали WARN `failed to load
  plugin`: Subagent Reporter — «must export a default definition with an id and an
  effect or setup function»; Agent Identity — `Cannot find package
  '@opencode-ai/plugin'` (runtime V1-зависимость, default-экспорта нет).
- Проверки: headless-прогон с субагентом — нативный stdout показывает только
  `✓ … General Agent` (внутренний стрим не виден — baseline для C); сбои плагинов
  сессию не ломают. Улики — `target/wave0b-i4/`.
- Вердикт: 🔴×2 для OpenCode 2.0.18. Рекомендации C: наблюдаемость — `--format
  json` или свой V2-плагин; атрибуция `role`/модель — собственная или нативные
  данные сессии.
- Дальше / риски: **B0-i5** (CC Safety Net) — пробы блокировок на temp-путях
  (`git reset --hard`, `rm -rf`, `.env`), сосуществование с permissions ролей.

---

## сервисная сессия · 28.09.2026 · B0-i5: CC Safety Net (готово; 🟢)

- Сделано: паспорт (v2.4.11, MIT; V2-энтрипоинт `./opencode/v2`);
  установка локальной копией в temp (`dist/` + root `index.ts`; `shell=pwsh`,
  `options.shell=powershell`); headless-проба: read `.env` — **BLOCKED**
  (`secret.basename.env`); `git push --force` — **BLOCKED** (`git.push-force`,
  предложен `--force-with-lease`); `git status --short` — прошёл; audit JSONL в
  `~/.cc-safety-net/logs/**` (копия — `target/wave0b-i5/`); `explain`:
  `git reset --hard` — BLOCKED в репо (в temp — ALLOWED, политика temp-root);
  `Remove-Item -Recurse -Force` — ALLOWED на standard (есть paranoid-правило).
- Проверки: сессия `opencode run --auto` (файлы не менялись); `explain` ×3;
  audit JSONL (5 записей с `sessionId`/`ruleId`/`intent`).
- Вердикт: 🟢 — перенос в служебную зону (решение владельца + `auditor`), форма —
  `opencode plugin add cc-safety-net@latest` + `options.shell=powershell`; вопрос
  paranoid-пресета — в C.
- Дальше / риски: **B0-i6** (snip, опция) — доступность CLI на Windows, замер
  экономии shell-вывода.

---

## сервисная сессия · 28.09.2026 · B0-i6: snip (готово; 🔴)

- Сделано: паспорт (MIT; V1-зависимость `@opencode-ai/plugin ^1.0.0`); CLI
  `snip --version` → CommandNotFound, `go version` → CommandNotFound (brew нет) —
  документированная установка на Windows неприменима; лоад-тест в temp → WARN
  `failed to load plugin` («must export a default definition…»); экономия не
  измерена (CLI недоступен); конфликты переноса: префикс `snip` ломает allowlist'ы
  ролей, POSIX-пайпы vs дисциплина одиночных команд.
- Проверки: `snip --version`, `go version`, `opencode run` (temp), лог.
- Вердикт: 🔴 — токен-политика C на базе B1/token-guard; snip пересмотреть при
  V2-порте + Windows-сборке. Улики — `target/wave0b-i6/`.
- Дальше / риски: **B0-i7** (Handoff) — имитация обрыва/лимита, continuation vs
  resume по `sessionID`.

---

## сервисная сессия · 28.09.2026 · B0-i7: Handoff (готово; 🔴)

- Сделано: паспорт (v0.5.0, MIT; V1-плагин `HandoffPlugin`, без default; runtime
  `@opencode-ai/plugin`/`@opencode-ai/sdk`/`zod`); faithful-проба в temp (src +
  root index.ts, доустановлены zod/sdk) → WARN `failed to load plugin`:
  гонка копирования, затем `Cannot find package '@opencode-ai/plugin'` (runtime
  V1; повтор ~16 раз); сравнение continuation-prompt vs resume по `sessionID`
  (данные для C): resume — полный контекст/дороже; continuation — сводка/дешевле;
  нативные аналоги — `session export` и самоотчёт роли.
- Проверки: `opencode run` (temp), лог, исходники кандидата.
- Вердикт: 🔴; C — `session-checkpoint`: resume как основной путь + опционально
  своя continuation-сводка. Улики — `target/wave0b-i7/`.
- Дальше / риски: **B0-i8** — отчёт, рекомендации C/D/E/F, пакет коммитов
  (по подтверждению владельца).

---

## сервисная сессия · 28.09.2026 · B0-i8: отчёт и финализация (готово; пакет — по подтверждению)

- Сделано: `wave0b-report.md` заполнен (вердикты, метрики, рекомендации C/D/E/F,
  вопросы владельцу §5); карточка `wave0b-plugins.md` — статус ✅; реестр T-15 —
  B0-i1…i7 ✅, B0-i8 🚧 (пакет — по подтверждению); чек-боксы i8 (отчёт/откат — [x],
  коммит — [ ]). Пробы: репо чист, канон не тронут; temp-полигон и улики
  `target/wave0b-i1…i7/` — на месте.
- Проверки: ревизия протоколов/улик все 7 итераций; `opencode stats --days 1`
  (23 сессии · 63 промпта · 219.9m токенов, 97.1% tool success); лента r3 — 268+ строк
  (при >300 — том 4).
- Итог по волне: 🟢 — CC Safety Net (перенос — решение владельца + auditor);
  🔴 — Telemetry, Subagent Reporter, Agent Identity, snip, Handoff; Shell Strategy —
  🔴 установка / 🟢 контент (C).
- Дальше / риски: подтверждение пакета коммитов (один, прямо в `develop`);
  аудит переноса ccsn; S/M-прогон (B1-F26).

---

## сервисная сессия · 28.09.2026 · подтверждение владельца: пакет `wave0b_records` + решения по B0

- Владелец подтвердил (2026-09-28) коммит документов B0 одним коммитом **прямо в
  `develop`** (пакет `wave0b_records`) — после фиксации правки отчёта. Пути:
  `docs/tasks/T-15-mcp-ready-process/` (`wave0b-plugins.md`, `wave0b-plan.md`,
  `wave0b-probes.md`, `wave0b-report.md`, `README.md`), `.opencode/mail/service-mcp-ready-r3.md`,
  `.opencode/memory/service.md` + по C13 отчёт роли `git` и `.opencode/memory/git.md`;
  сообщение — `chore(process): T-15 wave 0 фазы B (B0) — протоколы проб, вердикты,
  отчёт`; затем `git push origin develop`.
- Решения владельца: **своя обвязка B0-own** (P1–P5, `wave0b-report` §4) — мини-волна
  после коммита B0; **перенос CC Safety Net — позже** (пресет standard); **S/M-прогон
  (B1-F26)** — после B0-own.
- Основание: `wave0b-report.md` §4/§6; прогоны и вердикты — `wave0b-probes.md`;
  фиксация правок — этот пакет.

---

## git · 28.09.2026 · чекпойнт (пакет `wave0b_records`) — до `git add`

- Сделано: подтверждение владельца сверено по этой ленте (запись «сервисная сессия ·
  28.09.2026 · подтверждение владельца: пакет `wave0b_records` + решения по B0»):
  один коммит **прямо в `develop`**, сообщение —
  `chore(process): T-15 wave 0 фазы B (B0) — протоколы проб, вердикты, отчёт`,
  затем `push origin develop`. Снимок `git status --porcelain` до записей: 6 ` M`
  (`mail/service-mcp-ready-r3.md`, `README.md`, `wave0b-plugins.md`, `wave0b-plan.md`,
  `wave0b-probes.md`, `wave0b-report.md`) + `?? .opencode/memory/service.md`; чекпойнт
  `.opencode/memory/git.md` — эта запись (C13, входит в пакет).
- Проверки: чтение ленты `r3` (подтверждение) и `.opencode/rules/git-workflow.md`;
  `git status --porcelain` — снимок выше; посторонних правок нет.
- Дальше / риски: `git switch develop` → `git pull origin develop` → `git add` (ровно
  8 путей из пакета) → `git diff --cached --name-only` (сверка: 8) → commit →
  `push origin develop` → `log -2 --oneline` / `status -sb` / `show --stat HEAD`.
  После `push` в отслеживаемые файлы не пишу (F43): хеши верну `lead` ответом.
