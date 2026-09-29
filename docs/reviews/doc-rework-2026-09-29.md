# Приёмка волны `service-doc-rework` (D69–D74 + канон) — 29.09.2026

**Проверка:** исполнение D69–D74 (ретро-D §10; ревизия `SPECIFICATION.md`;
правила журнала BRIEF → `.opencode/rules/journal.md`; очистка `CHANGELOG`;
README×2; нормативный фокус `GRAMMAR`) + правки канона агентов (`AGENTS.md`,
`.opencode/**`). Итерация 1 — полный DoD волны; итерация 2 — адресная
перепроверка закрытия r1-P2/P3.

**Версия:** `develop`, HEAD `75b078c0d1f008039dfbcd581f75c4ee844d311a`
(= `origin/develop`, коммитов волны нет) + рабочее дерево (снимок 29.09.2026).

**Вердикт:** **принято — r2** (P1/P2/P3 нет; критичных проблем нет).

---

## r1 (29.09.2026) — отклонено (rework, iteration 1)

**P1:** нет.

**P2:**

- `‹P2-1›` `.opencode/rules/journal.md:4` — `[D71](../docs/decisions/D71-journal-rules-relocation.md)`
  от `.opencode/rules/` резолвится в `.opencode/docs/…` (не существует;
  `rg --files .opencode -g "docs/**"` → пусто). Ожидание: цель жива → правка
  `../../docs/decisions/…`.
- `‹P2-2›` `docs/decisions/D60-docs-ownership-sync.md:44` — «хронология
  прототипа» для `CHANGELOG.md` против `AGENTS.md:172` и D72 («кодовые изменения
  продукта»); таблица D60 объявлена каноничной (`:33`) и правилась волной
  (`:37` → D70).

**P3:**

- `‹P3-1›` титулы с шаблонным артефактом: `docs/SPECIFICATION.md:1`
  (`# DAR {} — …`), `docs/GRAMMAR.md:1` (`# GRAMMAR — каноническая грамматика DAR {} v0.1`)
  (тот же артефакт в `README.md:1` снят правкой аудита P3-3).
- `‹P3-2›` устаревшие указатели «Канон:» на удалённый `docs/BRIEF.md`:
  `.opencode/memory/validator.md:3`, `.opencode/memory/migrator.md:3`,
  `.opencode/memory/docs-writer.md:3-4` (F35).

Отчёт r1 в ленте: `## validator · 29.09.2026 · отклонено (rework)`.

---

## r2 (29.09.2026) — закрытие r1

- **P2-1 закрыт.** `.opencode/rules/journal.md:4` = `…[D71](../../docs/decisions/D71-journal-rules-relocation.md)`;
  цель существует (`rg --files docs/decisions -g "D71*"` → `D71-journal-rules-relocation.md`),
  путь от `.opencode/rules/` резолвится (два уровня вверх → корень → `docs/decisions/`).
  Единственная ссылка файла (`rg -n "\]\([^)]*\)"` → строка 4).
- **P2-2 закрыт.** `docs/decisions/D60-docs-ownership-sync.md:44` = «кодовые
  изменения продукта ([D72](D72-changelog-full-cleanup.md)); принципы тестов
  соответствия» — согласовано с `AGENTS.md:172`, `docs/README.md:15` и D72 п.1–2;
  в D60 иных правок нет (`git diff -U0` → только `:14` (BRIEF-де-линк r1-волны) и `:37`/`:44`).
- **P3-1 закрыт.** `docs/SPECIFICATION.md:1` = «# DAR — Финальная проектная
  спецификация»; `docs/GRAMMAR.md:1` = «# GRAMMAR — каноническая грамматика DAR v0.1».
  `rg -n "DAR \{\}"` — в титулах пусто; остались тело SPEC `:15` (определение
  языка), ASCII-рамка `:70` и заголовки `docs/features/{lsp,lsp_notebook,lexer}.feature`
  — это не титулы, оставлены кандидатами (приемлемо; см. «Наблюдения»).
- **P3-2 закрыт.** «Канон:» → живой канон: `memory/validator.md:3` =
  `.opencode/rules/review.md`; `.opencode/rules/journal.md` §5.3/§7 ·
  `memory/migrator.md:3` = `.opencode/rules/journal.md` (§2 ID, §4 шаблоны,
  §5.3 сверка) · `memory/docs-writer.md:3-4` = `docs/tasks/README.md`,
  `docs/features/README.md`, `.opencode/rules/journal.md` §5.6/§7. Правки — только
  строки «Канон:»: `git diff --numstat` → `validator.md` **1/1**, `migrator.md`
  и `docs-writer.md` — шапка (1/1) + ранее записанная хроника волны; фронтматтеры
  и права ролей не менялись.

**Проверки r2 (адресные):**

