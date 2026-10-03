# service-mcp-ready-r10 — лента операции: T-15, C9/C12 (лимит/записи/re-plan)

Открыта: 02.10.2026. **Сервисная операция** (программа T-15, фаза C; после r9).
Предмет: решения **C9** («правило останова ведущего») и **C12** («дробление
L-вызова») по данным S-пилота T-24 (разбор
`docs/analysis/T-24-run-2026-10-02-lead-session.md`; F67/F68/F58).

**Решения владельца 02.10.2026 (question, 3 развилки):**
1. C9: лимит `lead` — **16→24**;
2. C9: записи — **одна на действие** (`progress.yaml`); лента — отчёты ролей +
   **сегментный итог** `lead` (один на запуск);
3. C12: **глубокий план** (на весь участок до гейта/пакета) + **resume-синк без
   `analyst`** при совпадении плана с состоянием.

**Состав правок (к журналу Q85 → D88):**
- `lead.md` — `steps: 24`; цикл: запись в `progress` после действия; сегментный
  итог в ленту перед остановкой;
- `dispatch-loop.md` — каденция (re-plan при resume — только при расхождении),
  записи (1/действие + сегментный итог), правило останова, дробление тяжёлых
  L-вызовов, сплит лент >300 — обязательный пункт re-plan;
- `analyst.md` — очередь до гейта/пакета (не формальные 1–3); resume-политика —
  ссылка на `dispatch-loop`;
- `AGENTS.md` — §«Лимиты шагов» (`lead` 24), §«Память и почта» (сегментный
  итог).

**Протокол:** журнал → правки → `opencode reload` → `auditor` («инструкция ↔
права») → `validator` (адресная, без cargo) → гейт → `git` (develop, сервисный
пакет) → итог.

## сервисная сессия · 02.10.2026 · открытие

- Срез: `develop` = `origin/develop` = `af53e23` (r9) + пост-пакетные записи r9
  (лента, память) — подхват.
- Следующее действие — `dispatch migrator` (Q85 → D88 + карточка C9/C12).

## сервисная сессия · 02.10.2026 · правки канона — готово

- **Журнал:** Q85 → D88 (`migrator`); каталоги, `TRACEABILITY` (Q85: D88,
  `in work`, T-15); карточка T-15: `C9`/`C12` → 🚧 (service-mcp-ready-r10).
- **Правки (сервисная сессия):**
  - `lead.md` — `steps: 24`; цикл: **одна запись** в `progress` на действие;
    **сегментный итог** в ленте (один на запуск, обязателен перед остановкой);
    resume — сверка плана без `analyst` при совпадении;
  - `dispatch-loop.md` — §«Каденция»: очередь до гейта/пакета («1–3» —
    ориентир); записи 1/действие + сегментный итог; re-plan при resume — только
    при расхождении; дробление тяжёлых вызовов; §«Hard rules» — правило
    останова (пустой финал запрещён); §«Память…» — сплит лент 300 —
    обязательный пункт re-plan;
  - `analyst.md` — очередь до гейта/пакета; resume-политика; проверка порога
    ленты на каждом re-plan;
  - `AGENTS.md` — §«Лимиты шагов» (`lead` 24); §«Память и почта» (сегментный
    итог).
- **`opencode reload`** выполнен; `agents-perms --role lead` — `steps=24`;
  MCP-каталог восстановился (F65-факт).
- Следующее действие — `auditor` («инструкция ↔ права», agents-perms ×2).

## auditor · 02.10.2026 · аудит — расхождений нет

- P1/P2/P3 нет; D88 ↔ факт — 6/6 пунктов; `agents-perms.mjs` ×2 — «11 из
  18», `lead steps=24`; AGENTS.md 24 = фронтматтер = runtime; `review.md`
  синхронен; журнал/границы чисты; `cargo` не запускался.
- Следующее действие — `validator` (адресная приёмка, без cargo).

## сервисная сессия · 02.10.2026 · пакет r10 — гейт владельца

- Приёмка: `validator` — **accepted** (`service-c9-c12`; отчёт
  `docs/reviews/service-c9-c12-2026-10-02.md`). C9/C12 ✅;
  F58/F67/F68/F72 закрыты; Q85 остаётся `in work` (T-15 🚧, D82).
