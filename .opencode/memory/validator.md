# Память: validator (приёмка)

- **Канон:** `.opencode/rules/review.md`; `.opencode/rules/journal.md` §5.3/§7.
- **Правило:** чекпойнт **до** прогона тестов (что проверено, что запускаю) и
  **после** (результат, вердикт) — тесты прерывают сессию (R5). Кратко.

## Чекпойнты

- **2026-09-29 · приёмка «service-agent-tools» (Q56/D51, чекпойнт до прогона):**
  прочитаны лента `service-agent-tools.md`, `review.md:28-39,84-117`,
  `workspace.md:19-30`, фронтматтеры `validator.md:1-34` / `auditor.md:1-34`,
  скрипт `agents-perms.mjs`. Снимок: `develop`, HEAD `01a70fd` + рабочее дерево
  (13 M + 4 `??`); `git diff -- src tests Cargo.toml` пусто; `docs/features`
  — правки только текста шагов (2 хунка: `agents-cycle.feature:51`,
  `agents-audit.feature:17`), счётчики/состав не менялись → **cargo не
  запускаю** (D50, исключение не сработало — обосную в отчёте). Далее: смоук
  скрипта (сводка, `--role validator`, `--grep cargo`, `--help`,
  `--all --grep "opencode debug agents"|"opencode reload"`), адресные проверки
  (счётчики 278, §10 №51, D51/Q56/TRACEABILITY/README, ссылки, «1.9 КБ»,
  снимок строк с пометкой). Сырую `opencode debug agents` не запускаю — права
  нет (по замыслу D51).
- **2026-09-29 · приёмка «service-agent-tools» (Q56/D51, итог):** вердикт
  **принято с замечаниями**; P1/P2 нет; **P3** — `D51:7-12` (`Affects`) и
  `:47-49` («Решение» п. 4) не перечисляют `AGENTS.md:149-151` (строка —
  следствие D51/P2-1 аудита) → неполная трассировка «решение ↔ артефакты»;
  правка `migrator` — одна строка. Проверено: смоук `agents-perms.mjs`
  (сводка `agents: 11 из 18`; `--role validator` = фронтматтер, `steps=36`;
  `--grep cargo` = 22; `--json`; `--help`; `--all --grep "opencode debug
  agents"` → 0; `--all --grep "opencode reload"` → auditor), `review.md:86-97`
  ↔ 11 фронтматтеров, фичи 276+2=278 = `features/README.md:303`, `rg -c '^'` →
  272, канон/фичи без сырой команды, журнал Q56↔D51↔TRACEABILITY:27↔
  `questions/README:44`↔SPEC:872, «1.9 КБ» нет, пометка снимка (D51:75-76),
  ссылки живые, границы — 13 M + 4 `??`, `src`/`tests`/`Cargo.toml` не тронуты.
  **cargo не запускался — D50** (исключение по `docs/features/**` не сработало:
  счётчики/состав не менялись). Отчёт
  `docs/reviews/service-agent-tools-2026-09-29.md`; квитанция
  `service-agent-tools` iteration 1 — append. **Урок:** харнесс отдаёт
  «Exited with code 1» на любую неуспешную команду (`rg --nosuchflag` → 1, а не
  документированный 2) — коды выхода скриптов инструментально не различать,
  брать из исходника и писать это в отчёт.

- **2026-09-28 · приёмка «service-t11-closeout» (итог):** вердикт **принято с
  замечаниями**; P1/P2 нет; P3 — `docs/features/README.md:291-292` (легенда `🟡`
  «поведение не подтверждено» против строки `agents-rework` с фактом Run 4).
  **Критерий T-11 подтверждён 4/4:** права (`cargo test *` только `validator.md:20`;
  r3 + свежий аудит W8), память 11 ролей + `service.md`, resume `sessionID` 3×,
  пилот Run 4/T-03 (отчёты rework/`-r2`, квитанция `iteration 2`), `0a5832f`/
  `5019c45` в `develop`. Согласованность T-15/T-16/D38/фичи/счётчики 47-278 —
  ок; границы (`agents|rules/**`, `AGENTS.md`, `src`, `tests`) не тронуты;
  `rustfmt.toml` — только комментарий. DoD: `cargo fmt --check` pass,
  `cargo test --test features_inventory` 4/4; полный `--all` не гонялся
  (`src/tests` неизменны с W8-config). Отчёт
  `docs/reviews/T-11-closeout-2026-09-28.md`; квитанция `T-11-closeout` append.
  Урок: `git branch --contains` вне allow-list `validator` — заменять `git log`.
- **2026-09-28 · приёмка сервисного пакета «service-t11-closeout» (до прогона):**
  прочитаны лента операции, карточки T-11/T-15/T-16, `review.md`, отчёты r3/T-03,
  меморандумы W8 т.1/т.2 §0, реестр находок, `receipts.yaml`. Снимок: ветка
  `develop`, HEAD `bed3019` + рабочее дерево; изменены 10 файлов
  (`docs/**`, 2 памяти, `rustfmt.toml`) + новые `mail/service-t11-closeout.md`,
  `docs/tasks/T-16-stale-check-test/`; `.opencode/agents|rules/**`, `AGENTS.md`,
  `src/**`, `tests/**` не тронуты (`git status --porcelain`). `git diff -- src
  tests` пусто. Критерий T-11: права (debug agents, frontmatter — `cargo test *`
  только `validator`), память 11 ролей + `service.md`, пилот Run 4/T-03 (отчёты
  rework/`-r2`, квитанция iteration 2), коммиты `0a5832f`/`5019c45` в истории
  `develop` (`git log develop -50`), 6 шапок `agents-*.feature` с `# D38 (Q43)`,
  счётчики 47/278 (`features/README.md:300`). Далее: `cargo fmt --check` +
  `cargo test --test features_inventory` (README фич правлен текстом); полный DoD
  не гоняю — `src/**`/`tests/**` неизменны с W8-config. `git branch --contains`
  отклонён движком прав — заменил на `git log develop`.
- **2026-09-28 · приёмка сервисного пакета «C13 (F43) + T-15 + очистка логов»
  (в работе):** прочитаны `git-workflow.md`, `git.md`, `AGENTS.md` (§«Память и
  почта»), карточка T-15, `findings-registry.md`, `receipts.yaml`, `r3`-лента.
  Снимок `git status`: 9 удалений `mail/**`, 11 сбросов `memory/*.md`, правки
  канона (`AGENTS.md`, `git.md`, `git-workflow.md`), доков (реестр, T-15),
  новый `mail/service-mcp-ready-r3.md`; HEAD `9006ca1`. Далее: `opencode debug
  agents` ×2 (11 ролей, steps/права), сверка diff канона. Полный DoD не гоняю:
  `src/**`/`tests/**` с `W8-canon` не менялись — зафиксирую.
- **2026-09-28 · приёмка сервисного пакета «C13 (F43) + T-15 + очистка логов»
  (итог):** вердикт **принято с замечаниями**; P1/P2 нет; P3 — `T-15/README.md:126`
  («`git status` пуст» vs канон «правок роли `git` нет»). Проверено: `debug
  agents` ×2 (11 ролей, steps без изменений, права не тронуты), `git diff` канона
  (тело, не фронтматтеры), якорь §«Пакет и подтверждение» резолвится, `review.md`
  не менялся, F43/F15/F27 ↔ C13 согласованы, `mail`/`memory` очищены. `src/**`/
  `tests/**` == `W8-canon` (`22f7683..9006ca1` пусто) — DoD не перезапускался.
  Отчёт `docs/reviews/T-15-c13-2026-09-28.md`; квитанция `T-15-c13` append.
  Урок: бэкап `clean-logs` лежит в `%TEMP%` — роли недоступен
  (`external_directory: deny`); принимать только по строке вывода, писать это в отчёт.
- **2026-09-28 · адресная проверка `docs/questions/README.md`** (сводка журнала,
  `migrator`, untracked; HEAD `9006ca1`): полнота 12 строк ↔ 12 файлов `Q1`,
  `Q43–Q53`; шапка Q41 ok; ссылки живые; стиль ↔ `tasks/README.md` ok. Вердикт
  **принято с замечаниями**: P2 `questions/README.md:30` — Q47 «Связано: T-15»
  против `TRACEABILITY.md:13` (`—`) и `D42:10,54` («Задач не требуется»); P3
  `README.md:13–14` — указатель на `BRIEF.md` §5.1–§5.2 косвенный (сводка там не
  упомянута). Итог — в `mail/service-mcp-ready-r3.md`; не коммичено.
- Чекпойнтов ещё не было до этой записи.
- **2026-09-28 · приёмка сервисного пакета «service-permissions» (Q54/D49,
  чекпойнт до прогона):** прочитаны лента `service-t11-closeout.md`, отчёты
  `migrator`/`auditor`, канон `validator.md:28` и `review.md:81`,
  Q54/D49, SPEC §10 №49 (`:870`), `TRACEABILITY.md:20`,
  `questions/README.md:37`, `findings-registry.md:52` (F44). Снимок: `develop`,
  HEAD `6d4c840` + рабочее дерево (10 M + 2 `??`); `git diff -- ./.opencode` —
  ровно одна строка allowlist `validator` + синхронизация `review.md`;
  `git diff -- src tests` пусто; `AGENTS.md`/`opencode.json` не тронуты.
  Далее: `opencode debug agents` (один прогон, срез token-guard возможен) +
  смоук `git branch --contains 0a5832f`. Полный DoD не гоняю:
  `src/**`/`tests/**` неизменны (обоснование в отчёте).
- **2026-09-28 · приёмка «service-permissions» (Q54/D49, итог):** вердикт
  **принято**, P1/P2/P3 нет. Канон: `validator.md:28` — ровно одна строка
  allowlist `git branch --contains *`, `review.md:81` синхронизирован; иных
  правок канона нет. Машинный резолв (`opencode debug agents`, один прогон,
  без `reload`; срез token-guard ~36.9/51.4 КБ, блок `validator` прочитан) —
  право видно. **Смоук прошёл:** `git branch --contains 0a5832f` → `* develop`,
  `exp/agent-cycle-rerun`, `exp/agent-update-t15w0` (право подхватилось в
  свежей сессии). Журнал Q54/D49/SPEC §10 №49/TRACEABILITY:20/questions
  README:37/F44 согласованы, ID уникальны, ссылки живые. P3 аудитора закрыт —
  F44 ссылается на созданный `docs/reviews/service-permissions-2026-09-28.md`.
  DoD: `src/**`/`tests/**` неизменны с W8-config — `cargo test/fmt/clippy` не
  перезапускались (правка канона/прав → машинная сверка прав). Квитанция
  `service-permissions` iteration 1 — append. **Урок снят:** `git branch
  --contains` теперь штатное право роли — косвенную замену `git log` больше
  не применять.
- **2026-09-28 · приёмка «service-migration-q2q3» (чекпойнт до прогона):**
  прочитаны лента операции, D16, Q2/Q3, `OPEN_QUESTIONS.md` (:31–44 — только
  указатели), `TRACEABILITY.md:9–10`, `questions/README.md:26–27`,
  `SPECIFICATION.md:837` (§10 №16), `features/README.md:69` + счётчики :300,
  `CHANGELOG.md:19–24`, карточка T-14 и сводка `tasks/README.md:51`,
  `GRAMMAR.md:56` (§2, строка 7 — дрейф), `BRIEF.md` §2/§5.3/§5.6/§5.7/§7/§9.
  Снимок: `develop`, HEAD `491e153` + рабочее дерево (13 M + 4 `??`); `git diff
  -- src tests AGENTS.md opencode.json` пусто; `.opencode/agents|rules` — без
  правок (shell-доступ к пути запрещён — проверю через `git status`/`git diff`).
  3 шапки `# D16 (Q2, Q3)` (parser/lexer/execution) — ровно три. Далее:
  `cargo fmt --check` + `cargo test --test features_inventory` (ожидание 4/4;
  47/278). Полный `--all` не гоняю: `src/**`/`tests/**` неизменны с W8-config.
- **2026-09-28 · приёмка «service-migration-q2q3» (итог):** вердикт
  **принято**, P1/P2/P3 нет. Перенос (§7): в архиве только указатели
  (`OPEN_QUESTIONS.md:36–44`); `D16` = §10 №16 (`SPECIFICATION.md:837`),
  `Resolves/Spec/Affects/Tasks` заполнены, сверка ✅ + «Задач не требуется»;
  `GRAMMAR.md:56` (§2, таблица, строка 7) — дрейф, покрыт T-14 (карточка
  ведёт на §2; `§7` только в примечании :35). Q↔D↔`TRACEABILITY.md:9–10`↔
  `questions/README.md:26–27`↔§10 согласованы (статус `resolved` — конвенция
  пилота Q1/D15); слаг `D16-dsl-canon-regex-mvp` уникален; ссылки живые;
  `# D16 (Q2, Q3)` — ровно 3 фичи (parser/lexer/execution). Границы:
  `git diff -- src tests AGENTS.md opencode.json` пусто; `.opencode/agents|rules`
  не тронуты; архив не пополнялся, чужие записи не переписаны. DoD:
  `cargo fmt --check` pass; `cargo test --test features_inventory` **4/4 ok**
  (47/278, `features/README.md:300`); полный `--all` не гонялся — `src/tests`
  неизменны с W8-config. Отчёт `docs/reviews/migration-q2q3-2026-09-28.md`;
  квитанция `service-migration-q2q3` iteration 1 append. Урок: стилевые
  отклонения в файле с десятками длинных строк (таблицы) не считать находкой
  без последствия.
- **2026-09-28 · приёмка «service-canon-hygiene» (F45 + сокращение дублей
  `cargo`-правил, итог):** вердикт **принято с замечаниями**; P1/P2 нет; P3 —
  `.opencode/memory/service.md` без чекпойнта операции (лента есть: `:131-161`).
  Проверено: `opencode debug agents` (1 прогон, без `reload`; срез token-guard
  ~36 912/51 378 Б, видны `migrator`+`validator`; у `validator` —
  `cargo fmt|clippy|test *`, `git branch --contains *` (D49); блок `auditor` в
  срезе → `execute: deny` только статически `auditor.md:32`, полный файл вне
  зоны чтения); `rg "action: shell"` — `cargo test *` только `validator.md:20`,
  список `review.md:80-91` ↔ фронтматтеры 11 ролей совпадают; R2 — одно
  определение `dispatch-loop.md:43`, ссылки резолвятся; DoD — один список
  `AGENTS.md:304-306`, ссылки `review.md:49`/`validator.md:79-80`; реестр
  F45:53/F46:54 (+2 строки, F40–F44 не задеты); границы: `git diff -- src tests
  opencode.json` пусто, `docs/**` — только реестр. Отчёт
  `docs/reviews/service-canon-hygiene-2026-09-28.md`; квитанция
  `service-canon-hygiene` iteration 1 — append. Урок: `execute`-право в выводе
  `opencode debug agents` проверять по фронтматтеру, если блок роли в срезе, а
  полный файл недоступен (`external_directory: deny`).
