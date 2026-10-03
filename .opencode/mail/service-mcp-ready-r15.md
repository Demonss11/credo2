# service-mcp-ready-r15 — лента операции: T-15, C3-остаток + C4 (process-ветка, теги, фасад)

Открыта: 03.10.2026. **Сервисная операция** (T-15, фаза C; после r14).
Предмет (выбор владельца 03.10.2026): **C3-остаток** (session-commit: ветка
`process/*`, теги `session/*`, снапшот состояния) + **C4** (фасады
`/git/checkpoint`, `/git/status`; `.opencode/commands/**` — в область аудита
`auditor`).

**Источники (прочитано при открытии):** карточка T-15 (:101-125 — состав C,
:261-280 — черновик `checkpoint.md`, :298-300 — примечание о командах);
записка `mcp-ready-process.md` §6 (:148-168), :80-81 (R7), :197; фича
`docs/features/agents-session-checkpoint.feature` (4 сценария); правило
`git-workflow.md` §«Session-commit» (инкремент 1, D89 P3, :200-220); `D89`;
`lead.md`/`git.md` (права); `session-checkpoint.mjs` (B0-own P3);
`auditor.md` (область аудита); `commands/git/status.md`.

**Проект механики (к решению):**

- **Прогон** — исполнение процесса по задаче (`T-XX`), сессии — `sM`
  (`session_index`); ветка `process/<прогон>` создаётся первым чекпойнтом от
  текущего HEAD;
- **чекпойнт сессии** = лента + память + снапшот состояния
  (`.opencode/state/snapshots/<прогон>-s<M>/` — копия `state/current/*`);
  сообщение `chore(process): <прогон> s<M>`; тег `session/<прогон>-s<M>`;
  push ветки и тега; **без merge** в `develop`;
- live `state/current/*` — как сейчас: финальный process-коммит задачи идёт в
  её ветку → `develop`; процесс-ветка — носитель по-сессионных снапшотов
  (страховка/resume), не источник для develop;
- **сервисные операции** (lead-less, одна сессия) — без process-ветки: их
  лента/память идут обычным пакетом (как r9–r14);
- **C4:** `.opencode/commands/git/checkpoint.md` (по черновику карточки,
  с шагом снапшота/тега), `/git/status` — финализируется (теги сессий),
  `.opencode/commands/**` — в область аудита `auditor`;
- фича `agents-session-checkpoint.feature` — формулировка сценария 1
  приводится к принятому имени прогона (счётчик сценариев не меняется).

**Открытые развилки — владельцу:** (1) имя прогона (`T-XX` или глобальный
номер `runN`); (2) кто создаёт снапшот (скрипт по вызову фасада или `lead`).

## сервисная сессия · 03.10.2026 · открытие

- Срез: `develop` = `origin/develop` = `42e1296` (r14/предикат зачёта).
- Контекст: C3 — инкремент 1 принят (D89 P3: правило session-commit в текущую
  ветку); остаток (`process/runN`, теги, фасад) — «отдельным решением»
  (`git-workflow.md:219-220`), это решение — предмет r15.
- Следующее действие — ответы владельца по развилкам → журнал (Q90 → D93) →
  правки → `auditor` → `validator` → гейт → `git`.

## сервисная сессия · 03.10.2026 · развилки решены

- **Ответы владельца:** (1) имя прогона — **идентификатор задачи**
  (`process/<прогон>`, `session/<прогон>-s<M>`, `chore(process): <прогон> s<M>`);
  (2) снапшот — **скрипт по вызову фасада** (`session-checkpoint.mjs --snapshot`,
  право у роли `git`).
- **Уточнение механики (из разбора):** накопительная ветка `process/<прогон>` на
  общем рабочем дереве конфликтует с незакоммиченными файлами (switch перезаписал
  бы M-файлы) — чекпойнт идёт в **ветку сессии** `process/<прогон>-s<M>`,
  создаваемую от текущего HEAD (`git switch -c` — конфликтов нет); ветки сессий
  сохраняются, в `develop` не мержатся; тег `session/<прогон>-s<M>`.
- **Состав чекпойнта:** лента + память + снапшот
  `.opencode/state/snapshots/<прогон>-s<M>/` (копия `state/current/*` + meta);
  live state — финальным process-коммитом задачи (как раньше); сервисные
  операции — без process-ветки.
- Следующее действие — `dispatch migrator` (Q90 → D93).

## сервисная сессия · 03.10.2026 · правки канона r15 — готово

- **Журнал:** Q90 → D93 (`migrator`); каталоги, `TRACEABILITY` (Q90: D93,
  `in work`, T-15 🚧).
