# Сервисная лента: жизненный цикл TRACEABILITY — волна 1 (service-lifecycle-w1)

**Назначение:** служебная операция по решениям владельца (01.10.2026): словарь
жизненного цикла `docs/TRACEABILITY.md` — `open` · `in work` · `done`
(`resolved` убирается; `hold`/`deferred`/`dropped` не используются); волна 1 —
«точно закрытые» строки → `done`, все незакрытые → `open`; задачи в волне 1 не
заводятся (волна 2 — разбор `open`).

## сервисная сессия · 01.10.2026 · открытие

- Решения владельца: (1) статусы: `open` — есть вопрос (решения нет либо
  решение есть, но требуется задача) · `in work` — есть открытая задача ·
  `done` — закрыто (вопрос закрыт / задача закрыта); `dropped` — «это тоже done»;
  (2) волна 1 — аккуратно: только «точно закрытые» → `done`, остальные → `open`;
  (3) волна 2 — по каждой `open` решить, что нужно для `in work` или `done`.
- Раскладка волны 1 (подтверждена владельцем):
  - `done` (28): Q5, Q6, Q27, Q40, Q41, Q42, Q47–Q49, Q52–Q59, Q61, Q64,
    Q66–Q72, Q75, Q77 (применённые документно-процессные + Q27/Q42);
  - `open` (26): Q2, Q3, Q4, Q8, Q9, Q10, Q11, Q14, Q18, Q19, Q20, Q21, Q22,
    Q23, Q24, Q25, Q26, Q30, Q31, Q35, Q36, Q37, Q38, Q39, Q63, Q65;
  - `in work` (19) и `done` (4: Q28, Q43–Q45) не меняются.
- Маршрут: `migrator` (Q78/D82, легенда и строки, «Обновления» D63/D64, карточка
  T-18) → сервисная сессия (`journal.md` §3) → `auditor` → `validator` → гейт →
  `git` (коммит + push `develop`).
- Границы: `docs/TRACEABILITY.md`, `docs/questions/Q78.md`,
  `docs/decisions/D82-*.md`, `docs/decisions/D63/D64` («Обновления»),
  `docs/tasks/T-18-docs-journal-test/README.md`, каталоги `questions/README.md`
  и `decisions/README.md`; `.opencode/rules/journal.md` — сервисная сессия;
  `cargo` не запускается (D50). Журнальные статусы Q/D-файлов не меняются.
- Строка Q78/D82 в TRACEABILITY: `open` (волна 2 впереди — «требуется задача»).

## migrator · 01.10.2026 · готово

- **Сделано (шаги 1–4):**
  1. **Q78/D82:** созданы `docs/questions/Q78.md` (тема «жизненный цикл
     `TRACEABILITY`: как разгрести массу `resolved`», варианты а/б/в,
     рекомендация (а), статус `resolved by D82`),
     `docs/decisions/D82-traceability-lifecycle-waves.md` (слаг
     `traceability-lifecycle-waves`; `Resolves: Q78`, `Спека: —`, `Tasks: —`,
     вердикт сверки ⚪; словарь `open · in work · done`, волны 1–2); строки
     добавлены в `questions/README.md` и `decisions/README.md`.
  2. **Разметка `TRACEABILITY.md`:** 28 `resolved` → `done` (Q5, Q6, Q27,
     Q40–Q42, Q47–Q49, Q52–Q59, Q61, Q64, Q66–Q72, Q75, Q77); 26 `resolved` →
     `open` (Q2–Q4, Q8–Q11, Q14, Q18–Q26, Q30, Q31, Q35–Q39, Q63, Q65);
     `in work` (19) и существующие `done` (Q28, Q43–Q45) не тронуты; легенда
     переписана (`open · in work · done`, пометки про `resolved`/`dropped`),
     добавлена строка Q78/D82 (`open`, задачи `—`, реализация `—`).
  3. **«Обновления»:** D63 — смена словаря; D64 — проверки теста
     (`{open, in work, done}`, `in work` ⇒ ⬜/🚧, `done`/`open` ⇒ нет открытых
     задач, `resolved` не допускается).
  4. **T-18:** формулировка проверки v0.1 обновлена (ссылка на D82).
