# Приёмка: service-migration-q2q3 (перенос Q2, Q3 → D16)

**Проверка:** сервисная операция переноса блока **Q2 + Q3** из архива
`OPEN_QUESTIONS.md` в журнал (`SPECIFICATION.md` §10 №16) — по чек-листу
`validator.md` «Перенос Q/D» и `docs/BRIEF.md` §2, §4, §5.3, §5.6, §5.7, §7.
**Версия:** ветка `develop`, HEAD `491e153` + рабочее дерево (2026-09-28).
**Вердикт:** принято

**P1:** — критичных проблем нет
**P2:** — нет
**P3:** — нет

## Состав проверенного пакета

13 изменённых + 4 новых файла (рабочее дерево): `docs/OPEN_QUESTIONS.md`,
`docs/SPECIFICATION.md`, `docs/TRACEABILITY.md`, `docs/questions/README.md`,
`docs/features/README.md` + шапки `parser/lexer/execution.feature`,
`docs/CHANGELOG.md`, `docs/tasks/README.md` +
`docs/tasks/T-14-grammar-message-sync/README.md`, памяти `migrator`/`docs-writer`;
новые — `docs/questions/Q2.md`, `docs/questions/Q3.md`,
`docs/decisions/D16-dsl-canon-regex-mvp.md`,
`.opencode/mail/service-migration-q2q3.md`.

## 1. Перенос (BRIEF §7)

- Полные тексты Q2/Q3 удалены из архива, на их месте — короткие указатели
  (`OPEN_QUESTIONS.md:36–44`, «Мигрирован 2026-09-28 (блок Q2–Q3)» + ссылки на
  оба файла). Копии текста не осталось — «один факт — один канон».
- ID и даты сохранены: `Q2`/`Q3` (2026-09-24), решение — `D16` = строка
  `SPECIFICATION.md` §10 №16 (`:837`), дата `2026-09-24`.
- `D16`: `Resolves: Q2, Q3`, `Спека: SPECIFICATION.md §10, решение №16`,
  `Affects` заполнен (GRAMMAR §1–§4; `src/core.rs`; SPEC §1.3;
  `features/README.md`; три фичи), `Tasks: — (см. «Сверка с кодом»)`;
  раздел «Сверка с кодом» с вердиктом ✅ **соответствует** и явное
  «**Задач не требуется**» (`:95–97`).
- Статусы Q — `resolved by D16` (`Q2.md:3`, `Q3.md:3`); в `TRACEABILITY.md`
  (:9–10) и `questions/README.md` (:26–27) — `resolved` (установленная
  конвенция пилота Q1 → D15, `TRACEABILITY.md:8`).

## 2. Согласованность (BRIEF §2, §5.6, §5.7)

- Связки Q ↔ D ↔ `TRACEABILITY.md` ↔ `questions/README.md` ↔ §10 №16
  согласованы; слаг `D16-dsl-canon-regex-mvp` уникален (единственный `D16*`).
- Относительные ссылки живые: все цели новых ссылок существуют
  (`questions/Q2.md`, `Q3.md`, `decisions/D16-…md`, `GRAMMAR.md`,
  `SPECIFICATION.md`, `features/*.feature`, `T-14/README.md`, `D41-…md`,
  `analysis/findings-registry.md`).
- Обратная ссылка `# D16 (Q2, Q3): …` — ровно в трёх фичах
  (`parser.feature:2`, `lexer.feature:2`, `execution.feature:2`), по составу
  `Affects` D16; в шапке — форма BRIEF §5.6 п. 2. Примечание Q3 в
  `features/README.md:69` дополнено ссылками Q3/D16 без смены формулировки.
- SPEC §10 №16: текст строки сохранён, добавлен только указатель на D16.

## 3. Сверка с кодом (BRIEF §5.3)

- Вердикт D16 подтверждён: `src/core.rs::parse_rule` (`:436–441`) — regex
  (`Правило\s+(\w+)`, `Если\s*\((...)(<|>|==|!=)(...)\)`, `Решение\s*=`,
  `Причина\s*=`) с `Result<Rule, String>`; публичного AST/лексera нет — п. 2–3.
- `GRAMMAR.md`: §1 синтаксис, §2 «Ограничения v0.1 (regex-минимум)», §3 LSP-стратегия,
  §4 «Ключевые слова: статусы», §5 «Что дальше (v0.2)»; `EBNF.md` отсутствует.