| Зона | Команда | Результат |
|---|---|---|
| Ссылка journal | `rg -n "\]\([^)]*\)" .opencode/rules/journal.md`; `rg --files docs/decisions -g "D71*"` | `:4` → `../../docs/decisions/D71-…`; цель существует ✅ |
| D60 | `rg -n "CHANGELOG" docs/decisions/D60-docs-ownership-sync.md` | `:44` «кодовые изменения продукта ([D72])» ✅ |
| Титулы | `rg -n "^# " docs/SPECIFICATION.md docs/GRAMMAR.md README.md`; `rg -n "DAR \{\}" docs README.md AGENTS.md .opencode` | титулы без `{}`; `{}` только тело/рамка/фичи ✅ |
| Памяти | `rg -n "Канон:" .opencode/memory/{validator,migrator,docs-writer}.md`; `git diff --numstat -- …` | шапки → `journal.md`; правки только шапки ✅ |
| Границы | `git status --porcelain` | новых файлов/зон нет (состав тот же + `memory/validator.md` из P3-2) ✅ |
| DoD | `git diff --stat -- src tests Cargo.toml` | пусто → `cargo` не запускался (D50); git read-only |

## «Что проверено и ок» (r1, в силе)

- **D69 ✅** — 11 ретро-D (`D1–D5`, `D7–D11`, `D13`; `rg -l "оформлено
  ретроспективно"` → 12 включая D69), у всех 11 «Сверка с кодом»; каталог
  `decisions/README.md` — **74 строки = 74 D-файла** (`rg --files`); соответствие
  «`Спека: §10, №N`» ↔ строки прежней таблицы (`git grep -E "^\| (1|2|…|13) \|" HEAD`);
  Q65–Q70 — `resolved by [D69]…[D74]`.
- **D70 ✅** — `SPECIFICATION.md` **497** строк (было 924), §1–§11 последовательно;
  **§6 (`:371-389`) и §11 (`:468-497`) текстуально идентичны базе**
  (`git grep -A … HEAD`); §10 = указатель «Решения — журнал»; провенанс §1.3.1
  (`:47-49`) → `D22` (`Resolves: Q20`); `§10`/`OPEN_QUESTIONS`/`BRIEF` в SPEC — нет;
  относительные ссылки живы (замер 49 вхождений / 34 уникальные цели).
- **D71 ✅** — `docs/BRIEF.md` удалён; `journal.md` §1–§8; живых `[…](…BRIEF)` в
  каноне нет (`rg` по `AGENTS.md`, `docs`, `.opencode/rules`, `.opencode/agents` —
  пусто); `D62` — `superseded by D71` (файл `:3-5,:7`, каталог `:91`);
  `D66:37-38` — `BRIEF` убран из исключений doc-size; `D60` — таблица без §10.
- **D72 ✅** — `CHANGELOG.md` — 7 строк (заголовок + правило «только код» +
  пустой `0.1.0`), записей нет; `D68:36-38` дополнен; `AGENTS.md:172`,
  `.opencode/agents/docs-writer.md:84-85`, `docs/README.md:15` согласованы.
- **D73 ✅** — корневой `README.md` (42 строки), ссылки живы, канон-таблиц не
  дублирует; `docs/README.md` (48 строк) — `rg "Q\d|BRIEF"` пусто.
- **D74 ✅** — `GRAMMAR.md` (78 строк), `rg "Q\d+"` пусто, EBNF (`:9-25`) и
  пример (`:29-37`) целы, rationale → ссылки (`D16`/`D6`/`D18`/`D17`/`D13`/`D1`).
- **Регрессии ✅** — Q-строки 70/70 (`TRACEABILITY.md`, `questions/README.md`);
  фич 47 файлов, `features/README.md:314` «47 файлов, 278 сценариев»; свип
  `ожида(ет|ют) переноса` — только описания запрета (`T-18:42`, `D64:49`);
  `docs/reviews/**`, `docs/analysis/**` не тронуты; `node .opencode/scripts/agents-perms.mjs`
  → «agents: 11 из 18», права/`steps` без изменений.
- **DoD/D50 ✅** — пакет без `src/**`, `tests/**`, `Cargo.toml`
  (`git diff --stat` пусто) → `cargo`-прогоны не выполнялись; git — только
  read-only (`status`/`diff`/`log`/`grep`).

## Наблюдения (не находки)

- `docs/GRAMMAR.md:49` цитирует «Не найдено имя правила», код — «отсутствует
  заголовок правила» (`src/core.rs:447`, `:716`; `tests/mcp_draft.rs:417`) —
  известный дрейф с **открытой задачей `T-14`** (⬜, P2, `docs-writer`), в волну
  не входил; не блокер (задача есть).
- В SPEC 49 вхождений относительных ссылок (34 уникальные цели) против «44» в
  ожидании — расхождение методики подсчёта, все цели живы.
- `docs/decisions/README.md:18-20` («Пропуски номеров — исторические…») читается
  исторически; следующая фраза «Неперенесённых записей не осталось» снимает
  вопрос (74 = 74, номера 1–74).