- **Правки (сервисная сессия, по D93):**
  - `git-workflow.md` §«Session-commit» — финальная механика: ветка сессии
    `process/<прогон>-s<M>` от текущего HEAD, тег `session/<прогон>-s<M>`,
    сообщение `chore(process): <прогон> s<M>`, снапшот скриптом, push ветки и
    тега, без merge в `develop`; сервисные операции — без process-ветки;
    строка «остаток C3 — отдельным решением» снята;
  - `dispatch-loop.md` — session-commit: ветка/тег/снапшот (`D93`);
  - `.opencode/commands/git/checkpoint.md` — **новый** фасад (снапшот → ветка →
    add → commit → tag → push → назад; «нет записи — нет чекпойнта»;
    без shell-блоков);
  - `.opencode/commands/git/status.md` — теги сессий (`git tag -l "session/*"`);
  - `agents/git.md` — право на `session-checkpoint.mjs` + пункт Session-commit;
  - `agents/auditor.md` — `.opencode/commands/**` в источниках и чек-листе;
  - `AGENTS.md` — строка карты `.opencode/commands/`;
  - `scripts/session-checkpoint.mjs` — режим `--snapshot` (копия
    `state/current/*` + `meta.md`; `--dir` — для проб);
  - `features/agents-session-checkpoint.feature` — сценарий 1 под принятое имя
    (счётчик 4 не менялся).
- **Проба (сервисная сессия, не R2):** `--snapshot --dir <temp>` — каталог
  создан, файлы скопированы; временный каталог удаляется до пакета.
- Следующее действие — `auditor` (канон/права/команды, до коммита).

## auditor · 03.10.2026 · аудит — два P2 (закрыты)

- P1/P3 нет; D93 п.1–10 реализованы; границы чисты; `agents-perms.mjs` —
  «11 из 18».
- **P2-1:** `.opencode/rules/review.md` — в списке команд `git` не было
  `session-checkpoint.mjs` → добавлено.
- **P2-2:** `docs/features/README.md:299` — `process/runN` в карте фич (канон
  Q41) → приведено к D93 (ветка сессии/теги/снапшот/фасад).
- Временный каталог пробы удалён до пакета.
- Следующее действие — свежий аудит правок (узкий) → `validator`.

## validator · 03.10.2026 · приёмка — принято (docs_journal 14/0)

- P1/P2/P3 нет; D93 п.1–10 ↔ факт подтверждены; P2-фиксы аудита закрыты;
  `docs_journal` **14 passed / 0 failed**; `agents-perms.mjs` — «11 из 18»;
  границы чисты (`src/**`/`tests/**`/`Cargo.toml` не тронуты); полный DoD не
  запускался (D50). Отчёт `docs/reviews/service-process-branch-2026-10-03.md`;
  квитанция `service-process-branch` (iteration 1, accepted).
- Замечания (не находки): права `validator` на прогон скрипта нет — проверка
  чтением; карточка T-15 хранит `process/runN` (:108/:216/:276-277) — закрытие.
- Следующее действие — закрытие: `migrator` (C3/C4 ✅, синхронизация карточки),
  `docs-writer` (строка фичи 🟡→✅), затем гейт.

## сервисная сессия · 03.10.2026 · закрытие — готово; пакет r15 — гейт владельца

- **Закрытие:** карточка T-15 — `C3`/`C4` ✅ 03.10 (`service-process-branch`,
  D93); C-секция и примечание синхронизированы, черновик команды снят (Q41);
  фича `agents-session-checkpoint` — ✅ (`docs-writer`); Q90 остаётся `in work`
  (T-15 🚧, D82).
- **Пакет (снимок `git status`, 19 M + 5 ?? = 24 + память `service`/`git`):**
  - канон: `AGENTS.md`, `.opencode/rules/{git-workflow,dispatch-loop,review}.md`,
    `.opencode/agents/{git,auditor}.md`;
  - команды: `.opencode/commands/git/checkpoint.md` (новый), `status.md`;
  - скрипт: `.opencode/scripts/session-checkpoint.mjs` (режим `--snapshot`);
  - журнал: `docs/questions/Q90.md` (новый),
    `docs/decisions/D93-session-commit-process-branch.md` (новый), каталоги,
    `docs/TRACEABILITY.md`;
  - приёмка: `docs/reviews/service-process-branch-2026-10-03.md` (новый),
    `.opencode/state/current/receipts.yaml`;
  - карточка/фичи: `docs/tasks/T-15-mcp-ready-process/README.md`,
    `docs/features/{README.md,agents-session-checkpoint.feature}`;
  - ленты/память: `.opencode/mail/service-mcp-ready-r15.md` (новый),
    `.opencode/memory/{auditor,docs-writer,migrator,validator}.md`.