- **2026-09-29 · приёмка «service-migration-q4» (чекпойнт до прогона):**
  прочитаны лента операции, `Q4.md`, `D17-priority-out-of-mvp.md`,
  `OPEN_QUESTIONS.md:46-49` (указатель, полного текста нет), `TRACEABILITY.md:11`,
  `questions/README.md:28`, `SPECIFICATION.md:838` (§10 №17), `GRAMMAR.md:91`
  (§4, `Приоритет` — ❌) / `:97` (§5), нота `features/README.md:75-79`, счётчики
  `:301` (47/278), `CHANGELOG.md:25-28`, `features_inventory.rs`, `review.md`,
  `BRIEF.md` §2/§4/§5.3/§5.6/§5.7/§7. Снимок: `develop`, HEAD `f488085` +
  рабочее дерево (13 M + 3 `??`); `git diff -- src tests AGENTS.md opencode.json`
  пусто; `rg -i приоритет src` — только semver-комментарий `core.rs:414` и
  флаг `main.rs:56`; `parse_rule` — regex `Правило`/`Если`/`Решение`/`Причина`.
  4 шапки `# D17 (Q4)` (parser/execution/editor/lsp) + `client_explanation:8`
  `(D17/Q36)`; слаг `D17-priority-out-of-mvp` — 1 файл. Далее: `cargo fmt --check`
  + `cargo test --test features_inventory` (ожидание 4/4, 47/278); полный `--all`
  не гоняю — `src/**`/`tests/**` неизменны с W8-config.
- **2026-09-29 · приёмка «service-migration-q5q6» (чекпойнт до прогона):**
  прочитаны лента операции, `Q5.md`, `Q6.md`, `D19-statuses-priorities-canon.md`,
  архив `OPEN_QUESTIONS.md:51-64` (только указатели; Q6 — «закрыт попутно»),
  `TRACEABILITY.md:12-13`, `questions/README.md:29-30`, `SPECIFICATION.md:840`
  (§10 №19), нота `features/README.md:12-20`, `CHANGELOG.md:29-37`,
  `BRIEF.md` §7 (`:269-320`, правило попутного вопроса `:291-292`), §6,
  `review.md:78-95`. Снимок: `develop`, HEAD `788aa4c` + рабочее дерево
  (7 M + 4 `??`); `git diff -- src tests AGENTS.md opencode.json` пусто;
  `.opencode/agents|rules` не тронуты. Далее: `cargo fmt --check` +
  `cargo test --test features_inventory` (ожидание 4/4, 47/278); полный `--all`
  не гоняю — `src/**`/`tests/**` неизменны с `22f7683` (реформат W8-config).
- **2026-09-29 · приёмка `service-migration-q5q6` (итог):** **принято**,
  P1/P2/P3 нет. Проверки: `cargo fmt --check` — pass; `cargo test --test
  features_inventory` — 4/4 (47/278); `git diff -- src tests AGENTS.md
  opencode.json` — пусто; `git diff 22f7683..HEAD -- src tests` — пусто;
  `.opencode/agents|rules` не тронуты. Перенос Q5 (+ попутный Q6) →
  `D19-statuses-priorities-canon`: указатели в архиве (`OPEN_QUESTIONS.md:51-54`,
  `:60-64`), `D19` = §10 №19 (`SPECIFICATION.md:840`), `Resolves: Q5` (Q6 **не**
  в `Resolves` — BRIEF §7, попутная конвенция соблюдена: ссылки Q6 → D19,
  пометки «закрыт попутно»); `TRACEABILITY.md:12-13`, `questions/README.md:29-30`,
  нота `features/README.md:12-14`, `CHANGELOG.md:29-37`; сверка ✅ + «Задач не
  требуется»; SPEC §6.1/§6.2 — ссылки на README без дубля таблицы; слаг уникален.
  Отчёт `docs/reviews/migration-q5q6-2026-09-29.md`; квитанция
  `service-migration-q5q6` iteration 1 — append. **Урок:** в приёмке переноса
  проверять попутный вопрос по BRIEF §7 (`:291-292`) отдельным пунктом — он не
  входит в `Resolves` D, и «формальная» проверка поля может дать ложное P1.
- **2026-09-29 · приёмка «service-dod-scope» (чекпойнт до записи):** прочитаны
  лента операции, `review.md` §«Порог существенности» (:28-38), `validator.md`
  (фронтматтер :1-34, чек-лист :83-92), `BRIEF.md` §5.7 (:241-257), `Q55.md`,
  `D50-dod-by-package-scope.md`, §10 №50 (`SPECIFICATION.md:871`),
  `TRACEABILITY.md:26`, `questions/README.md:43`, `AGENTS.md` §«Сборка»
  (:298-307), `dispatch-loop.md` §«Hard rules» (R2 :43-47), `receipts.yaml`.
  Снимок: `develop`, HEAD `0ee25f5` + рабочее дерево (9 M + 3 `??`); области —
  журнал/канон-текст/доки/памяти; `src/**`, `tests/**`, `Cargo.toml`,
  `opencode.json` не тронуты; `docs/features/**` не тронуты. **cargo не
  запускаю — ровно по D50** (состав пакета). Далее: адресные проверки
  (ссылки, ID, поля), отчёт `docs/reviews/service-dod-scope-2026-09-29.md`,
  квитанция, лента.
- **2026-09-29 · приёмка «service-dod-scope» (итог):** **принято**, P1/P2/P3 нет.
  **Правило D50 применено к себе:** cargo-прогонов нет — `src/**`/`tests/**`/
  `Cargo.toml` не менялись, исключение (`docs/features/**`) не сработало.
  Проверено адресно: `git status` (9 M + 3 `??`), `git diff --stat` (+72/−3,
  только журнал/канон-текст/доки/памяти), diff канона (два хунка: `validator.md`
  :84-89, `review.md` :26-38; фронтматтер не тронут → `opencode debug agents`
  не запускал, обоснование в отчёте), `rg "Cargo.toml"` (review.md:29,35;
  BRIEF:253; D50 ×9), `rg "^\| 50 \|"` (одна строка :871), `rg "D50-|Q55"`,
  `rg "features_inventory"` (старой формулировки нет). Q55/D50 ↔ TRACEABILITY:26
  ↔ questions/README:43 ↔ §10 №50; сверка ⚪ + «Задач не требуется»; P3-1/P3-2
  аудита закрыты (перечень с `Cargo.toml` + пометка об аудите; запись сервисной
  сессии `mail:195-215`). **Урок:** если канон-пакет меняет только тело роли,
  машинную сверку прав не дублировать — обосновывать неизменностью фронтматтера
  и прогоном аудитора в том же снимке. Отчёт
  `docs/reviews/service-dod-scope-2026-09-29.md`; квитанция `service-dod-scope`
  iteration 1 — append.
- **2026-09-29 · приёмка `service-migration-q4` (итог):** **принято**, P1/P2/P3 нет.
  Проверки: `cargo fmt --check` — pass; `cargo test --test features_inventory` —
  4/4 (47/278); `git diff -- src tests AGENTS.md opencode.json` — пусто. Перенос
  Q4 → `D17-priority-out-of-mvp`: указатель в архиве (`OPEN_QUESTIONS.md:46-49`),
  `D17` = §10 №17 (`SPECIFICATION.md:838`), строки `TRACEABILITY.md:11` /
  `questions/README.md:28`, 4 шапки `# D17 (Q4)` (parser/execution/editor/lsp) +
  `client_explanation:8` `(D17/Q36)`; сверка ✅ + «Задач не требуется». Отчёт
  `docs/reviews/migration-q4-2026-09-29.md`; квитанция `service-migration-q4`
  iteration 1 — append. **Урок:** обоснование «`src/tests` неизменны» привязывать
  к точному хешу последнего изменения Rust (`22f7683` — реформат W8-config), а не
  к имени пакета: `git diff a7eac82..HEAD -- src tests` не пуст.
- **2026-09-29 · приёмка `service-migration-q7` (чекпойнт до отчёта):** прочитаны
  лента `service-migration-q7.md`, `Q7.md`, `D52-glossary-terms-canon.md`,
  `BRIEF.md` §5.7/§7, `review.md` §«Доступные команды»/«Порог»,
  `OPEN_QUESTIONS.md:44-69,441-443`, `SPECIFICATION.md:865-906` (§10 №44-52, §11),
  `TRACEABILITY.md`, `questions/README.md`, `publish.feature:1-14`, `CHANGELOG.md:25-46`,
  `tasks/T-07-meta-fields/README.md`, `src/core.rs:68-87`, `src/lib.rs:524-539`,
  `receipts.yaml`. Снимок: `develop`, HEAD `5883a17` + рабочее дерево (8 M + 3 `??`);
  `git diff 22f7683..HEAD -- src tests` пусто; `AGENTS.md`/`.opencode/agents|rules`/
  `docs/tasks/**`/`docs/features/README.md` не в статусе; правка `publish.feature`
  — только шапка (3+/1-, `Сценарий:` = 7 без изменений) → **cargo не запускаю — D50**
  (исключение не сработало). Далее: отчёт `docs/reviews/migration-q7-2026-09-29.md`,
  квитанция, лента.
- **2026-09-29 · приёмка `service-migration-q7` (итог):** вердикт **принято с
  замечаниями**; P1/P2 нет; **P3** — `OPEN_QUESTIONS.md:66` указатель Q7 без
  темы/даты («### Q7. → перенесён») против соседей (`Q4:46`, `Q5:51`, `Q6:60`:
  `### Qn. ✅ <тема> (решено <дата>) → перенесён`) → потеря контекста в архиве;
  правка `migrator` — одна строка (BRIEF §7 буквально соблюдён, не блокер).
  Проверено: архив — только указатель (`:66-69`, полного текста нет; `:443`
  «открытый Q7» в блоке Q13 — историческое, как и ссылки на Q4/Q5/Q6 в
  неперенесённых блоках); `D52` = §10 №52 (`:873`, номер уникален, стиль ↔ №44-51);
  `Resolves: Q7`, `Спека` №52, `Affects`/`Tasks` заполнены; сверка ⚪ + факты
  (`core.rs:71-81` — `CheckMeta` без `display_name`/`source_hash`/`compiler_version`;
  `lib.rs:526-539` — `ManifestEntry`/`service_hash`; §11 — 14 новых статей + 2
  уточнённых); Q7↔D52↔`TRACEABILITY:14`↔`questions/README:31`↔§10 №52 согласованы;
  `publish.feature:7-11` `[Q7]→[D52]`; `CHANGELOG:38-46`; слаг уникален; ссылки
  живые; границы — 8 M + 3 `??`, `src`/`tests`/`Cargo.toml`/канон/`docs/tasks` не
  тронуты, архив не пополнялся. **cargo не запускался — D50.** Отчёт
  `docs/reviews/migration-q7-2026-09-29.md`; квитанция `service-migration-q7`
  iteration 1 — append.
- **2026-09-29 · приёмка `service-migration-q8q10q42` (чекпойнт до отчёта):**
  прочитаны лента `service-migration-q8q10q42.md`, `D21-core-semantics-v01.md`,
  `Q8/Q9/Q10/Q42.md`, `BRIEF.md` §2/§4/§5.3/§5.6/§5.7/§7, `review.md`,
  `OPEN_QUESTIONS.md:75-88,137-140`, `SPECIFICATION.md:842`, `TRACEABILITY.md:15-18`,
  `questions/README.md:32-35`, шапки пяти фич, `CHANGELOG.md:47-62`,
  `src/core.rs` (`EvalError`, `Explanation`, тесты `*_q8…q42`). Снимок: `develop`,
  HEAD `8f0c9d9` + рабочее дерево (12 M + 6 `??`); `git diff --name-only -- src
  tests Cargo.toml` пусто; правки `docs/features/**` — только 3 строки-комментария
  в шапке каждой из пяти фич (additive, 0 удалений); счётчики не задеты:
  `rg -c --stats` = 278 сценариев / 47 файлов = `features/README.md:303`
  → **cargo не запускаю — D50** (исключение по счётчикам/составу не сработало).
  Находка (P1): битые ссылки на непересённые Q — `Q8.md:57` и `Q42.md:54`
  ([Q11](Q11.md)), `Q10.md:11,62` ([Q36](Q36.md)–[Q39](Q39.md)); файлов нет
  (`docs/questions` — 26 файлов, Q11–Q41 не мигрированы), конвенция соседей —
  текстом (`Q7.md:11` «Q13 (ожидает переноса)»); `review.md` «Что блокер» +
  BRIEF §5.7/§9. Остальное (перенос, §10 №21, TRACEABILITY, README, сверка ✅,
  границы) — ок. Вердикт — **отклонено** (возврат `migrator`). Далее: отчёт
  `docs/reviews/migration-q8q10q42-2026-09-29.md`, квитанция `rework`.
- **2026-09-29 · `service-migration-q8q10q42` (iteration 2, повторная приёмка,
  итог):** вердикт **принято**, P1/P2/P3 нет. P1 iteration 1 закрыт: markdown
  снят во всех 6 местах (`Q8.md:57` — «Q11 (ожидает переноса)», `Q42.md:54` —
  «(Q11 (ожидает переноса))», `Q10.md:11,62` — «Q36–Q39 (ожидают переноса)»),
  стиль = `Q7.md:11`; `rg "Q(11|36|39)\.md\)" docs/questions` пусто, вне
  `docs/questions` — только цитаты в отчёте iteration 1. Пакет не изменился:
  `git diff --stat` по файлам пакета = iteration 1 (прирост лишь в зонах записи
  памяти/квитанций); `Q9.md` (61 строка) и `D21` (`:5/:22/:151`, 186 строк) не
  тронуты; границы — 14 M + 7 `??`, `src`/`tests`/`Cargo.toml`/канон/`docs/tasks`
  чистые; счётчики 278/47 = `features/README.md:303`. **cargo не запускался —
  D50.** Отчёт `docs/reviews/migration-q8q10q42-2026-09-29-r2.md`; квитанция
  `iteration 2` (`accepted`) — append; отчёт — в ленту. **Урок:** при повторной
  приёмке новых (untracked) файлов `git diff` бесполезен — сверять с прежним
  снимком чтением + `rg -c "^"` и якорями `rg -n`.