- **Пакет (снимок `git status`, 15 M + 4 ?? = 19 путей + запись роли `git`
  F43 = 20):**
  - канон: `.opencode/agents/{analyst,lead}.md`,
    `.opencode/rules/dispatch-loop.md`, `AGENTS.md`;
  - журнал: `docs/questions/Q85.md`, `docs/decisions/D88-c9-c12-loop-tuning.md`,
    `docs/{questions,decisions}/README.md`, `docs/TRACEABILITY.md`;
  - карточка T-15 + findings; отчёт приёмки
    `docs/reviews/service-c9-c12-2026-10-02.md`;
  - подхват r9: `.opencode/mail/service-mcp-ready-r9.md`,
    `.opencode/memory/service.md`;
  - state/память: `.opencode/state/current/receipts.yaml`,
    `.opencode/memory/{auditor,migrator,validator}.md`;
  - лента r10 `.opencode/mail/service-mcp-ready-r10.md`;
  - + `.opencode/memory/git.md` (запись роли, F43).
- Сообщение: `chore(process): T-15 r10 — C9/C12 (лимит lead 24, записи, re-plan; D88); приёмка`.
- База: `develop` = `origin/develop` = `af53e23`; ветки нет; master — по
  решению гейта.

## сервисная сессия · 02.10.2026 · гейт пакета — подтверждён

- **question:** owner-gate пакета r10 (20 путей).
- **owner_response (дословно):** «Коммит + push develop (Recommended)».
- Порядок: коммит в `develop` + `push origin develop`; `master` не трогаем.
- Следующее действие — `dispatch git`.

## сервисная сессия · 02.10.2026 · итог r10

- **Коммит:** `2958bf5` — `chore(process): T-15 r10 — C9/C12 (лимит lead 24,
  записи, re-plan; D88); приёмка` (20 путей: 16 M + 4 A; +659/−31); push
  `af53e23..2958bf5` → `origin/develop`; ветки/master не тронуты.
- **Итог:** C9/C12 решены (D88): лимит `lead` 24 + правило останова; записи
  1/действие + сегментный итог; глубокий план до гейта/пакета; resume без
  `analyst` при совпадении; дробление тяжёлых вызовов; сплит лент 300 —
  обязательный пункт re-plan. F58/F67/F68/F72 закрыты; C9/C12 ✅.
- **Остаток:** F26/F27 — проверка C9/C12 «чистым» прогоном; Q85 — `in work`
  (T-15 🚧).
- Пост-пакетные записи — подхват следующим пакетом (F43).

## migrator · 02.10.2026 · готово

- Сделано: заведены `docs/questions/Q85.md` (`resolved by D88`) и
  `docs/decisions/D88-c9-c12-loop-tuning.md` (6 пунктов: лимит `lead`
  16→24 + правило останова; 1 запись/действие + сегментный итог; глубокий план
  `analyst`; resume-синк без re-plan; дробление тяжёлых вызовов; сплит лент
  ≥300 — обязательный пункт re-plan). Строки в `questions/README.md`,
  `decisions/README.md`, `TRACEABILITY.md` (Q85: D88, `in work`, T-15 🚧,
  реализация — правки канона цикла). Карточка T-15: `C9` ⏸→🚧 и `C12` ⬜→🚧
  (`service-mcp-ready-r10`, 02.10); иные строки не тронуты.
- Проверки: ID свободны (последние — Q84/D87); `git diff --check` пусто;
  ссылки живые (`docs/decisions/*`, `.opencode/agents/{lead,analyst}.md`,
  `.opencode/rules/dispatch-loop.md`, `AGENTS.md`, findings, разбор, T-15,
  `mcp-ready-process.md` — glob); `D88|Q85` в `docs/` — только мои 11
  вхождений (дублей нет). Сверка D88 — ⚪ (процесс/документы; `cargo` не
  запускался, D50).
- Файлы: Q85, D88, 2 каталога, TRACEABILITY, карточка T-15. Новых находок нет.
- Дальше / риски: правки канона сервисной сессией (`lead.md` `steps: 24`,
  `analyst.md`, `dispatch-loop.md`, `AGENTS.md`) → `opencode reload` →
  `auditor` («инструкция ↔ права») → `validator` (адресная, без cargo) →
  гейт → `git` (develop). F58/F67/F68 остаются открытыми до приёмки.

## auditor · 02.10.2026 · готово

- Сделано: независимый аудит r10 до коммита — **D88 ↔ факт (6 пунктов)**,
  «инструкция ↔ права», журнал/карточка/ссылки, границы.
