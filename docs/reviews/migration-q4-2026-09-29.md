# Приёмка: service-migration-q4 (перенос Q4 → D17)

**Проверка:** сервисная операция переноса блока **Q4** из архива
`OPEN_QUESTIONS.md` в журнал (`SPECIFICATION.md` §10 №17) — по чек-листу
`validator.md` «Перенос Q/D» и `docs/BRIEF.md` §2, §4, §5.3, §5.6, §5.7, §7.
**Версия:** ветка `develop`, HEAD `f488085` + рабочее дерево (2026-09-29).
**Вердикт:** принято

**P1:** — критичных проблем нет
**P2:** — нет
**P3:** — нет

## Состав проверенного пакета

13 изменённых + 3 новых файла (рабочее дерево): `docs/OPEN_QUESTIONS.md`,
`docs/SPECIFICATION.md`, `docs/TRACEABILITY.md`, `docs/questions/README.md`,
`docs/features/README.md`, шапки `parser/execution/editor/lsp.feature`,
`docs/features/client_explanation.feature`, `docs/CHANGELOG.md`, памяти
`migrator`/`docs-writer`; новые — `docs/questions/Q4.md`,
`docs/decisions/D17-priority-out-of-mvp.md`,
`.opencode/mail/service-migration-q4.md`.

## 1. Перенос (BRIEF §7)

- Полный текст Q4 удалён из архива, на его месте — короткий указатель
  (`OPEN_QUESTIONS.md:46–49`, «Мигрирован 2026-09-29 (блок Q4)» + ссылки на
  вопрос и решение). Копии текста не осталось — «один факт — один канон»;
  diff `OPEN_QUESTIONS.md` — единственный хунк (замена блока).
- ID и дата сохранены: `Q4` (2026-09-24) — шапка указателя и `Q4.md:4`;
  решение — `D17` = строка `SPECIFICATION.md` §10 №17 (`:838`), дата
  `2026-09-24` (`D17-priority-out-of-mvp.md:4`).
- `D17`: `Resolves: Q4`, `Спека: SPECIFICATION.md §10, решение №17`,
  `Affects` заполнен (`GRAMMAR.md` §4/§5; `features/README.md` нота Q4; пять
  фич — целевое v0.2), `Tasks: — (см. «Сверка с кодом»)`;
  раздел «Сверка с кодом» — вердикт ✅ **соответствует** (`:52–53`) и явное
  «**Задач не требуется:**» (`:88–92`).
- Статус Q — `resolved by D17` (`Q4.md:3`); в `TRACEABILITY.md:11` и
  `questions/README.md:28` — `resolved` (конвенция пилота Q1 → D15).

## 2. Сверка с кодом (BRIEF §5.3)

- Вердикт D17 подтверждён фактами: `src/core.rs::parse_rule`
  (`:437–441`) использует regex только на `Правило`, `Если`, `Решение`,
  `Причина`; ветки разбора `Приоритет` нет. Поиск `rg -i "приоритет"` по
  `src/` даёт лишь два несвязанных комментария — semver-приоритет
  (`core.rs:414`) и приоритет CLI-флагов (`main.rs:56`), как и указано в D17.
- `GRAMMAR.md:91` (§4, таблица ключевых слов) — `Приоритет` — ❌, «вне MVP
  (решение Q4), вернётся с конвейерами»; `GRAMMAR.md:97` (§5) — `Приоритет` в
  списке v0.2. §4 и §5 согласованы с решением.
- Фактический список потребителей подтверждён: `Приоритет` есть в
  `parser.feature` (сценарий 2), `execution.feature`, `editor.feature`,
  `lsp.feature`; требование сортировки причин — в
  `client_explanation.feature:34`. Фичи помечены целевым состоянием v0.2,
  решению не противоречат.
- Факт «`explain.feature` больше не содержит `Приоритет`» — верен: поиск по
  `docs/features` его не находит (его сценарии — `rule_name`/`condition`/
  `actual_value`, `features/README.md:66`). Уточнённый список в ноте Q4
  (`features/README.md:74–79`) — без `explain.feature`. `semver.feature` —
  иной смысл (приоритет MAJOR), к языковому `Приоритет` не относится.

## 3. Согласованность (BRIEF §2, §5.6, §5.7)

- Связки Q ↔ D ↔ `TRACEABILITY.md:11` ↔ `questions/README.md:28` ↔
  `SPECIFICATION.md` §10 №17 согласованы; ID уникальны; слаг
  `D17-priority-out-of-mvp` уникален (единственный файл `D17*`).
- Относительные ссылки живые: цели новых ссылок существуют
  (`questions/Q4.md`, `decisions/D17-priority-out-of-mvp.md`, `GRAMMAR.md`,
  `SPECIFICATION.md`, `features/*.feature`, `OPEN_QUESTIONS.md`, `Q3.md`,
  `D16-…md`).