- **2026-09-29 · приёмка `service-migration-q8q10q42` (итог):** вердикт
  **отклонено** (возврат `migrator`, iteration 1). **P1** — 6 битых ссылок на
  непересённые Q: `Q8.md:57`, `Q42.md:54` ([Q11](Q11.md)), `Q10.md:11,62`
  ([Q36](Q36.md)–[Q39](Q39.md)); P2/P3 нет. Проверено: архив — только
  указатели (`:75-88`, `:137-140`, блок Q11 не тронут); §10 №21 — строка
  существовала, добавлена ссылка (diff 1 строка); `D21` — `Resolves` сцепкой,
  `Спека` №21, `Affects`, «Задач не требуется», сверка ✅ подтверждена кодом
  (`core.rs:97,100,138,146,581,201,601`); `Q↔D21↔TRACEABILITY:15-18↔
  questions/README:32-35↔§10` согласованы; шапок `# D21 (Q8–Q10, Q42)` = 5;
  `docs/features/**` — только комментарии (5×3, 0 удалений), счётчики 278/47 =
  `features/README.md:303`; границы — 12 M + 6 `??`, `src`/`tests`/`Cargo.toml`/
  канон/`docs/tasks` не тронуты. **cargo не запускался — D50.** Отчёт
  `docs/reviews/migration-q8q10q42-2026-09-29.md`; квитанция `rework` — append;
  отчёт — в ленту. **Урок:** ссылки на ещё не перенесённые `Qn` (`Q11–Q41`)
  оформлять текстом ID, не markdown — иначе §5.7/§9 (живые ссылки) нарушается.
- **2026-09-29 · `service-migration-q11` — §«Следствия» D53 (H5, адресная):**
  `D53:63-65` — список шапочных пометок Q11 дополнен `errors.feature` (четыре
  фичи), совпадает с `Affects` (`:11-16`), `TRACEABILITY.md:18` и
  `errors.feature:5`; прочие секции без изменений — якоря сдвинуты ровно на +1
  (`## Сверка` 68→69, `## Альтернативы` 127→128, `## Ссылки` 139→140, объём
  148→149). **Согласовано — замечаний нет.** Квитанция не менялась;
  `cargo`/git не запускались. Отчёт в ленту.
- **2026-09-29 · `service-migration-q11` — закрытие P3 (H5, адресная):** `TRACEABILITY.md:18`
  — строка Q11 дополнена `errors.feature` (четыре фичи, ↔ `D53:11-16` `Affects` и
  шапке `errors.feature:5`); сверка полного файла с прежним снимком — изменена
  только строка 18. **P3 закрыт — принято, замечаний нет.** Квитанция не менялась;
  `cargo`/git не запускались (по указанию). Отчёт в ленту.
- **2026-09-29 · `service-migration-q11` (чекпойнт + итог, без cargo):** прочитаны
  лента `service-migration-q11.md`, `BRIEF.md` §2/§4/§5.3/§5.6/§5.7/§7,
  `review.md`, `Q11.md`, `D53-error-messages-language.md`, `OPEN_QUESTIONS.md:84-105`,
  `SPECIFICATION.md` §10 (:871-874), `TRACEABILITY.md:18`, `questions/README.md:35`,
  `CHANGELOG.md:40-76`, шапки 4 фич, `Q8.md:48-63`, `Q42.md:44-59`, `src/rest.rs`,
  `src/mcp.rs:36-67`, `src/lib.rs:427`, тесты. Снимок: `develop`, HEAD `c9193da` +
  рабочее дерево (13 M + 3 `??`); `git status` суженный по `src tests Cargo.toml
  AGENTS.md docs/tasks docs/features/README.md` — пусто; фичи — только
  комментарии шапок (+1/0 в каждой), счётчики 47/278 (`features/README.md:303`)
  → **cargo не запускался — D50** (исключение по фичам не сработало). Вердикт —
  **принято с замечаниями**; P1/P2 нет; **P3** — `TRACEABILITY.md:18` не
  перечисляет `errors.feature`, хотя `D53:15-16` держит её в `Affects`, а
  `errors.feature:5` несёт обратную ссылку `# D53 (Q11)` → односторонняя связь в
  канонической цепочке; правка — одна строка (`docs-writer`/`lead`). Проверено:
  указатель в архиве (`:90-93`, 4+/46-, полного текста нет), §10 №53 уникален и в
  стиле соседей, сверка ✅ подтверждена `rg` по `rest.rs:79,121,135,158,166,191,199`,
  `mcp.rs:54-66` (10 кодов) + `:43-46` (Q33), `lib.rs:427`, тесты
  (`rest.rs:95`, `common/mod.rs:222`, `mcp_errors.rs:13`), Q8/Q42 — по 1/-1,
  шапки `# D53 (Q11)` ×4, CHANGELOG `:63-76`, границы чистые. Отчёт
  `docs/reviews/migration-q11-2026-09-29.md`; квитанция `service-migration-q11`
  iteration 1 — append. **Урок:** при бонусных правках в чужих Q сверять не только
  замену пометки, но и «Feature»-колонку TRACEABILITY с `Affects` D-файла —
  обратная ссылка в фиче без строки в таблице даёт одностороннюю трассировку.
- **2026-09-29 · `service-migration-q7` — закрытие P3 (H5, адресная):** заголовок
  указателя `OPEN_QUESTIONS.md:66` приведён к виду соседей — `### Q7. ✅ Термины в
  глоссарии (решено 2026-09-26) → перенесён`; тело (`:68-69`) не изменено, ссылки
  живые. **P3 закрыт — принято, замечаний нет.** Квитанция не менялась; git/cargo
  не запускались (по указанию) — проверка файловая.
- **2026-09-29 · `service-migration-q12q15` (чекпойнт + итог, без cargo):**
  прочитаны лента `service-migration-q12q15.md`, образец
  `docs/reviews/migration-q11-2026-09-29.md`, `review.md`, архив `:90-134`,
  `Q12–Q15.md`, `D54`/`D14`/`D55`/`D56`, §10, `TRACEABILITY.md:19-22`,
  `questions/README.md`, `tasks/T-17`, `CHANGELOG.md:70-114`,
  `features/README.md:132-161`, шапки 13 фич, `Q7.md`, `D52:30-43`, `src`.
  Снимок: `develop`, HEAD `711a3c8` + рабочее дерево (24 M + 10 `??`); нумерация
  фич — 16 строк в 13 файлах, `+16/−0`; счётчики `features/README.md:308` —
  47/278 → **cargo не запускался — D50** (исключение по фичам не сработало).
  Вердикт — **отклонено (rework, iteration 1)**; **P1 нет**; **P2** — `Q15`
  остался в перечне «ожидают переноса» в трёх канонических файлах:
  `questions/Q12.md:14`, `decisions/D54-source-of-truth-flow.md:130-132`,
  `questions/README.md:36` (Q15 перенесён в этом же пакете, `Q15.md` + указатель
  `OPEN_QUESTIONS.md:119-122`) → ложный статус в каноне, правка `migrator` —
  3 строки; **P3** — `OPEN_QUESTIONS.md:104,109` «блок Q12–Q15» против
  `:114,119` «блок Q14–Q15»; `TRACEABILITY.md:19` без T-01 (есть `D54:18-21`,
  `tasks/README.md:38`). Проверено: архив (один хунк, 129 строк вырезано,
  4 указателя, полных текстов нет), §10 (`:835` №14 со ссылкой, `:875-877`
  №54–56 уникальны, numstat SPEC 4+/1−), Affects = колонка Feature (6/5/2/3),
  D-поля и «Сверка с кодом» (факты `lib.rs:342,383,397`, `mcp.rs:335-340`,
  `core.rs:71-81`), шапки `# Dn (Qx)` (16 в 13 фичах), T-17 (P2 — на
  подтверждение владельца), CHANGELOG `:77-104`, бонус Q7/Q11/README (Q13 →
  живая ссылка), границы (только `docs/**`, 3 памяти, лента; архив не пополнялся).
  Отчёт `docs/reviews/migration-q12q15-2026-09-29.md`; квитанция
  `service-migration-q12q15` iteration 1 (`rework`) — append. **Урок:** чек
  `rg "ожидает переноса"` в единственном числе пропускает групповые пометки
  «ожидают переноса» — проверять `ожида(ет|ют) переноса`; при многозвенных
  вызовах `migrator` сверять артефакты ранних вызовов после переноса соседних Q
  (метка «ожидает переноса» становится ложной).
- **2026-09-29 · `service-migration-q12q15` — повторная приёмка `-r2` (итог):**
  вердикт **принято, замечаний нет**; P2/P3 закрыты (`Q12.md:13-14`,
  `D54-source-of-truth-flow.md:130-133`, `questions/README.md:36` — Q13/Q15 живыми
  ссылками, под пометкой только Q29/Q30/Q32/Q33 и др.; `OPEN_QUESTIONS.md:106/111/
  116/121` + поля «Перенос» `Q12:12`/`Q13:11`/`Q14:9`/`Q15:10` = «блок Q12–Q15»;
  `TRACEABILITY.md:19` = T-16, T-08, T-01). Регрессий нет: `git diff --numstat --
  docs` per-file совпадает с iteration 1 (CHANGELOG 28/0, OPEN_QUESTIONS 19/129,
  SPEC 4/1, TRACEABILITY 4/0, features/README 5/0, фичи +16/0, Q11 2/1, Q7 1/1,
  README 6/2, tasks 1/0); §10 `:835/:875-877` и шапки фич без изменений; счётчики
  47/278 (`features/README.md:308`); длины Q13/Q14/Q15 + D14/D55/D56 совпадают с
  приёмкой, Q12 73, D54 138→139 (переразбивка «Связанных» при закрытии P2);
  границы 26 M + 11 `??` (`src`/`tests`/`Cargo.toml` чисты). **cargo не
  запускался — D50.** Отчёт `docs/reviews/migration-q12q15-2026-09-29-r2.md`;
  квитанция iteration 2 (`accepted`) — append. **Урок:** при in-place правках
  (перенос строк внутри абзаца, замена метки) `git diff --numstat` не отличает
  iteration 1 от iteration 2 — регрессии ловить чтением заявленных строк +
  `rg -c "^"` (число строк) по новым (untracked) файлам.
- **2026-09-29 · `service-migration-q16q19` (чекпойнт + итог, без cargo):**
  прочитаны лента `service-migration-q16q19.md`, образцы
  `docs/reviews/migration-q12q15-2026-09-29.md` + `…-r2.md`, `review.md`,
  архив `:92-148`, `Q16–Q19.md`, `D32/D57/D35/D58`, §10 `:845-879`,
  `TRACEABILITY.md:23-26`, `questions/README.md:35-43`, `CHANGELOG.md:100-132`,
  `features/README.md:125-169,298-313`, 7 шапок фич, `Q13.md`/`D14`,
  `tasks/README.md:39-53` + карточки T-02/T-06/T-17, `Q10`/`Q11`/`Q12`/`Q14`/
  `Q15` (свип). Снимок: `develop`, HEAD `5a7fb01` + рабочее дерево (19 M + 9
  `??`); `src`/`tests`/`Cargo.toml` в `git status` нет → **cargo не
  запускался — D50** (исключение по фичам не сработало: `+1/−0` строки шапок,
  счётчики 47/278 = `features/README.md:309`). Вердикт — **принято,
  замечаний нет**; P1/P2/P3 нет. Проверено: архив (4 указателя, полных
  текстов нет, numstat 19/173, метка «блок Q16–Q19» ×8), свип
  `ожида(ет|ют) переноса` (+многострочный) чист (Q16–Q19 не ожидают), D-поля
  ✅, «Сверка с кодом» D32 🟡/D57 🟡/D35 ✅/D58 ⚪, §10 №32 (`:853`) /№35
  (`:856`) /№57 (`:878`) /№58 (`:879`) уникальны и в стиле, `Affects` =
  колонка Feature (Q16 2, Q17 1, Q18 3, Q19 1), шапки `# Dn (Qx)` ровно в 7
  фичах, CHANGELOG `:105-130` после Q12–Q15, бонус `Q13.md:14`+`D14:55,146-148`
  (Q18 живой ссылкой), ссылки/ID живы, границы (только `docs/**`, 4 памяти,
  лента; архив не пополнялся). T-17 под Q17 — кросс-ссылка (источник D56/Q15),
  атрибутирована в `D57:94-95`; прецедент T-01 в Q12 / T-07 в Q7 → не находка.
  Отчёт `docs/reviews/migration-q16q19-2026-09-29.md`; квитанция
  `service-migration-q16q19` iteration 1 (`accepted`) — append. **Урок:**
  `git branch --show-current` вне allow-list `validator` (есть только
  `git branch --contains`) — HEAD брать из `git log --oneline`; свип «ожидают
  переноса» линеен — проверять и многострочной формой
  (`rg -n -U "Q1[6-9][^\n]*\n?[^\n]{0,60}ожида(ет|ют) переноса" docs`,
  ловит разрывы вроде `D54:132-133`).