- **Проверки (команды → результат):**
  - `git diff --stat` → 6 `M` (TRACEABILITY, D63, D64, каталоги Q/D, T-18) +
    3 `??` (Q78, D82, лента); D82 — правлен по D65 (снят адрес ленты волны).
  - `git diff` по файлам → только целевые правки; `git diff --check` → пусто.
  - grep `resolved` в `TRACEABILITY.md` → только упоминание в легенде
    («`resolved` упразднён»); ячеек `| resolved |` нет.
  - Счётчики: `open` = 27 (26 + Q78), `in work` = 19, `done` = 32 (28 + 4) —
    сходится с раскладкой 28/26.
  - Финальная сверка (read-only): Q78/D82 в `TRACEABILITY` = `open`; Q78↔D82 в
    обоих каталогах; легенда = `{open, in work, done}`; сессионных адресов в
    моих файлах нет; `journal.md` не трогал.
  - `cargo` не запускался (D50).
- **Остаток/риски:** правка `.opencode/rules/journal.md` §3 — зона сервисной
  сессии (в `git status` видна как ` M`, не моя). Коммит не делал. Дальше по
  маршруту: сервисная сессия → `auditor` → `validator` → гейт → `git`.

## auditor · 01.10.2026 · аудит волны 1 — расхождений нет

**Аудит:** `journal.md` §3, `docs/TRACEABILITY.md` (словарь, счётчики, легенда),
Q78/D82 и каталоги, «Обновления» D63/D64, карточка T-18, границы, «инструкция ↔
права».
**Бюджет:** 11 файлов чтения, 9 `rg`.

**P1/P2/P3:** критичных проблем нет; находок нет.

**Проверки:**
- `git diff -- ./.opencode/rules/journal.md` — один хунк, §3: `open` · `in work` ·
  `done` + ссылка на D82 (`resolved`/`dropped` в таблице не используются); строки
  про Q/D/задачу (:28–30) не тронуты. ✓
- Счётчики `TRACEABILITY`: `open` = 27, `in work` = 19, `done` = 32; ячеек
  `resolved`/`dropped` нет. Раскладка 28/26 сверена поимённо с лентой: `open`-26 +
  Q78 = 27; `done`-28 + Q28/Q43–Q45 = 32; `in work` (19) — тот же набор, не
  тронут. ✓
- Легенда (:84–92): определения `open`/`in work`/`done`, упоминание упразднения
  `resolved`, `dropped` → `done`; сохранены пояснения про «Задачи»/«Реализацию»/
  колонки Q и D (D63, D76). ✓
- Q78/D82: формы журнала (Статус/Дата/Приоритет/Связано; `Resolves`, «Задач не
  требуется», «Сверка с кодом» ⚪), сессионных адресов нет (D65); строки в обоих
  каталогах; Q78/D82 = `open`. ✓
- «Обновления»: D63/D64 — append-пункты в «Следствиях» (numstat `5/0`, `4/0`),
  тела не переписаны; проверки T-18 согласованы с D82 (`{open, in work, done}`;
  `in work` ⇒ ⬜/🚧; `done`/`open` ⇒ открытых задач нет; `resolved` не
  допускается). ✓
- `node .opencode/scripts/agents-perms.mjs` ×2 — `agents: 11 из 18`, расхождений
  нет.
- Границы: `git status --porcelain` — ожидаемый состав; `git diff --stat -- src
  tests Cargo.toml AGENTS.md` пусто; `cargo` не запускался (D50). ✓

**Инструкция ↔ права:** расхождений нет (правки — текст процесса; фронтматтеры
не тронуты).

