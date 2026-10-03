# Приёмка: сервисная операция r17 — открытие фазы E (дизайн process-MCP)

- **Дата:** 2026-10-03
- **Тип:** сервисная документная операция (D50 — полный DoD не требуется)
- **Предмет:** Q92 → D95, задача `T-26-mcp-server-design`
- **Снимок:** `develop`, рабочее дерево r17 (пакет не закоммичен)
- **Вердикт:** принято (P1/P2/P3 нет)

## Режим и прогон

Адресная документная сверка + адресные тесты (F74): `cargo test --test
docs_journal`, `cargo test --test features_inventory`, `node
.opencode/scripts/agents-perms.mjs`. Полный DoD не запускался: `src/**`,
`tests/**`, `Cargo.toml` не тронуты (D50).

| Проверка | Ожидание | Факт |
|---|---|---|
| `cargo test --test docs_journal` | 14 passed / 0 failed | **14 passed / 0 failed** (1.96s) |
| `cargo test --test features_inventory` | 4 passed / 0 failed | **4 passed / 0 failed** (0.55s) |
| `node .opencode/scripts/agents-perms.mjs` | 11 из 18 | **11 из 18** |

`git diff --numstat -- src tests Cargo.toml` — пусто; `git diff --check` — пусто.

## Проверено (пункты задания)

1. **Адресные тесты.** `docs_journal` — 14/0, в т.ч. ключевые
   `traceability_tasks_exist_and_match_registry`,
   `features_are_named_in_traceability_and_exist`,
   `traceability_lifecycle_matches_task_openness`,
   `no_addresses_to_removable_or_session_data`. `features_inventory` — 4/4
   (`feature_files_are_valid_documents`, `feature_files_match_readme_inventory`,
   `scenario_counts_match_readme`, `readme_totals_match_files`); фича
   `agents-mcp-readiness` не менялась, README:302 счётчик `4` и «Итого: 47
   файлов, 278 сценариев» согласованы.

2. **D95 ↔ факт.**
   - фаза E = отдельная задача `T-26` (⬜, P1; `D95:34-35,53`, карточка:3-4) —
     совпадает; P1 обоснован (`D95:53-55`: продолжение программы T-15);
   - предмет — **только** process-MCP (storage + validation + query);
     git-MCP (Приложение A) вне предмета (`D95:39-41`, карточка:56-57) — да;
   - старт после заморозки процесса (фаза D T-15); вход — подготовленный B2
     `B2_PROFILE` (`D95:42-45`, карточка:6-7,58-61) — да;
   - класс априори L, финально — `analyst` (`D95:46-47`, карточка:25-26,62) — да;
   - референсы kibi / Semantic Anchors / BRHP / Telemetry DB (`D95:48-49`) — да;
   - «решений не принимает» (`D95:37-38`, карточка:63-64) — да;
   - «Сверка с кодом» ⚪ не применимо (`D95:73-75`) — обоснованно (процессное
     решение, кода CREDO не касается).

3. **Задача T-26.** Карточка полная: Источник `D95 (Q92)` / `D78` → «Что
   сделать» (6 пунктов: 6–10 инструментов §5.4, контракты, validation, query,
   traceability, дизайн-документ) → Критерий готовности → Примечания.
   Видимость: `tasks/README.md:59` (P1, «Зависит от: T-15 фаза D», ⬜);
   `TRACEABILITY.md:96` (`in work`, `[T-26] ⬜`); каталоги `questions/README.md:114`,
   `decisions/README.md:124`; карточка T-15 `:161-165` — фаза E со ссылкой на
   `T-26`. Q92↔D95 парны (`Resolves`/`resolved by`).

4. **P2 аудита — подтверждён записью.** Чекпойнт `migrator` за r17 внесён:
   `.opencode/memory/migrator.md:450-463` (Q92→D95, `T-26` ⬜, реестр `:59`,
   T-15 `:165`, фича не тронута, грабля «лимит шагов», аудит P2 закрыт).

5. **Границы.** `git status --porcelain`: 5 `M` (TRACEABILITY, decisions/README,
   questions/README, tasks/README, T-15) + memory auditor/migrator + 4 `??`
   (лента r17, `Q92`, `D95`, `T-26/**`) — строго по списку операции.
   `src/**`, `tests/**`, `Cargo.toml` не тронуты; фичи не тронуты.

## Находки

**P1:** нет. **P2:** нет. **P3:** нет.

## Что проверено и ок

D95 ↔ факт (все пункты решения), Q92↔D95, карточка T-26 (полнота и ссылки),
связность реестр/TRACEABILITY/каталоги/T-15, фича `agents-mcp-readiness`
(не изменена — корректно), P2 аудита (чекпойнт `migrator`), границы файлов,
адресные тесты `docs_journal`/`features_inventory`, машинная сверка прав.

## Технические замечания

- Пакет не закоммичен (сервисная операция завершается гейтом и `git`).
- Канон не правился; статусы не менялись (закрытие — не предмет этой операции).
