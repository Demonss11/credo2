# Память: auditor (аудит системы агентов)

- **Канон:** `AGENTS.md` §Рабочая группа агентов; чек-лист — в промпте роли.
- **Правило:** чекпойнт — что аудировано, находки P1–P3, что осталось. Кратко.

## Чекпойнты

- 03.10.2026, r13 (T-15, фаза C; C2 `validate-state.mjs`, до коммита): аудит
  D91↔факт + журнал + границы. **Инструкция ↔ права по формам команд —
  расхождений нет** (frontmatter `validator` = `review.md` по существу; новые
  allow `validate-state.mjs`/`*`; «11 из 18», `steps=36`). **5×P2:** (1) карточка
  `C2` ✅/`service-c2` — приёмки не было, квитанции/отчёта нет, TRACEABILITY:92
  `in work` → фиктивная улика; (2) `dispatch-loop.md` pre-flight назначен `lead`,
  у которого нет права на скрипт; (3) `crossCheck` сверяет единое `iteration`
  только plan↔state, а не 4 артефакта (D91 п.4/schema п.1); (4) `question` при
  `surface_to_user` не проверяется (D91 п.3); (5) `SCHEMA_DATE=03.10` vs D86=02.10
  — записи 02.10 мягче канона. **P3:** `review.md:90` без bare-формы
  `validate-state.mjs`, тело `validator.md:97` использует bare. Ок: место/роль
  скрипта, точки применения, поля/enum/даты/`result`/`next`, коды 0/1/2 и флаги,
  `iteration≥1`/`rework`/`session_index`/`surface_to_user`, мягкий режим +
  `--strict`; Q88↔D91 парны, каталоги/TRACEABILITY, «Сверка с кодом» ⚪; нет
  `docs/analysis/**`-адресов и номеров строк; границы — `src/tests/Cargo.toml`/
  `opencode.json` не тронуты. Сам `validate-state.mjs` не прогнал (нет права у
  `auditor` — by design); контракт сверен чтением. `cargo` не запускался (D50).
  Отчёт — лента r13 (append). Канон не правил. Урок: при аудите скрипта сверять
  **реализацию инвариантов**, а не только перечисление в шапке/D.

- 03.10.2026, r12 (T-15, фаза C; C5+C7, до коммита): аудит D90↔факт + журнал +
  границы. **P1/P2 нет; инструкция ↔ права — расхождений нет.** C5: 7 полей
  `re_raise` и 4 категории совпадают D90↔`dispatch-loop.md` §«Re-raise»↔
  `state-schema.md`↔`analyst.md`/`lead.md`; `owner_override` вне метрики, повтор
  — сигнал; зеркало `replan_reason` (enum 4 кат. при `action: re-plan`). C7:
  шаблон сегментного итога в §«Каденция» (`Задача/Фаза/Статус/Лента` +
  `Сделано/Дальше/Риски`); фичи `agents-re-raise` ✅, `agents-state-schema` 🟡,
  `agents-session-checkpoint` 🟡 (заметки; счётчики 47/278 без изменений; `.feature`
  шапки не тронуты — конвенция семьи `# T-15 (проект)`). Журнал: Q87
  `resolved by D90`, TRACEABILITY:91 (Q87/D90/in work/T-15 🚧), карточка `C5`/`C7`
  🚧; ссылки живы, номеров строк нет. `agents-perms.mjs` ×2 идентичны, 11/18,
  фронтматтеры не менялись. Границы: `.opencode/rules|agents` (4 файла) + docs
  журнал/`features`/карточка + лента/память; `AGENTS.md`/`opencode.json`/`src`/
  `tests`/`Cargo.toml` не тронуты. **P3 (наблюдение, не блокирует):**
  `service-mcp-ready-r11.md` при catch-up перекодирован UTF-16→UTF-8
  (40 304→21 423 Б; контент сохранён + секция «итог r11») — учесть `validator`
  при пакете. `cargo` не запускался (D50). Отчёт — лента r12 (append). Канон не
  правил.

