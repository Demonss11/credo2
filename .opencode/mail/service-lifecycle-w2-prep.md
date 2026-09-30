# Сервисная лента: подготовка волны 2 жизненного цикла (service-lifecycle-w2-prep)

**Назначение:** формализация волны 2 (разбор 26 `open`-строк `TRACEABILITY`) по
решению владельца (01.10.2026): «волну 2 будем делать не сейчас, а отдельной
задачей» — подготовить новый вопрос, решение и задачу.

## сервисная сессия · 01.10.2026 · открытие

- Решение владельца (01.10.2026): волна 2 — отдельная задача (не сейчас);
  подготовить `Q79` (вопрос), `D83` (решение — процедура волны 2), `T-20`
  (задача на исполнение).
- Состав подготовки:
  - `docs/questions/Q79.md` — вопрос: как разгрести 26 `open`-строк волны 1
    (что нужно каждой для `in work`/`done`); варианты: (а) отдельная задача с
    процедурой (принято владельцем); (б) продолжить волной без задачи
    (отклонено); (в) оставить как есть (отклонено);
  - `docs/decisions/D83-*.md` — решение: процедура волны 2 — разбор каждой
    строки (задача → `in work` или закрытие → `done`), пакетное подтверждение
    владельцем, v0.2-задачи с пометой «v0.2» и зависимостью «приёмка v0.1»;
    границы `tasks/README.md` уточняются (v0.2-следствия допускаются как
    задачи);
  - `docs/tasks/T-20-*/README.md` — задача «Волна 2 `TRACEABILITY`: разбор
    `open`-строк» (источник — D83 (Q79), связано D82 (Q78));
  - сопутствующее: строка T-20 в `tasks/README.md`, строки Q79/D83 в каталогах,
    строки Q79/D83 и Q78/D82 в `TRACEABILITY.md` (обе → `in work`, задача
    T-20 ⬜).
- Маршрут: `migrator` (Q79/D83/T-20 + сопутствующее) → `validator` (приёмка) →
  гейт → `git` (коммит + push `develop`). `cargo` не запускается (D50).

## migrator · 01.10.2026 · готово

- **Сделано (шаги 1–5):**
  1. **Q79:** создан `docs/questions/Q79.md` — тема «волна 2 `TRACEABILITY` — как
     разобрать оставшиеся `open`-строки», статус `resolved by D83`, варианты
     (а)/(б)/(в), рекомендация (а); «Связано» — Q78/D82, D63/D64, T-18, T-20.
  2. **D83:** создан `docs/decisions/D83-traceability-wave2.md` (слаг
     `traceability-wave2`; `Resolves: Q79`, `Спека: —`, `Tasks: T-20`,
     `Affects`: TRACEABILITY, tasks/README, T-20, Q78/D82; вердикт сверки ⚪,
     «Задач не требуется сверх T-20») — процедура волны 2 (разбор каждой
     `open`; задача → `in work` либо закрытие → `done`), v0.2-помета и
     зависимость «приёмка v0.1», уточнение границ реестра, финал волны.
  3. **T-20:** создан `docs/tasks/T-20-traceability-wave2/README.md` (Статус ⬜,
     P3, Зависит от —, Источник D83 (Q79), связано D82 (Q78); разбор 26 строк →
     досье → завод задач → синхронизация; критерий готовности; исполнитель
     `migrator`). Исправлен дефект черновика: `[D78]` в «Контексте» →
     `[Q78](../../questions/Q78.md)/[D82]`.
  4. **`tasks/README.md`:** строка T-20 в сводке (P3, источник D83 (Q79), ⬜);
     абзац «Границы» дополнен пунктом «Исключение» (D83) — v0.2-следствия
     решений допускаются задачами с пометой «v0.2» и зависимостью «приёмка v0.1».
  5. **`TRACEABILITY.md` и каталоги:** Q78/D82 `open` → `in work` + `T-20 ⬜` в
     «Задачи»; добавлена строка Q79/D83 (`in work`, `T-20 ⬜`, реализация `—`);
     строки Q79/D83 — в `questions/README.md` и `decisions/README.md`.
- **Проверки (команды → результат):**
  - `git status --porcelain` → 4 `M` (TRACEABILITY, `questions/README.md`,
    `decisions/README.md`, `tasks/README.md`) + 4 `??` (лента, `Q79.md`,
    `D83-*.md`, папка T-20) — ровно целевой состав.
  - `git diff --stat` → 4 files changed, 8 insertions(+), 1 deletion(-);
    `git diff` → только целевые правки; `git diff --check` → пусто.
  - grep `\.opencode/(mail|state)` и `reviews/|research/|analysis/` по Q79/D83/T-20
    → пусто (сессионных адресов нет; D65).
  - `git grep -c "| resolved |"` → вхождений нет (словарь не нарушен; упоминание
    `resolved` — только в легенде).
  - Счётчики `TRACEABILITY`: `open` = 26 (было 27), `in work` = 21 (было 19,
    +Q78 +Q79), `done` = 32 (не тронуто).
  - Финальная сверка (read-only): Q78/D82 и Q79/D83 = `in work` с `T-20 ⬜`;
    Q79/D83 в обоих каталогах; T-20 в сводке `tasks/README.md` (P3, D83 (Q79), ⬜).
  - `cargo` не запускался (D50).