- Дрейф канона языка подтверждён по факту: `GRAMMAR.md:56` (§2, таблица
  «Ограничения v0.1», строка 7) цитирует устаревшее «Не найдено имя правила»,
  тогда как код (`src/core.rs:447`) и `draft.feature` — «отсутствует заголовок
  правила». Покрыт действующей задачей **T-14**; карточка T-14 актуализирована
  на §2 (см. ниже), новой задачи не заведено.

## 4. Границы

- `git diff -- src tests AGENTS.md opencode.json` — пусто.
- `.opencode/agents/**`, `.opencode/rules/**` не тронуты (`git status` их не
  показывает; изменения `.opencode/` — только две памяти ролей и новая лента).
- Архив не пополнялся: diff `OPEN_QUESTIONS.md` — единственный хунк, только
  замена блоков Q2/Q3 указателями; чужие записи журнала не переписаны
  (`TRACEABILITY.md` +2 строки, `questions/README.md` +2 строки,
  `SPECIFICATION.md` — 1 строка, `CHANGELOG.md` +6 строк).

## 5. Актуализация T-14

- Карточка `T-14/README.md`: заголовок и все живые вхождения `§7` → `§2
  (таблица «Ограничения v0.1», строка 7)`; `§7` остаётся только в примечании
  `:35` как намеренная историческая цитата (D41/§10 №41) — согласуется с
  политикой «исторические §7 не переписываются».
- Сводка `tasks/README.md:51` — заголовок `GRAMMAR:` без `§7`; `Источник`
  (D41/Q46) и прочие поля не тронуты.

## 6. DoD / счётчики

- `cargo fmt --check` — pass (без вывода).
- `cargo test --test features_inventory` — **4/4 ok** (`feature_files_are_valid_documents`,
  `scenario_counts_match_readme`, `readme_totals_match_files`,
  `feature_files_match_readme_inventory`); счётчики `features/README.md:300`
  (47 файлов / 278 сценариев) согласованы — тест прошёл.
- Полный `cargo test --all` не перезапускался: `src/**`/`tests/**` неизменны с
  `W8-config` (пустой `git diff` по ним); правка `features/README.md` — текст
  примечания, счётчики не менялись (подтверждено тестом инвентаризации).
- `cargo clippy` не перезапускался — Rust не менялся.

## Проверки (команды → результат)

- `git status --porcelain` → 13 M + 4 `??`; `src`, `tests`, `AGENTS.md`,
  `opencode.json`, `.opencode/agents|rules` среди них нет.
- `git diff -- src tests AGENTS.md opencode.json` → пусто.
- `git diff --stat` → 14 файлов, 78 insertions / 52 deletions (только `docs/**`
  и 2 памяти).
- `git log --oneline -3` → `491e153` (HEAD), `e1e90d4`, `6d4c840`.
- `cargo fmt --check` → без вывода (pass).
- `cargo test --test features_inventory` → `4 passed; 0 failed`.
- `rg "# D16 \(Q2, Q3\)" docs/features` → 3 совпадения (parser/lexer/execution).
- `rg "GRAMMAR|T-14" docs/tasks/README.md` → строка T-14 без `§7`.
- Чтение адресное: `D16`, `Q2`, `Q3`, `OPEN_QUESTIONS.md:31–44`,
  `TRACEABILITY.md:9–10`, `questions/README.md:26–27`, `SPECIFICATION.md:837`,
  `GRAMMAR.md:35–64`, `src/core.rs` (`parse_rule`), `CHANGELOG.md:19–24`,
  `T-14/README.md`, `BRIEF.md` §2/§5.3/§5.6/§5.7/§7/§9.

## Что проверено и ок

- Полнота переноса, отсутствие дублей канона, живучесть ссылок, уникальность
  ID/слага, согласованность Q↔D↔TRACEABILITY↔README↔§10 №16.
- Вердикт сверки D16 соответствует фактам кода и `GRAMMAR.md`; дрейф `GRAMMAR §2`
  действительно покрыт T-14, карточка ведёт на §2.
- Границы (код/тесты/канон агентов/opencode.json) не нарушены; архив не
  пополнялся; чужие записи журнала не переписаны.
- DoD: `fmt` pass, `features_inventory` 4/4; полный `--all` обоснованно не гонялся.

**Замечание к снимку:** пакет принят по состоянию рабочего дерева `develop`
(HEAD `491e153`); после приёмки правки этих файлов вне порога существенности
требуют повторной проверки (`review.md`, «Возврат на доработку»).
