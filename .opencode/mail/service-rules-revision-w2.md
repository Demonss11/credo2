# Сервисная лента: ревизия канона агентов — волна 2 (service-rules-revision-w2)

**Назначение:** служебная операция по решению владельца (01.10.2026): волна 2
ревизии канона агентов — (1) якорь `R5` (висячий: метка используется, дома нет),
(2) затем `.opencode/agents/**` по политике Q80/D84 (та же, что волна 1).

## сервисная сессия · 01.10.2026 · открытие

- Инструкция владельца: «делаем волну 2, но начнем с якоря».
- **Якорь `R5` — решение: определить** (а не снимать): волна 1 признала R2/R5/R7
  несущими (решение владельца «сжать, как советует аудитор»); метку уже
  используют `coder.md:52`, `tester.md:50`, `validator.md:58`, `AGENTS.md:135`
  и память `validator`; паттерн R2/R7 — определения в `dispatch-loop.md`
  §«Hard rules». Норма («чекпойнт перед тяжёлой операцией») живёт в
  `AGENTS.md` §«Память и почта»; в §«Hard rules» — компактная строка с меткой.
- Затем: ревизия `.opencode/agents/**` по политике D84 (карта аудитора: снять
  декоративные Q/D-приписки и F-метки-декор, историю; шаблоны; дедупликация;
  несущие якоря — R-метки, F43, ссылки-основания — сохранить).
- Маршрут: якорь (сервисная сессия) → agents-правки (сервисная сессия) →
  аудит `auditor` («инструкция ↔ права» + ничего не потеряно) → приёмка
  `validator` → гейт → `git` (один коммит «канон + записи», `develop`).
- `cargo` не запускается (D50).

## сервисная сессия · 01.10.2026 · волна 2 правок (якорь R5 + agents/**) — готово

- **Якорь `R5` определён**: в `dispatch-loop.md` §«Hard rules» добавлена строка
  «Чекпойнт перед тяжёлой операцией (R5): отчёт в ленту и память — **до**
  операции, дополнение результатом — после» (паттерн R2/R7; метки ролей и
  `AGENTS.md:135` резолвятся; норма-первоисточник — `AGENTS.md` §«Память и
  почта»).
- **`.opencode/agents/**` по политике Q80/D84** (7 карточек):
  - `analyst.md` — сняты D48 (комментарий прав), D39 ×2, D40;
  - `docs-writer.md` — «Политика Q41» → «Политика «один факт — один канон»»,
    D72;
  - `lead.md` — D39;
  - `migrator.md` — история (`OPEN_QUESTIONS.md`/D61, «исторически = № строки
    §10»/D70 ×2), D63, пример «Q6 при Q5» → «закрыт основным вопросом»;
  - `tester.md` — Q8/Q9 (примеры кейсов);
  - `auditor.md` — «политика Q41», F35;
  - `validator.md` — «политика Q41»;
  - `git.md` — `F43` сохранён (несущий якорь, как в `git-workflow.md`);
    `coder.md`/`rust-expert.md`/`researcher.md` — чисто (только якоря R2/R5).
- Проверки: `git diff -- .opencode/agents` — только целевые правки; поиск
  `[DQF]\d` → остался только `F43` (`git.md:79`); история/даты/пробы — пусто;
  `cargo` не запускался (D50).
- Дальше: «Обновление» D84 (`migrator`) → аудит `auditor` («инструкция ↔
  права») → приёмка `validator` → гейт → `git`.

## migrator · 01.10.2026 · «Обновление» D84 (журнальный якорь волны 2) — готово

- Сделано: в конец раздела «## Следствия» `docs/decisions/D84-rules-revision.md`
  добавлен один пункт (стиль «Обновлений» — bold-буллет, как в D38/D49):
  «**Обновление 01.10.2026:** волна 2 исполнена — п.6 закрыт: якорь `R5`
  определён в `dispatch-loop.md` §«Hard rules» («чекпойнт перед тяжёлой
  операцией»); в `.opencode/agents/**` сняты декоративные Q/D-приписки и
  F-метки-декор (7 карточек), несущие якоря (метки R2/R5/R7, `F43`) сохранены.»
  Тело решения не переписывал; сессионных адресов нет (D65).
- Проверки: D84 теперь **отслеживается** (`M`), `git diff --
  docs/decisions/D84-rules-revision.md` → ровно один ханк, +4 строки (только
  добавленный пункт); `git diff --check` пусто; `git grep` сессионных адресов
  (`opencode/mail`, `opencode/state`) по D84 → пусто (упоминания
  `service-rules-revision` в Контексте/Сверке — имена операции, не адреса);
  якорь `R5` подтверждён в `dispatch-loop.md:41` (§«Hard rules»); `cargo` не
  запускался (D50).