- 03.10.2026, r11 (T-15, фаза C; P1+P2+P3, до коммита): аудит D89↔факт (3
  пункта) + «инструкция ↔ права» + журнал + границы. **Все 3 пункта
  реализованы:** P1 — `git-workflow.md` §«Старт задачи» п.4 + §«Пакет и
  подтверждение» п.5, `dispatch-loop.md` §«Git-пакет», `analyst.md` (in-flight,
  права) + `review.md`; P2 — `dispatch-loop.md`, `lead.md`, `analyst.md`;
  P3 — §«Session-commit» + `lead.md`/`analyst.md`/`dispatch-loop.md`. Дублей в
  `AGENTS.md` нет. `agents-perms.mjs` ×2 идентичны, «11 из 18», runtime
  `analyst` = `branch -a *`/`--list *`/`rev-parse *`. Q86↔D89 парны,
  TRACEABILITY:90 (in work/T-15 🚧), карточка `C3`/`C15`/`C16` 🚧, F80–F82 →
  D89, ссылки живы (форма `../../.opencode/**` — норма D81/D86–D88), номеров
  строк нет. Границы: только 5 канон-файлов + журнал/TRACEABILITY/карточка/
  findings/лента/память; `src/tests/Cargo.toml`/`AGENTS.md` не тронуты.
  **P1/P2/P3 нет; инструкция ↔ права — расхождений нет.** Урок: паттерн
  `cmd *` матчит bare-команду `cmd` (проверено `git branch -a`); `validator`
  `branch --contains` без `*` в `review.md` — ок. `opencode mcp list` — вне
  прав; F65-факт с ленты. Отчёт — лента r11 (append). Канон не правил.

- 02.10.2026, C1 (T-15, служебная операция service-mcp-ready-r8, класс L, до
  коммита): аудит канона схемы состояния. Состав/границы — ровно 9 M + 4 ??,
  `git diff -- src tests Cargo.toml` пусто. Q83↔D86 парны, TRACEABILITY Q83:
  D86/`in work`/T-15 🚧; F15 не закрыт — корректно;
  `agents-perms.mjs` — 11 из 18, команды = `review.md`; «инструкция ↔ права»:
  расхождений нет. Находки: **P2×2** — D86 `Affects` без `analyst.md`/`lead.md`;
  `state-schema.md` enum `progress.action` без `wait_for_user`; **P3** — `idle`
  в `status` без триггера. Вердикт: правки P2 до коммита, затем свежий аудит.
  Технический факт: `read` больших state/README-файлов срезан `token-guard`.
  Отчёт — лента r8 (append). Канон не правил.

- 02.10.2026, C1 — повторный аудит после правок: **P2-1** (D86 `Affects` +
  `analyst.md`/`lead.md`) и **P2-2** (`wait_for_user` в enum `progress.action`)
  закрыты; первый **P3** (`idle`) закрыт — определение есть, `analyst.md` синхрон.
  Регресс чист: 10 `M` + 4 `??` (2 из них — записи памяти), `src/tests/Cargo.toml`
  не тронуты, `git diff --check` пусто, номеров строк нет, ссылки живы,
  `agents-perms.mjs` — 11 из 18, «инструкция ↔ права» — расхождений нет. Новая
  **P3** (одна строка): `lead.md:88` шаблон отчёта несёт `done`, а схема — `idle`;
  не блокирует. Вердикт: P1/P2 нет. Отчёт — лента r8 (append). Канон не правил.

- 02.10.2026, T-20 (волна 2 `TRACEABILITY`, класс L, до коммита): аудит
  документного пакета. 9 строк → `done` (Q8–Q11, Q14, Q18, Q20, Q21, Q23);
  Q22 `in work` + T-24 ⬜; 16 `open`; Q81 `open`; Q78/Q79 `in work` (✅);
  T-20 🚧 в сводке и карточке; T-24 (Q22/D26) — структура как у T-22,
  видимость через Q22 (D77). Границы — зона `migrator`, `src/tests/Cargo.toml`
  не тронуты; права не менялись. Вердикт: **P1/P2/P3 нет, расхождений нет**.
  Технический факт: shell недоступен → `git status`/`agents-perms.mjs` не
  запускались; `read` TRACEABILITY срезан `token-guard` (дочитан offset).
  Отчёт — лента T-20 (append). Канон не правил.

