# D97: C8 — развязка кандидатов B0 (фазы C/D/E/F)

- **Статус:** accepted
- **Дата:** 2026-10-03
- **Resolves:** [Q94](../questions/Q94.md)
- **Спека:** —
- **Affects:** карточка [`T-15`](../tasks/T-15-mcp-ready-process/README.md)
  (строка `C8`; пункт «кандидаты из отчёта B0»),
  [`TRACEABILITY.md`](../TRACEABILITY.md) (строка [Q94](../questions/Q94.md)/D97);
  отчёт [`wave0b-report.md`](../tasks/T-15-mcp-ready-process/wave0b-report.md) §4 —
  пометка «разведено» (зона `docs-writer`, отдельно)
- **Tasks:** [`T-15`](../tasks/T-15-mcp-ready-process/README.md) (строка `C8`)

## Контекст

Строка `C8` карточки [T-15](../tasks/T-15-mcp-ready-process/README.md) —
«кандидаты B0 → C/D/E/F» — имела пометку `⏸ ждёт B0`. Волна B0 (B0-i1…i8) и
мини-волна B0-own (P1–P5) **закрыты** 28.09–02.10.2026
([`wave0b-report.md`](../tasks/T-15-mcp-ready-process/wave0b-report.md) §1, §6;
[`wave0b-own-report.md`](../tasks/T-15-mcp-ready-process/wave0b-own-report.md) §1,
§5); пометка устарела.

Кандидаты §4 отчёта B0 разошлись по фазам: часть закрыта **отдельными
решениями** (C9/[D88](D88-c9-c12-loop-tuning.md), C10/[D87](D87-t24-pilot-fixes.md),
C3/[D93](D93-session-commit-process-branch.md), [D86](D86-state-schema.md),
[D91](D91-c2-validate-state.md), [D96](D96-state-metrics.md)), часть вынесена за
closeout C8 (фазы E/F). При этом решения владельца по **Shell Strategy** и
**snip** жили **только в отчёте** (§4, §6) — не в журнале; это нарушает политику
«один факт — один канон» (Q41): канон ссылается на канон, а не на отчёт волны.
Полный контекст и варианты — [Q94](../questions/Q94.md).

Решение владельца 03.10.2026 (сервисная операция r19): развязка оформляется
**одной парой** Q→D со **ссылками** на уже принятые решения; Shell Strategy —
«реализовано» (вариант a), snip — «отклонён».

## Решение

**Таблица развязки** («кандидат → фаза → диспозиция → основание»); закрытые
кандидаты — **ссылками** на уже принятые решения, без переписывания их
формулировок (Q41):

| Кандидат B0 (§4) | Фаза | Диспозиция | Основание |
|---|---|---|---|
| Shell Strategy | C | **реализовано** — канон «non-interactive shell» | канон `AGENTS.md` §«Служебная зона и аудит» (одиночные неинтерактивные команды); подтверждено владельцем 03.10.2026 |
| CC Safety Net | C | **реализовано** — перенос 02.10, пресет standard | C10 (карточка T-15); отчёт §4 |
| CC Safety Net CLI (`status/doctor/explain/logs/gui`) | F | диагностический слой, **вне closeout C8** | отчёт §4; карточка T-15, фаза F |
| Handoff | C | **реализовано** — правило останова + `session-checkpoint` | C9/[D88](D88-c9-c12-loop-tuning.md); C3/[D93](D93-session-commit-process-branch.md) |
| Наблюдаемость субагентов / Agent Identity | C | **реализовано/свёрнуто** — `wave0-observe`, атрибуция | [`wave0b-own-report.md`](../tasks/T-15-mcp-ready-process/wave0b-own-report.md) P1/P4/P5; C6/[D96](D96-state-metrics.md) |
| Opencode Telemetry | D | **отклонён** — V1; заменён нативными `stats`/`export` + `metrics-report.mjs`/`state-metrics.mjs` | отчёт §4 (D); C6/[D96](D96-state-metrics.md) |
| BRHP | E | **референс** (не установлен) — база схемы/валидатора | [D95](D95-phase-e-mcp-design.md) (фаза E, `T-26`); отчёт §4; канон схемы/валидатора — [D86](D86-state-schema.md)/[D91](D91-c2-validate-state.md) |
| snip | C | **отклонён** | владелец 03.10.2026; пересмотр при V2-порте upstream / Windows-сборке |
| Plannotator / нативные уведомления V2 / `/git/*` | F | **вне closeout C8** | карточка T-15, фаза F; отчёт §4 |

Следствия развязки:

1. Пометка `C8` карточки «⏸ ждёт B0» **снимается** — условие наступило; строка
   переводится в рабочий статус (🚧 на время правки, ✅ — после приёмки).
2. Дубль «кандидаты из отчёта B0» (пункт §4 в карточке) **сжимается до ссылки**
   на D97 — снятие дублирования (Q41).
3. Решения по Shell Strategy и snip перестают жить «только в отчёте»: они
   зафиксированы здесь; отчёт §4 получает пометку «разведено» (зона
   `docs-writer`, отдельной правкой).