- `git status --porcelain` — ожидаемый состав: 7 `M .opencode/agents/*`,
  `M .opencode/rules/dispatch-loop.md`, `M docs/decisions/D84-rules-revision.md`,
  `?? .opencode/mail/service-rules-revision-w2.md` (правки волны 2 —
  сервисной сессии, не мои). Коммит не делал (гейт).

## auditor · 01.10.2026 · аудит волны 2 — готово

- **Аудит:** (1) якорь `R5` в `dispatch-loop.md` §«Hard rules» + резолв ссылок;
  (2) 7 карточек `.opencode/agents/**` — целевые снятия по Q80/D84, нормы целы;
  (3) машинная сверка прав; (4) D84 «Обновление»; (5) границы.
- **P1:** нет.
- **P2:** нет.
- **P3:** нет.
- **Проверки:**
  - `git diff ./.opencode/agents/<7>.md` — только целевые правки: `analyst` −D48
    (комментарий прав смысл сохранён), −D39×2; `docs-writer` Q41→«один факт —
    один канон», −D72; `lead` −D39; `migrator` −история `OPEN_QUESTIONS`/D61,
    −«§10 истории»/D70×2, −D63, пример «Q6 при Q5»→«закрыт основным вопросом»
    (пример обобщён, норма «не заводи новый D» цела); `tester` −Q8/Q9 (примеры
    кейсов, норма про негативные кейсы цела); `auditor` −Q41, −F35;
    `validator` −Q41. `F43` сохранён (`git.md:79`, несущий). Фронтматтеры по
    существу не менялись: `analyst` `+4/−4`, только комментарий `:10`; effective
    permissions те же.
  - `dispatch-loop.md:41–42` — строка «Чекпойнт перед тяжёлой операцией (R5)»
    есть; ссылки `coder:52`, `tester:50`, `validator:58`, `AGENTS.md:135` и
    память `validator:5,185` — резолвятся (дом R2-паттерн `:39`).
  - `node .opencode/scripts/agents-perms.mjs` ×2 → `agents: 11 из 18`, без
    расхождений; `.opencode/rules/review.md:86–104` ↔ 11 фронтматтеров — совпадает.
  - D84:76–79 — «Обновление 01.10.2026» один bold-буллет, append-only,
    сессионных адресов нет (D65); тело решения не переписано.
  - `git status --porcelain` — 7 `M` agents + `M dispatch-loop.md` + `M D84` +
    `M .opencode/memory/migrator.md` (чекпойнт migrator'а) +
    `?? .opencode/mail/service-rules-revision-w2.md`; посторонних артефактов нет.
    `git diff --stat -- src tests Cargo.toml AGENTS.md` — пусто; `cargo` не
    запускался (D50).
  - Шум-контроль: `[DQF]\d` в `.opencode/agents` → пусто; `Q41|Q8|Q9`,
    `Q6 при Q5|OPEN_QUESTIONS|D61|D63|D70|§10`, «пробы/Run/BRIEF/фаза C» → пусто
    (бинарные совпадения `package-lock.json` игнорированы).
- **Инструкция ↔ права:** расхождений нет.
- **Вердикт:** волна 2 готова к приёмке `validator`. Максимум — P3 (ниже),
  не блокирует.
- **P3 (опц., предсуществующее, вне диффа):** `.opencode/agents/coder.md:54`,
  `tester.md:51`, `rust-expert.md:62` — фраза «(R2 — `.opencode/rules/
  dispatch-loop.md` §«Hard rules»)» теряет глагол («см.»/«определение —»);
  якорь `R5` определён в том же §, при желании — симметричная ссылка.
- **Грабли:** `git diff` с двумя и более dot-путями под `--` отклонён движком
  (в этот раз — «Permission denied») — читал по одному пути; `git grep` без `-n`
  правом отклонён — заменил инструментом `grep`.

## validator · 01.10.2026 · приёмка — принято

- **Вердикт: принято**, P1/P2/P3 нет. Версия: `develop` = `origin/develop` =
  `HEAD` @ `52d8989` + рабочее дерево. Отчёт
  `docs/reviews/service-rules-revision-w2-2026-10-01.md`; квитанция —
  `receipts.yaml` (`service-rules-revision-w2`, iteration 1, accepted).
- **Проверки:**
  - `git diff` по 7 карточкам — только целевые снятия (Q/D-декор, F-декор,
    история, примеры: `analyst` −D48/−D39×2/−D40; `docs-writer` Q41→плейн,
    −D72; `lead` −D39; `migrator` −история `OPEN_QUESTIONS`/D61,
    −§10/D70×2, −D63, пример обобщён; `tester` −Q8/Q9; `auditor` −Q41/−F35;
    `validator` −Q41); нормы целы; `coder`/`rust-expert`/`researcher`/`git`
    не тронуты; `F43` — `git.md:79` (единственный `[DQF]\d` в agents).
  - Якорь `R5`: `dispatch-loop.md:41–42` §«Hard rules» (паттерн R2 :39/R7 :61);
    ссылки `coder:52`, `tester:50`, `validator:58`, `AGENTS.md:135`, память —
    резолвятся. `dispatch-loop.md` — один ханк +2 строки.
  - `node .opencode/scripts/agents-perms.mjs` ×2 → `agents: 11 из 18`,
    идентично; фронтматтеры по существу не менялись (`analyst` — только
    комментарий :10).
  - D84 — один ханк +4 строки (:76–79), append-only, один bold-буллет, тело
    решения цело; сессионных адресов нет (D65).
  - Шум-контроль: `rg '[DQF][0-9]'` → только `F43`; Q/D-метки, «проба/Run/
    BRIEF/перенесено» → пусто; `git diff --check` пусто.
  - Границы: `git status --porcelain` = 7 `M` agents + `M dispatch-loop.md` +
    `M D84` + `M memory/{auditor,migrator}` (+ мои записи: отчёт, квитанция,
    память, лента); `git diff --stat -- src tests Cargo.toml AGENTS.md` пусто;
    `cargo` не запускался (D50).
- **P3 (информационно, вне диффа):** предсуществующая фраза в `coder:54`,
  `tester:51`, `rust-expert:62` — ссылки на `R2` без служебного глагола;
  правка однострочная, кандидат на будущую волну, не блокер.
- **Окончательный состав пакета для гейта:** `.opencode/agents/{analyst,
  auditor,docs-writer,lead,migrator,tester,validator}.md`,
  `.opencode/rules/dispatch-loop.md`,
  `docs/decisions/D84-rules-revision.md`,
  `.opencode/memory/{auditor,migrator,validator}.md`,
  `.opencode/mail/service-rules-revision-w2.md`,
  `.opencode/state/current/receipts.yaml`,
  `docs/reviews/service-rules-revision-w2-2026-10-01.md`. `src/**`,
  `tests/**`, `Cargo.toml`, `AGENTS.md` — не входят (не менялись).
- **Дальше:** гейт `surface_to_user` → `git` (один коммит «канон + записи»,
  `develop`).

## сервисная сессия · 01.10.2026 · подтверждение пакета (гейт)

- Гейт пройден: владелец подтвердил **коммит + push** (`question`, ответ
  «Коммит + push (Recommended)») — операция `service-rules-revision-w2`.
- База: `develop` @ `52d8989` (= `origin/develop`); режим — коммит в `develop`,
  затем `git push origin develop`.
- Пакет — 15 путей + записи `lead`/`git` = 17:
  - `.opencode/agents/analyst.md`, `.opencode/agents/auditor.md`,
    `.opencode/agents/docs-writer.md`, `.opencode/agents/lead.md`,
    `.opencode/agents/migrator.md`, `.opencode/agents/tester.md`,
    `.opencode/agents/validator.md`
  - `.opencode/rules/dispatch-loop.md`
  - `.opencode/mail/service-rules-revision-w2.md` (лента)
  - `.opencode/memory/auditor.md`, `.opencode/memory/git.md` (чекпойнт роли
    `git`, новый до `add`), `.opencode/memory/migrator.md`,
    `.opencode/memory/validator.md`
  - `.opencode/state/current/progress.yaml`,
    `.opencode/state/current/receipts.yaml`
  - `docs/decisions/D84-rules-revision.md`
  - `docs/reviews/service-rules-revision-w2-2026-10-01.md`
- Сообщение коммита:
  `docs(Q80/D84): волна 2 канона агентов — якорь R5, чистка agents/** (service-rules-revision-w2)`
- Состояние: приёмка `validator` — принято (P1/P2/P3 нет).

## git · 01.10.2026 · пакет сервисной волны 2 — готово

- Подтверждение сверено по записи «сервисная сессия · 01.10.2026 · подтверждение
  пакета (гейт)»: состав 17 путей, режим «коммит + push» в `develop`.
- База сверена: `## develop...origin/develop`, `HEAD` `52d8989`; снимок
  15 `M` + 2 `??` (лента и отчёт приёмки — новые) = ровно пакет.
- Записи до `add` (F43): этот отчёт и чекпойнт в памяти `git`.
- Дальше: `add` 17 путей (`./`-префикс) → сверка staged (`git diff
  --cached --name-status`: 15 `M` + 2 `A`) → `commit` → `push origin develop`.
  Хеши — в ответе `lead` (F43).

## сервисная сессия · 01.10.2026 · P3 закрыт (быстрая правка) — готово

- Инструкция владельца: «сделай быструю правку» — закрыть P3 волны 2
  («(R2 — …)» без служебного глагола).
- Сделано: в 4 карточках добавлено «см.» в ссылке на `R2`: `coder.md:54`,
  `tester.md:51`, `rust-expert.md:62`, `validator.md:44` (четвёртая — тот же
  дефект, найден при правке; в P3 аудитора не значилась). Нормы не менялись —
  только формулировка ссылки.
- Проверки: `git diff` по 4 карточкам — только добавление «см.»; поиск
  «(R2 — `.opencode/rules`» без «см.» → пусто; `cargo` не запускался (D50).
- Дальше: приёмка `validator` (порог существенности: канон — новый отчёт +
  `agents-perms` ×2) → гейт → `git`.

## validator · 01.10.2026 · приёмка P3-правки (быстрая) — готово

- **Вердикт: принято** (P1/P2/P3 нет). Версия: `develop` @ `c8ffb0f`
  (= `origin/develop` = `HEAD`, коммит волны 2) + рабочее дерево. Отчёт
  `docs/reviews/service-rules-revision-w2-p3-2026-10-01.md`; квитанция
  `service-rules-revision-w2` iteration 2, accepted.
- Сделано (проверено): `git diff` по каждой из 4 карточек (`./`-форма — иначе
  движок отклоняет) → ровно один ханк `+1/−1`, только вставка «см.» в
  `(R2 — …)`; `rg` ссылок `(R2 — `.opencode/rules/dispatch-loop.md` §«Hard
  rules»)` без «см.» → пусто (exit 1); все 4 `(R2 — ` c «см.» (`validator:44`,
  `coder:54`, `rust-expert:62`, `tester:51`); дом резолвится
  (`dispatch-loop.md:37` `## Hard rules`).
- Проверки: `node .opencode/scripts/agents-perms.mjs` ×2 → `agents: 11 из 18`,
  идентично, фронтматтеры целы; границы `git status --porcelain` = 4 `M` agents
  + `M` лента, посторонних нет; `git diff --stat -- src tests Cargo.toml
  AGENTS.md` пусто; `git diff --check` пусто; `cargo` не запускался (D50).
- Дальше: гейт `surface_to_user` → `git` (коммит «P3 закрыт» в `develop`).

## сервисная сессия · 01.10.2026 · подтверждение пакета (гейт) — коммит + push + merge в master

- Гейт пройден: владелец подтвердил **«commit — push — merge в master»**
  (`question`, ответ «commit - pusth - merge в master») — операция
  `service-rules-revision-w2`, закрытие P3.
- База: `develop` @ `c8ffb0f` (= `origin/develop`); режим:
  1) коммит «P3 закрыт» в `develop` + `git push origin develop`;
  2) `git switch master` → `git merge --no-ff develop -m "Слияние develop в master"` → `git push origin master` (master @ `9173fc4` — предок `develop`, конфликтов нет; тег/CHANGELOG не требуются — синхронизация, не релиз).
