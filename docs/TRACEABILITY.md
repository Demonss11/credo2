# TRACEABILITY — связи Q → D → feature → задача

Сводная таблица журнала ([`BRIEF.md`](BRIEF.md) §6). Обновляется вместе с записями;
строки не удаляются. Статус совпадает со статусом вопроса.

| Q | D | Feature | Задачи | Статус |
|---|---|---|---|---|
| [Q1](questions/Q1.md) — SPEC: план «с нуля» или доработка `credo2`? | [D15](decisions/D15-evolution-credo2.md) — эволюция `credo2`, не greenfield | [`features/README.md`](features/README.md) (шапка) | [T-10](tasks/T-10-workspace-phase-0/README.md) | resolved |
| [Q43](questions/Q43.md) — каким должен быть цикл работы команды агентов? | [D38](decisions/D38-agent-cycle.md) — Agile-петля, единый тестировщик, память и почта | [`agents-cycle.feature`](features/agents-cycle.feature) и др. `agents-*.feature` | [T-11](tasks/T-11-agent-cycle/README.md) | resolved |
| [Q44](questions/Q44.md) — как разгрузить `lead` и сделать цикл агентов durable? | [D39](decisions/D39-loop-dispatcher.md) — loop-диспетчер, эфемерный `analyst`, состояние на диске | [`agents-cycle.feature`](features/agents-cycle.feature) и др. `agents-*.feature` (6 файлов) | [T-12](tasks/T-12-agent-loop/README.md) | resolved |
| [Q45](questions/Q45.md) — какой порог scope-решения в цикле диспетчера? | [D40](decisions/D40-scope-threshold.md) — узкий порог + триггер частичного покрытия | — | [T-13](tasks/T-13-agent-hardening/README.md) | resolved |
| [Q46](questions/Q46.md) — какие уточнения канона цикла диспетчера нужны по итогам Run 3? | [D41](decisions/D41-dispatch-refinements.md) — уточнения `dispatch-loop` после Run 3 | — | [T-13](tasks/T-13-agent-hardening/README.md), [T-14](tasks/T-14-grammar-message-sync/README.md) | resolved |

Легенда статусов: `open` — ждёт решения · `resolved by Dn` — закрыт решением ·
`dropped` — снят без решения. В колонке «Задачи» — `T-XX` из
[`tasks/`](tasks/README.md) либо `—`; задачи появляются из сверки решения
с кодом ([`BRIEF.md`](BRIEF.md) §5.3).

Пока миграция не завершена, остальные вопросы живут в
[`OPEN_QUESTIONS.md`](OPEN_QUESTIONS.md); после переноса каждый получает строку
здесь, а в архиве остаётся указатель.