- **Остаток/риски:** правка `.opencode/rules/journal.md` (если появится) — зона
  сервисной сессии. Коммит не делал. Дальше по маршруту: `validator` → гейт →
  `git`. Грабли: черновик T-20 содержал ошибочную ссылку `D78` — пойман
  перечитыванием «Контекста»; проверяйте новые карточки глазами после записи.

## validator · 01.10.2026 · принято

- **Вердикт:** принято, P1/P2/P3 нет. Отчёт
  `docs/reviews/service-lifecycle-w2-prep-2026-10-01.md`; квитанция
  `service-lifecycle-w2-prep` iteration 1, `accepted`.
- **База:** `develop` @ `b04a77a` (= `origin/develop` = `HEAD`) + рабочее дерево;
  линейно после w1 (`cb7d159` ∈ `develop`).
- **Счётчики `TRACEABILITY`:** `open`=26, `in work`=21, `done`=32; 79 строк
  (`^| [Q`), арифметика точна; ячеек `resolved`/`dropped` нет (только легенда);
  каждое `in work` имеет задачу (правило D82).
- **Диффы:** TRACEABILITY — Q78/D82 `open`→`in work` +`T-20 ⬜`, новая строка
  Q79/D83 `in work` +`T-20 ⬜`; `tasks/README.md` — +строка T-20 и абзац
  «Исключение» (D83); каталоги +1/+1. Тела D82/D63/D64/T-18/`journal.md` не
  тронуты (дифф пусто).
- **Q79/D83/T-20:** формы журнала/карточки, `Resolves`, «Сверка с кодом» ⚪,
  `Tasks: T-20`, критерий готовности; ссылки живые; сессионных адресов нет (D65).
- **Границы:** 5 `M` + 4 `??` (до моих записей) — ровно план;
  `git diff --stat -- src tests Cargo.toml AGENTS.md` пусто.
- **Права:** `agents-perms.mjs` ×2 → `11 из 18`, расхождений нет.
- **DoD:** `cargo` не запускался (D50 — пакет документный, код не затронут).
- **Гейт:** состав пакета — `M`: `docs/TRACEABILITY.md`, `docs/questions/README.md`,
  `docs/decisions/README.md`, `docs/tasks/README.md`, `.opencode/memory/migrator.md`;
  `??`: `.opencode/mail/service-lifecycle-w2-prep.md`, `docs/questions/Q79.md`,
  `docs/decisions/D83-traceability-wave2.md`, `docs/tasks/T-20-traceability-wave2/`;
  улики приёмки — `docs/reviews/service-lifecycle-w2-prep-2026-10-01.md` и
  `receipts.yaml` (по праву роли).

## сервисная сессия · 01.10.2026 · подтверждение пакета (гейт)

- Гейт пройден: владелец подтвердил **коммит + push** (`question`, ответ
  «Коммит + push (Recommended)») — операция `service-lifecycle-w2-prep`.
- База: `develop` @ `b04a77a` (= `origin/develop`); режим — коммит в `develop`,
  затем `git push origin develop`.
- Пакет — 13 путей + запись роли `git` (F43, `memory/git.md`, новый до `add`) = 14:
  - `.opencode/mail/service-lifecycle-w2-prep.md` (лента)
  - `.opencode/memory/git.md` (чекпойнт роли `git`)
  - `.opencode/memory/migrator.md`
  - `.opencode/memory/validator.md`
  - `.opencode/state/current/progress.yaml`
  - `.opencode/state/current/receipts.yaml`
  - `docs/TRACEABILITY.md`
  - `docs/decisions/D83-traceability-wave2.md`
  - `docs/decisions/README.md`
  - `docs/questions/Q79.md`
  - `docs/questions/README.md`
  - `docs/tasks/README.md`
  - `docs/tasks/T-20-traceability-wave2/README.md`
  - `docs/reviews/service-lifecycle-w2-prep-2026-10-01.md`
- Сообщение коммита:
  `docs(Q79/D83): волна 2 TRACEABILITY — разбор open-строк задачей T-20 (service-lifecycle-w2-prep)`
- Состояние: приёмка `validator` — принято (P1/P2/P3 нет).

## git · 01.10.2026 · готово

- **Сделано (до `add`, F43):** сверена база — `git status -sb` → `## develop...origin/develop`,
  `git log -1 --oneline` → `b04a77a` (= `origin/develop`); снимок рабочего дерева совпал
  с пакетом: 8 `M` + 5 `??` (лента, `Q79.md`, `D83-*.md`, `reviews/*.md`, папка `T-20/`)
  + мой чекпойнт = 14. Отчёт и чекпойнт записаны до `add`.
- **Дальше:** `add` 14 точных путей (`./`-префикс, без `--`) → сверка
  `git diff --cached --name-status` (ожидается 9 `M` + 5 `A`) → `commit` сообщением
  гейта → `push origin develop` (таймаут ≥ 5 мин). Хеши — в ответе `lead` (F43).