- Пакет коммита — 8 путей + записи `lead`/`git` = 10:
  - `.opencode/agents/coder.md`, `.opencode/agents/rust-expert.md`,
    `.opencode/agents/tester.md`, `.opencode/agents/validator.md`
  - `.opencode/mail/service-rules-revision-w2.md` (лента)
  - `.opencode/memory/git.md` (чекпойнт роли `git`, новый до `add`)
  - `.opencode/memory/validator.md`
  - `.opencode/state/current/progress.yaml`
  - `.opencode/state/current/receipts.yaml`
  - `docs/reviews/service-rules-revision-w2-p3-2026-10-01.md`
- Сообщение коммита:
  `docs(Q80/D84): канон ролей — «см.» в ссылках на R2 (P3, service-rules-revision-w2)`
- Состояние: приёмка `validator` — принято (P1/P2/P3 нет).

## git · 01.10.2026 · P3-коммит и merge master — готово

- Подтверждение сверено по записи «сервисная сессия · 01.10.2026 · подтверждение
  пакета (гейт) — коммит + push + merge в master»: состав 10 путей, режим
  «коммит `develop` + push → merge `develop` → `master` + push».
- База сверена: `## develop...origin/develop`, `HEAD` `c8ffb0f`; снимок
  9 `M` + 1 `??` (отчёт приёмки — новый) = ровно пакет + мой чекпойнт = 10.
- Записи до `add` (F43): этот отчёт и чекпойнт в памяти `git`.
- Дальше: `add` 10 путей (`./`-префикс) → сверка staged
  (`git diff --cached --name-status`: 9 `M` + 1 `A`) → `commit` →
  `push origin develop` → `switch master` → `merge --no-ff develop` →
  `push origin master`. Хеши — в ответе `lead` (F43).