- 02.10.2026, T-20 (волна 2 `TRACEABILITY`, класс L, **финальный пакет**, до
  коммита): финал после закрытия. 9 `done` (Q8–Q11,Q14,Q18,Q20,Q21,Q23) без
  открытых задач; Q22 `in work` + T-24 ⬜; 16 `open`; Q81 `open` (—/—);
  **Q78/Q79 `done` с видимой `[T-20] ✅`** (D77-часть теста выполнима); легенда
  D82 соблюдена (in work ⇒ открытая задача, done/open ⇒ нет). Сверены все
  **23 пары** «реестр ↔ TRACEABILITY» — 0 расхождений; T-20 ✅ (сводка:66 +
  карточка:3); T-24 ⬜ (реестр:69), ссылки живые, номеров строк нет. Границы:
  `src|tests|Cargo.toml|AGENTS.md|opencode.json` — `git diff --numstat` пусто;
  `.opencode/rules|agents` и `opencode.json` вне `git status`. Права: зона
  `migrator`/`validator` покрывает правки, статусы ролей не менялись;
  `agents-perms.mjs` дважды «11 из 18». **P1/P2/P3 нет; инструкция ↔ права —
  расхождений нет.** Тех.факт: составные shell-команды отклонялись правами
  (одиночные — ок); `read` TRACEABILITY срезан `token-guard` (дочитан). Бюджет
  чтения слегка превышен (18 > 15) — отмечено владельцу. Отчёт — лента T-20
  (append). Канон не правил.

- 02.10.2026, r10 (T-15, фаза C; правки по C9/C12, до коммита): аудит
  D88↔факт (6 пунктов) + «инструкция ↔ права». Все 6 пунктов реализованы
  (lead 24 + правило останова; 1 запись/действие + сегментный итог; глубокий
  план; resume-синк без analyst; дробление вызовов; сплит 300 — пункт re-plan)
  в `lead.md`/`analyst.md`/`dispatch-loop.md`/`AGENTS.md`; противоречий уровней
  нет. `agents-perms.mjs` ×2 идентичны, «11 из 18», `lead steps=24`; AGENTS 24
  ↔ фронтматтер 24 ↔ agents-perms 24 — расхождений нет; `review.md` команды
  синхронны. Q85↔D88 парны, TRACEABILITY:89 (Q85/D88/in work/T-15 🚧), карточка
  C9/C12 🚧, ссылки живы, номеров строк нет. Границы: только агенты/rules/
  AGENTS/журнал/TRACEABILITY/карточка/лента-память; `src/tests/Cargo.toml` не
  тронуты. **P1/P2/P3 нет.** Не проверено (вне прав `auditor`): runtime
  `opencode debug agents` (24 — по agents-perms) и `opencode mcp list`
  (F65-факт с ленты). `cargo` не запускался (D50). Отчёт — лента r10 (append).
  Канон не правил.

- 02.10.2026, r9 (T-15, фаза C; правки по S-пилоту T-24, до коммита): аудит
  «инструкция ↔ права» + D87 ↔ факт. `agents-perms.mjs` ×2 — «11 из 18»,
  новые команды видны в runtime (lead `git rev-parse --short HEAD`; git
  `branch --list *`/`ls-remote --heads origin *`; tester `cargo clippy *`);
  `review.md` §«Доступные команды» синхрон фронтматтерам (diff подтвердил).
  D87 4 пункта = фактические правки (lead/git/analyst/tester + review/
  git-workflow/dispatch-loop), иных правок в каноне нет. Q84↔D87 парны,
  TRACEABILITY Q84: D87/`in work`/T-15 🚧; карточка C14 🚧 + F26/F27,
  адресов `.opencode/mail|state` и номеров строк нет. Границы: изменения
  только `.opencode/{agents,rules,memory,mail,state}` + `docs/**`; `src/**`,
  `tests/**`, `Cargo.toml` не тронуты. **P1/P2 нет; P3 — одна (C14 ссылается
  на разбор `docs/analysis/**` именем, адресных ссылок нет).** Не проверено:
  `opencode mcp list` — вне прав (`opencode reload` only); F65-факт взят с
  ленты r9. Отчёт — лента r9 (append). Канон не правил.
