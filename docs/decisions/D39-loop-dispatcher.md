# D39: Разгрузка `lead` — loop-диспетчер, эфемерный `analyst`, состояние на диске

- **Статус:** accepted
- **Дата:** 2026-09-27
- **Resolves:** [Q44](../questions/Q44.md)
- **Спека:** [`SPECIFICATION.md`](../SPECIFICATION.md) §10, решение №39
- **Affects:** [`AGENTS.md`](../../AGENTS.md) §Рабочая группа агентов;
  `.opencode/agents/lead.md`, `.opencode/agents/analyst.md` (new);
  `.opencode/rules/dispatch-loop.md` (new), `.opencode/rules/review.md`,
  `.opencode/rules/git-workflow.md`;
  `.opencode/memory/lead.md`, `.opencode/memory/analyst.md`;
  `.opencode/.gitignore`;
  [`features/agents-cycle.feature`](../features/agents-cycle.feature) и другие
  `agents-*.feature` (6 файлов); [`docs/README.md`](../README.md),
  `docs/analysis/README.md` (new);
  [`docs/CHANGELOG.md`](../CHANGELOG.md)
- **Tasks:** [T-12](../tasks/T-12-agent-loop/README.md)

## Контекст

Цикл R1–R5 ([D38](D38-agent-cycle.md)) работает по дисциплине, но **не доводит
задачи до конца, когда ведёт один `lead`**: разведка канона и смежных карточек
съедает шаги родителя; оба прогона (мета-сессия T-11 и чистый Run 2 —
внешние анализы сессий 26.09.2026) упёрлись в `steps` до закрытия задачи;
контекст накапливает весь пилот; scope-решения принимаются волюнтарно в ленте
(факт: граница конверта ошибок `check.create` ↔ T-04); обрыв сессии не имеет
durable-точки восстановления. Вопрос — [Q44](../questions/Q44.md).

Уточнения к исходному brief (сверено с репозиторием 27.09.2026): фактические
лимиты ролей — `lead` 28, `git` 28, `coder` 44, `validator` 36 и др. (в brief
были 24/16 — устарело); `.opencode/.gitignore` отслеживается и не игнорирует
себя; память/почта — рабочие данные **в git** (v2.1); часть рекомендаций T-11
(одиночные команды, «пути без `--`», порог H5, чекпойнт `K ≈ steps/3`,
`debug agents` ×2, headless `--model --auto`) уже внедрена.

## Решение

Минимальный объём архитектуры «loop-диспетчер + эфемерный `analyst` + состояние
на диске»:

1. **`lead` — loop-диспетчер** (имя сохранить, логику заменить): `mode: primary`,
   `default_agent`, `steps: 16`; исполняет `next_action.yaml` буквально
   («do not embellish, do not improvise»); **решений не принимает**, канон/код
   не читает, содержательных вопросов не задаёт; права: `edit` —
   `.opencode/memory/lead.md`, `.opencode/mail/**`,
   `.opencode/state/current/progress.yaml`; shell —
   `rg`, `git status|log|diff|show`, `git branch --show-current`; `subagent` —
   `analyst` + командные роли;
   `question: allow`.
2. **`analyst` — эфемерный суб-агент** (новый): `steps: 20`; одно решение за
   вызов: читает `mail`, `memory`, карточку, `Dn`, `features`, пишет досье и
   план, выходит; контекст не накапливается. Досье —
   `docs/analysis/<T-XX>-<дата>.md` (один файл на задачу, обновляется по ходу;
   ≤ 40 строк для S, ≤ 80 для M/L; пункты: цель, источник, скоуп, границы,
   открытые вопросы, класс, риски, критерии приёмки). Права: `edit` —
   `docs/analysis/**`, своя память, `mail/**`,
   `.opencode/state/current/next_action.yaml`,
   `.opencode/state/current/current_state.yaml`; shell — `rg`,
   `git status|log|diff|show`, `git grep`; **без cargo**; `subagent: deny`,
   `question: deny`. BA + SA не заводятся (условие пересмотра — реальный
   заказчик-человек или домен сложнее DSL; отдельным Q/D).
3. **Состояние на диске** — `.opencode/state/current/`, четыре файла:
   `next_action.yaml` (task, iteration, status `in_progress|awaiting_user|done|blocked`,
   `progress_marker`, очередь `next`, `resume_hint`) и `current_state.yaml` (фаза,
   с какого времени, артефакты, статус приёмки, счётчик rework) — **пишет только
   `analyst`**; `progress.yaml` (append-only механическая отметка исполнения
   действий и ответов владельца, без смысловых правок плана) — **пишет только
   `lead`**; `receipts.yaml` (append-only квитанции приёмки: task, iteration,
   verdict, report, DoD-цифры, снимок) — **пишет только `validator`** (закрывает
   находку «приёмка не подтверждена»). Resume читает `next_action` + `progress`.
   Права `edit` на файлы состояния — поэлементные у `analyst`/`lead`/`validator`;
   «один писатель на файл» обеспечен и правами.
   `state/` — вне git (`.opencode/.gitignore` += `state/`).
   Переезд схемы — одна строка в `dispatch-loop.md`.