- **Вне пакета:** нет (`.credo/sandbox.json` — чист).
- Следующее действие — подтверждение владельца → `dispatch git`.

## migrator · 03.10.2026 · готово

- Сделано: оформлена пара **Q90 → D93** —
  [`Q90.md`](../../docs/questions/Q90.md) («C3-остаток: process-ветка, теги,
  снапшот, фасад `/git/checkpoint`»; варианты A/B/C, решение владельца —
  **A + идентификатор задачи**, снапшот — **скрипт по вызову фасада**;
  статус `resolved by D93`) и
  [`D93-session-commit-process-branch.md`](../../docs/decisions/D93-session-commit-process-branch.md)
  (10 пунктов: прогон=`T-XX`/сессия `sM`; ветка сессии
  `process/<прогон>-s<M>` от текущего HEAD, накопительная ветка отклонена;
  состав/тег/сообщение/push; live state — финальным process-коммитом задачи;
  снапшот `.opencode/state/snapshots/<прогон>-s<M>/` скриптом `--snapshot`,
  право у `git`; «постоянный» пакет без изменений D89 P3; сервисные операции —
  без process-ветки; фасад `/git/checkpoint` без shell-блоков, «нет записи —
  нет чекпойнта»; `/git/status` + теги сессий, `.opencode/commands/**` — в
  аудит `auditor`; фича — сценарий 1 к принятому имени, счётчик 4; catch-up как
  D89 P3). Строки в `questions/README.md`, `decisions/README.md`,
  `docs/TRACEABILITY.md` (Q90: D93, `in work`, T-15 🚧).
- Проверки: ID свободны (Q89/D92 — последние); ссылки живые (D89/D92, карточка
  T-15, записка §6, фича, `git-workflow.md`, `session-checkpoint.mjs`,
  findings); `cargo` не запускался (D50). Карточка T-15 и её C-секция/строки
  `C3`/`C4` не тронуты (🚧; закрытие — после приёмки).
- Дальше / риски: сверка D93 — ⚪ (процесс/служебная зона). Далее правки канона
  сервисной сессией (`git-workflow.md`, `dispatch-loop.md`, `commands/git/*`,
  `agents/{git,auditor}.md`, `session-checkpoint.mjs`, `AGENTS.md`, фича) →
  `reload` → `auditor` → `validator`.

## auditor · 03.10.2026 · готово

- Сделано: аудит r15 (D93↔факт, права/команды, журнал, границы) до коммита.
  D93 п.1–10 реализованы: `git-workflow.md` §«Session-commit» — ветка
  `process/<прогон>-s<M>` от текущего HEAD, тег `session/<прогон>-s<M>`,
  сообщение `chore(process): <прогон> s<M>`, push ветки+тега, без merge;
  строка «остаток C3 — отдельным решением» снята; `checkpoint.md` — 7 шагов
  (снапшот → switch -c → add → commit → tag → push → switch -), «нет записи —
  нет чекпойнта», идемпотентно, shell-блоков нет, `agent: git`/`subagent: true`;
  `status.md` — `git tag -l "session/*"`; `git.md` — право
  `session-checkpoint.mjs` + пункт Session-commit; `auditor.md` —
  `.opencode/commands/**` в источниках (:60-61) и чек-листе (:95-96); `AGENTS.md`
  :24 — `/git/checkpoint`, `/git/status`; `session-checkpoint.mjs::26-53` —
  `--snapshot` (4 артефакта + meta.md, `--dir`, значения из
  `current_state.yaml`); фича :13-14 — `process/<прогон>-s<M>`, счётчик 4.
  Проба сервисной сессии: каталог + 5 файлов (факт с ленты); временный каталог
  удалён. `dispatch-loop.md` :107-111; D89 P3 замещён; D92 — session-commit не
  дефект; Q90↔D93 парны, каталоги :112/:122, TRACEABILITY:94 (in work/T-15 🚧).
  Границы: `src/**`/`tests/**`/`Cargo.toml` не тронуты (`git diff --numstat`
  пусто).
- Проверки: `node .opencode/scripts/agents-perms.mjs` → `agents: 11 из 18`,
  git = frontmatter (allow `session-checkpoint.mjs`/`*` виден). `cargo` не
  запускался (D50).
