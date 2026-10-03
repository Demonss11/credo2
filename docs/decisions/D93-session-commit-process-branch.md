# D93: C3-остаток и C4 — session-commit в ветку сессии, теги, снапшот, фасад `/git/checkpoint`

- **Статус:** accepted
- **Дата:** 2026-10-03
- **Resolves:** [Q90](../questions/Q90.md)
- **Спека:** —
- **Affects:** [`git-workflow.md`](../../.opencode/rules/git-workflow.md)
  (§«Session-commit»), [`dispatch-loop.md`](../../.opencode/rules/dispatch-loop.md)
  (правило останова), `.opencode/commands/git/checkpoint.md`,
  [`.opencode/commands/git/status.md`](../../.opencode/commands/git/status.md),
  [`.opencode/agents/git.md`](../../.opencode/agents/git.md),
  [`.opencode/agents/auditor.md`](../../.opencode/agents/auditor.md),
  [`.opencode/scripts/session-checkpoint.mjs`](../../.opencode/scripts/session-checkpoint.mjs),
  `AGENTS.md`, [`agents-session-checkpoint.feature`](../features/agents-session-checkpoint.feature)
- **Tasks:** [T-15](../tasks/T-15-mcp-ready-process/README.md) (фаза C, строки
  реестра `C3`, `C4`)

## Контекст

[D89](D89-branch-topology-freeze-session-commit.md) P3 принял **первый инкремент
C3** — session-commit: на границе сессии процесс-слой (ленты, память, `state/**`)
фиксируется отдельным коммитом **в текущую рабочую ветку** ролью `git` по
«постоянному» пакету (подтверждение один раз на прогон). Правило внесено в
[`git-workflow.md`](../../.opencode/rules/git-workflow.md) §«Session-commit».

**Остаток C3** прямо отложен «отдельным решением»: носитель по-сессионных
снапшотов состояния, теги сессий и фасад владельца `/git/checkpoint`
(место — то же §«Session-commit»). Предмет задан запиской
[`mcp-ready-process.md`](../tasks/T-15-mcp-ready-process/mcp-ready-process.md) §6
и фичей
[`agents-session-checkpoint.feature`](../features/agents-session-checkpoint.feature)
(4 сценария). Строки `C3` (остаток) и `C4` (фасады `/git/checkpoint`,
`/git/status`; `.opencode/commands/**` — в область аудита `auditor`) карточки
[T-15](../tasks/T-15-mcp-ready-process/README.md) — 🚧.

Коллизия топологии: накопительная ветка `process/<прогон>` на общем рабочем
дереве конфликтует с незакоммиченными файлами (`git switch` перезаписал бы
M-файлы); данные F80–F82 (T-16/T-25, грязное дерево на `branch_start` —
[F82](../analysis/findings-registry.md)). Полный контекст —
[Q90](../questions/Q90.md).

Решения владельца 03.10.2026 (сервисная операция r15, лента): имя прогона —
**идентификатор задачи** (`T-XX`); снапшот — **скрипт по вызову фасада**. Правки
канона (`.opencode/**`, `AGENTS.md`) — зона сервисной сессии; решение лишь
канонизирует их состав.

## Решение

1. **Прогон и сессия.** Прогон — исполнение процесса по задаче, его имя —
   **идентификатор задачи** (`T-XX`); сессия внутри прогона — `session_index`
   (`sM`). Имена ветки/тега/снапшота — от `<прогон>` = `T-XX`.
2. **Чекпойнт сессии — топология.** Ветка сессии `process/<прогон>-s<M>`
   создаётся **от текущего HEAD** (`git switch -c` — без конфликтов с
   незакоммиченными файлами). Накопительная ветка прогона (`process/<прогон>`)
   **отклонена**: на общем рабочем дереве конфликтует с M-файлами (switch
   перезаписал бы их). Состав процесс-коммита: лента (`.opencode/mail/**`),
   память (`.opencode/memory/**`), снапшот
   (`.opencode/state/snapshots/<прогон>-s<M>/`). Тег — `session/<прогон>-s<M>`;
   сообщение — `chore(process): <прогон> s<M>`; push ветки и тега; возврат на
   рабочую ветку. В `develop` ветка сессии **не мержится**.
3. **Live state.** `state/current/*` — как раньше: финальный process-коммит
   задачи идёт в её ветку → `develop`. Процесс-ветка — носитель по-сессионных
   снапшотов (страховка/resume), не источник для `develop`.
4. **Снапшот.** Копия `state/current/*` + `meta.md` в
   `.opencode/state/snapshots/<прогон>-s<M>/`; создаёт
   `session-checkpoint.mjs --snapshot` (значения `task`/`session_index` — из
   `current_state.yaml`; `--dir` — для проб). Право запуска — роль `git`
   (фасад/чекпойнт работает и после упора `lead` в лимит).
5. **«Постоянный» пакет** — подтверждение **один раз на прогон** (запись `lead`
   в ленте); без изменений относительно D89 P3; идемпотентность и catch-up — там
   же.
6. **Сервисные операции** (без ведущего `lead`, одна сессия) — **без
   process-ветки**: лента/память идут обычным пакетом операции (как r9–r14).
7. **Фасад `/git/checkpoint`** (`.opencode/commands/git/checkpoint.md`,
   `agent: git`): сверка записи подтверждения → снапшот скриптом → `switch -c` →
   `add` process-путей → `commit` → `tag` → `push` (ветка + тег) → `switch -`;
   **без shell-блоков**; «нет записи — нет чекпойнта»; идемпотентно.
