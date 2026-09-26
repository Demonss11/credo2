# TRACEABILITY — связи Q → D → feature → задача

Сводная таблица журнала ([`BRIEF.md`](BRIEF.md) §6). Обновляется вместе с записями;
строки не удаляются. Статус совпадает со статусом вопроса.

| Q | D | Feature | Задачи | Статус |
|---|---|---|---|---|
| [Q1](questions/Q1.md) — SPEC: план «с нуля» или доработка `credo2`? | [D15](decisions/D15-evolution-credo2.md) — эволюция `credo2`, не greenfield | [`features/README.md`](features/README.md) (шапка) | [T-10](tasks/T-10-workspace-phase-0/README.md) | resolved |

Легенда статусов: `open` — ждёт решения · `resolved by Dn` — закрыт решением ·
`dropped` — снят без решения. В колонке «Задачи» — `T-XX` из
[`tasks/`](tasks/README.md) либо `—`; задачи появляются из сверки решения
с кодом ([`BRIEF.md`](BRIEF.md) §5.3).

Пока миграция не завершена, остальные вопросы живут в
[`OPEN_QUESTIONS.md`](OPEN_QUESTIONS.md); после переноса каждый получает строку
здесь, а в архиве остаётся указатель.
