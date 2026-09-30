# Сервисная лента: service-doc-refactor («Рефакторинг содержимого `docs/`»)

**Назначение:** сервисная волна рефакторинга содержимого `docs/` по запросу
владельца (30.09.2026). Операция №1 — формат `docs/TRACEABILITY.md`
(колонки Q и D).

**Решение владельца (30.09.2026, `question` + ответ):**
- колонки Q и D в `TRACEABILITY.md` — **только ссылки** `[Qn]`/`[Dn]`, без
  описаний (структура — [D63](../../docs/decisions/D63-journal-index-lifecycle.md));
- темы — в каталогах `questions/README.md` и `decisions/README.md`, формулировки —
  в Q/D-файлах ([D60](../../docs/decisions/D60-docs-ownership-sync.md)/
  [D65](../../docs/decisions/D65-reference-policy.md));
- применить сразу ко всем существующим строкам и закрепить для новых.

**План:** Q72 + D76 (`migrator`) вместе с применением формата ко всем строкам →
приёмка `validator` → пакет `git` (по подтверждению владельца).

---

## сервисная сессия · 30.09.2026 · открытие операции

- База: `develop` = `origin/develop` = `6182bb5`; рабочее дерево чистое.
- Свободные ID: `Q72`, `D76` (подтверждено памятью `migrator` и лентой
  `service-git-efficiency`).
- Маршрут: журнальная операция (`AGENTS.md` §«Другие маршруты»): `migrator` →
  `validator`; канон `.opencode/rules/**` не меняется — аудит не требуется.
- Границы: `docs/**` (журнал и `TRACEABILITY.md`), лента и память ролей;
  `src/**`, `tests/**`, `Cargo.toml`, `.opencode/rules/**`, `.opencode/agents/**`
  не трогаются; `cargo` не запускается (D50 — документная волна).

---

## migrator · 30.09.2026 · готово

- Сделано: **Q72 → D76** — `docs/questions/Q72.md` (⚪ оформление; сразу
  `resolved by [D76]`) и `docs/decisions/D76-traceability-links-only.md`
  (`accepted`, `Resolves: [Q72]`, `Спека: —`, `Affects` — `TRACEABILITY.md`).
  Формат `TRACEABILITY.md` применён ко всем строкам (Q1–Q71 + Q72/D76): клетки
  Q/D — только ссылки `[Qn]`/`[Dn]`, у Q6 снята пометка «закрыт попутно при Q5»,
  легенда дополнена пояснением (D63, D76); строки в `questions/README.md` и
  `decisions/README.md` (+1). Tasks: `—` («задач не требуется»; машинная
  проверка — T-18/D64).
- Проверки: `rg -c "^\| \[Q" docs/TRACEABILITY.md` → **72**;
  `rg -n "^\| \[Q\d+\]\(questions/Q\d+\.md\) — "` → пусто (и аналог для D);
  `rg -c "^\| [^|]+ \| [^|]+ \| [^|]+ \| [^|]+ \| [^|]+ \|\r?$"` → **73**
  (шапка + 72 данных, 5 колонок); `rg -c "^"` = `rg -c "\r$"` = **84** (CRLF и
  финальный перевод строки целы); выборочно сверены Q6/Q28/Q32/Q71/Q72 —
  колонки 3–5 дословны; ссылки Q72↔D76 живые. Сверка с кодом — ⚪
  (документы/процесс), `cargo` не запускался (D50).
- Дальше / риски: **факт обрыва по шагам** — первая часть вызова завершилась
  после правки `TRACEABILITY.md` (7 блочных `edit`), оставшиеся пункты
  (Q72/D76, каталоги, отчёт) выполнены продолжением; `TRACEABILITY.md` этим
  продолжением не трогался. Приёмка — `validator`; пакет — за `git` (по
  подтверждению владельца).

---

## validator · 30.09.2026 · принято

- Сделано: приёмка `service-doc-refactor` — Q72 → D76 и формат колонок Q/D
  `TRACEABILITY.md`. Вердикт — **принято**, P1/P2/P3 нет; отчёт
  `docs/reviews/service-doc-refactor-2026-09-30.md`; квитанция
  `service-doc-refactor` iteration 1 (`accepted`) — append.
- Проверки: `git status` = 7 путей; `git diff --stat -- src tests Cargo.toml`
  пусто → cargo не запускался (D50); 72 строки Q (HEAD 71 + Q72), строгий
  5-колоночный формат всех 72, клетки Q/D без « — »; колонки 3–5 совпали с HEAD
  (`| resolved |` 54/53, `| in work |` 14/14, `| done |` 4/4, ⬜15/15, 🚧1/1,
  ✅8/8, `](features/` 43/43, `](tasks/` 19/19); Q6 :10 дословно = HEAD, маркер
  снят, канон Q6/D19 цел; Q72 :76 `resolved | — | —`; легенда = прежний текст +
  предложение (D63, D76); CRLF 84/84; §4-шаблон Q72/D76, ⚪, `Tasks: —`, §10 нет;
  каталоги 72/76 строк; ссылки живые/относительные; лента и память `migrator` —
  ок.
- Дальше / риски: пакет `git` — по подтверждению владельца (7 путей, `develop`
  @ `6182bb5`). Замечаний нет.

---

## сервисная сессия · 30.09.2026 · подтверждение пакета

- Гейт пройден: владелец подтвердил **коммит + push** (`question`, ответ
  «Коммит + push (Recommended)») — сервисная волна `service-doc-refactor`,
  один коммит «канон + записи» (D75).
- База: `develop` @ `6182bb5` (= `origin/develop`); режим — коммит прямо в
  `develop` (как предыдущие сервисные волны), затем `git push origin develop`.
- Пакет (10 путей операции + запись роли `git` по F43 = 11):
  - `.opencode/mail/service-doc-refactor.md`
  - `.opencode/memory/git.md` (чекпойнт роли `git`, новый до `add`)
  - `.opencode/memory/migrator.md`
  - `.opencode/memory/validator.md`
  - `.opencode/state/current/receipts.yaml`
  - `docs/TRACEABILITY.md`
  - `docs/decisions/D76-traceability-links-only.md`
  - `docs/decisions/README.md`
  - `docs/questions/Q72.md`
  - `docs/questions/README.md`
  - `docs/reviews/service-doc-refactor-2026-09-30.md`
- Сообщение коммита:
  `docs(Q72/D76): TRACEABILITY — только ссылки Qn/Dn; темы — в каталогах`.
- Замечание: запись роли `git` (лента + `memory/git.md`) формируется **до**
  `add` и входит в коммит (F43); после `push` в отслеживаемые файлы не пишем
  (хеши — ответом `lead`).

---

## git · 30.09.2026 · пакет выполняется

- Сделано (до `add`): сверка состава — `git status --porcelain` = ровно 10
  путей операции из пакета (M 6, ?? 4), посторонних нет; `git-check.mjs`
  подтвердил базу `develop` = `origin/develop` = `6182bb5`. Записи F43 (эта
  запись и `memory/git.md`) сделаны **до** `add` и входят в коммит.
- Проверки: `git status --porcelain` → 10/10 путей совпали; staged-сверка —
  после `add` (`--expect=11`), отдельным шагом.
- Дальше / риски: `add` 11 точных путей → `commit`
  `docs(Q72/D76): TRACEABILITY — только ссылки Qn/Dn; темы — в каталогах` →
  `push origin develop` (≥ 5 мин). Хеши — ответом `lead` (F43).