- Дальше / риски: **P1 нет. P2×2:**
  (1) `.opencode/rules/review.md:104-108` — `git`: в списке «Доступные команды»
  нет `node .opencode/scripts/session-checkpoint.mjs` (есть clean-logs/git-check),
  а фронтматтер `git.md:34-35` его даёт; чек-лист `auditor` требует совпадения —
  ложный флаг «право без применения» + скрипт невидим как санкционированная
  команда роли. Правка: добавить строку в список `git`.
  (2) `docs/features/README.md:299` — строка `agents-session-checkpoint` держит
  `process/runN` (дважды) как live-механику; D93 п.1-2 принял `T-XX` /
  `process/<прогон>-s<M>` — канон-карта фич противоречит решению (Q41). Файл в
  периметр r15 не входил; правка нужна до приёмки. **P3 нет.**
  Инструкция↔права: расхождение — единственное (review.md, P2-1).
  Итог: вердикт — требуются правки P2 до коммита; затем свежий аудит.
- Отчёт: лента r15 (эта секция). Канон не правил.

## auditor · 03.10.2026 · аудит P2-фиксов (свежий, узкий)

- Сделано: аудит закрытия P2-1 и P2-2 после правок канона; регрессии на
  соседних строках/списках. Бюджет: 4 файла, 2 rg, 1 shell.
- Проверки. **P2-1 закрыт:** `review.md:104-109` — у `git` в «Доступные
  команды» есть `node .opencode/scripts/session-checkpoint.mjs`; совпадает с
  фронтматтером `git.md:34-35` (allow + `*`);
  `node .opencode/scripts/agents-perms.mjs` → `agents: 11 из 18`, без новых
  расхождений, список `review.md` ↔ фронтматтеры синхронен. **P2-2 закрыт:**
  `docs/features/README.md:299` — строка `agents-session-checkpoint` описывает
  ветку сессии `process/<прогон>-s<M>`, теги, снапшот, фасад; согласуется с
  D93 (:52-66) и `git-workflow.md:207-214`; счётчик сценариев 4
  (`agents-session-checkpoint.feature` — 4 `Сценарий:`); статус 🟡 (закрытие —
  после приёмки, вне периметра правки). **Регрессий нет:** соседние строки
  `README.md` (:298, :300) и списки `review.md`/`git.md` не задеты;
  противоречий D93 в изменённых файлах нет.
- Отчёт: лента r15 (эта секция). Канон не правил.

## validator · 03.10.2026 · до прогона

- Сделано: адресная документная/канонная сверка r15 (D93↔факт, P2-фиксы аудита,
  границы) — чтением/rg/git. Снимок HEAD `f58e27c` + рабочее дерево.
  D93 п.1–10 ↔ `git-workflow.md` §«Session-commit» (:200-227),
  `dispatch-loop.md:107-111`, `commands/git/{checkpoint,status}.md`,
  `agents/{git,auditor}.md`, `AGENTS.md:24`, `session-checkpoint.mjs:25-53`,
  фича (:13-14, счётчик 4) — совпадают. P2-1 `review.md:104-109` (у `git` есть
  `session-checkpoint.mjs`) и P2-2 `features/README.md:299` (D93-именование;
  соседи :298/:300 целы) — подтверждены. Границы: `git status --porcelain` —
  ровно периметр операции; `git diff --numstat -- src tests Cargo.toml` пусто;
  `git diff --check` пусто; `state/snapshots/**` нет.
- Проверки (готовлю): `cargo test --test docs_journal` (ориентир 14/0),
  `node .opencode/scripts/agents-perms.mjs` (ориентир 11 из 18).
- Дальше / риски: заметка к закрытию — карточка T-15 хранит `process/runN`
  (:108, :216, :276-277); актуализация — `migrator` при закрытии (статусы не
  меняю). Полный DoD не запускается (D50).

## validator · 03.10.2026 · после прогона — принято

- Сделано: завершена приёмка r15. **P1/P2/P3 нет** (техническое замечание —
  нет прав `validator` на `session-checkpoint.mjs`; проверка чтением). Границы
  чисты; D93↔факт и оба P2-фикса аудита подтверждены; карточка `runN` — hand-off
  `migrator`.
- Проверки: `cargo test --test docs_journal` — **14 passed / 0 failed** (0.06s);
  `node .opencode/scripts/agents-perms.mjs` — **11 из 18**; `git status
  --porcelain` — ровно периметр; `git diff --numstat -- src tests Cargo.toml` и
  `git diff --check` пусто; `state/snapshots/**` нет. Полный DoD не запускался
  (D50).