- **2026-09-29 · `service-decisions-readme` (чекпойнт + итог, без cargo):**
  прочитаны лента `service-decisions-readme.md`, `questions/README.md` (образец),
  новый `docs/decisions/README.md`, `docs/README.md`, `docs/CHANGELOG.md`,
  `BRIEF.md:76-96,175,284-285`, `TRACEABILITY.md`, шапки 29 D-файлов (12
  выборочно), `receipts.yaml`. Снимок: `develop`, HEAD `e141476` + рабочее
  дерево (6 M + 2 `??`); `src`/`tests`/`Cargo.toml` нет → **cargo не
  запускался — D50**; счётчики 47/278 = `features/README.md:309` не менялись.
  Вердикт — **принято, замечаний нет**; P1/P2/P3 нет. Проверено: 29 строк
  (`rg -c "^\| \[D"`) ↔ 29 D-файлов (glob), каждый один раз, порядок возрастает;
  поля 12 D-файлов (D14/D16/D19/D21/D32/D35/D50/D52/D53/D54/D57/D58) = шапки;
  темы из `# Dn:` (D40 «триггер» = `D40:23`); все ссылки живы (29 D,
  Q1–Q19/Q42–Q56, T-02/06/07/08/10/11/12/13/14/16/17, `BRIEF.md`); шапка
  зеркальна `questions/README.md`, статусы = `BRIEF.md:95-96`, §5.2 =
  `BRIEF.md:175`; карта `:18-19` (выравнивание col 21) + `:48`; CHANGELOG
  `:131-136` после Q16–Q19 (`:105-130`); границы — `docs/decisions/README.md`
  (новый) + `docs/README.md` + `docs/CHANGELOG.md` + лента/память (включая 2
  закрывающие записи Q16–Q19: `mail/service-migration-q16q19.md:160-170`,
  `memory/service.md:236-242`). P3-кандидат (`decisions/README.md:17` — пропуски
  номеров как «до-журнальные»): `migrator` уточнил `:17-19` (до-журнальные +
  решения Q20–Q41, D-номера при переносе; прецедент D32/D35) **до отчёта**,
  принято повторной адресной сверкой: абзац в новой редакции, таблица 29 строк
  и шапка не двигались, 55 строк = 53 + 2. Отчёт
  `docs/reviews/service-decisions-readme-2026-09-29.md`; квитанция
  `service-decisions-readme` iteration 1 (`accepted`) — append. **Урок:**
  отказ движка прав не симметричен: `git diff -- docs/README.md` проходит, а
  `git diff -- .opencode/memory/...` и `git diff --stat` — нет (читать файлы
  `read`-ом, объём — `rg -c "^"`); `tail` тоже вне allow-list.
- **2026-09-29 · `service-migration-q20q23` (чекпойнт + итог, без cargo):**
  прочитаны лента `service-migration-q20q23.md`, образец
  `docs/reviews/migration-q16q19-2026-09-29.md`, `review.md`-контекст (по
  памяти), архив `:140-171`, `Q20–Q23.md`, `D22/D23/D25/D26`, §10 `:843-850`,
  `TRACEABILITY.md:27-30`, `questions/README.md:35,44-47`,
  `decisions/README.md`, `CHANGELOG.md:130-164`, 8 шапок фич, `Q11`/`D53`,
  `receipts.yaml`. Снимок: `develop`, HEAD `48c5b77` + рабочее дерево
  (17 M + 9 `??`); `src`/`tests`/`Cargo.toml` в `git status` нет → **cargo не
  запускался — D50** (исключение по фичам не сработало: `+N/0` строки шапок,
  счётчики 47/278 = `features/README.md:309`). Вердикт — **принято,
  замечаний нет**; P1/P2/P3 нет. Проверено: архив (4 указателя :148/153/158/163,
  метка «блок Q20–Q23» ×8, полных текстов нет, Q24 полный с :168; один хунк
  `-148,207 +148,19`, numstat 17/205), свип `ожида(ет|ют) переноса`
  (+многострочный) чист (Q20–Q23 не ожидают), D-поля ✅ у всех 4 («Сверка с
  кодом» ✅, «Задач не требуется»), §10 №22/№23/№25/№26 — ссылки D22/D23/D25/D26
  (маркеры/соседи целы, новые строки не нужны), `Affects` = TRACEABILITY = фичи
  (Q20 4, Q21 4, Q22 2, Q23 4), шапки 14 строк в 8 фичах без дублей, счётчики
  47/278 не задеты, `decisions/README.md` 33 строки (D22:33/D23:34/D25:35/D26:36;
  parenthetical :18 «Q24–Q41»), бонусы §1.3.1/§4.4 + `Q11` + `D53` (только
  ссылки), CHANGELOG `:137-160` после Q16–Q19, ссылки/ID живы, границы (только
  `docs/**`, 3 памяти, лента; `tasks`/`BRIEF`/`features/README` чисты). Наблюдение
  (не находка): прозаичные Q-пометки в шапках оставлены — прецедент Q11. Отчёт
  `docs/reviews/migration-q20q23-2026-09-29.md`; квитанция
  `service-migration-q20q23` iteration 1 (`accepted`) — append. **Урок:**
  `glob`-инструмент в этой среде может отклониться (`permission.rejected:
  shell`) — список файлов брать `rg --files <dir> -g "<glob>"` (одиночной
  командой), а не полагаться на `glob`.
- **2026-09-29 · `service-migration-q24q26` (чекпойнт + итог, без cargo):**
  прочитаны лента `service-migration-q24q26.md`, образец
  `docs/reviews/migration-q20q23-2026-09-29.md`, `review.md`, архив `:160-199`,
  `Q24–Q26.md`, `D24/D36/D37`, §10 `:835-864`, `TRACEABILITY.md:24-39`,
  `questions/README.md:33-52`, `decisions/README.md`, `CHANGELOG.md:155-189`,
  фичи + `features/README.md:270-311`, `receipts.yaml`. Снимок: `develop`,
  HEAD `87cbe13` + рабочее дерево (16 M + 7 `??`); `src`/`tests`/`Cargo.toml`
  в `git status` нет → **cargo не запускался — D50** (исключение по фичам не
  сработало: `+1/0` строки шапок, счётчики 47/278 = `features/README.md:309`).
  Вердикт — **принято, замечаний нет**; P1/P2/P3 нет. Проверено: архив
  (3 указателя `:168/173/178`, метка «блок Q24–Q26» ×6, полных текстов нет,
  Q27 полный `:183`, numstat 14/119), свип `ожида(ет|ют) переноса`
  (+многострочный) чист, D-поля ⚪ + «Задач не требуется» (Resolves по одному),
  §10 №24 `:847`/№36 `:859`/№37 `:860` — diff -U0 ровно 3 строки (только
  D-ссылки), `decisions/README.md` 36 строк (D24:35/D36:40/D37:41; parenthetical
  `:18` «Q27–Q41»; 36 ↔ 36 D-файлов), `Affects` = TRACEABILITY = фичи
  (`batch:9`/`client_explanation:10`/`import_export:7`), статусы ⏸/⏳
  (`:274-276`) и счётчики 47/278 не задеты, CHANGELOG `:161-182` после
  Q20–Q23, бонусы (`D22:147`, `Q20/Q21/Q23`) — только ссылки, границы чисты.
  Наблюдение (не находка): `D22:146` — `Q24` прозаичной ссылкой рядом с живым
  `[Q26]` (Q4/Q12/Q15 там же прозаичны → однострочной правкой не выровнять).
  Отчёт `docs/reviews/migration-q24q26-2026-09-29.md`; квитанция
  `service-migration-q24q26` iteration 1 (`accepted`) — append.
- **2026-09-29 · `service-migration-q28q29` («MCP-контракты»):** прочитаны лента,
  образец `migration-q24q26`, `review.md`, архив `:213-231`, `Q28/Q29.md`,
  `D31/D34`, §10 `:854/:857`, SPEC `:500-544`, `TRACEABILITY.md:33-36`,
  `questions/README.md:24-58`, `decisions/README.md`, фичи, `CHANGELOG.md:155-209`,
  памяти `migrator`/`docs-writer`/`service`, `receipts.yaml`. Снимок: `develop`,
  HEAD `2e0edcb` + рабочее дерево (28 M + 5 `??`); `src`/`tests`/`Cargo.toml` в
  `git status` нет → **cargo не запускался — D50** (счётчики 47/278 =
  `features/README.md:309`, состав фич не менялся). Вердикт — **отклонено
  (rework)**: P2 `D34:420` — `Q18 (semver)` в перечне «ожидают переноса», хотя
  Q18 перенесён (`D35:5`, `TRACEABILITY.md:25`, `README:41`, коммит `e141476`);
  свип `rg "ожида[ею]т переноса" docs` даёт ложный сигнал → правка одна строка
  (`migrator`) → `-r2`. Прецедент: P2 `service-migration-q12q15` (Q15 в
  «ожидает переноса»). Остальное ок: архив (`:215/:220`, Q30 цел, numstat
  `10/346`, второй хунк `@@ -1050 +714 @@` Q29→D34), D-поля (даты 2026-09-26,
  «Сверка с кодом» ✅/🟡, Tasks T-03/T-04/T-05), дрейф «полные схемы» снят
  (единый адрес D34: SPEC `:507/:525/:857`; OPEN_QUESTIONS в SPEC — только
  `:816`), `decisions/README.md` **38 строк** (D26→D31→D32→D34→D35; `:18`
  «(Q27, Q30–Q41)»), `Affects` = TRACEABILITY = 7 шапок в 6 фич, CHANGELOG
  `:183-205` после Q24–Q26, порядок блоков ленты, границы (`docs/**` + память +
  лента). Отчёт `docs/reviews/migration-q28q29-2026-09-29.md`; квитанция
  `service-migration-q28q29` iteration 1 (`rework`) — append. **Урок:** при
  свипе «ожидают переноса» проверять не только «нет ли наших Q», но и нет ли в
  перечнях **ранее перенесённых** Q (новые D могут тянуть stale-списки).
- **2026-09-29 · `service-migration-q28q29` — r2 (итог):** вердикт **принято,
  замечаний нет** (P1/P2/P3 нет). P2 закрыт одной правкой `migrator`:
  `D34:420-421` — `[Q18](../questions/Q18.md) / [D35](D35-semver-v01.md)`
  (semver), `Q33 (draft-first) — ожидает переноса`; `rg "ожида[ею]т переноса"
  D34` → 1 (`:421`, Q33), `rg "Q18 (semver) — ожидают" docs` → пусто; Q33 —
  полный блок в архиве `:360`. Регрессий нет: `D34` 427 → 428 строк (только
  правка; якоря `:1-419/:326/:390/:395/:422` совпадают со снимком r1);
  `git diff --numstat` всех отслеживаемых файлов пакета идентичен r1 (кроме
  памяти `migrator` 45→53 и моих `validator.md`/`receipts.yaml` — вне пакета).
  **cargo не запускался — D50** (кода нет; счётчики/состав фич не менялись).
  Отчёт `docs/reviews/migration-q28q29-2026-09-29-r2.md`; квитанция
  `service-migration-q28q29` iteration 2 (`accepted`) — append. **Приём:** для
  untracked D сверка «изменена ровно эта строка» — по приросту `rg -c "^"`
  (427→428) + совпадению якорных срезов + неизменному `numstat` остальных файлов.
- **2026-09-29 · `service-migration-q27q30` (4б-2 «Транспорт и запуск»,
  чекпойнт + итог, без cargo):** прочитаны лента, образцы
  `migration-q24q26/q28q29`, `review.md`, `BRIEF.md` §3/§5.3/§7, архив
  `:178-213`, `Q27/Q30.md`, `D27/D29`, §10 `:847-855`, `TRACEABILITY:31-38`,
  `questions/README`, `decisions/README`, фичи, `CHANGELOG:185-229`, handoff,
  `memory/service.md`, `receipts.yaml`. Снимок: `develop`, HEAD `bc76060`
  (синхрон с origin) + рабочее дерево (**16 M + 6 `??`**); `src`/`tests`/
  `Cargo.toml`/`docs/reviews` не тронуты → **cargo не запускался — D50**
  (счётчики 47/278 = `features/README.md:309`, состав фич не менялся).
  Вердикт — **отклонено (rework, iteration 1)**: **P2**
  `D29-notebook-mcp-transport.md:7,40-41,68-71,105` — пары «§-раздел SPEC ↔
  строки» сдвинуты на один раздел (§2.2↔строки §2.3; §2.3↔`:267–270` §4.1 —
  диапазон невозможен; §4.1↔`:286/:306–307` §4.2; `Affects` без §4.2
  `:303–317`); карта `rg -n "^#{1,3} " docs/SPECIFICATION.md` (§2.2=127,
  §2.3=143, §4.1=253, §4.2=281). **P3** лента `:169` — «34 указателя» против
  фактических **31** (`rg -c "→ перенесён"` архива). Остальное ок: архив
  (`:183`/`:203` указатели, метка «блок 4б-2…», Q28/Q29 и Q31+ целы, numstat
  10/57), §10 №27 `:850`/№29 `:852` (D-ссылки, маркеры и соседи целы),
  `decisions/README` 40 строк (D27 `:38`/D29 `:39`, `:18` «Q31–Q41»),
  TRACEABILITY `:34`/`:37`, `questions/README` `:51`/`:54`, D-поля + «Сверка» ✅
  (улики `main.rs`/`mcp.rs` верны), свип «ожида[ею]т переноса» (Q27/Q30 нет,
  субъекты Q31+; мини-правка D54/D56), кросс-ссылки/битые ссылки (нет), фичи
  (1×`# D29 (Q30)`, нота Q30, счётчики), CHANGELOG `:206-223`, порядок ленты,
  границы. Отчёт `docs/reviews/migration-q27q30-2026-09-29.md`; квитанция
  `service-migration-q27q30` iteration 1 (`rework`) — append. **Урок:** в D-файле
  сверять не только «ссылки живые», но и **пары «§-раздел ↔ строки»** по карте
  заголовков SPEC — сдвиг на один раздел не ловится проверкой существования
  файлов/ссылок.
