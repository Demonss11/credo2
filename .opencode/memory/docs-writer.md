# Память: docs-writer (документация и статусы)

- **Канон:** `docs/tasks/README.md`, `docs/features/README.md`,
  `docs/BRIEF.md` §9.
- **Правило:** чекпойнт — какие файлы/статусы менялись, что осталось, ссылки.
  Кратко.

## Чекпойнты

- 2026-09-28 · операция `service-t11-closeout` · закрытие T-11: статус ✅ в
  карточке `docs/tasks/T-11-agent-cycle/README.md` и сводке `docs/tasks/README.md`;
  «Примечания» с уликами (r3 `reviews/T-11-2026-09-26-r3.md`, Run 4/T-03 `-r2`
  `reviews/T-03-2026-09-27*.md`, меморандумы W8 т.1/т.2, коммиты
  `0a5832f`/`5019c45`); `docs/features/README.md` — уточнены `agents-rework`
  (🟡, остаток D42 → T-15 B1-F15) и блок «Пилоты состоялись»;
  `docs/tasks/T-15-mcp-ready-process/README.md` — зависимость снята, H7 → T-16;
  `docs/CHANGELOG.md` — запись «Закрытие T-11 и задача T-16». Счётчики сценариев
  не менялись. Сводка T-15 синхронизирована — зависимость снята. P3 приёмки
  T-11-closeout закрыт: легенда блока «Пилоты состоялись» в
  `docs/features/README.md` приведена в соответствие со строкой `agents-rework`
  (🟡 — структурная готовность, подтверждено не полностью).
  Остаток: приёмка `validator` и пакет `git`.
- 2026-09-28 · операция `service-migration-q2q3` · сопутствующие документы к
  переносу Q2+Q3 → D16: `docs/features/README.md` :69 — ссылки `[Q3]`/`[D16]`
  в ноте Q3 (счётчики 47/278 не менялись); обратные ссылки `# D16 (Q2, Q3)` в
  шапках `parser.feature`/`lexer.feature`/`execution.feature`; `docs/CHANGELOG.md`
  :19–24 — запись «Перенос Q2, Q3 → D16». Ноты Q4/Q36/Q38 не трогал. Остаток:
  приёмка `validator` (docs) и пакет `git`.
- 2026-09-29 · операция `service-migration-q4` · сопутствующие к переносу Q4 → D17:
  `docs/features/README.md` :75–79 — ссылки `[Q4]`/`[D17]`, список фич уточнён
  (убран `explain.feature`, остались `parser`/`execution`/`editor`/`lsp`/
  `client_explanation`), счётчики 47/278 не менялись; обратные ссылки
  `# D17 (Q4)` в шапках `parser.feature`, `execution.feature`, `editor.feature`,
  `lsp.feature`; `client_explanation.feature` :8 `(Q4/Q36)` → `(D17/Q36)`;
  `docs/CHANGELOG.md` :25–28 — запись «Перенос Q4 → D17». Журнал/реестр/SPEC/
  tasks не трогал. Остаток: приёмка `validator` (docs) и пакет `git`.
- 2026-09-29 · операция `service-migration-q5q6` · сопутствующие к переносу
  Q5, Q6 → D19: `docs/features/README.md` :12–14 — в ноте «Решения Q5/Q40»
  обратная ссылка `[Q5]`→`[D19]`; `docs/CHANGELOG.md` §«Документация» :29–37 —
  запись «Перенос Q5, Q6 → D19». Шапки фич не трогал: D19 — канон статусов и
  приоритетов целиком, отдельных фич не адресует (в отличие от Q4 → D17).
  Счётчики 47/278 не менялись; журнал/реестр/SPEC/tasks не трогал.
  Остаток: приёмка `validator` (docs) и пакет `git`.