4. **Правило цикла** — новый `.opencode/rules/dispatch-loop.md` (~40 строк):
   loop-контракт `lead`; действия `dispatch`, `surface_to_user`, `wait_for_user`,
   `complete`; **каденция**: очередь планируется до точки ветвления, re-plan —
   при исчерпании очереди, несовпадении ожидаемого маркера (`expect`) с отчётом
   роли и всегда при resume; `lead` фиксирует результаты механически.
   **Hard rules**: `cargo test` — только `validator` (R2 сохраняется);
   scope-решения (границы задач, контракты, форматы) — обязательная запись Q/D
   через `migrator` **до** исполнения; ≥ 3 итерации без прогресса
   (`progress_marker` не меняется) → `surface_to_user`; natural checkpoint
   каждые 6 действий (в headless — запись без паузы); принцип «состояние — это
   проект, ты проходишь через него».
5. **Fast path для S**: класс фиксирует `analyst` в досье (guard «сценарии
   `features/` → не ниже M» сохраняется); очередь: `git` (ветка) → `coder` →
   `validator` → `surface_to_user` (git-пакет) → `git` → `complete`;
   `rust-expert`/`tester` — только по обоснованию `analyst`. `validator` в fast
   path обязателен (R2).
6. **Канон и права**: `AGENTS.md` §Рабочая группа — таблица ролей (+`analyst`,
   новый смысл `lead`), схема `lead → analyst → роли`, ссылка на
   `dispatch-loop.md`, fast path. `review.md`: строка команд `analyst` (без
   cargo) + пункт автопроверки `auditor` «списки команд ↔ фронтматтеры» (закрыть
   регресс). `validator.md`: чек в `receipts.yaml`. Память: `lead.md`
   перепрофилируется («моя память — состояние; сюда только аварийные
   чекпойнты»), `analyst.md` — новый. Git-подтверждение R7 сохраняется:
   `next_action` содержит `surface_to_user` с пакетом, `lead` задаёт один вопрос
   и фиксирует ответ, `git` сверяет подтверждение (состояние + лента).
7. **Объём и пилот**: реализация — задача [T-12](../tasks/T-12-agent-loop/README.md)
   (волны W1–W4); пилот **Run 3** на T-03 — после мержа, отдельным прогоном,
   **без изменения** карточки/сценариев/критериев T-03 (P3-ссылку править после
   Run 3); сравнительная таблица Run 1/2/3; базис Run 2 (уточнённый):
   «88 passed / 0 failed исполнено, процесс не доведён — упор в лимит после
   приёмки; P3×1; возвратов в `coder` — 0».

## Следствия

- Упор в `steps` перестаёт быть фатальным (потолок — на одно решение, не на
  задачу); сессия crash-recoverable по построению; контекст не раздувается;
  задачи доводятся до коммита; scope-решения — в журнале.
- R2/R7, `review.md` (порог H5, `-rN`), зоны и «один писатель на файл»
  сохраняются.
- Требования `agents-*` и счётчики обновляются в T-12; полный канон процесса —
  `AGENTS.md` §Рабочая группа агентов (ссылка, без дублей — Q41).

## Сверка с кодом

Вердикт: ⚪ **не применимо** — решение процессное; контрактов и поведения
продуктового кода не меняет; задач кода не порождает (реализация — процессная
задача T-12).

Что проверено: канон процесса ([`AGENTS.md`](../../AGENTS.md),
`.opencode/agents/**`, `.opencode/rules/**`), фичи `agents-*`, фактические лимиты
и права ролей. Сверено с репозиторием 27.09.2026: лимиты `steps` — `lead` 28,
`git` 28, `coder` 44, `validator` 36, `auditor` 32, `tester` 28, `migrator` 28,
`rust-expert` 24, `docs-writer` 20, `researcher` 20 (в brief — 24/16, устарело);
`.opencode/.gitignore` отслеживается (содержит `node_modules`, `package.json`,
`package-lock.json`, `bun.lock`) и себя не игнорирует — `state/` добавляется
строкой. `cargo`-прогоны к вердикту не привлекаются: продуктовый код решением
не затрагивается; проверка прав ролей после правок — машинно в рамках T-12
(`opencode debug agents`, аудит `auditor`).

## Альтернативы

- **Поднять `steps` у `lead`** — лечит симптом; контекст и resume не решает
  (подтверждено Run 2). Отклонено.
- **BA + SA** — пересечение зон, дубли, +1 источник дрейфа «инструкция ↔
  права». Отклонено.
- **Полный перенос архитектуры arai** — специфика другого продукта. Отклонено.
- **«Умный `lead` + analyst» без смены логики `lead`** — упрётся снова.
  Отклонено.

## Ссылки

- Вопрос: [Q44](../questions/Q44.md)
- Краткий канон: [`SPECIFICATION.md`](../SPECIFICATION.md) §10, решение №39
- Предшествующее решение цикла: [D38](D38-agent-cycle.md) ([Q43](../questions/Q43.md))
- Канон процесса: [`AGENTS.md`](../../AGENTS.md) §Рабочая группа агентов
- Задача: [T-12](../tasks/T-12-agent-loop/README.md)
- Связи: [`TRACEABILITY.md`](../TRACEABILITY.md)