4. Задача-носитель — строка `C8` карточки
   [`T-15`](../tasks/T-15-mcp-ready-process/README.md); отдельная задача не
   заводится.

## Следствия

- Устаревшая пометка `⏸ ждёт B0` снята; `C8` становится закрываемой строкой
  фазы C.
- «Решения в отчёте» (Shell Strategy, snip) переведены в журнал; отчёт §4 —
  навигация, не несущий текст.
- Закрытые кандидаты остаются у своих решений (C9/C10/D86/D87/D88/D91/D93/
  D96) — без переписывания (Q41); BRHP — референс фазы E (D95/T-26).
- Продуктовый код `src/**` и контракты CREDO не затрагиваются: правка —
  документы/журнал; служебная зона `.opencode/**` не правится.

## Сверка с кодом

Вердикт: ⚪ **не применимо** (процесс/документы) — решение фиксирует развязку
кандидатов B0 по фазам; продуктовый код и контракты CREDO не меняет.

Что проверено (чтением, 03.10.2026), чем подтверждено:

- **Свобода номеров:** созданные [Q94](../questions/Q94.md) и `D97`; последние
  занятые — [Q93](../questions/Q93.md)/[D96](D96-state-metrics.md)
  ([`questions/README.md`](../questions/README.md),
  [`decisions/README.md`](../decisions/README.md)); нумерация сквозная (§2
  [`journal.md`](../../.opencode/rules/journal.md)).
- **Задача-носитель:** `C8` — строка карточки
  [`T-15`](../tasks/T-15-mcp-ready-process/README.md) (состав); отдельная задача
  не заводится.
- **Кандидаты §4:** [`wave0b-report.md`](../tasks/T-15-mcp-ready-process/wave0b-report.md)
  §4 (C/D/E/F-рекомендации) и §6 (решения владельца 28.09.2026); B0-own P1–P5 —
  [`wave0b-own-report.md`](../tasks/T-15-mcp-ready-process/wave0b-own-report.md)
  §1, §3, §5 (P1/P3/P4 перенесены, P5 свёрнут).
- **Реализованность артефактов (существование файлов):**
  `.opencode/plugins/wave0-observe.ts`; `.opencode/scripts/session-checkpoint.mjs`,
  `.opencode/scripts/metrics-report.mjs`, `.opencode/scripts/state-metrics.mjs`,
  `.opencode/scripts/validate-state.mjs` — присутствуют (glob).
- **Основания-решения:** [D86](D86-state-schema.md) (схема),
  [D87](D87-t24-pilot-fixes.md) (права/CCSN),
  [D88](D88-c9-c12-loop-tuning.md) (C9),
  [D90](D90-c5-c7-re-raise-selfreport.md),
  [D91](D91-c2-validate-state.md) (валидатор),
  [D93](D93-session-commit-process-branch.md) (session-commit/checkpoint),
  [D95](D95-phase-e-mcp-design.md) (фаза E),
  [D96](D96-state-metrics.md) (метрики из состояния) — файлы существуют (glob).
- **Канон Shell Strategy:** `AGENTS.md` §«Служебная зона и аудит» — норма
  «Shell-команды давай одиночными (без `;`, пайпов и перенаправлений)»;
  подтверждено владельцем 03.10.2026 (вариант a — «реализовано»).

`cargo` не запускался (§5.3, [D50](D50-dod-by-package-scope.md)): решение
процессное/документное, кода CREDO не касается; адресный прогон
(`docs_journal`) — за `validator` на приёмке (R2).

**Задача —** [`T-15`](../tasks/T-15-mcp-ready-process/README.md) (`C8`): строка
`C8` и сжатие пункта «кандидаты из отчёта B0» до ссылки на D97; статус строки —
🚧 на время работы, ✅ — после приёмки.

## Альтернативы

- **(B) Правка карточки без D** — отклонено: оставляет развязку вне журнала и не
  снимает «решения в отчёте» (Shell Strategy, snip) — нарушение Q41.
- **(C) D на каждого кандидата** — отклонено: большинство записей дублировали бы
  уже принятые решения (Q41), раздувая журнал.

## Ссылки

- Вопрос: [Q94](../questions/Q94.md)
- Основания-решения: [D86](D86-state-schema.md), [D87](D87-t24-pilot-fixes.md),
  [D88](D88-c9-c12-loop-tuning.md), [D90](D90-c5-c7-re-raise-selfreport.md),
  [D91](D91-c2-validate-state.md), [D93](D93-session-commit-process-branch.md),
  [D95](D95-phase-e-mcp-design.md), [D96](D96-state-metrics.md)
- Артефакты: карточка [`T-15`](../tasks/T-15-mcp-ready-process/README.md)
  (строка `C8`); отчёты
  [`wave0b-report.md`](../tasks/T-15-mcp-ready-process/wave0b-report.md) §4–§6,
  [`wave0b-own-report.md`](../tasks/T-15-mcp-ready-process/wave0b-own-report.md)
  §1–§5 (навигация, не несущее)
- Сервисная операция 03.10.2026 (r19) — решение владельца
- Связи: [`TRACEABILITY.md`](../TRACEABILITY.md)
