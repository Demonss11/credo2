# D90: C5/C7 — re-raise (объект/категории, `replan_reason`) и шаблон самоотчёта + статусы фич

- **Статус:** accepted
- **Дата:** 2026-10-03
- **Resolves:** [Q87](../questions/Q87.md)
- **Спека:** — (SPEC §10 — указатель [D70](D70-spec-reduction.md); запись в
  `SPECIFICATION.md` не требуется)
- **Affects:** `.opencode/rules/dispatch-loop.md`,
  `.opencode/rules/state-schema.md`, `.opencode/agents/analyst.md`,
  `.opencode/agents/lead.md`, [`../features/README.md`](../features/README.md)
- **Tasks:** [T-15](../tasks/T-15-mcp-ready-process/README.md) (фаза C; строки
  реестра `C5`, `C7`)

## Контекст

Открытие сервисной операции 03.10.2026 (лента r12): два остаточных пункта
фазы C программы T-15 — re-raise и шаблон самоотчёта + статусы фич.

- **C5 — re-raise.** Записка
  [`mcp-ready-process.md`](../tasks/T-15-mcp-ready-process/mcp-ready-process.md)
  §5.5 задаёт объект re-raise (поля `id`, `category`, `origin`, `failed_clause`
  (`expectation`, `observation`), `fix`, `blocking`, `resolved`) и категории
  `expect_mismatch` · `owner_override` · `plan_gap` · `role_failure`; §7 —
  метрику конвергенции (число re-plan без `owner_override`). Контур
  `lead`/re-plan настроен (D88), но объект re-raise не канонизирован — метрика
  конвергенции неотличима от шума.
- **C7 — самоотчёт и фичи.** Обёртки на лимите шумны и дороги (F61: 18 369 и
  12 984 символа, EN, reasoning-утечки); серия упоров на прогоне (F58) —
  мотивация короткого сегментного самоотчёта. Статусы фич `agents-*` не
  отражают фактическое состояние фазы C: C1 ✅ (D86), C3 инкремент 1 + C9 ✅
  (D88/D89), C2 ⬜, остаток C3 (`process/runN`/теги/фасад).

Владелец на открытии сервисной операции 03.10.2026 (лента r12) выбрал вариант
(A) — «Только C5+C7». Полный контекст — [Q87](../questions/Q87.md). Правки
процесса (`.opencode/**`) — зона сервисной сессии и вносятся отдельно;
статусы фич — зона `docs-writer`. Решение лишь канонизирует их состав.

## Решение

1. **C5 — re-raise.** При re-plan `analyst` фиксирует объект `re_raise` в плане
   (`next_action.re_raise`): `id`, `category`, `origin`, `failed_clause`
   (`expectation`/`observation`), `fix`, `blocking`, `resolved`. Категории —
   `expect_mismatch` · `owner_override` · `plan_gap` · `role_failure`;
   `lead` зеркалит категорию в `progress.replan_reason`. `owner_override` —
   вне метрики конвергенции; повтор `owner_override` на одном участке — сигнал
   к разбору плана, не норма. Основание — записка §5.5/§7.
2. **C7 — самоотчёт и фичи.** Короткий шаблон сегментного самоотчёта (задача /
   фаза / статус / лента + сделано / дальше / риски) — канон цикла. Статусы фич
   `agents-*` в [`features/README.md`](../features/README.md) обновляются
   (`docs-writer`): `agents-re-raise` ⬜→✅, `agents-state-schema` ⬜→🟡
   (C1 ✅, C2 ⬜), `agents-session-checkpoint` ⬜→🟡 (C3 инкремент 1 + C9 ✅;
   остаток `process/runN`/теги/фасад); счётчики файлов/сценариев без изменений.
   Основание — F58/F61.

## Следствия

- Метрика конвергенции считается из состояния: re-plan классифицирован,
  `owner_override` вынесен за метрику, повтор на участке — явный сигнал
  (F‑метрика, записка §7).
- Упоры лимита и обёртки перестают быть дорогими: сегментный итог — короткий
  шаблон вместо длинной обёртки (F58/F61).
- Статусы фич `agents-*` отражают фактическое состояние фазы C; счётчики
  файлов/сценариев не меняются.
- Правки процесса (`.opencode/**`) — зона сервисной сессии; статусы фич —
  `docs-writer`; продуктовый код `src/**` и контракты не затрагиваются.

## Сверка с кодом

Вердикт: ⚪ **не применимо** (процесс/документы) — решение меняет канон
агентского процесса (объект re-raise, шаблон самоотчёта, статусы фич),
продуктовый код и контракты не меняет.

Что проверено (чтением, 03.10.2026), чем подтверждено:

- **Свобода номеров:** `docs/questions/Q87.md` (создан этим пакетом) и
  `docs/decisions/D90-*`; последние занятые — Q86/D89
  ([`questions/README.md`](../questions/README.md),
  [`decisions/README.md`](../decisions/README.md)); нумерация сквозная (§2
  [`journal.md`](../../.opencode/rules/journal.md)).
- **Основания:** F58/F61 в
  [`findings-registry.md`](../analysis/findings-registry.md); объект и
  категории — записка §5.5, метрика — §7 — чтением.
- **Периметр правок:** `.opencode/rules/{dispatch-loop,state-schema}.md`,
  `.opencode/agents/{analyst,lead}.md` — зона сервисной сессии;
  [`features/README.md`](../features/README.md) (статусы фич) — `docs-writer`;
  правки вносятся отдельно.

`cargo` не запускался (§5.3, [D50](D50-dod-by-package-scope.md)): решение
документное, кода не касается; адресный прогон — `validator` на приёмке (R2).

**Задача —** [T-15](../tasks/T-15-mcp-ready-process/README.md): решение
легитимирует правки объекта re-raise и шаблона самоотчёта в рамках программы
(фаза C, строки реестра `C5`, `C7`).

## Альтернативы

- **Оставить как есть** (re-raise без объекта/категорий, длинные обёртки,
  статусы фич ⬜) — отклонено: метрика конвергенции неотличима от шума
  (записка §5.5/§7), упоры и обёртки дороги (F58/F61), статусы фич не
  отражают состояние фазы C.
- **Только C5** (re-raise без самоотчёта/фич) — отклонено: F61 остаётся;
  пункты связаны через метрики и состояние (F58/F61).
- **Отложить** — отклонено: остаток фазы C тормозит заморозку (записка §8,
  re-raise — критерий готовности).

## Ссылки

- Вопрос: [Q87](../questions/Q87.md)
- Основания: F58/F61 в
  [`findings-registry.md`](../analysis/findings-registry.md); объект/категории
  и метрика — записка [`mcp-ready-process.md`](../tasks/T-15-mcp-ready-process/mcp-ready-process.md)
  §5.5, §7
- Смежное: [D88](D88-c9-c12-loop-tuning.md) (re-plan-политика),
  [D89](D89-branch-topology-freeze-session-commit.md) (session-commit —
  первый инкремент C3)
- Артефакты: карточка [T-15](../tasks/T-15-mcp-ready-process/README.md)
  (фаза C, строки реестра `C5`, `C7`)
- Сервисная операция 03.10.2026 — решение владельца (лента r12)
- Связи: [`TRACEABILITY.md`](../TRACEABILITY.md)