- Проверено (D88 пункт → где подтверждён):
  - п.1 лимит `lead` 16→24: `lead.md:6` | `AGENTS.md:132`;
  - п.1 правило останова (пустой финал запрещён): `dispatch-loop.md:49–51`;
  - п.2 одна запись/действие + сегментный итог: `lead.md:59–61`,
    `dispatch-loop.md:33–35`, `AGENTS.md:105–107`;
  - п.3 глубокий план до гейта/пакета («1–3» — ориентир): `analyst.md:67–68,
    86–89`, `dispatch-loop.md:29–32`;
  - п.4 resume-синк без `analyst` при совпадении: `lead.md:50–51`,
    `analyst.md:90–92`, `dispatch-loop.md:36–37`;
  - п.5 дробление тяжёлых вызовов: `dispatch-loop.md:39–41`;
  - п.6 сплит лент 300 — обязательный пункт re-plan: `analyst.md:93–94`,
    `dispatch-loop.md:169–171`.
  Противоречий уровней (тело ↔ rules ↔ AGENTS) нет; дублирования методики нет —
  формат почты в `AGENTS.md`, каденция — в `dispatch-loop.md`/роли.
- **Инструкция ↔ права:** `node .opencode/scripts/agents-perms.mjs` — дважды,
  вывод идентичен, состав полный «agents: 11 из 18»; `lead [primary] steps=24`.
  Согласованность **AGENTS.md 24 ↔ фронтматтер 24 ↔ agents-perms 24** —
  расхождений нет. `review.md` §«Доступные команды» (строки 86–111)
  синхронен фронтматтерам — дрейфа команд нет (списки сверены по всем ролям,
  включая `auditor`/`git`/`validator`).
  **Вердикт: «Инструкция ↔ права: расхождений нет».**
- **Журнал:** Q85↔D88 парны (`Q85` `resolved by D88`; `D88` `Resolves: Q85`,
  `accepted`); `questions/README.md:107`, `decisions/README.md:117`;
  `TRACEABILITY.md:89` — Q85: D88, `in work`, T-15 🚧; карточка T-15 —
  `C9`🚧 (`:220`), `C12`🚧 (`:223`) с `service-mcp-ready-r10`; номеров строк в
  ссылках/текстах нет; все относительные ссылки Q85/D88 живы (glob: `D88`,
  `Q85`, `findings-registry.md`, разбор T-24, `mcp-ready-process.md`, D50/
  D70/D78/D87).
- **Границы:** изменены ровно `.opencode/agents/{lead,analyst}.md`,
  `.opencode/rules/dispatch-loop.md`, `AGENTS.md`,
  `docs/{questions,decisions}/**`, `docs/TRACEABILITY.md`, карточка T-15,
  лента/память — совпадает с заявленным периметром. `src/**`, `tests/**`,
  `Cargo.toml` не тронуты (`git status --porcelain` — только процессные/док.
  пути; `git diff --stat` — 11 M + 3 нов. файла, кода нет). Ветка `develop`.
- **Проверки:** `agents-perms.mjs` ×2 — 11/18, ok; `git status --porcelain`,
  `git diff --stat`, `git branch --show-current` — ок.
- Дальше / риски: **P1/P2/P3 нет; расхождений нет** — пакет к коммиту готов.
  Не проверено: runtime-резолв `opencode debug agents` и `opencode mcp list` —
  **вне прав `auditor`** (`review.md`: `auditor` — `rg`, git read-only,
  `agents-perms.mjs`, `opencode reload`); runtime 24 и F65-факт (MCP-каталог
  восстановлен) взяты со слов ленты r10 и `agents-perms` (читает фронтматтер).
  `cargo` не запускался (D50; процессный пакет).

## validator · 02.10.2026 · принято

- Сделано: адресная документная приёмка r10 (D50 + решение владельца
  02.10.2026 «тесты только при правках кода» — `cargo` не запускался:
  `src/**`, `tests/**`, `Cargo.toml` не тронуты). Снимок: `develop` =
  `origin/develop` @ `af53e23` + рабочее дерево.
- Проверено: **D88 ↔ факт — 6/6** (лимит `lead.md:6`=`AGENTS.md:132`; правило
  останова `dispatch-loop.md:49-51`; 1 запись/действие + сегментный итог
  `lead.md:59-61`/`dispatch-loop.md:33-35`/`AGENTS.md:105-107`; глубокий план
  `analyst.md:67-68,:86-89`; resume без `analyst` `lead.md:50-51`/`analyst.md:90-92`;
  дробление `dispatch-loop.md:39-41`; сплит `analyst.md:93-94`/`dispatch-loop.md:169-171`).
  `agents-perms.mjs` — «11 из 18», `lead steps=24`. Журнал: Q85↔D88 парны,
  каталоги `:107`/`:117`, `TRACEABILITY.md:89` (Q85: D88, `in work`, T-15 🚧),
  карточка T-15 `C9`🚧(`:220`)/`C12`🚧(`:223`); ссылки живы, номеров строк нет.
  Статические гейты `docs_journal.rs` зелёные по чтению. Границы:
  `git diff --numstat -- src tests Cargo.toml` пусто; `git diff --check` пусто.
