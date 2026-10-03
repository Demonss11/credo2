# D96: C6 — метрики из состояния (скрипт `state-metrics.mjs`)

- **Статус:** accepted
- **Дата:** 2026-10-03
- **Resolves:** [Q93](../questions/Q93.md)
- **Спека:** —
- **Affects:** `.opencode/scripts/state-metrics.mjs` (новый),
  `.opencode/rules/dispatch-loop.md` (ссылка),
  `AGENTS.md` (карта скриптов),
  [`agents-metrics.feature`](../features/agents-metrics.feature) (статус —
  `docs-writer`),
  карточка [`T-15`](../tasks/T-15-mcp-ready-process/README.md),
  [`TRACEABILITY.md`](../TRACEABILITY.md) (строка [Q93](../questions/Q93.md)/D96)
- **Tasks:** [`T-15`](../tasks/T-15-mcp-ready-process/README.md) (строка `C6`)

## Контекст

Записка [`mcp-ready-process.md`](../tasks/T-15-mcp-ready-process/mcp-ready-process.md)
§7 задаёт целевые метрики процесса: dispatch по ролям, шаги и упоры по
сессиям, re-plan по категориям, `outcome`; очереди `deferred_by_owner`/`blocked`;
метрику конвергенции (re-plan без `owner_override`); **источник** — `progress` +
`receipts` + состояние, ручные таблицы меморандума (W8 §3, §9–§11) уходят.
Чек-лист готовности к MCP §8 требует, чтобы «метрики прогона выводились из
состояния без ручного труда». Схема состояния уже закреплена
([D86](D86-state-schema.md)), валидатор есть ([D91](D91-c2-validate-state.md)),
но читателя (query-слоя) из состояния нет; `metrics-report.mjs` (B0-own P4) даёт
**нативные** метрики сессий (`stats`/`export`), а не слой процесса. Полный
контекст и варианты — [Q93](../questions/Q93.md).

Решение владельца 03.10.2026 (сервисная операция r18): носитель — **новый
скрипт** `.opencode/scripts/state-metrics.mjs` (метрики/очереди из `progress.yaml`
+ `receipts.yaml` + `state`); формат — **CLI + `--json`**; в каноне — ссылка
«метрики — скриптом», без дублирования таблиц.

## Решение

1. **Новый скрипт** `.opencode/scripts/state-metrics.mjs` — query-слой метрик
   процесса: считает метрики и очереди **из состояния**
   (`.opencode/state/current/{progress,receipts,current_state,next_action}.yaml`),
   без участия текстов прогонов.
2. **Формат вывода — CLI + `--json`.** CLI-вывод — markdown-секции для чтения;
   `--json` — машиночитаемо (для приёмки и мета-запросов; вход фазы E/MCP
   query-слоя — [D95](D95-phase-e-mcp-design.md)).
3. **Секции отчёта:**
   (1) прогон — задачи/сессии/шаги по ролям и упоры лимита по сессиям;
   (2) dispatch по ролям;
   (3) re-plan по категориям (`replan_reason`) + метрика конвергенции
   (re-plan без `owner_override`) + категория текущего состояния;
   (4) очереди `deferred_by_owner`/`blocked` (из состояния/плана);
   (5) квитанции (`verdict`, `iteration`).
4. **Источник — только состояние.** Нативные сессионные метрики (стоимость,
   токены, цепочки `stats`/`export`) остаются у `metrics-report.mjs` — **вне C6**;
   два слоя не смешиваются.
5. **Фильтры** — по потребности (`--task`, `--session`); `--out` — выгрузка
   отчёта в файл; `--dir` — источник (пробы).

6. **Канон — ссылка, не копия.** В `dispatch-loop.md` (и карте скриптов
   `AGENTS.md`) — ссылка «метрики выводятся скриптом», **без дублирования
   таблиц**. Фича [`agents-metrics`](../features/agents-metrics.feature) —
   зона `docs-writer`.

## Следствия

- Пункт чек-листа §8 («метрики прогона выводятся из состояния без ручного
  труда») получает механизм; ручные таблицы меморандума (записка §7)
  уходят.
- [`agents-metrics.feature`](../features/agents-metrics.feature) (3 сценария:
  метрики прогона, конвергенция, машиночитаемые очереди) получает исполнение;
  статус и обратная пометка `# D96 (Q93): …` — зона `docs-writer`
  ([`journal.md`](../../.opencode/rules/journal.md) §5.6).