- Обратная ссылка `# D17 (Q4): …` — ровно в четырёх шапках
  (`parser.feature:3`, `execution.feature:3`, `editor.feature:2`,
  `lsp.feature:2`), форма — BRIEF §5.6 п. 2; в `client_explanation.feature:8`
  `(Q4/Q36)` → `(D17/Q36)` (дублирования Q4/D17 нет). Совокупно соответствует
  составу `Affects` D17.
- SPEC §10 №17: текст строки сохранён, добавлен только указатель на `[D17]`;
  инлайн-нота Q4 в §7 (`:760`) не тронута.

## 4. Границы

- `git diff -- src tests AGENTS.md opencode.json` — пусто.
- `.opencode/agents/**`, `.opencode/rules/**` не тронуты; изменения
  `.opencode/` — только три памяти ролей (`migrator`, `docs-writer`,
  `validator`) и новая лента.
- Архив не пополнялся: diff `OPEN_QUESTIONS.md` — только замена блока Q4
  указателем; чужие записи журнала не переписаны (`TRACEABILITY.md` +1 строка,
  `questions/README.md` +1 строка, `SPECIFICATION.md` — 1 строка,
  `features/README.md` — нота Q4, `CHANGELOG.md` +4 строки).

## 5. DoD / счётчики

- `cargo fmt --check` — pass (без вывода).
- `cargo test --test features_inventory` — **4/4 ok**
  (`feature_files_are_valid_documents`, `scenario_counts_match_readme`,
  `readme_totals_match_files`, `feature_files_match_readme_inventory`);
  счётчики `features/README.md:301` (47 файлов / 278 сценариев) согласованы —
  тест прошёл.
- Полный `cargo test --all` не перезапускался: `src/**`/`tests/**` неизменны с
  последнего изменения Rust-кода — `git diff 22f7683..HEAD --stat -- src tests`
  пусто (`22f7683` — «rustfmt 80 + edition 2024 … реформат src/tests» в составе
  пакета W8-config). Правки задачи — только `docs/**` и памяти; полный DoD
  закрыт прогоном `fmt` + инвентаризации фич.
- `cargo clippy` не перезапускался — Rust не менялся.

## Проверки (команды → результат)

- `git status --porcelain` → 14 M + 3 `??`; `src`, `tests`, `AGENTS.md`,
  `opencode.json`, `.opencode/agents|rules` среди них нет.
- `git log -1 --oneline` → `f488085` (HEAD, `docs(D16): …`).
- `git diff -- src tests AGENTS.md opencode.json` → пусто.
- `git diff 22f7683..HEAD --stat -- src tests` → пусто.
- `git diff --stat` → 13 файлов, 40 insertions / 25 deletions (только `docs/**`
  и 2 памяти), плюс 3 новых файла.
- `git diff --stat -- ./.opencode` → только 3 памяти.
- `rg -i "приоритет" src` → `main.rs:56`, `core.rs:414` (оба — не язык).
- `rg "D17 \(Q4\)" docs` → 4 совпадения (parser/execution/editor/lsp).
- `rg "D17-priority-out-of-mvp" docs` → единственный файл решения.
- `cargo fmt --check` → без вывода (pass).
- `cargo test --test features_inventory` → `4 passed; 0 failed`.
- Чтение адресное: `Q4.md`, `D17-priority-out-of-mvp.md`,
  `OPEN_QUESTIONS.md:46–49`, `TRACEABILITY.md:11`, `questions/README.md:28`,
  `SPECIFICATION.md:760,838`, `GRAMMAR.md:82–98`, `features/README.md:44–98,
  299–303`, `CHANGELOG.md:8–28`, `features_inventory.rs`, лента
  `service-migration-q4.md`, `BRIEF.md` §2/§4/§5.3/§5.6/§5.7/§7, `review.md`.

## Что проверено и ок

- Полнота переноса, отсутствие дублей канона, живучесть ссылок, уникальность
  ID/слага, согласованность Q↔D↔TRACEABILITY↔README↔§10 №17.
- Вердикт сверки D17 соответствует фактам кода (`parse_rule`) и `GRAMMAR.md`
  §4/§5; фактический список фич-потребителей подтверждён; «Задач не требуется»
  обосновано.
- Границы (код/тесты/канон агентов/`opencode.json`) не нарушены; архив не
  пополнялся; чужие записи журнала не переписаны.
- DoD: `fmt` pass, `features_inventory` 4/4, счётчики 47/278; полный `--all`
  обоснованно не гонялся (`src/tests` неизменны с `22f7683`).

**Замечание к снимку:** пакет принят по состоянию рабочего дерева `develop`
(HEAD `f488085`); после приёмки правки этих файлов вне порога существенности
требуют повторной проверки (`review.md`, «Возврат на доработку»).