8. **`/git/status`** — дополняется **тегами сессий**. `.opencode/commands/**` —
   **в область аудита `auditor`** (команды не обходят permission-поток).
9. **Фича `agents-session-checkpoint.feature`** — сценарий 1 приводится к
   принятому имени прогона (`process/<прогон>-s<M>`,
   `chore(process): <прогон> s<M>`); **счётчик сценариев 4 не меняется**.
10. **Идемпотентность/catch-up** — как D89 P3: повтор пропускается (пустой
    staged — успех без коммита); оборванный чекпойнт добирает следующая сессия
    тем же пакетом.

## Следствия

- C3-остаток и C4 закрыты в каноне: у каждой сессии — восстановимый снапшот
  состояния и тег; фасад работает и после упора `lead` в лимит (право у `git`).
- Носитель по-сессионных снапшотов отделён от `develop`: live state не
  дублируется, процесс-ветки сессий сохраняются, но не мержатся.
- Топология снимает конфликт с незакоммиченными файлами (F82): чекпойнт не
  переключает рабочее дерево на накопительную ветку.
- `.opencode/commands/**` входит в область аудита `auditor` — команды не
  обходят permission-поток.
- Правки процесса (`.opencode/**`, `AGENTS.md`) — зона сервисной сессии;
  продуктовый код `src/**` и контракты CREDO не затрагиваются.

## Сверка с кодом

Вердикт: ⚪ **не применимо** (процесс/служебная зона) — решение канонизирует
контур session-commit (ветка/тег/снапшот/фасад) агентского процесса; продуктовый
код CREDO и контракты не меняет.

Что проверено (чтением, 03.10.2026), чем подтверждено:

- **Свобода номеров:** `docs/questions/Q90.md` (создан этим пакетом) и
  `docs/decisions/D93-*`; последние занятые — Q89/D92
  ([`questions/README.md`](../questions/README.md),
  [`decisions/README.md`](../decisions/README.md)); нумерация сквозная (§2
  [`journal.md`](../../.opencode/rules/journal.md)).
- **Зафиксированный остаток:** [`git-workflow.md`](../../.opencode/rules/git-workflow.md)
  §«Session-commit» (строка об остатке `process/runN`, теги, фасад) — чтением.
- **Шаблон фасада и права:** черновик `checkpoint.md` — карточка
  [T-15](../tasks/T-15-mcp-ready-process/README.md) §«Черновик команды-фасада»;
  право роли `git` на снапшот-скрипт — [`.opencode/agents/git.md`](../../.opencode/agents/git.md);
  область аудита — [`.opencode/agents/auditor.md`](../../.opencode/agents/auditor.md).
- **Фича:** [`agents-session-checkpoint.feature`](../features/agents-session-checkpoint.feature)
  — 4 сценария, правится формулировка сценария 1 (§5.6 — зона `docs-writer`).
- **Периметр правок:** `.opencode/**`, `AGENTS.md` — зона сервисной сессии;
  правки вносятся отдельно; команда `/git/checkpoint` пока отсутствует
  (`.opencode/commands/git/` — только `status.md`).

`cargo` не запускался (§5.3, [D50](D50-dod-by-package-scope.md)): решение
процессное, кода CREDO не касается; адресный прогон — `validator` на приёмке
(R2).

**Задача —** [T-15](../tasks/T-15-mcp-ready-process/README.md): решение
легитимирует остаток C3 и C4 в рамках программы (фаза C, строки реестра `C3`,
`C4`).

## Альтернативы

- **Глобальный номер прогона (`runN`, вариант B Q90)** — отклонён: имя прогона
  теряет связь с задачей (`T-XX`); накопительная ветка `process/runN` на общем
  дереве конфликтует с M-файлами (switch перезаписал бы их).
- **Без process-ветки (инкремент 1 + теги, вариант C Q90)** — отклонён: нет
  носителя по-сессионных снапшотов/resume, противоречит сценарию «Восстановление
  по снапшоту» фичи
  [`agents-session-checkpoint.feature`](../features/agents-session-checkpoint.feature).
- **Снапшот силами `lead`** — отклонён: фасад/чекпойнт должен работать и после
  упора `lead` в лимит (право — у роли `git`, записка
  [`mcp-ready-process.md`](../tasks/T-15-mcp-ready-process/mcp-ready-process.md) §6).

## Ссылки

- Вопрос: [Q90](../questions/Q90.md)
- Предыдущий инкремент: [D89](D89-branch-topology-freeze-session-commit.md) (P3,
  C3 инкремент 1); [D92](D92-credited-run-predicate.md) (session-commit — не
  дефект зачёта)
- Основания: записка
  [`mcp-ready-process.md`](../tasks/T-15-mcp-ready-process/mcp-ready-process.md) §6;
  фича
  [`agents-session-checkpoint.feature`](../features/agents-session-checkpoint.feature);
  [F82](../analysis/findings-registry.md); правило
  [`git-workflow.md`](../../.opencode/rules/git-workflow.md) §«Session-commit»
- Артефакты: карточка [T-15](../tasks/T-15-mcp-ready-process/README.md)
  (фаза C, строки реестра `C3`, `C4`); скрипт
  [`.opencode/scripts/session-checkpoint.mjs`](../../.opencode/scripts/session-checkpoint.mjs)
- Сервисная операция 03.10.2026 (r15) — решение владельца (лента)
- Связи: [`TRACEABILITY.md`](../TRACEABILITY.md)