**Следующие шаги:**
- [ ] `validator` — приёмка пакета (гейт → коммит)`

## validator · 01.10.2026 · принято

- **Вердикт:** принято. P1/P2/P3 — нет. База `develop` @ `cb7d159`
  (= `origin/develop` = `HEAD`) + рабочее дерево. Отчёт
  `docs/reviews/service-lifecycle-w1-2026-10-01.md`; квитанция
  `service-lifecycle-w1` iteration 1 `accepted`.
- **Сделано:** независимо воспроизведён DoD-состав пакета.
  - Словарь/счётчики `TRACEABILITY`: `open` = 27, `in work` = 19, `done` = 32;
    ячеек `resolved`/`dropped` нет; раскладка поимённо = ленте (done-28 +
    Q28/Q43–Q45; open-26 + Q78); легенда (:84–92) точна.
  - `journal.md` §3 — один хунк (3/2), сводный цикл + ссылка на D82; строки
    :28–30 не тронуты. D63/D64 — append-«Обновления» (5/0, 4/0), тела целы.
    T-18 — 3/1 (проверка словаря + D82). Каталоги Q/D — по +1 строке.
  - Q78/D82: формы журнала, `Resolves`, «Сверка с кодом» ⚪, «Задач не
    требуется»; строка Q78/D82 = `open`; сессионных адресов нет (D65).
  - Границы: 8 `M` + 3 `??` (до моих записей); `src/tests/Cargo.toml/AGENTS.md`,
    `docs/features`, `SPECIFICATION.md`/`CHANGELOG.md` — пусто.
  - `agents-perms.mjs` ×2 → `11 из 18`; `cargo` не запускался (D50).
- **Дальше / риски:** гейт пакета → `git` (коммит + push `develop`). Грабли:
  голый dot-путь `.opencode/...` под `--` отклонён движком — рабочая форма
  `./.opencode/...`.

## сервисная сессия · 01.10.2026 · подтверждение пакета (гейт)

- Гейт пройден: владелец подтвердил **коммит + push** (`question`, ответ
  «Коммит + push (Recommended)») — операция `service-lifecycle-w1`.
- База: `develop` @ `cb7d159` (= `origin/develop`); режим — коммит в `develop`,
  затем `git push origin develop`.
- Пакет — 15 путей + записи `lead`/`git` = 17:
  - `.opencode/mail/service-lifecycle-w1.md` (лента)
  - `.opencode/memory/auditor.md`
  - `.opencode/memory/git.md` (чекпойнт роли `git`, новый до `add`)
  - `.opencode/memory/migrator.md`
  - `.opencode/memory/validator.md`
  - `.opencode/rules/journal.md` (§3)
  - `.opencode/state/current/progress.yaml`
  - `.opencode/state/current/receipts.yaml`
  - `docs/TRACEABILITY.md`
  - `docs/decisions/D63-journal-index-lifecycle.md`
  - `docs/decisions/D64-journal-integrity-test.md`
  - `docs/decisions/D82-traceability-lifecycle-waves.md`
  - `docs/decisions/README.md`
  - `docs/questions/Q78.md`
  - `docs/questions/README.md`
  - `docs/tasks/T-18-docs-journal-test/README.md`
  - `docs/reviews/service-lifecycle-w1-2026-10-01.md`
- Сообщение коммита:
  `docs(Q78/D82): жизненный цикл TRACEABILITY — open · in work · done; волна 1 разметки (service-lifecycle-w1)`
- Состояние: приёмка `validator` — принято (P1/P2/P3 нет). 

## git · 01.10.2026 · пакет — готово (записи до `add`)

- Сделано: сверена подтверждённая запись гейта; база `develop` @ `cb7d159`
  (= `origin/develop`), рабочее дерево — 12 `M` + 4 `??`, ровно пакет (+ мой
  чекпойнт = 17). По F43 записи (эта строка и чекпойнт `.opencode/memory/git.md`)
  сформированы **до** `git add` и входят в коммит.
- Проверки: `git status -sb` → `## develop...origin/develop`; `git log -1
  --oneline` → `cb7d159`.
- Дальше: `add` 17 путей (`./`-префикс, без `--`) → сверка staged
  (`diff --cached --name-status`, ожидается 13 `M` + 4 `A`) → `commit`
  «docs(Q78/D82): жизненный цикл TRACEABILITY — open · in work · done; волна 1
  разметки (service-lifecycle-w1)» → `push origin develop`. Хеши — в ответе
  `lead` (F43).
- Риски: расхождение снимка и списка — стоп без коммита. Грабли: голый
  dot-путь `.opencode/...` под `--` отклоняется движком — рабочая форма
  `./.opencode/...`.