- `--json` — вход query-слоя будущего process-MCP (фаза E,
  [T-26](../tasks/T-26-mcp-server-design/README.md)); нативные метрики сессий
  (B0-own P4) не затрагиваются.
- Продуктовый код `src/**` и контракты CREDO не затрагиваются: правка —
  служебная зона `.opencode/**` + `AGENTS.md`; аудит `auditor` (`L`).

## Сверка с кодом

Вердикт: ⚪ **не применимо** (процесс/служебная зона) — решение вводит скрипт
метрик процесса из состояния и канон-ссылку; продуктовый код и контракты CREDO
не меняет.

Что проверено (чтением, 03.10.2026), чем подтверждено:

- **Свобода номеров:** созданные [`Q93`](../questions/Q93.md) и `D96`; последние
  занятые — [Q92](../questions/Q92.md)/[D95](D95-phase-e-mcp-design.md)
  ([`questions/README.md`](../questions/README.md),
  [`decisions/README.md`](../decisions/README.md)); нумерация сквозная (§2
  [`journal.md`](../../.opencode/rules/journal.md)).
- **Задача-носитель:** `C6` — строка карточки
  [`T-15`](../tasks/T-15-mcp-ready-process/README.md) (состав; реестр — строка
  `C6`); отдельная задача не заводится.
- **Целевые метрики:** записка §7 — dispatch/шаги/упоры, re-plan по
  категориям, `outcome`; очереди `deferred_by_owner`/`blocked`; конвергенция
  (re-plan без `owner_override`); источник — `progress` + `receipts` + состояние.
- **Чек-лист:** записка §8 — «метрики прогона выводятся из состояния без
  ручного труда» (незакрытый пункт, закрывает `C6`).
- **Фича:** [`agents-metrics.feature`](../features/agents-metrics.feature) — 3
  сценария (метрики прогона из `progress`/`receipts`/состояния; конвергенция;
  машиночитаемые очереди `deferred_by_owner`/`blocked`).
- **Смежный слой:** `.opencode/scripts/metrics-report.mjs` (B0-own P4) — нативные
  метрики сессий (`stats`/`export`); в C6 не входит (разные источники/назначение).
  `state-metrics.mjs` на 03.10.2026 в `.opencode/scripts/` **отсутствует** — файл
  создаётся реализацией `C6`.

`cargo` не запускался (§5.3, [D50](D50-dod-by-package-scope.md), D50 — по брифу):
решение процессное, кода CREDO не касается; адресный прогон (`docs_journal`) — за
`validator` на приёмке (R2); прогон самого `state-metrics.mjs` — адресно на
приёмке.

**Задача —** [`T-15`](../tasks/T-15-mcp-ready-process/README.md) (`C6`): скрипт
`state-metrics.mjs` + ссылка в каноне/карте (`AGENTS.md`); статус строки — 🚧 на
время работы, ✅ — после приёмки.

## Альтернативы

- **(B) Расширить `metrics-report.mjs`** — отклонено: смешивает два слоя
  (нативные сессионные метрики `stats`/`export` и метрики процесса из состояния);
  разные источники, потребители и жизненный цикл.
- **(C) Оставить ручную сборку** — отклонено: не закрывает чек-лист §8,
  сохраняет ручной труд и расхождение со свойством «запрашиваемость» (§2).

## Ссылки

- Вопрос: [Q93](../questions/Q93.md)
- Основания: записка
  [`mcp-ready-process.md`](../tasks/T-15-mcp-ready-process/mcp-ready-process.md)
  §7, §8 (чек-лист), §2 (запрашиваемость);
  [D86](D86-state-schema.md) (схема), [D90](D90-c5-c7-re-raise-selfreport.md)
  (`replan_reason`), [D91](D91-c2-validate-state.md) (`validate-state.mjs`)
- Артефакты: карточка [`T-15`](../tasks/T-15-mcp-ready-process/README.md)
  (строка `C6`); [`agents-metrics.feature`](../features/agents-metrics.feature);
  `metrics-report.mjs` (нативные, вне C6)
- Сервисная операция 03.10.2026 (r18) — решение владельца (лента)
- Связи: [`TRACEABILITY.md`](../TRACEABILITY.md)