- Вердикт: **принято — P1/P2/P3 нет.** Отчёт
  `docs/reviews/service-c9-c12-2026-10-02.md`; квитанция `service-c9-c12`
  iteration 1, accepted.
- Дальше / риски: `migrator` — закрытие (`C9`/`C12` ✅, Q85 → `done` + видимая
  `[T-15] ✅`, D82/D77); затем гейт пакета → `git` (develop) → `complete`.
  F58/F67/F68 — закрытие по приёмке.

## migrator · 02.10.2026 · готово

- Сделано: закрытие r10 по приёмке `validator` (`service-c9-c12`, accepted).
  Карточка T-15: `C9`/`C12` → «✅ 02.10 — принято (`service-c9-c12`); D88».
  Реестр находок: **F58/F67/F68** закрыты (D88); **F72** закрыт D88 п.6
  (порог 300 — обязательный пункт re-plan + машинный контроль `validate-state.mjs`;
  проверка — прогоном). F26/F27/F59 не тронуты.
- **Q85 — оставлен `in work`** (не `done`): T-15 остаётся 🚧, `done` не
  допускает открытых задач (D82; прецедент Q83). Колонка «Реализация» без
  правки (D80: колонка — токены фич, не проза). Строка Q85 несёт `[T-15] 🚧`.
- Проверки: `git diff --check` пусто; ссылки (D88, review
  `service-c9-c12-2026-10-02.md`) живы; `cargo` не запускался (D50).
- Файлы: `docs/tasks/T-15-mcp-ready-process/README.md`,
  `docs/analysis/findings-registry.md`; `docs/TRACEABILITY.md` не тронут
  (строка Q85 — из пакета r10, статус верен). Новых Q/D/задач нет.
- Дальше / риски: гейт пакета → `git` (develop, сервисный) → `complete`.

## git · 02.10.2026 · готово

- Сделано (F43-записи до `add`): сверено подтверждение из ленты (секция «гейт
  пакета — подтверждён», `owner_response` дословно «Коммит + push develop
  (Recommended)»); база `develop` = `origin/develop` = `HEAD` = `af53e23`,
  ветки нет. Снимок `git status --porcelain` совпал с пакетом — 15 `M` + 4 `??`
  = 19; + запись роли `git` в `memory/git.md` (F43) = 20.
- Дальше / риски: `add` точными путями (20) → `diff --cached --name-status`
  (ожидание 16 M + 4 A) → `commit` (`chore(process): T-15 r10 — C9/C12 (лимит
  lead 24, записи, re-plan; D88); приёмка`) → `push origin develop`.
  `master` и ветки не трогаются; после `push` в отслеживаемые файлы не пишу
  (хеши — в ответе `lead`).

## migrator · 03.10.2026 · находки прогона T-16/T-25

- Сделано: разбор `docs/analysis/T-16-t25-run-2026-10-02-c9c12-check.md`
  (сессии `ses_f0208b20…` T-16, `ses_f01ccf60…` T-25) → реестр
  `docs/analysis/findings-registry.md`. **F73 закрыт** (02.10.2026: T-25 —
  адреса сняты, `docs_journal` зелёный 14/0; системный остаток — F74).
  **F26/F27/F58** дополнены третьим набором данных (T-16/T-25), статусы —
  открыты. **Новые: F74–F79** (журнальные пакеты без адресного теста журнала;
  неразделимость пакетов двух задач; лимиты `git`/`analyst`; `progress.yaml`
  1775 строк; EN-шум обёртки лимита; дрейф базы плана).
- Проверки: `git diff --check` пусто; ID F1–F79 уникальны (F80 свободен);
  ссылки живые (источник-разбор, связи F9/F43/F57/F58/F61/F73/F76, T-15/T-16/
  T-25, D88); `cargo` не запускался (D50). Новых Q/D нет (указание брифа).
- Файлы: `docs/analysis/findings-registry.md` (+ память/лента). `src/tests/
  Cargo.toml/канон` не тронуты.
- Дальше / риски: `validator` — приёмка реестра (адресная, без cargo); ссылки
  в самих F-строках сохраняют формы `../decisions/…` и `docs/analysis/…` —
  это исторические метки источника (D65/D48), не адреса канона.