- **2026-09-29 · `service-migration-q27q30` — r2 (итог): принято, замечаний
  нет** (P1/P2/P3 нет). P2 закрыт `migrator` по варианту «б»: разделы
  §2.2/§2.3/§4.1 оставлены намеренными (как «Обновлено» в `Q30.md:59`), строки
  приведены к ним — `§2.2 (:134)`, `§2.3 (:178, :182–187)`, `§4.1 (:267–270)` —
  и **добавлен §4.2 (`:303–317`; Q30 `:307`, схема `:286`, «правок не
  требуется» `:318–319)`**; пары в `D29:7-8/41-42/70-77/110` внутри разделов
  (карта §2.2=127–142, §2.3=143–190, §4.1=253–280, §4.2=281–340), сдвинутых пар
  нет; лента `:125/:131-132` выровнена. P3 закрыт: лента `:170` — «31 указатель»
  (`rg -c "→ перенесён"` = 31). Регрессий нет: `numstat` tracked-файлов пакета
  идентичен r1; изменены только `D29` (untracked, 106 → 111 строк, якоря
  `:1-33/:39-53/:55-108` целы), лента и память `migrator` (56 → 67). **cargo не
  запускался — D50.** Отчёт `docs/reviews/migration-q27q30-2026-09-29-r2.md`;
  квитанция `service-migration-q27q30` iteration 2 (`accepted`) — append.
  **Приём r2:** для untracked D регрессия — по приросту строк (`rg -c "^"`) +
  совпадению якорных срезов + неизменному `numstat` остальных файлов; при
  выборе варианта правки «раздел ↔ строки» фиксировать, какой вариант принят
  (здесь «б» — разделы намеренны), иначе фикс неоднозначен.
- **2026-09-29 · `service-migration-q31q33` («Notebook-функции», Q31→D12 /
  Q33→D30; чекпойнт + итог, без cargo):** прочитаны лента
  `service-migration-q31q33.md`, `review.md`, `BRIEF.md` (§3/§5.6),
  архив `:200-309`, `Q31/Q33.md`, `D12/D30`, §10 `:828-859`,
  `TRACEABILITY:33-44`, `questions/README:33-60`, `decisions/README`,
  фичи + `features/README:225-329`, `CHANGELOG:200-249`, `receipts.yaml`,
  лента/память q27q30 (F43-записи). Снимок: `develop`, HEAD `ce9f8d7`
  (**ahead 1** от `origin/develop` = `bc76060`) + рабочее дерево (**24 M +
  5 ??**); `src`/`tests`/`Cargo.toml`/`docs/reviews`/`docs/analysis`/`tasks`
  не тронуты → **cargo не запускался — D50** (счётчики 47/278 =
  `features/README.md:309`, состав сценариев не менялся — только строки-
  комментарии `# Dn (Qn)`). Вердикт — **принято с замечаниями**, iteration 1;
  **P1/P2 нет**; **P3** `D30:7-13` — `Affects` только §10, тогда как решение
  фиксирует §2.2/§4.1/§4.2/§7 (сам файл `:47-49/:142-143`, `Q33.md:16-17`);
  улика симметрии — `D27:7-8`, `D29:7-10`, `D32:8-9`, `D36:7-10` (разделы в
  `Affects`). P3 закрыт адресно `migrator`: `:7` = `§2.2, §4.1, §4.2, §7 и §10`;
  регрессий нет (D30 144 строки, правка только строки `Affects`; `numstat`
  tracked-файлов идентичен снимку; D12 119 / Q31 48 / Q33 63). Остальное ок:
  архив (указатели `:213`/`:284`, метка «блок «Notebook-функции», Q31+Q33»,
  Q32/Q34 целы, numstat 10/74, `rg -c "→ перенесён"` = 33), §10 `:835/:853`
  (diff ровно 2 строки, маркеры/соседи целы), `decisions/README` **42 строки**
  (D12 `:27` первой, D30 `:41` между D29/D31; `:18` «(Q32, Q34–Q41)»),
  TRACEABILITY `:38/:39`, `questions/README` `:55/:56` (+5/−3), 4 мини-правки
  (`D58:103`, `Q19.md:12`, `questions/README:43/:49`), свип «ожида[ею]т
  переноса» чист (subject Q32/Q34+/Q36/Q37), D12 ⚪ / D30 🟡 с верными уликами
  (`mcp.rs:143/:214/:282/:470`, `VersionDeprecated:43-46`, `stale_false…:772`;
  `core.rs:136/:153`; `tests/mcp_draft.rs:172/:352/:367`), кросс-ссылки живые,
  7 шапок фич, ноты `:229/:240`, CHANGELOG `:224-246`, границы. Отчёт
  `docs/reviews/migration-q31q33-2026-09-29.md`; квитанция
  `service-migration-q31q33` iteration 1 (`accepted_with_notes`) — append.
  **Урок:** у D-файла сверять не только ссылки/улики, но и **полноту поля
  `Affects`** относительно его же «Краткого канона» и «Следствий» (раздел SPEC,
  упомянутый там, но отсутствующий в `Affects`, — P3; правка одна строка).
  **Приём:** правку в **untracked** D (`git diff` пусто) подтверждать чтением
  строки + неизменной длиной файла (`rg -c "^"`) + идентичным `numstat`
  остальных файлов пакета. Наблюдение (не находка): запись `migrator` о закрытии
  P3 вставлена в ленту перед записью `docs-writer`, а не в конец — strict append
  нарушен, на вердикт не влияет.
- **2026-09-29 · `service-migration-q32` («Git-контур», Q32→D28; чекпойнт +
  итог, без cargo):** прочитаны лента `service-migration-q32.md`, `review.md`,
  `BRIEF.md` §5.3/§7, архив `:210-299`, `Q32.md`, `D28-two-git-contours.md`,
  §10 `:845-858`, `TRACEABILITY.md`, `questions/README.md`,
  `decisions/README.md`, 8 фич + `features/README.md`, `CHANGELOG.md:238-269`,
  `D14`, `D31`, `D55`, `T-06`, `receipts.yaml`. Снимок: `develop` @ `61d6471`
  (= `origin/develop`) + рабочее дерево (**36 M + 3 `??`**); `src`/`tests`/
  `Cargo.toml` в `git status` нет → **cargo не запускался — D50** (в фичах
  только по одной строке-комментарию `# D28 (Q32)`, счётчики 47/278 =
  `features/README.md:309`). Вердикт — **принято, замечаний нет**; P1/P2/P3 нет.
  Проверено: архив (`:219–223` указатель, метка «блок «Git-контур», Q32»,
  полного текста нет, Q34 полный `:231`, numstat 5/64, один хунк), §10 №28
  `:851` `Q32 (полный контекст — [D28])`, diff ровно 1 строка, второй столбец
  и соседи `:850/:852` целы, `decisions/README` **43 строки** (D28 `:40` между
  D27/D29; `:18` = «(Q34–Q41)»), `TRACEABILITY:39`, `questions/README:56`
  (+7/−6, свип :37/:39/:41/:42/:50/:54), D28-поля + «Сверка с кодом» 🟡 (улики
  подтверждены чтением: `src/lib.rs:342/:383/:462/:317`, `list_from_ref`
  `:241`/`:245–248` — 3 сегмента; `src/mcp.rs:282/:344/:399/:413`;
  `src/rest.rs:25–29`; тестов пути нет; T-06 ⬜ :3, Источник Q13/Q32 :6),
  свип «ожида[ею]т переноса» (Q32/Q31/Q33 не под маркерами; субъекты только
  Q34–Q41; `D14:50-51`/`:148` — живые `[Q32]` + Q34-маркер; `D31:151`
  «Q34+ — ожидают переноса»), кросс-ссылки живые (целевые файлы существуют:
  Q1–Q56, D12–D58, T-01…T-17), ровно 8 строк `# D28 (Q32)` без дублей
  (каждая 1/0), ноты `:98/:219` со ссылками, счётчики не задеты, CHANGELOG
  `:247–263` после Q31/Q33, порядок ленты, границы. Отчёт
  `docs/reviews/migration-q32-2026-09-29.md`; квитанция
  `service-migration-q32` iteration 1 (`accepted`) — append. Наблюдение (не
  находка): в §10 №28 форма `Q32 (полный контекст — [D28])` отличается от
  соседской «тема (Qxx; полный контекст — [Dnn])», но оригинал начинался с
  `Q32:` — правка минимальна и текст не теряет. **Приём:** у untracked D
  (`Q32.md`/`D28-…md`) регрессия — не применима (r1); счёт «43 строки» сводки
  и «8 фич» проверять `rg -c`/`rg -l`, а живой маркер перенесённого Q —
  сверять парой «`[Q32]`-ссылка + соседний `Q34 (ожидает)`» (см. `D14:148`).
- **2026-09-29 · `service-migration-q34`** («Тесты», особый случай — расширение
  D32; чекпойнт + итог, без cargo): прочитаны лента, `review.md`, `BRIEF.md` §7,
  архив `:218-282`, `Q34.md`, `D32`, §10 `:848-857`, `TRACEABILITY:26-42`,
  `questions/README:33-58`, `decisions/README`, 4 фичи, `features/README`,
  `CHANGELOG:252-283`, `receipts.yaml`. Снимок: `develop` @ `4af0b1f`
  (= `origin/develop`) + рабочее дерево (**20 M + 2 `??`**); `src`/`tests`/
  `Cargo.toml` в `git status` нет → **cargo не запускался — D50**. Вердикт —
  **принято, замечаний нет**; P1/P2/P3 нет. Проверено: нового D-файла нет
  (`git status`: в `docs/decisions` только `M D32`); `D32:5` `Resolves: [Q16],
  [Q34]`, `:18` T-02 (открыта), все 5 пунктов Q34 (`:38-40/:39/:49-50/
  :107/:30-34,:47/:50-52`), `Affects` полон; архив — один хунк `-231,36
  +231,6`, метка «блок «Тесты»; в рамках D32», Q35 цел, numstat 6/36; §10 №32
  `:855` — diff ровно 1 строка (маркер снят), соседи `:854/:856` целы;
  `decisions/README` **43 строки таблицы** (D32 `:44` = `[Q16], [Q34]`; `:18` =
  «(Q35–Q41)»); TRACEABILITY `:41`; `questions/README` `:58` + свип `:40/:43`;
  свип «ожида[ею]т переноса» — субъекты только Q35–Q41, Q34 не под маркером,
  `D31:151` = «Q35+»; правки пометок → живые `[Q34]` (`D14:148`, `D58:103`,
  `Q13:14`, `Q16:13`, `Q19:12`); 4 шапки `# D32 (Q34)` без дублей, `# D32
  (Q16)` целы; ноты `features/README:130/:236`; счётчики `:309` 47/278 не
  менялись; CHANGELOG `:264-272`; границы. Отчёт
  `docs/reviews/migration-q34-2026-09-29.md`; квитанция `service-migration-q34`
  iteration 1 (`accepted`) — append. **Урок:** при ожидании «нового D-файла
  нет» не полагаться на `rg … Dn` — номер `Dn` может быть занят ранее
  перенесённым решением другой строки §10 (здесь `D34` = Q29); отсутствие
  нового файла подтверждать `git status` (`M`/`??` в `docs/decisions`).
  **Приём:** «43 строки» сводки `decisions/README` — это строки **таблицы**
  (`rg -c '^\| \[D'`, сейчас D-строки `:27-69`), а не строки файла (69).
- **2026-09-29 · `service-migration-q35` («Создание workspace и шаблоны»,
  Q35→новый D59; чекпойнт + итог, без cargo):** прочитаны лента
  `service-migration-q35.md`, `review.md`, образец `migration-q34`, архив Q35
  (через `git diff -U0`; `git show` срезан token-guard), `Q35.md`, `D59`, §10
  `:876-884`, `TRACEABILITY:39-43`, `decisions/README:14-70`,
  `questions/README` (+diff), `Q19.md`, `D58`, `D31`, фичи, `features/README`
  `:199-210/:305-310`, `CHANGELOG:260-293`, `receipts.yaml`, `src`-улики
  (`main.rs:28-57`, `lib.rs:772-821`, `mcp.rs:770-809`). Снимок: `develop`
  @ `e0e06f7` + рабочее дерево (**14 M + 4 `??`**); `src`/`tests`/`Cargo.toml`/
  `docs/tasks`/`docs/reviews` не тронуты → **cargo не запускался — D50**
  (в фичах по одной строке-комментарию `# D59 (Q35)`, счётчики 47/278 =
  `features/README.md:309`). Вердикт — **принято, замечаний нет**; P1/P2/P3 нет.
  Проверено: D59 (все 4 пункта Q35 `:29-58` + «Следствия» `:60-76`; `Resolves:
  [Q35]` `:5`; `Tasks: —` `:17` + «Задач не требуется» `:115-119`; ⚪ сверка
  `:80` — конвенция `D58:58`); архив — один хунк `-238,41 +238,5` (+5: указатель
  `:238-242`, метка «блок «Создание workspace и шаблоны»», Q34-указатель
  `:231-236` и Q36 полный `:244` целы); §10 **№59** `:882` (diff ровно 1 строка;
  №58 `:881`, пустая `:883`, `---` `:884`); `decisions/README` **44 строки**
  (D59 `:70` после D58 `:69`; `:18` «(Q36–Q41)»; 44 = 44 D-файла);
  `TRACEABILITY:42` (Q34 `:41`, Q42 `:43`); `questions/README:59` + свип Q19
  `:43`; свипы `D31:151` «Q36+», `D58:103`/`Q19.md:12` — живые `[Q35]`; SPEC
  `:347/:351/:375` — канон (нота без D-ссылки = стиль Q19/Q34/Q37); ровно
  1×`# D59 (Q35)` на файл, прежние `# Q35`/`# D32 (Q34)` целы; `features/README`
  не менялся; CHANGELOG `:273-286` после Q34 `:264-272`, до `### Процесс` `:288`;
  свип «ожида[ею]т переноса» — субъекты только Q36–Q41, Q35 нет; ссылки живые
  (T-10, D50, GRAMMAR §1, D12/D30/D32/D58); границы и порядок ленты. Отчёт
  `docs/reviews/migration-q35-2026-09-29.md`; квитанция `service-migration-q35`
  iteration 1 (`accepted`) — append. **Приём:** архивный блок сверять не
  `git show` (срезается token-guard на ~25 КБ из 42), а `git diff -U0` целевого
  файла — он даёт удалённые строки блока точно и целиком.
- **2026-09-29 · `service-migration-q36q38` («Конвейеры + LSP-состав»,
  Q36+Q38→новый D18; итог, без cargo):** прочитаны лента
  `service-migration-q36q38.md`, `review.md`, образец `migration-q35`, архив
  Q36/Q38 (через `git diff -U0 -- docs/OPEN_QUESTIONS.md`), `Q36.md`, `Q38.md`,
  `D18`, §10 `:836-845`, `TRACEABILITY:42-45`, `decisions/README:15-33`,
  `questions/README:43-62`, `CHANGELOG:262-305`, фичи (`lsp`/`graph_view`),
  `features/README:65-94`, свипы (`D17/D22/D31/D36/D37`, `Q10/Q20/Q21/Q24/Q25`),
  `memory/service.md`, `receipts.yaml`. Снимок: `develop` @ `681c4b1`
  (= `origin/develop`) + рабочее дерево (**24 M + 4 `??`**); `src`/`tests`/
  `Cargo.toml`/`docs/tasks`/`docs/reviews` не тронуты → **cargo не запускался —
  D50** (в фичах по одной строке-комментарию `# D18 (…)`, счётчики 47/278 =
  `features/README.md:311`). Вердикт — **принято с замечаниями**; P1/P2 нет;
  **P3** `docs/questions/Q25.md:15` — `Q37` остался без пометки «ожидает
  переноса» после снятия Q36 из группового маркера (Q37 ещё в архиве
  `OPEN_QUESTIONS:250`); consequence — локальная потеря адреса в sweep-карте
  Q37 (центральные маркеры целы: `D31:151`, `D37:113`, `Q10:11/:62`,
  `questions/README:49`, `decisions/README:18`); правка одна строка. Остальное
  ок: Q36.md/Q38.md/D18 созданы, `Resolves: [Q36], [Q38]` `:5`, все пункты
  обоих блоков + «Обновить» в D18 `Affects` (урок q31q33), `Tasks: —` ↔ «Задач
  не требуется» `:107` ↔ ⚪ `:76` (конвенция D58:58/D59:80); архив — 2 хунка
  `-244,15`/`-304,19`, указатели `:244`/`:294` с меткой «Конвейеры +
  LSP-состав», Q37 полный `:250`, Q39 `:300`, numstat 10/34; §10 **№18**
  `:841` — diff ровно 1 строка (соседи №17 `:840`/№19 `:842` целы);
  `decisions/README` **45 строк** (D18 `:32` между D17 `:31` и D19 `:33`;
  `:18` «(Q37, Q39–Q41)»; 45 = 45 D-файлов); `TRACEABILITY:43/:44` (Q35 `:42`,
  Q42 `:45`; Feature Q36 = lsp+graph_view, Q38 = lsp); `questions/README:60/:61`
  (Q35 `:59`, Q42 `:62`) + свипы `:44/:45/:48/:49` ([Q36] живые, Q37-маркер
  `:49` цел); свип `D31:151` «Q37+», `D36:100`, `D37:84/:113`, `Q10:11/:62`
  («Q37, Q39»); `D17:107`/`D22:148` — живые `[Q36]`/`[Q38]`; фичи ровно 1×
  `# D18 (Q36, Q38)` (`lsp.feature:6`, комментарий Q37 `:3-5` цел) и 1×
  `# D18 (Q36)` (`graph_view.feature:2`); нота `features/README:83-85`
  (refs живые), счётчики 47/278 не менялись; CHANGELOG `:287-303` после Q35
  `:273-286`, до `### Процесс` `:305`, ссылки живые; q35-поправка
  (`mail/service-migration-q35.md`, `memory/git.md`) вне этого блока
  (`memory/service.md` — обе записи). Наблюдения (не находки): `D37:84`
  `(конвейеры))` — двойная `)` корректна (вложенность внутр. + внеш. скобки),
  читаемо; `D37:112` — строка выросла до ~95 символов (перенос не однострочный);
  в задании «D19 `:32`» — off-by-one, фактически `:33`. Отчёт
  `docs/reviews/migration-q36q38-2026-09-29.md`; квитанция
  `service-migration-q36q38` iteration 1 (`accepted_with_notes`) — append.
  **Урок:** при снятии Q из **группового** маркера «— ожидают переноса»
  проверять, не потеряли ли пометку остальные Q группы (особенно когда рядом
  нет второй пометки, в отличие от `D37:84`+`:113`).
- **2026-09-29 · `service-migration-q36q38` — P3 закрыт (итог):** вердикт блока
  в силе — **принято с замечаниями** (P1/P2 нет, P3 закрыт). `migrator`:
  `docs/questions/Q25.md:15` = `Q37 (БД/справочники) — ожидает переноса;
  [Q36](Q36.md) (конвейеры);` — пометка восстановлена, читается корректно,
  соседние ссылки не задеты. Проверено: `rg "Q37|ожида[ею]т переноса"
  Q25.md` — «ожидает переноса» ровно 1× (`:15`), Q37 `:51/:62` — проза;
  `git diff --numstat -- docs/questions/Q25.md` = `1/1`; `git diff --numstat --
  docs/questions` идентичен снимку приёмки (прочие файлы не задеты); группы
  маркеров целы (`questions/README:49`, `D31:151`, `D37:113`, `Q10:11/:62`).
  Отчёт `docs/reviews/migration-q36q38-2026-09-29.md` дополнен разделом
  «Обновление 2026-09-29 — P3 закрыт»; квитанция `service-migration-q36q38`
  iteration 1 дополнена полем `p3_closure` (дальнейших записей не писал — по
  прецеденту q31q33 P3-закрытие не создаёт новую итерацию). **Приём:** закрытие
  P3 подтверждать не только чтением строки, но и `numstat` целевого файла +
  `numstat` каталога против снимка приёмки (ловит случайные правки соседей).
- **2026-09-29 · `service-migration-q37` («Реестр полей», Q37→новый D33; итог,
  без cargo):** прочитаны лента `service-migration-q37.md`, образец
  `migration-q36q38`, `review.md`, архив (через `git diff -U0`), `Q37.md`,
  `D33`, §10 `:854-857`, `TRACEABILITY:43-45`, `questions/README:44-62`,
  `decisions/README`, фичи, `features/README:249,311`, `CHANGELOG:292-320`,
  свипы, `receipts.yaml`. Снимок: `develop` @ `b536df0` (= `origin/develop`) +
  рабочее дерево (**18 M + 3 `??`**); `src`/`tests`/`Cargo.toml`/`docs/tasks`/
  `docs/reviews` не тронуты → **cargo не запускался — D50** (счётчики 47/278 =
  `features/README.md:311`). Вердикт — **принято, замечаний нет** (P1/P2/P3
  нет). Проверено: архив (один хунк `-250,43 +250,5`; указатель `:250-254`
  «блок «Реестр полей»», Q36/Q38/Q39 целы, порядок; numstat 5/43), 6 пунктов +
  «Следствие для кода» + «Обновлено» → `Q37.md:50-82`/`D33:34-60`, `Affects`
  полон (урок q31q33 не воспроизвёлся), `Tasks: —` ↔ «Задач не требуется» ↔ ⚪,
  §10 №33 `:856` diff 1 строка (соседи №32/№34 целы), `decisions/README` **46**
  (D33 `:46` между D32/D34, `:18` «Q39–Q41»; 46 = 46 D-файлов), TRACEABILITY
  `:44`, `questions/README:61`, свип Q25 `:49` живая, свип `ожида[ею]т
  переноса` — только Q39 (Q40/Q41 без маркеров, Q37 нигде), адресные правки
  (`D31:151`, `Q10:11/:62`, `D37:112-113`, `D18:135`, `Q38:12-13`, `Q36.md`
  чист), фичи 1×`# D33 (Q37)` каждая (прежние пометки целы), нота
  `features/README:249`, `client_explanation` не тронут (проза, не адресат),
  CHANGELOG `:304-318` после Q36/Q38 до `### Процесс`, границы и порядок ленты.
  Наблюдения (не находки): `OPEN_QUESTIONS:390` диапазон `Q31–Q37` — перечень
  ID, не маркер (как Q36–Q39 `:391`); внешний `DECISIONS.md` в `Affects` без
  ссылки — корректно. Отчёт `docs/reviews/migration-q37-2026-09-29.md`;
  квитанция `service-migration-q37` iteration 1 (`accepted`) — append.
- **2026-09-29 · `service-migration-q39` («LSP-поведение», Q39→новый D6;
  чекпойнт + итог + P3-закрытие, без cargo):** прочитаны лента
  `service-migration-q39.md`, образец `migration-q37`, `review.md`,
  `BRIEF.md` §5.3, архив (через `git diff -U0`), `Q39.md`, `D6`, §10 `:829`,
  `TRACEABILITY:46`, `questions/README:63`, `decisions/README`, фичи,
  `features/README:213,256,311`, `CHANGELOG:304-336`, `receipts.yaml`. Снимок:
  `develop` @ `710eb7b` (= `origin/develop`, `git status -sb` без ahead/behind)
  + рабочее дерево (**17 M + 3 `??`**); `src`/`tests`/`Cargo.toml`/`docs/tasks`/
  `docs/reviews` не тронуты → **cargo не запускался — D50** (тест
  `features_inventory` считает строки `Сценарий:` и парсит README, комментарий
  `# D6 (Q39)` на него не влияет; счётчики 47/278 = `features/README.md:311`).
  Вердикт — **принято с замечаниями** (P1/P2 нет); **P3**
  `D6-lsp-degradation.md:91-93` — ссылки на строки сценариев
  `lsp_notebook.feature` устарели на +1 после вставки `docs-writer` строки
  `# D6 (Q39)` в `:6` (HEAD-факт `38/47/55` → текущие `39/48/56`); P3 закрыт
  адресно (`:39–46`, `:48–54`, `:56–61`; D6 136 строк — только числа;
  `rg "38–45|47–53|55–60"` пусто), вердикт в силе, `p3_closure` в квитанции
  (без новой итерации — прецедент q31q33/q36q38). Проверено: архив (один хунк
  `-262,38 +262,5`, указатель `:262-266` «блок «LSP-поведение»», Q36/Q37/Q38
  указатели целы, Q40 полный `:272`, порядок; numstat 5/38), все 4 пункта +
  «Следствие» + «Обновлено» → `Q39.md:50-80`/`D6:31-68`, `Affects` полон, ⚪ +
  «Задач не требуется», §10 №6 diff ровно 1 строка (текст решения не переписан),
  `decisions/README` **47 = 47** (D6 первой строкой `:27`, `:18` «(Q40–Q41)»),
  `TRACEABILITY:46`, `questions/README:63` (между Q38/Q42), свип
  `ожида[ею]т переноса` (вкл. многострочный) — ровно 1 (`D31:151` «Q40+»), фича
  1×`# D6 (Q39)`, нота `:256`, счётчики не менялись, CHANGELOG `:319-334` за Q37.
  **Особый свип (OAuth ↛ Q39): принято, замечаний нет** — `D26:25/:107/:115`,
  `Q22:11-12`, `questions/README:46` (узел убран целиком), живых связок нет,
  реальные `[Q39]` целы, тексты только цитатно, источник подтверждён
  `48c5b77:docs/OPEN_QUESTIONS.md:267` = «OAuth/JWT/mTLS … (Q39).». Отчёт
  `docs/reviews/migration-q39-2026-09-29.md`; квитанция `service-migration-q39`
  iteration 1 (`accepted_with_notes`) + `p3_closure` — append. **Урок: при сверке
  `Dn` проверять не только живые ссылки, но и внутренние ссылки на строки чужих
  файлов — они ломаются последующими правками того же блока (вставка шапочной
  строки `+1`); сверять адрес против фактических границ (`rg -n "Сценарий:"`) и
  `rg -c "^"` целевого файла против снимка приёмки.**
- **2026-09-29 · `service-migration-q40q41` («Процесс», финал архива; чекпойнт +
  итог, без cargo):** прочитаны лента `service-migration-q40q41.md`, образец
  `migration-q39`, `review.md`, архив (`git diff -U0`), `Q40/Q41.md`,
  `D20/D60`, §10 `:811-885`, `TRACEABILITY:44-51`, `questions/README:62-67`,
  `decisions/README`, фичи, `features/README:10-25/282-335`, `CHANGELOG:330-353`,
  `ci.yml`, `tests/features_inventory.rs`, `receipts.yaml`. Снимок: `develop`
  @ `8fa027e` (= `origin/develop`, без ahead/behind) + рабочее дерево
  (**15 M + 6 ??**); `src`/`tests`/`Cargo.toml`/`docs/tasks`/`docs/reviews`/
  `.github`/`BRIEF.md`/`docs/README.md` не тронуты → **cargo не запускался —
  D50** (в фичах только строки-комментарии, состав/счётчики 47/278 =
  `features/README.md:313`). Вердикт — **принято с замечаниями** (P1/P2 нет).
  Проверено: §10 №20 `:843` diff 1 строка + №60 `:883` + `[D60]` `:811`
  (numstat 3/2), архив (один хунк `-272,73 +272,11`, указатели `:272`/`:278`,
  42 указателя, матрица `:286-297`, шапка цела), `decisions/README` 49 = 49
  (+`rg --files` 49) D20 `:36`/D60 `:76`, `:6-7` не тронуты, TRACEABILITY
  `:47-48`, `questions/README` `:64-65`, Q5/D19/D31 — живые ссылки, свипы
  обеих форм + многострочный чисты (вне reviews; «не перенесённ» — только
  финишные `decisions/README:7`/`SPEC:815`), фичи/CHANGELOG, границы,
  ссылки теста/CI/SPEC в D20 (кроме трёх устаревших). **P3:**
  `D20:81-82` — `:12–20`→`:12–21`, `:331`→`:333`, `:284`→`:285–286`
  (сдвиг от вставок `docs-writer` в `features/README`); `D60:107` —
  `OPEN_QUESTIONS.md:341` не разрешается (Q41→указатель `:278-282`, `:341` =
  матрица «Аутентификация | Q22»). Отчёт
  `docs/reviews/migration-q40q41-2026-09-29.md`; квитанция
  `service-migration-q40q41` iteration 1 (`accepted_with_notes`) — append.
  **Урок:** в D-файле адреса на `features/README.md` ломаются любым дописыванием
  шапки/строк фич тем же пакетом — для untracked D сверять каждое число против
  фактического `rg`-адреса, а не против снимка HEAD; ссылка-улика на **архивный**
  блок не переживает замену блока указателем (архив правится в том же пакете).
  **P3 закрыт 2026-09-29 (адресно, без новой итерации):** D20 `:81-82` —
  `:12–21`/`:333`/`:285–286`; D60 `:107` — `OPEN_QUESTIONS.md:341` →
  `[Q41](../questions/Q41.md)` (`:51–55`); `rg ":12–20|:331|:284|
  OPEN_QUESTIONS\.md:341"` вне reviews пусто; `rg -c "^"` D20 118 / D60 145 /
  Q41 59 (без изменений — in-place); вердикт в силе (`accepted_with_notes`),
  `p3_closure` в квитанции. **Приём:** для untracked D регрессия правки — по
  неизменной длине (`rg -c "^"`) + точечному `rg`-срезу адресов + неизменному
  `numstat` трекаемых файлов (как q28q29-r2/q27q30-r2).
- **2026-09-29 · `service-docs-hygiene` шаг 1 (Q57–Q61; чекпойнт + итог, без
  cargo):** прочитаны лента `service-docs-hygiene.md`, `BRIEF.md` §4/§5.1/§7/§9,
  `Q57–Q61.md`, `Q56.md`, `TRACEABILITY.md`, `questions/README.md`,
  `CHANGELOG.md`, `clean-logs.mjs`, `receipts.yaml`. Снимок: `develop`
  @ `a7359fe` (= `origin/develop`, без ahead/behind) + рабочее дерево
  (**5 M + 6 ??**); `src`/`tests`/`Cargo.toml` в `git status` нет → **cargo не
  запускался — D50** (исключение по фичам не сработало: `features/**` не в
  пакете, счётчики 47/278 = `features/README.md:313`). Вердикт — **принято,
  замечаний нет**; P1/P2/P3 нет. Проверено: шаблон §4/§5.1 у всех 5 Q (Статус
  `open`, 2026-09-29, ⚪; нет полей «Перенос»/`Resolves`/«Итог»), D-файлов и
  §10-строк нет (`rg "Q5[7-9]|Q6[01]" SPEC decisions` пусто), темы = состав
  операции; TRACEABILITY `:64-68` (после Q56 `:63`/до легенды `:70`), 61 Q-строка
  (было 56), `questions/README` `:81-85` (5 колонок, `—`/`open`/перекрёстные),
  61 Q-файл; CHANGELOG `:352-363` (после `:335-351`, до `### Процесс` `:365`;
  numstat 12/0), ссылки живые (лента `:353`, Q `:362-363`), формулировок
  решений нет; факты: `BRIEF.md:321/:323/:372`, BRIEF 395 строк / 11 ссылок на
  архив, 42 Q-файла с «Перенос:», F46 (`findings-registry.md:54`); границы
  чисты. Отчёт `docs/reviews/docs-hygiene-q57q61-2026-09-29.md`; квитанция
  `service-docs-hygiene-step1` iteration 1 (`accepted`) — append. **Наблюдение
  (не находка):** ссылки новых Q-файлов и CHANGELOG на
  `.opencode/mail/service-docs-hygiene.md` умрут после `clean-logs.mjs`
  (удаляет `mail/*.md`, `:7,110-113`); прецедент `questions/README.md:80`
  (Q56), политика ссылок на рабочие данные — предмет Q61 → **не правил**,
  зафиксировал в отчёте и ленте. **Урок:** в PowerShell-шелле `rg` не
  раскрывает `*`/`[…]` в путях (`os error 123`) — шаблоны файлов передавать
  через `-g "Q5[7-9].md"`, а не в аргументе пути.
- **2026-09-29 · `service-docs-hygiene` шаг 3 (D61–D65 + T-18; чекпойнт + итог,
  без cargo):** прочитаны лента `service-docs-hygiene.md`, `review.md`, образец
  `docs-hygiene-q57q61-2026-09-29.md`, D61–D65, T-18, §10 `:881-890`, статусы
  Q57–Q61/`TRACEABILITY:61-73`/`questions/README:78-85`, `decisions/README`,
  `tasks/README:51-54`, `CHANGELOG:330-390`, `findings-registry.md`,
  `clean-logs.mjs:107-113`, `receipts.yaml`. Снимок: `develop` @ `04cd7bf`
  (= `origin/develop`) + рабочее дерево (**16 M + 6 ??**); `src`/`tests`/
  `Cargo.toml` в `git status` нет, `git diff --numstat -- src tests Cargo.toml`
  пусто → **cargo не запускался — D50** (счётчики 47/278 = `features/README.md`
  не в статусе). Вердикт — **отклонено (rework, iteration 1)**: **P2**
  `D65:58-60` и `:98-100` (плюс `:80` «8 адресов») — перечень «Существующие
  адреса `mail/**` в каноне» неполон: живые ссылки канона есть и в старых
  записях журнала — `Q54.md:12,18`, `Q55.md:18`, `Q56.md:20` (цели живы), плюс
  `docs/tasks/T-15-mcp-ready-process/wave0*.md`; исполнение по D65 их не снимет,
  а тест D64 п.8 (whitelist — только `findings-registry`) на них покраснеет;
  `clean-logs.mjs` удаляет все `mail/*.md` → F47 закрывается лишь частично.
  Правка требует выбора объёма (расширить перечень либо зона-исключение
  «записи до 2026-09-29») — отсюда P2, а не P3. **P3**
  `T-18/README.md:36-38` — запреты без whitelist-исключения из источника
  (`D64:50-53`). Остальное ок: D-файлы (accepted, Resolves 1↔1, Спека §10
  №61–65, разделы, ⚪/⬜+задача), §10 `:884-888` (diff 5/0, №60 `:883` цел),
  статусы (`resolved by [D6x]`; `TRACEABILITY:64-68`; `questions/README:81-85`),
  `decisions/README` **54 = 54** D-файла (D61 `:77`→D65 `:81`), T-18 +
  `tasks/README:54`, CHANGELOG `:364-385` (code-span `tests/docs_journal.rs`),
  границы (архив/`BRIEF`/`docs/README`/`features`/`GRAMMAR`/`T-16`/`.opencode/agents`
  не тронуты; 42 поля «Перенос»; 47/278), свип «ожида(ет|ют) переноса» (только
  перечни запретов T-18:36/Q60:33), ссылки живые. Отчёт
  `docs/reviews/docs-hygiene-d61d65-2026-09-29.md`; квитанция
  `service-docs-hygiene-step3` iteration 1 (`rework`) — append. **Урок:**
  проверять не только «новые адреса соответствуют политике», но и **обратную
  полноту**: правило «канон не ссылается на удаляемые рабочие данные» (D65 п.4)
  + whitelist (D64 п.8) должны быть сверены со **всем** деревом канона
  (`rg "\.opencode/mail" docs`), иначе объявленный «полный перечень» (и объём
  исполнения) оставляет старые адреса, а будущий тест не может стать зелёным.
  Комбинировать: узкое `rg` по журналу/задачам обязательно (широкий свип режется
  `token-guard`).
- **2026-09-29 · `service-docs-hygiene` шаг 3 — r2 (P2/P3 закрыты; итог, без
  cargo):** прочитаны лента (`migrator` · rework `:404-446` + таблица свипа),
  `D65` (п.4 `:46-72`, «Сверка» `:99-121`, «Следствия» `:89`), `Q61:44-61`,
  `T-18/README.md:15-43`, отчёт `-r1`. Снимок: `develop` @ `04cd7bf` +
  рабочее дерево (**18 M + 7 ??**). Вердикт — **принято, замечаний нет**
  (P1/P2/P3 нет). P2 закрыт: **самостоятельный** свип
  `rg -n "]\([^)]*opencode/mail/"` по канону (вне `reviews/**`/`analysis/**`) —
  markdown-цели: `Q54:12,18`, `Q55:18`, `Q56:20`, `Q57:12`, `Q58:9`, `Q59:11`,
  `Q60:11`, `Q61:10`, `questions/README:80`, `D49:69`, `D50:31,87,115`,
  `D51:32,107`, `CHANGELOG:353,365` — совпадает с `D65:58-68` пофайлово;
  не-ссылочные упоминания (Q43/Q48/Q51/D38/D39/D43/D46/`decisions/README:59`/
  `TRACEABILITY:55`/`SPECIFICATION:861,866,869`) — `*/**`, `T-XX.md` или
  описания правил, не адреса; оговорки D65 про строку D43 и про `T-15`/
  исторические зоны верны. P3 закрыт: `T-18/README.md:39-40` — whitelist
  `findings-registry.md` (D48), согласован с `D64:50-53`/`D65:54-57`. Границы:
  изменены только `D65` (131 → 140), `Q61.md` (+22/−2), `T-18` (+3);
  `numstat` остальных файлов шага 3 = снимок `-r1`; архив/`BRIEF`/`docs/README`/
  `features`/`GRAMMAR`/`T-16`/`.opencode/agents` не тронуты; 47/278; ссылки
  живы; **cargo не запускался — D50**; git только чтение. Отчёт — раздел
  «Обновление 2026-09-29 — r2» в `docs/reviews/docs-hygiene-d61d65-2026-09-29.md`;
  квитанция `service-docs-hygiene-step3` iteration 2 (`accepted`) — append
  (первая запись, `rework`, остаётся). **Урок/приём:** для «закрытия P2 по
  варианту а» строить **собственный** свип и сверять его с перечнем решения
  пофайлово (+ отдельно классифицировать не-ссылочные упоминания `*/**`/
  `T-XX.md`, иначе они дают ложные «пропуски»); границы r2 подтверждать
  сравнением `numstat` **всех** файлов блока со снимком `-r1` (правки могли
  задеть соседей).
- **2026-09-29 · `service-docs-hygiene` исполнение D61–D65 — iteration 1
  (обрыв по лимиту шагов, без отчёта/квитанции; итог восстановлен):** снимок
  `develop` @ `6ff997b` + рабочее дерево (**81 M + 1 D**, `??` нет; тот же
  набор, что и в r2). Проверено: D61 (архив `D`, 42 поля «Перенос» = Q1–Q42
  текстом, шапки, служебная зона, `agents-perms.mjs` дважды «11 из 18»),
  D63 (TRACEABILITY `Q|D|Жизненный цикл|Задачи|Реализация` 61 + легенда;
  каталоги 61/54), D62 (BRIEF 304; §7/§8; DoD → `AGENTS.md`), D65 (mail-ссылок
  нет, `F47`/`F48` закрыты), P3 аудита (нет указателей §9; запись-подтверждение
  `6ff997b` — лента `:489-496`), регрессии (47/278; свип «ожида(ет|ют)
  переноса» — Q60:33/T-18:36; `src`/`tests`/`Cargo.toml` не тронуты; `cargo`
  не запускался — D50). Вердикт **rework**: **P2** — канон ссылается на
  `docs/analysis/**` (~40: D40–D51, Q45–Q54, `T-11`/`T-13`, `CHANGELOG`),
  а `D65` п.4/`D64` п.8 запрещали это (только `findings-registry`) → будущий
  тест `T-18` покраснеет; фикс требовал выбора объёма. **P3** — неполный
  перечень в `D65` «Сверка с кодом»; `Q57`/`D61` без пометки «историч.».
- **2026-09-29 · `service-docs-hygiene` исполнение D61–D65 — iteration 2
  (rework; итог, без cargo):** прочитаны запись rework (`:998-1028`), `D64`
  п.8 `:50-55`, `D65` п.4 `:46-55`/«Сверка» `:104-110`/«Следствия» `:86`,
  `Q61:44-48`, `T-18:37-40`, `Q57:19,:67`, `D61:27,:68`, `clean-logs.mjs`,
  `BRIEF:289-304`, §10 `:887-888`, `git diff -U0 -- docs/SPECIFICATION.md`.
  Снимок тот же (81 M + 1 D; `??` нет). Вердикт — **отклонено (rework,
  iteration 2)**: rework закрыл мой `-r1` P2 *в решениях* (зона запрета =
  `.opencode/mail/**`/`.opencode/state/**`; `docs/analysis/**` — допустим как
  провенанс: `clean-logs.mjs` чистит только `memory`/`mail`), но **остался P2**:
  `BRIEF.md:292-295` (§8), `SPECIFICATION.md:887` (§10 №64), `:888` (§10 №65)
  всё ещё запрещают ссылки канона на `docs/analysis/**` → канон противоречит
  решению (Q41/D60); реализатор `T-18` по §10 №64 запрограммирует запрет.
  P3-a/P3-b закрыты. Инварианты `-r1` держатся. Отчёт
  `docs/reviews/docs-hygiene-exec-2026-09-29.md`; квитанции
  `service-docs-hygiene-exec` iteration 1/2 (`rework`) — append. **Урок:**
  (1) переписывая правило в решении (rework), сразу проверять **все** его
  канонические формулировки — `SPECIFICATION.md` §10 (строка решения, у D —
  `Спека`) и `BRIEF.md` §8: изменение правила без синхронизации §10 = новое
  противоречие канона; (2) `edit` квитанции: append-only файл — вставлять
  **с конца**, а не по якорю середины (`snapshot`+`at` не уникальны: у
  соседних записей те же строки) — иначе ломается YAML предыдущего блока;
  (3) чекпойнт и квитанцию писать **до** тяжёлой проверки: обрыв сессии
  оставил iteration 1 без отчёта/квитанции.
- **2026-09-29 · `service-docs-hygiene` исполнение D61–D65 — iteration 3
  (P2 закрыт; итог, без cargo):** прочитаны записи `migrator`/`docs-writer`
  P2-r2 (`:1067-1119`), `BRIEF:289-304`, §10 `:887-888`, `D64:50-55`,
  `D65:46-55`, `Q61:44-48`, `T-18:37-40`. Снимок `develop` @ `6ff997b` +
  рабочее дерево (набор тот же, что `-r2`; новых файлов нет, кроме моего
  отчёта). Вердикт волны — **принято, замечаний нет** (P1/P2/P3 нет).
  Единый смысл правила «время жизни адреса» подтверждён во всех шести местах:
  запрет — `.opencode/mail/**` (`clean-logs --mail-only`) + `.opencode/state/**`;
  `docs/analysis/**` — допустим как провенанс; whitelist —
  `findings-registry.md` (D48). Границы: `git diff -U0 -- docs/SPECIFICATION.md`
  = ровно 3 хунка (шапка `:815-816` из `-r1`; `:887-888` из `-r3`); `BRIEF`
  304 → **307** (+3 — единственный пункт §8). Инварианты держатся (архив `D`,
  42 «Перенос», TRACEABILITY 61 + легенда, каталоги 61/54, §10 №61–65 = 5,
  47/278 = `features/README.md:314`, ссылок на архив/mail нет кроме `T-15/**`,
  `src`/`tests`/`Cargo.toml` пусто). Отчёт — раздел «Обновление 2026-09-29 —
  r3 (P2 закрыт)» в `docs/reviews/docs-hygiene-exec-2026-09-29.md`; квитанция
  `service-docs-hygiene-exec` iteration 3 (`accepted`) — append. **Урок:**
  «нулевой» прогон закрытия P2 проверять по **диффу** (`git diff -U0` даёт
  точный перечень хунков) и по счётчику строк изменённого файла — это дешевле и
  надёжнее, чем повторное чтение всего документа, и прямо отвечает на «изменены
  только ожидаемые строки».
- **2026-09-29 · `service-doc-tools` шаг 2 (D66–D68/Q62–Q64 + T-19; чекпойнт +
  итог, без cargo):** прочитаны лента `service-doc-tools.md`, `review.md`,
  `D66/D67/D68`, `Q62–Q64`, `T-19/README.md`, §10 `:888-891`,
  `TRACEABILITY:60-79`, `questions/README:66-86`, `decisions/README`,
  `tasks/README:40-61`, `CHANGELOG:384-435`, `D64:30-89`, `D65:30-89`,
  `T-18/README.md`, `docs/research/doc-quality-checks-2026-09-29.md`,
  `receipts.yaml`. Снимок: `develop` @ `bfbfec5` (= `origin/develop`) + рабочее
  дерево (**10 M + 9 ??**); `git diff -- src tests Cargo.toml` пусто →
  **cargo не запускался — D50** (изменения только `docs/**` + памяти/лента/
  research/T-19, чистые вставки; счётчики 47/278 = `features/README.md:314`).
  Вердикт — **принято с замечаниями** (P1/P2 нет). Проверено: §10 `:889-891`
  (+3/0, после №65, живые `[D66]…[D68]`), D-поля и вердикты (D66 ⬜/T-19,
  D67/D68 ⚪/«задач не требуется»), Q `resolved by [D6x]`, `TRACEABILITY:72-74`
  (Q62 `in work` + `[T-19] ⬜`; Q63/Q64 `resolved`, задачи `—`; Q-строк 64,
  словарь D63 цел), `questions/README:84-86`, `decisions/README` **57 = 57**
  (D66 `:81` → D68 `:83`), карточка T-19 (:3/:4/:7/:67, link-check → T-18
  :51/:71) + строка `:55`, CHANGELOG `:412-430` (после D61–D65 `:386-411`, до
  `### Процесс` `:432`), D65-совместимость (нет markdown-ссылок на `mail/**` в
  новых Q/D/T-19; `tests/docs_journal.rs` — только code-span как D64), все
  относительные цели живы, архив отсутствует, свип `ожида(ет|ют) переноса` вне
  reviews — только описания маркера (`T-18:36`, `Q60:33`), история/права не
  тронуты. **P3:** `docs/research/doc-quality-checks-2026-09-29.md:8` —
  markdown-ссылка на `.opencode/mail/service-doc-tools.md` (удаляемые рабочие
  данные; после `clean-logs --mail-only` станет битой — класс F47); зона не
  канон и вне охвата doc-инструментов (`T-19:33`) → P3, одна строка
  (провенанс формой «имя без URL», как Q62–Q64), адресат `researcher`. Отчёт
  `docs/reviews/doc-tools-d66d68-2026-09-29.md`; квитанция
  `service-doc-tools-step2` iteration 1 (`accepted_with_notes`) — append.
  **Урок:** «D65-совместимость» проверять не только по перечню ожиданий (новые
  Q/D), но и по **всем** новым файлам пакета (`docs/research/**` тоже): политика
  перечисляет как допустимый провенанс только `docs/analysis/**`, поэтому URL на
  `mail/**` в research — кандидат в P3, даже если формально зона не канон.
  **Приём:** `git rev-parse HEAD` вне allow-list `validator` (отклонён движком) —
  заменять на `git log -3 --oneline develop`.
- **2026-09-29 · `service-doc-tools` шаг 2 — P3 закрыт (адресно, без новой
  итерации):** `researcher` заменил в
  `docs/research/doc-quality-checks-2026-09-29.md:8-9` markdown-ссылку на
  `.opencode/mail/service-doc-tools.md` на имя без URL (code-span
  `service-doc-tools` + «`.opencode/mail/**` — рабочие данные»). Снимок тот же
  (`develop` @ `bfbfec5` + дерево). Проверено: `rg "\]\([^)]*opencode/mail"
  docs/research` → пусто; правка in-place (файл **160** строк, блок 2→2), прочие
  ссылки живы (D64/D65/T-18/`AGENTS.md`, внешние https), файл один; состав пакета
  не изменился; `cargo`/git не трогались (D50). Вердикт в силе —
  **принято с замечаниями (P3 закрыт)**; поле `p3_closure` в квитанции
  `service-doc-tools-step2`, раздел «Обновление 2026-09-29 — P3 закрыт» в отчёте,
  запись в ленте. **Приём:** для untracked-файла закрытие P3 подтверждать парой
  «свип пуст + чтение строки + неизменность длины (`rg -c "^"`) + неизменный
  состав `git status --porcelain`» (сравнить нельзя `numstat` — файл
  неотслеживаемый).

- **2026-09-29 · `service-doc-rework` (приёмка D69–D74 + канон; r1 + r2, без
  cargo):** прочитаны лента `service-doc-rework.md`, `review.md`, канон/фичи,
  D1–D13/D60–D74, Q65–Q70, `decisions/README`, `TRACEABILITY`, `CHANGELOG`,
  `GRAMMAR`, SPEC §6/§11 vs база, `receipts.yaml`. Снимок: `develop` @ `75b078c`
  (= `origin/develop`; коммитов волны нет) + рабочее дерево (39 M + 21 `??`);
  `git diff --stat -- src tests Cargo.toml` пусто → **cargo не запускался — D50**.
  **r1 — отклонено (rework):** P1 нет; **P2-1** `.opencode/rules/journal.md:4` —
  `[D71](../docs/decisions/…)` резолвится в `.opencode/docs/…` (битая;
  канонично `../../docs/decisions/…`); **P2-2** `D60:44` — «хронология прототипа»
  против D72/`AGENTS.md:172`; **P3-1** титулы `SPECIFICATION.md:1`/`GRAMMAR.md:1`
  с `{}`; **P3-2** «Канон:» → удалённый `docs/BRIEF.md` в
  `memory/{validator,migrator,docs-writer}.md` (F35). Отчёт
  `docs/reviews/doc-rework-2026-09-29.md`; квитанция iteration 1 (`rework`).
  **r2 — принято (iteration 2, `accepted`):** все P2/P3 закрыты адресно —
  journal.md `../../docs/decisions/D71-…` (цель существует), `D60:44` =
  «кодовые изменения продукта ([D72])» (иных правок в D60 нет), титулы
  SPEC/GRAMMAR без `{}` (остались тело SPEC `:15`/рамка `:70` и фичи — не
  титулы), шапки «Канон:» → `.opencode/rules/journal.md` (только шапки:
  `numstat` 1/1 / шапка + прежняя хроника; права/`steps` не менялись).
  Инварианты r1 держатся: D69 (11 ретро-D, 74=74, §10-номера ↔ база), D70
  (SPEC 497, §6/§11 = база, §10-указатель, §1.3.1→D22/Q20), D71 (BRIEF нет,
  живых BRIEF-ссылок в каноне нет, D62 superseded, D66 без BRIEF, D60 без §10),
  D72 (CHANGELOG 7 строк, D68 дополнен), D73 (README 42/48, без Q/BRIEF),
  D74 (GRAMMAR 78, `Q\d+` пусто, EBNF целы); Q 70/70, 47/278
  (`features/README.md:314`), свип чист, `reviews`/`analysis` не тронуты,
  `agents-perms` «11 из 18». Наблюдение (не находка): `GRAMMAR:49` «Не найдено
  имя правила» vs код «отсутствует заголовок правила» — открытая задача `T-14`.
  **Урок:** битую ссылку из нового канона ловит проверка «база пути»
  (`rg --files <каталог> -g "<глоб>"`), а не «ссылка выглядит правдоподобно»:
  доки живут в `docs/**`, а канон — в `.opencode/rules/**`, уровней вверх на
  один больше. **Приём:** правки CRLF-файла (`memory/**`) через `edit` требуют
  якоря **внутри строки** (без `\n`): переводы строк в этих файлах CRLF,
  многострочный якорь не матчится.
- **2026-09-30 · `service-git-efficiency` (D75, чекпойнт до прогона, без cargo):**
  прочитаны лента (факты разбора, аудит D75 `:236-273`, закрытие P2/P3 `:304-334`),
  `git.md` (`steps: 14`, §-ссылка `:82-87`, allowlist `:30-31`),
  `git-workflow.md` (цикл `:85-95`, шаблон промпта `:68-70`, идемпотентность
  `:126-134`), `review.md:84-118`, `git-check.mjs` (read-only по коду),
  `memory/git.md` (34 строки), `Q71`, `D75`, `TRACEABILITY:75`,
  `findings-registry:57-63` (F49–F55), `questions/README` (71 строка),
  `decisions/README` (75 строк), SPEC §10 упразднена (D70) — строки D там нет и
  не требуется. Снимок: `develop` @ `75b078c` + рабочее дерево
  (`git diff --shortstat` 43 файла: 42 M + 1 D; + 31 `??` — две волны,
  `service-doc-rework` и `service-git-efficiency`); `src`/`tests`/`Cargo.toml` в
  `git status` нет, `git diff --stat -- src tests Cargo.toml` пусто →
  **cargo не запускаю — D50**. Далее:
  `agents-perms.mjs` ×2 (`11 из 18`), попытка прогона `git-check.mjs` (права
  `validator` его не покрывают — ожидаю отказ → чтение кода), адресные сверки.
  **Итог: принято с замечаниями (P3, не блокер)** — `D75:7-11` (`Affects`) без
  `AGENTS.md` (строка `:25` правилась исполнением D75 п.5; прецедент `D51:13-14`);
  P1/P2 нет. Проверено: `agents-perms` ×2 (идентичны, `11 из 18`, `git steps=14`
  + `git-check.mjs`, изменяющие `ask`; права `validator` = `review.md:86-87`);
  `git-check.mjs` — прогон `permission.rejected`, read-only по коду `:1-105`;
  3 живых вхождения «Минимальный цикл» (дом + 2 ссылки); `decisions/README`
  75 = 75 D-файлов; `questions/README` 71; `TRACEABILITY` 71, Q71 `:75`;
  `findings-registry:57-63` F49–F55 «закрывается D75»; `AGENTS.md:24-25`;
  `memory/git.md` 34 строки ↔ канон (F35) без расхождений; ссылки живые; лента/
  память `auditor`/`migrator`/`git` согласованы. **cargo не запускался — D50**
  (`git diff --stat -- src tests Cargo.toml` пусто; фичи не тронуты, 47/278).
  Отчёт `docs/reviews/git-efficiency-2026-09-30.md`; квитанция
  `service-git-efficiency-exec` iteration 1 (`accepted_with_notes`) — append.
  **Урок:** `agents-perms.mjs` показывает **эффективные** права из
  `opencode debug agents` (а не текст `.md`) — годится как независимое
  подтверждение правок фронтматтера (здесь `git steps=14` + новый allowlist);
  паритет `review.md` ↔ фронтматтеры он не проверяет — это ручная сверка.
  **Приём:** `rg -c "\r" <файл>` различает CRLF/LF (правки CRLF-файлов —
  однострочными якорями; `state/receipts.yaml`/лента — LF, многострочный якорь
  проходит).
- **2026-09-30 · `service-git-efficiency` — r2 (уточнения D75, итог):** вердикт
  **принято** (P1/P2 нет; P3 закрыты — их три: r1 `D75:12` `Affects`+`AGENTS.md`;
  аудита уточнений — `D75:46-47` пометка «уточнено: 18» и
  `git-workflow.md:161-162` F22-исключение). Проверено: `git.md:6` `steps: 18`,
  `:89-91` блок «Стартовое чтение — минимум»; `git-workflow.md:94-97`
  (`./`-префикс, без `--`, staged отдельным шагом), `:98-100` (сервисные волны —
  один коммит; `chore(process)` — кодовые задачи, D46), `:68-71` усиленный
  шаблон `lead`; `D75:63-77` блок «Обновление 30.09.2026», `:46-47`; `agents-perms`
  ×2 идентичны (`11 из 18`, `git steps=18`, прочие роли без сдвигов); F49–F55
  «закрыт 30.09.2026 (D75; …)» (`findings-registry:57-63`). **cargo не
  запускался — D50** (`git diff --stat -- src tests Cargo.toml` пусто в дереве и
  по `75b078c..HEAD`). Снимок r2: `develop @ 0872dfa` (= `origin/develop`) +
  6 M. Отчёт `docs/reviews/git-efficiency-2026-09-30.md` (раздел r2); квитанция
  `service-git-efficiency-exec` iteration 2 (`accepted`, `p3_closure`) — append.
  **Урок:** после коммита волны уточнения проверять, что память роли не хранит
  устаревшее число в датированном чекпойнте (`memory/git.md:31-34` «28→14» при
  актуальном 18) — F35 относится к «Знанию», но хроника-число лучше освежать.
  **Приём:** партию «роль/правило/решение» удобно сверять одной осью
  (`steps` 18 ↔ `D75:71` ↔ `git.md:6`) плюс `agents-perms` как независимым
  подтверждением эффективных прав.
- **2026-09-30 · `service-doc-refactor` (Q72 → D76, приёмка, без cargo):**
  прочитаны лента `service-doc-refactor.md`, `journal.md` §4/§5.7, `review.md`,
  `TRACEABILITY.md`, `Q72.md`, `D76-traceability-links-only.md`,
  `questions/README.md`, `decisions/README.md`, память/лента `migrator`,
  `receipts.yaml`. Снимок: `develop`, HEAD `6182bb5` + рабочее дерево (4 M + 3
  `??` = 7 путей); `git diff --stat -- src tests Cargo.toml` пусто → **cargo не
  запускался — D50** (документная волна). Вердикт — **принято, замечаний
  нет**; P1/P2/P3 нет. Проверено: 72 строки Q (HEAD `| [Q`=71 + Q72); строгий
  5-колоночный формат всех 72 (`rg -v` оставляет 12 «не-строк»); клетки Q/D без
  « — »; колонки 3–5 = HEAD по агрегатам (`| resolved |` 54/53(+Q72), `| in
  work |` 14/14, `| done |` 4/4, ⬜15/15, 🚧1/1, ✅8/8, `](features/` 43/43,
  `](tasks/` 19/19); Q6 :10 дословно = HEAD (маркер «закрыт попутно» снят, канон
  `Q6.md`/`D19` цел); Q72 :76 `resolved | — | —`; легенда :78–82 = HEAD
  :77–81 дословно + предложение :82–84; CRLF/финал `^`=`\r$`=84; Q72/D76 §4
  (поля+разделы), D76 ⚪, `Tasks: —`, §10 нет (D70); каталоги 72/76; ссылки
  живые/относительные, без `mail`/`state` (`decisions/README:72` — плейнтекст
  D43, не ссылка); лента и память `migrator` (запись + обрыв по шагам).
  **Тех. заметка:** `git diff -L…` не поддерживается этой версией git
  (`invalid option: -L…`), полный diff TRACEABILITY срезается token-guard —
  колонки 3–5 брал агрегатами HEAD↔дерево + Q6 построчно. Отчёт
  `docs/reviews/service-doc-refactor-2026-09-30.md`; квитанция
  `service-doc-refactor` iteration 1 (`accepted`) — append.