- Отчёт: `docs/reviews/service-process-branch-2026-10-03.md`; квитанция
  `service-process-branch` iteration 1, accepted (append в `receipts.yaml`).
- Дальше: гейт пакета → `git` (develop) → `migrator` (C3/C4 ✅, Q90 → done,
  актуализация карточки) → `complete`. Канон не правил; статусы не менял.

## migrator · 03.10.2026 · готово

- Сделано: закрытие r15 — актуализирована карточка
  [`T-15`](../../docs/tasks/T-15-mcp-ready-process/README.md) под приёмку
  (`service-process-branch`) и D93: строки `C3`/`C4` (реестр фаз) → ✅ с
  принятым именованием (ветка сессии `process/<прогон>-s<M>`, тег
  `session/<прогон>-s<M>`, снапшот, скрипт; `/git/checkpoint`,
  `/git/status` + теги сессий; `.opencode/commands/**` — в аудите `auditor`);
  C-секция (пункты session-commit и команд-фасадов) синхронизирована;
  черновик фазы A `/git/checkpoint.md` снят (Q41) — заменён ссылкой на
  действующий файл; примечание про `/git/status` актуализировано.
- **Проверки (чтением, D50 — `cargo` не запускался):** `rg "process/runN"` по
  карточке — **пусто** (в т.ч. `runN`/`session/runN`); заголовок строки `C3`
  приведён к принятому имени (было `process/runN`); diff локализован —
  `:108-115`, `:218-219`, `:263-266`, `:285`; ссылки D93 живы; переиспользования
  ID нет; `docs/TRACEABILITY.md` **не тронут** (Q90 `in work`, T-15 🚧).
- Задачи: новых Q/D/задач нет (закрытие ранее заведённой пары Q90→D93).
- Файлы: `docs/tasks/T-15-mcp-ready-process/README.md` (+ лента r15, память).
- Замечания: пункт 7 брифа — Q90 остаётся `in work` (T-15 🚧, D82); «Q90 → done»
  из секции `validator` не выполняется (закрытие — на `complete` T-15, вне r15).
  Канон не правил; статусы задач/фич не менял.

## docs-writer · 03.10.2026 · готово

- Сделано: закрытие статуса фичи по приёмке `validator` (`service-process-branch`,
  iteration 1) и D93. `docs/features/README.md:299` — строка
  `agents-session-checkpoint`: статус **🟡 → ✅**; примечание актуализировано
  (C3/C4 закрыты: ветка сессии `process/<прогон>-s<M>`, теги
  `session/<прогон>-s<M>`, снапшот `state/snapshots/<прогон>-s<M>/`, фасад
  `/git/checkpoint`; D93, `service-process-branch`; C9 ✅). Счётчик сценариев 4
  не менялся.
- Проверки (чтением, `cargo` не запускался — D50). Diff локализован:
  `git diff --numstat -- docs/features` → `README.md 1/1`,
  `agents-session-checkpoint.feature 2/2`. Ссылки живые:
  `D93-session-commit-process-branch.md` существует, формулировки строки
  совпадают с D93 (:52-66) и с фичей (:13-14). Фича
  `agents-session-checkpoint.feature` — правок не требует: сценарий 1 уже
  приведён к D93 (ветка `process/<прогон>-s<M>`, сообщение
  `chore(process): <прогон> s<M>`); `Сценарий:` — **4** (`rg -c`).
- Границы: правил только `docs/features/README.md`; счётчики/таблицы не
  пересобирал; журнал/канон/`docs/reviews/**` не трогал.
- Файлы: `docs/features/README.md` (+ лента r15, память).

## git · 03.10.2026 · чекпойнт до коммита

- Сделано: верификация пакета r15 (F43, до `git add`): снимок
  `git status --porcelain` = 20 M + 5 ?? = 25 путей, совпал с пакетом промпта;
  подтверждение владельца — лента r15, секция «закрытие — готово; пакет r15 —
  гейт владельца» (:115). База `develop` = `origin/develop` = `HEAD` = `f58e27c`;
  ветки нет (сервисная операция — коммит в `develop`). Коммит/push не выполнены
  (идемпотентность: нечего пропускать).
- Осталось: `add` 26 точными путями (25 + память `git`, F43) →
  `diff --cached --check` (пусто) → коммит
  `chore(process): T-15 r15 — session-commit: ветка сессии, теги, снапшот, фасад /git/checkpoint (C3/C4; D93); приёмка`
  → `push origin develop` (≥ 5 мин). После `push` в отслеживаемые файлы не пишу
  (F43).
