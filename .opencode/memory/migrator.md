# Память: migrator (журнал Q/D)

- **Канон:** `.opencode/rules/journal.md` (§2 ID, §4 шаблоны, §5.3 сверка).
- **Правило:** чекпойнт — состояние записи (Q/D), что проверено, следующее
  действие, ссылки. Кратко.

## Чекпойнты

- 02.10.2026, C1 (T-15): заведены Q83 → D86 (`state-schema`), строки
  `questions/README.md`, `decisions/README.md`, TRACEABILITY (Q83: `in work` /
  T-15 🚧), карточка C1 ⬜ → 🚧. Сверка D86 — ⚪ (процесс/документы; `cargo` не
  запускался, D50). F15 **не** закрыт — после приёмки C1. Следующее: правки
  канона сервисной сессией (`state-schema.md` + ссылки
  `dispatch-loop.md`/`AGENTS.md`), затем `auditor`, `validator`.
- 02.10.2026, P2-1 аудита C1: в D86 `Affects` добавлены
  `.opencode/agents/analyst.md`, `.opencode/agents/lead.md` (синк полей схемы).
  D86 ещё untracked (`??`) — `git diff` по нему пуст, сверка чтением.
- 02.10.2026, после приёмки C1 (`T-15-c1`): закрыт F15 (статус «открыт» →
  «закрыт 02.10.2026»; механику `1→2` ждёт B1-F15); карточка T-15 — C1 🚧 → ✅,
  пометка B1-F15 уточнена. Ссылок на удаляемые зоны нет; `cargo` не запускался.
- 02.10.2026, T-20 применение (волна 2 `TRACEABILITY`, ветка
  `feature/T-20-traceability-wave2`): 9 строк → `done` (Q8–Q11, Q14, Q18, Q20,
  Q21, Q23), Q22 → `in work` + карточка `T-24-rest-cli-contour-test` (Q22/D26,
  P3, ⬜), T-20 ⬜→🚧 (сводка+карточка); 16 строк v0.2 и Q81 — `open`;
  Q78/Q79 не тронуты (`in work` до ✅ T-20, D82). Сверка ⚪ (документный
  пакет, D50, `cargo` не запускался). Новых Q/D нет. Проверки: `git diff
  --check` пусто, ссылки/номера строк — чисто. Следующее: `auditor` →
  `validator`; затем шаг закрытия (T-20 🚧→✅ + Q78/Q79 → `done`).
- 02.10.2026, T-20 участок №2 (rework P2): исправлена одна битая ссылка —
  `docs/tasks/T-24-rest-cli-contour-test/README.md:53`:
  `[D50](../D50-…)` → `../../decisions/D50-dod-by-package-scope.md`. Все прочие
  относительные ссылки карточки резолвятся (glob: D26/D27, `../README.md`,
  T-21/T-22, D50, `.opencode/rules/journal.md`). **Грабля:** T-24 — untracked
  (`??`), поэтому `git diff`/`--numstat` по ней пусты не из-за отсутствия
  правок, а из-за пустого baseline; содержимое сверить чтением. `git diff
  --check` пусто. `cargo` не запускался (D50). Следующее: `validator` `-r2`.
- 02.10.2026, T-20 участок №3 (rework P1): `docs/TRACEABILITY.md:82,:83`
  (Q78/D82, Q79/D83) `[T-20] ⬜` → `🚧` (реестр `tasks/README.md:66` = 🚧).
  Сверены все 24 пары «реестр ↔ TRACEABILITY» — расхождение устранено.
  Ссылки живые, `git diff --check` пусто, `cargo` не запускался (D50).
  **Грабля:** `git diff` по этому файлу тянет и правки участка №1 (та же
  незакоммиченная ветка) — судить о своих правках по локализации строк.
  Следующее: `validator` `-r3`; затем закрытие (T-20 🚧→✅ + Q78/Q79 `done`
  с видимой `[T-20] ✅`).

- 02.10.2026, T-20 закрытие: `tasks/README.md:66` T-20 🚧→✅; карточка
  `T-20-…/README.md:3` 🚧→✅; `TRACEABILITY:82,:83` (Q78/D82, Q79/D83)
  `in work`→`done` с сохранённой **видимой** `[T-20] ✅` (D77). Проверки:
  все 24 пары «реестр ↔ TRACEABILITY» — 0 расхождений; строки `done` без
  открытых задач; ссылки живые; `git diff --check` пусто; `cargo` не
  запускался (D50). **Грабля:** baseline карточки был `⬜` (🚧 не был
  закоммичен участками №1/№2) — правка сразу на ✅. Новых Q/D нет.
  Следующее: `auditor` (L) → гейт → `git` → `complete`.
- 02.10.2026, T-24 закрытие (F57/D77/D82): `docs/TRACEABILITY.md:26` (Q22/D26)
  `in work`→`done`, `[T-24] ⬜`→`✅` (видимая `[T-24] ✅`). Источник — бриф
  плана (маркер `t24_closeout_package`) + лента T-24 (docs-writer ✅ карточки/
  реестра; validator `-r2` accepted, квитанция iteration 2). Сверка факта
  чтением: карточка `T-24-…/README.md:3` = `✅ сделана`; `tasks/README.md:69`
  концевая ячейка `✅`. Другие строки не тронуты — `git diff` = ровно 1 строка
  (`--numstat` 1/1), `git diff --check` пусто. `cargo` не запускался (D50,
  документная правка). Новых Q/D нет (условие: нет). Файлы: только
  `docs/TRACEABILITY.md`. Следующее: pre_gate пакета → `git` branch_end →
  `complete`.

- 02.10.2026, T-24 находки (разбор `docs/analysis/T-24-run-2026-10-02-lead-session.md`):
  обновлены F26/F27 (дополнены фактами T-24, статус «открыт»), F58 (второй
  набор данных T-24), добавлены F67–F72 (цена шага `lead`, churn re-plan,
  права↔проверки, модель `#max`, CCSN `git.push-delete`, лента >300 без сплита).
  Проверки: `git diff --check` пусто; 72 строки, ID F1–F72 уникальны; ссылки
  живые. `cargo` не запускался (D50). Новых Q/D нет. Файлы: только
  `docs/analysis/findings-registry.md` (+ лента T-24). Следующее: `validator`
  (приёмка реестра).

- 02.10.2026, Q84 → D87 (service-mcp-ready-r9, T-15 C14): заведены `Q84`
  («Права/процедуры по T-24») и `D87-t24-pilot-fixes` (4 пункта: права/CCSN/
  state/модель). Строки `questions/README.md`, `decisions/README.md`,
  TRACEABILITY (Q84: `in work` / T-15 🚧); карточка T-15 — C14 🚧 + данные T-24
  в F26/F27. Сверка D87 — ⚪ (процесс/документы; `cargo` не запускался, D50).
  Проверки: Q83/D86 — последние (свобода Q84/D87), `git diff --check` пусто,
  ссылки живые (`.opencode/agents/*`, `.opencode/rules/*`, D47/D50/D70/D78,
  findings, T-15, journal). Файлы: Q84, D87, 2 каталога, TRACEABILITY, карточка
  T-15. Новых находок нет. Следующее: правки канона сервисной сессией →
  `opencode reload` → `auditor` → `validator`.

- 02.10.2026, закрытие r9 (T-15 C14, приёмка `validator` accepted
  `service-t24-fixes`): F69/F70/F71 → «закрыт 02.10.2026» (основание + D87:
  права / `#default` / CCSN); F67/F68 дополнены «остаток — C9/C12 (решения
  отдельно)»; F26/F27/F58 не тронуты. Карточка T-15: C14 → ✅ 02.10 accepted
  (`service-t24-fixes`); **P3 аудита/приёмки закрыт** — из «Источника» C14
  убрано имя разбора `T-24-run-2026-10-02-lead-session.md`, осталось
  «F67–F72 (реестр находок)» (канон: задачи не ссылаются на `docs/analysis/**`).
  Проверки: `git diff --check` пусто, ссылки D87/review живы, `cargo` не
  запускался (D50). Новых Q/D/задач нет. Файлы: `findings-registry.md`,
  карточка T-15. **Грабля:** baseline реестра/карточки был до правок T-24-пакета
  (F67–F72, C14 ещё не закоммичены), поэтому `git diff` показывает больше
  строк, чем мои — судить по локализации. Следующее: гейт пакета → `git`
  (develop, сервисный пакет) → `complete`.

- 02.10.2026, Q85 → D88 (service-mcp-ready-r10, T-15 C9/C12): заведены `Q85`
  («C9/C12: лимит и записи `lead`, политика re-plan и дробление вызовов» по
  данным S-пилота T-24) и `D88-c9-c12-loop-tuning` (6 пунктов: лимит
  `lead` 16→24 + правило останова; 1 запись/действие + сегментный итог;
  глубокий план `analyst`; resume-синк без re-plan; дробление тяжёлых
  вызовов; сплит лент ≥300 — пункт re-plan). Строки
  `questions/README.md`, `decisions/README.md`, TRACEABILITY (Q85: D88,
  `in work` / T-15 🚧, реализация — правки канона). Карточка T-15: C9 ⏸→🚧,
  C12 ⬜→🚧 (`service-mcp-ready-r10`, 02.10); иные строки не тронуты.
  Сверка D88 — ⚪ (процесс/документы; `cargo` не запускался, D50).
  Проверки: Q84/D87 — последние (свобода Q85/D88), `git diff --check` пусто,
  ссылки живые (`lead.md`/`analyst.md`/`dispatch-loop.md`/`AGENTS.md`,
  D47/D50/D70/D78/D87, findings, разбор, T-15, `mcp-ready-process.md`);
  `D88|Q85` — только мои вхождения. Файлы: Q85, D88, 2 каталога,
  TRACEABILITY, карточка T-15 (+ лента r10). Новых находок нет. Следующее:
  правки канона сервисной сессией → `opencode reload` → `auditor` →
  `validator`. F58/F67/F68 — открыты до приёмки.

- 02.10.2026, T-16 синк ячейки: `docs/TRACEABILITY.md:16` (Q12/D54)
  `[T-16] ⬜` → `🚧` — в равенство с реестром (`tasks/README.md:57` T-16 🚧),
  тест T-18 `traceability_tasks_exist_and_match_registry`. Строка Q12 осталась
  `in work` (T-08 ⬜ не закрыта), T-08 ⬜ / T-01 ✅ не тронуты. Проверки:
  `--numstat` 1/1, `git diff` — одна строка, `--check` пусто; `cargo` не
  запускался (D50; адресный прогон — `validator`). Файлы: только
  `docs/TRACEABILITY.md` (+ лента T-16). Новых Q/D/F нет. Следующее: `coder`.

- 02.10.2026, закрытие r10 (T-15 C9/C12, приёмка `validator` accepted
  `service-c9-c12`): карточка T-15 — C9/C12 → ✅ 02.10 (`service-c9-c12`; D88)
  (baseline C9 был `⏸ ждёт F27`, 🚧 не был закоммичен — правка сразу на ✅);
  реестр находок — F58/F67/F68 закрыты (D88), F72 закрыт D88 п.6 (сплит-контроль
  внесён; проверка — прогоном); F26/F27/F59 не тронуты. TRACEABILITY — Q85
  оставлен `in work` (T-15 🚧, D82/прецедент Q83), колонка «Реализация» без
  правки (D80: только токены фич). Проверки: `git diff --check` пусто, ссылки
  (D88, review `service-c9-c12-2026-10-02.md`) живы, `cargo` не запускался (D50).
  Новых Q/D/задач нет. Файлы: карточка T-15, findings-registry. Следующее:
  гейт пакета → `git` (develop, сервисный) → `complete`.

- 02.10.2026, T-16 базовый P1 (участок `t16_baseline_p1`, resume после лимита):
  сняты 7 D65-адресов на сессионный разбор T-24 из журнала — Q84/Q85
  («Связано»), D87 («Контекст»/«Сверка»/«Ссылки»), D88 («Сверка»/«Ссылки»);
  факты остались в реестре F67–F72. Заведена **T-25** (карточка
  `T-25-d65-analysis-addresses`, класс S/P1/⬜, источник Q61/D65 + базовый P1
  T-16 + F73); строки в `tasks/README.md` (P1, после T-16) и `TRACEABILITY.md`
  (Q61/D65, `[T-25] ⬜`); **F73** в `findings-registry.md` (последняя F72,
  статус «открыт → задача T-25»). Проверки: по 4 файлам адресов
  `docs/analysis/<file>` нет (только whitelisted `../analysis/findings-registry.md`);
  `git diff --check` пусто; `git diff --numstat` — целевые файлы/строки; `cargo`
  не запускался (D50). Файлы: Q84, Q85, D87, D88, tasks/README, tasks/T-25,
  TRACEABILITY, findings-registry (+ лента/память). `src/tests/state/канон` не
  тронуты; пакет отдельный от T-16 (F43). Новых Q/D нет. Следующее: адресная
  проверка `docs_journal` (`validator`) → `-r2` приёмка T-16; F73 закроется
  приёмкой T-25.
  **Грабля:** baseline Q84/Q85/D87/D88 = коммиты T-15 r9/r10 (`af53e23`/`2958bf5`)
  — `git diff` покажет ровно правки адресов; карточка T-25 untracked (`??`),
  `git diff` по ней пуст — сверять чтением.

- 02.10.2026, T-16 базовый P1 (участок `t16_baseline_p1_lifecycle`, rework
  адресной проверки `validator`): `docs/TRACEABILITY.md:65` (Q61/D65) lifecycle
  `done` → `in work` — открытая T-25 ⬜ не допускается при `done`
  (`traceability_lifecycle_matches_task_openness`, `tests/docs_journal.rs`; D82
  §Следствия:55-56; прецедент Q12/D54 при T-16 🚧). Ячейка `[T-25] ⬜` сохранена;
  файловые статусы Q61/D65 не менялись (D82 §4). Проверки: `git diff` по
  TRACEABILITY — ровно строка Q61/D65 (одна ячейка lifecycle), `--check` пусто,
  `cargo` не запускался (D50). Файлы: только `docs/TRACEABILITY.md` (+ лента/память).
  Новых Q/D/F нет. Следующее: `validator` адресно `cargo test --test docs_journal`
  (ожидается 14/0) → `-r2` приёмка T-16.
  **Грабля:** `git diff` по TRACEABILITY в этой незакоммиченной ветке тянет и
  правку T-16-ячейки (⬜→🚧, ранний сегмент) — судить о моей правке по локализации
  (строка Q61).

- 02.10.2026, T-16 closeout (приёмка `validator` accepted -r3): `docs/TRACEABILITY.md:16`
  (Q12/D54) ячейка `[T-16] 🚧` → `✅` — в равенство с реестром
  (`tasks/README.md:57` T-16 ✅, docs-writer). Строка Q12 осталась `in work`
  (T-08 ⬜), T-08 ⬜ / T-01 ✅ не тронуты. H7 (`findings-registry.md:25`)
  закрыт: «открыт → задача T-16 заведена» → «закрыт приёмкой T-16 (02.10.2026,
  accepted -r3; `cargo test --all` 145/0)», источник скорректирован (коррекция:
  приёмка T-16, не удалять связи), связь `T-16 (Q12, test_draft.feature)`
  сохранена. Q61/D65 и T-25 **не трогал**. Проверки: `git diff --check` пусто;
  `--unified=0` по TRACEABILITY — :16 (моя) + :65 (Q61/D65, предсуществующая
  правка участка `t16_baseline_p1_lifecycle`, не моя), `--numstat` 2/2 (не 1/1
  — baseline не чист). `cargo` не запускался (D50; адресно — `validator`).
  Файлы: `docs/TRACEABILITY.md`, `findings-registry.md` (+ лента/память).
  Новых Q/D/F нет. Следующее: `surface_to_user` (пакет) → `git branch_end` →
  `complete`. **Грабли:** в несохранённой ветке baseline TRACEABILITY уже грязен
  (Q61/D65), поэтому numstat больше моих правок — судить по локализации. Q61 не
  откатывать (запрет + нужна `docs_journal`); пакеты T-16/S-P1(T-25) раздельны
  (F43).

## Заметки

- `git status --porcelain` в этой задаче показывает и рабочие пути других
  ролей (state, почта, память) — их не трогать; `docs/analysis/T-20-…` уже был
  untracked (аналитик), не мой.

- 02.10.2026, T-25 closeout (совмещённый пакет T-16+T-25, mixed worktree,
  HEAD @ 2958bf5): `docs/TRACEABILITY.md:65` (Q61/D65) — ячейка задач
  `[T-25] ⬜`→`✅` и lifecycle `in work`→`done` (единственная задача линии ✅,
  открытых нет; в отличие от Q12/D54, где `in work` из-за T-08 ⬜). Сверено
  чтением: реестр `tasks/README.md:58` = ✅, карточка `T-25-…/README.md:3` =
  «✅ сделана». Проверки: `git diff --check` пусто; `-U0` по TRACEABILITY —
  :65 (моя) + :16 (T-16-closeout, предсуществующая, не тронута); `--numstat`
  2/2 (не 1/1 — baseline несёт T-16). `cargo` не запускался (D50; адресный
  `docs_journal` — `validator`, ориентир 14/0). Файл только TRACEABILITY
  (+лента/память). Новых Q/D нет. Следующее: гейт пакета `surface_to_user` →
  `git branch_end` (hunk-разбор TRACEABILITY: :16 T-16 / :65 T-25).
  **Грабля:** в этой ветке baseline TRACEABILITY уже содержит правку :16
  (T-16) — numstat > моей правки; судить по локализации.

- 03.10.2026, находки прогона T-16/T-25 (разбор
  `docs/analysis/T-16-t25-run-2026-10-02-c9c12-check.md`, сессии `ses_f0208b20…`
  T-16 / `ses_f01ccf60…` T-25): **F73** «открыт → задача T-25» → «закрыт
  02.10.2026: T-25 — адреса сняты, `docs_journal` зелёный (14/0); системный
  остаток — F74»; **F26/F27/F58** дополнены третьим набором данных (статусы
  остались открытыми); добавлены **F74–F79**. Проверки: `git diff --check`
  пусто; ID F1–F79 уникальны (F80 свободен); ссылки/связи живые (F9/F43/F57/
  F58/F61/F73/F76, T-15/T-16/T-25, D88, источник-разбор); `cargo` не
  запускался (D50). Новых Q/D **нет** (указание брифа). Файлы: только
  `docs/analysis/findings-registry.md` (+ лента r10/память). Следующее:
  `validator` — приёмка реестра (адресная, без cargo). **Грабля:** F-строки
  намеренно сохраняют формы `docs/analysis/…` и `../decisions/…` — источники
  находок/связи (исторические метки, D48/D65); не путать с адресами канона,
  которые гейтит `tests/docs_journal.rs`.

- 03.10.2026, Q86 → D89 (service-mcp-ready-r11, T-15 C3/C15/C16): заведены
  `Q86` («P1/P2/P3: топология веток дочерних задач, заморозка решений,
  session-commit») и `D89-branch-topology-freeze-session-commit` (3 пункта:
  P1 ветка дочерней от родителя + права `analyst`; P2 заморозка
  `awaiting_user`/`deferred_by_owner`; P3 session-commit — первый инкремент C3).
  Строки `questions/README.md`, `decisions/README.md`, TRACEABILITY (Q86: D89,
  `in work` / T-15 🚧); карточка T-15 — `C3` ⬜→🚧 (остаток `process/runN`/теги/
  фасад — отдельным решением) + `C15`/`C16` 🚧; реестр — **F80–F82**.
  Сверка D89 — ⚪ (процесс/документы; `cargo` не запускался, D50). Проверки:
  ID свободны (Q85/D88, F79 — последние); ссылки живые (глоб/чтение;
  T-25-слаг исправлен на `T-25-d65-analysis-addresses`);
  `git diff --check`/адресный `docs_journal` — вне прав `migrator` (shell
  denied), за `validator`. **Грабля:** `removable_addresses` гейтит и
  `docs/analysis/<файл>`, и относительные `../analysis/<файл>` внутри
  journal-файлов — упоминание разбора давал голым именем (без зоны), ссылки на
  реестр — через whitelisted `findings-registry.md`. Файлы: Q86, D89, 2
  каталога, TRACEABILITY, карточка T-15, findings (+ лента r11). Новых задач
  нет. Следующее: правки канона сервисной сессией → `reload` → `auditor` →
  `validator`.

- 03.10.2026, закрытие r11 (T-15, приёмка `validator` accepted
  `service-p1p2p3`; отчёт `docs/reviews/service-p1p2p3-2026-10-03.md`; адресный
  `docs_journal` 14/0): карточка T-15 — `C15`/`C16` 🚧→✅ 03.10 (`service-p1p2p3`;
  P1/P2, D89); `C3` остаётся 🚧 с пометкой «инкремент 1 принят
  (`service-p1p2p3`): правило session-commit; остаток — `process/runN`, теги,
  фасад». Реестр — F74 дополнен пометкой «первое применение 03.10.2026 (r11:
  адресный `docs_journal` 14/0 по решению владельца); закрепление правила в
  исключении D50 — отдельным решением», статус «открыт». Q86/TRACEABILITY **не
  трогал** (указание: T-15 🚧, Q86 `in work`/D82). Проверки: `git diff --check`
  пусто; ссылка review резолвится (glob); `cargo` не запускался (D50). Новых
  Q/D/задач нет. Файлы: карточка T-15, findings (+ лента/память). **Грабля:**
  `--numstat` findings 13/4 > моей правки — baseline несёт незакоммиченный
  r11-пакет (F73 update, F75–F82); судить по локализации (`-U0`: правка — только
  F74-строка). F-строки сохраняют форму `docs/analysis/…` как источник (не
  адрес канона, гейт допускает). Следующее: гейт пакета → `git` (develop) →
  `complete`.

- 03.10.2026, Q87 → D90 (service-mcp-ready-r12, T-15 C5+C7): заведены `Q87`
  («C5/C7: re-raise (объект/категории, `replan_reason`) и шаблон самоотчёта +
  статусы фич») и `D90-c5-c7-re-raise-selfreport` (2 пункта: C5 — объект
  `re_raise` в `next_action`: `id/category/origin/failed_clause/fix/blocking/
  resolved`, категории `expect_mismatch`·`owner_override`·`plan_gap`·
  `role_failure`, зеркало `replan_reason` в `progress`, `owner_override` вне
  метрики; C7 — короткий сегментный самоотчёт + статусы фич `agents-re-raise`
  ⬜→✅, `agents-state-schema` ⬜→🟡, `agents-session-checkpoint` ⬜→🟡,
  счётчики без изменений). Строки `questions/README.md`, `decisions/README.md`,
  TRACEABILITY (Q87: D90, `in work` / T-15 🚧); карточка T-15 — `C5`/`C7` ⬜→🚧
  (D90). Сверка D90 — ⚪ (процесс/документы; `cargo` не запускался, D50).
  Проверки: ID свободны (Q86/D89 — последние), `git diff --check` пусто
  (warning CRLF по r11-ленте — не мой файл), ссылки Q87/D90 + каталогов
  резолвятся (grep/glob; канон-адрес §5.5/§7 — в форме
  `mcp-ready-process.md`). Новых находок нет. **Грабля:** D90 — untracked,
  `git diff`/`--check` по нему пусты по baseline; сверка чтением. Файлы: Q87,
  D90, 2 каталога, TRACEABILITY, карточка T-15 (+ лента r12/память).
  Следующее: правки канона сервисной сессией
  (`.opencode/rules/{dispatch-loop,state-schema}.md`,
  `.opencode/agents/{analyst,lead}.md`) + статусы фич `docs-writer` →
  `reload` → `auditor` → `validator` (адресная).

- 03.10.2026, фикс P1 приёмки r12 (T-15, C5+C7; квитанция `rework` iteration 1,
  отчёт `docs/reviews/service-c5c7-2026-10-03.md`): `docs/TRACEABILITY.md:91`
  (Q87, «Реализация») — вилдкард «`(статусы `agents-*`)`» запрещён тестом
  `features_are_named_in_traceability_and_exist` (D80) → заменён поимённым
  перечнем (стиль Q74/D78): `agents-re-raise.feature`,
  `agents-state-schema.feature`, `agents-session-checkpoint.feature`. Иных
  строк/файлов не тронуто. Проверки: `rg "agents-\*" docs/TRACEABILITY.md` —
  пусто; `git diff --numstat -- docs/TRACEABILITY.md` = `1 0` (ровно одна
  строка), `--check` пусто; три `.feature` существуют (glob). `cargo` не
  запускался (D50; адресный `docs_journal` — `validator`, ожидание 14/0).
  Новых находок нет. **Грабля:** F74-практика (адресный `docs_journal` на
  приёмке) поймала вилдкард до develop. Следующее: `validator` `-r2` →
  гейт → `git` (develop) → `complete`.

- 03.10.2026, закрытие r12 (T-15, приёмка `validator` accepted `service-c5c7`
  iteration 2; отчёт `docs/reviews/service-c5c7-2026-10-03-r2.md`; адресный
  `docs_journal` 14/0): карточка T-15 — `C5`/`C7` → «✅ 03.10 — принято
  (`service-c5c7`, `-r2`); D90» (`:216,:218`); `docs/TRACEABILITY.md:91` (Q87)
  **не тронута** — `in work` / `[T-15] 🚧` (T-15 🚧, `done` не допускает
  открытых задач — D82, прецеденты Q83/Q85/Q86); реестр — F61 (строка 73)
  дополнена «03.10.2026: короткий шаблон самоотчёта внесён (C7/D90);
  EN-префикс обёртки харнесса — остаток (F78)», статус «открыт»; иные находки не
  тронуты. Проверки: `git diff -U0` — карточка ровно 2 строки, findings ровно 1;
  `git diff --numstat -- docs/TRACEABILITY.md` = `1 0` (baseline untracked
  r12-пакета; строка Q87 прочитана — совпадает); `git diff --check` — только
  CRLF-предупреждение r11-ленты; `cargo` не запускался (D50). Новых Q/D/задач
  нет. Файлы: карточка T-15, findings-registry (+ лента r12/память). **Грабля:**
  baseline карточки был `⬜` (🚧 не закоммичен) — правка сразу на ✅.
  Следующее: гейт пакета → `git` (develop) → `complete`.

- 03.10.2026, Q89 → D92 (service-mcp-ready-r14, T-15 C17): заведены `Q89`
  («Что считать зачётным прогоном (чистота для F26/F27/F15)») и
  `D92-credited-run-predicate` (ядро-предикат: ведущий `lead`; нет упоров
  `steps`; нет незапланированных прерываний владельца (каждый `owner_response`
  — гейт плана `surface_to_user`); нет ручных восстановлений (пустые финалы/
  backfill/ручная реконструкция); не-дефекты: плановые гейты, `owner_override`,
  обязательный rework `validator` `-rN`, CCSN-шаг, session-commit; проверка
  машинная `progress.yaml` + экспорт/`metrics-report.mjs`; **не ретроактивно**).
  `Спека` — `—`; `Affects` — `.opencode/rules/state-schema.md` (раздел
  «Зачётный прогон»); `Tasks` — T-15 (`C17`; B1-F26/F27/F15). Строки
  `questions/README.md`, `decisions/README.md`, TRACEABILITY (Q89: D92,
  `in work` / T-15 🚧); карточка T-15 — строка `C17` 🚧 + в B1-F26/F27/F15
  «зачёт — по §„Зачётный прогон“ (`state-schema.md`, D92)». Реестр — F83
  (`.credo/**` в git вопреки `AGENTS.md`/`.gitignore`), F84 (live state:
  неизвестные схеме `blocker`/`route_done`/`re_raise`, мягкий режим) — открыты.
  Сверка D92 — ⚪ (процесс/документы; `cargo` не запускался, D50). Проверки: ID
  свободны (Q88/D91 — последние), ссылки резолвятся (D58/D86/D89/D91, T-15,
  findings). **Грабля:** shell у migrator нет — `git diff --check` делает `git`
  (в r14 команда отклонена, `permission.rejected`); не пытаться.
  Файлы: Q89, D92, 2 каталога, TRACEABILITY, карточка T-15, findings (+ лента
  r14/память). Следующее: правки канона сервисной сессией
  (`.opencode/rules/state-schema.md`) → `reload` → `auditor` → `validator`.

- 03.10.2026, фикс P3 аудита r14 в D92 (service-mcp-ready-r14, T-15 C17;
  D92 ещё не закоммичен — правка в пакете): (1) п.1.3 ядра — устранена
  тавтология: «соответствует `surface_to_user`, **стоявшему в плане участка
  (`next[]`)**; ответов вне плана и „продолжай“ нет» (было «соответствует
  гейту плана `surface_to_user`»); (2) п.3 — путь разнесён: «экспорт сессии
  (`.opencode/scripts/session-analysis/`) и
  `.opencode/scripts/metrics-report.mjs`» (было `metrics-report.mjs` без
  пути). Существо решения/не-дефекты/неретроактивность не тронуты; иных
  строк нет. Проверки: чтением — diff локализован (`D92:33-35`, `:41-43`);
  `cargo` не запускался (D50). Новых Q/D/находок нет. Файлы: D92 (+ лента
  r14/память). Следующее: `validator` (адресная + `docs_journal`) →
  гейт → `git` (develop).

- 03.10.2026, закрытие r14 (T-15, приёмка `validator` accepted
  `service-credited-run` iteration 1; отчёт
  `docs/reviews/service-credited-run-2026-10-03.md`; адресный `docs_journal`
  14/0): карточка T-15 — `C17` (`:230`) 🚧 → «✅ 03.10 — принято
  (`service-credited-run`); D92». **`docs/TRACEABILITY.md:93` НЕ тронут** —
  Q89 `in work` / `[T-15] 🚧` (T-15 🚧; `done` не допускает открытых задач —
  D82, прецеденты Q83/Q85–Q88). F83/F84 не тронуты
  (`findings-registry.md:95-96`, открыты). Проверки: чтением — diff
  локализован (ровно 1 строка `:230`); ссылки живые (D92, review,
  TRACEABILITY `:93`); `cargo` не запускался (D50; адресный `docs_journal` —
  за `validator`, 14/0). **Грабля:** (повтор) shell у migrator нет — `git
  diff --check`/`numstat` за ролью `git`; сверка чтением. Файлы: карточка
  T-15 (+ лента r14/память). Новых Q/D/задач нет. Следующее: гейт пакета →
  `git` (develop) → `complete`.

- 03.10.2026, Q90 → D93 (service-mcp-ready-r15, T-15 C3-остаток + C4): заведены
  `Q90` («C3-остаток: process-ветка, теги, снапшот, фасад `/git/checkpoint`»;
  варианты A/B/C; решение владельца 03.10.2026 — **A + идентификатор задачи**,
  снапшот — **скрипт по вызову фасада**) и
  `D93-session-commit-process-branch` (10 пунктов: прогон=`T-XX`/сессия `sM`;
  ветка сессии `process/<прогон>-s<M>` от текущего HEAD (`switch -c`, без
  конфликтов), накопительная ветка отклонена; состав = лента+память+снапшот;
  тег `session/<прогон>-s<M>`; сообщение `chore(process): <прогон> s<M>`; push
  ветки+тега; возврат; в `develop` не мержится; live state — финальным
  process-коммитом задачи; снапшот `.opencode/state/snapshots/<прогон>-s<M>/`
  скриптом `--snapshot` (`task`/`session_index` из `current_state.yaml`),
  право у `git`; «постоянный» пакет без изменений D89 P3; сервисные операции —
  без process-ветки; фасад `/git/checkpoint` без shell-блоков, «нет записи —
  нет чекпойнта», идемпотентно; `/git/status` + теги сессий,
  `.opencode/commands/**` — в аудит `auditor`; фича — сценарий 1 к принятому
  имени, счётчик 4; catch-up как D89 P3). Строки `questions/README.md`,
  `decisions/README.md`, TRACEABILITY (Q90: D93, `in work` / T-15 🚧). Сверка
  D93 — ⚪ (процесс/служебная зона; `cargo` не запускался, D50). Проверки: ID
  свободны (Q89/D92 — последние); ссылки живые; прочитаны `docs_journal.rs`
  (тесты 1–8: контигуальность ID, парность, каталоги, «Сверка», lifecycle,
  задачи, фичи, D65) — роут-совместимы. Карточка T-15 и C-секция/строки
  `C3`/`C4` **не тронуты** (🚧; закрытие — после приёмки; синхронизация текста
  C-секции — при закрытии). **Грабля:** `.opencode/state/...` в D-файле — под
  `removable_addresses`; безопасно, т.к. имя с `<прогон>` даёт placeholder, а
  `current/*`/`.../` — не «файл с точкой»; держать `<...>` в путях снапшота.
  Файлы: Q90, D93, 2 каталога, TRACEABILITY (+ лента r15/память). Следующее:
  правки канона сервисной сессией → `reload` → `auditor` → `validator`.

- 03.10.2026, Q91 → D94 (service-mcp-ready-r16, T-15 C11 — B2 профиль-флаг):
  заведены `Q91` («C11/B2: профиль-флаг — подготовка без включения»; варианты
  A/B/C; решение владельца 03.10.2026 — **A**: константа в файле `B2_PROFILE`,
  тест — служебный скрипт `.opencode/scripts/token-guard-test.mjs`) и
  `D94-b2-profile-flag` (7 пунктов: профиль-константа `off`/`codemode`/`mcp`;
  таблицы `B2_PREFIXES_CODEMODE`/`_MCP` из черновых правил плагина; `off` —
  no-op; включение — смена константы, B1-срез не меняется; тест на синтетике;
  C11 не держит closeout T-15; фаза E — отдельная задача; карточка C11 →
  «🟡 подготовлен»). D45 п.3 — обратная пометка «уточнено D94». Строки
  `questions/README.md`, `decisions/README.md`, TRACEABILITY (Q91: D94,
  `in work` / T-15 🚧); карточка T-15 строка `C11` `:226` 🟡 (фазу E `:161-164`
  не трогал). Сверка D94 — ⚪ (процесс/служебная зона; `cargo` не запускался,
  D50). Проверки: ID свободны (Q90/D93 — последние); ссылки живые (Q91/D94,
  D45, F28, T-15, `token-guard.ts`, `AGENTS.md`, `journal.md`, D50,
  `findings-registry.md`); читал `tests/docs_journal.rs` (каталоги, «Сверка»,
  `Resolves`, T-15 🚧 ↔ TRACEABILITY 🚧) — роут-совместимы. **Грабля:** shell у
  migrator нет — `git diff --check`/`numstat` за ролью `git`; диф судить
  чтением. `token-guard-test.mjs` в D94 — plain text (файл новый, ещё не
  создан; ссылка была бы битой). Файлы: Q91, D94, D45, 2 каталога,
  TRACEABILITY, карточка T-15 (+ лента r16/память). Новых задач нет (C11 уже в
  T-15). Следующее: правки служебной зоны сервисной сессией (плагин/тест/
  `AGENTS.md`) → `auditor` → `validator`.

- 03.10.2026, закрытие r15 (T-15, приёмка `validator` accepted
  `service-process-branch` iteration 1; отчёт
  `docs/reviews/service-process-branch-2026-10-03.md`; `docs_journal` 14/0):
  актуализирована **только карточка** `docs/tasks/T-15-mcp-ready-process/README.md`
  — строки `C3`/`C4` (`:218-219`) ✅ с принятым именованием (ветка сессии
  `process/<прогон>-s<M>`, тег `session/<прогон>-s<M>`, снапшот, скрипт;
  фасады `/git/checkpoint`/`/git/status`; `.opencode/commands/**` в аудите
  `auditor`); C-секция `:108-115` (session-commit + фасады) синхронизирована;
  черновик фазы A `:263-266` снят (Q41) → ссылка на действующий файл;
  примечание `/git/status` `:284-285` актуализировано. **Доп.:** заголовок
  строки `C3` тоже нёс `process/runN` — приведён к принятому имени (иначе
  проверка `rg` не пуста). `docs/TRACEABILITY.md` **НЕ тронут** — Q90
  `in work` / `[T-15] 🚧` (D82; прецеденты Q83/Q85–Q88; бриф п.7; «Q90 → done»
  из секции `validator` — только на `complete` T-15, не в r15). Проверки:
  `rg "process/runN|session/runN|runN"` — пусто; ссылки D93 живы; diff
  локализован (`:108-115`, `:218-219`, `:263-266`, `:284-285`); `cargo` не
  запускался (D50; адресный `docs_journal` — за `validator`, 14/0).
  **Грабля:** shell у migrator нет — `git diff --check` за ролью `git`; диф
  судить чтением/локализацией. Новых Q/D/задач нет. Файлы: карточка T-15
  (+ лента r15/память). Следующее: гейт пакета → `git` (develop) → `complete`
  (там: Q90 → done, T-15 → ✅, строка фичи 🟡→✅).

- 03.10.2026, закрытие r16 (T-15, C11 — приёмка `validator` accepted
  `service-b2-profile` iteration 1; отчёт
  `docs/reviews/service-b2-profile-2026-10-03.md`; `docs_journal` 14/0):
  актуализирована **только карточка** `docs/tasks/T-15-mcp-ready-process/README.md`
  — строка `C11` (`:226`): 🟡 → **🟢 подготовлен и принят (`service-b2-profile`)**;
  факт приёмки — профиль-флаг `B2_PROFILE` (`off`/`codemode`/`mcp`), тест
  профилей, включение — `codemode:false`, closeout не держит (D94). **Не ✅** —
  живой проверки в MCP-канале не было (осознанное решение; прецедент — 🟢 как
  «подготовлен и принят»). `docs/TRACEABILITY.md` **НЕ тронут** — Q91 `in work` /
  `[T-15] 🚧` (бриф; прецеденты Q83/Q85–Q90; «Q91 → done» — только на `complete`
  T-15). Проверки: `git diff --stat` локализован (карточка — 1 файл от
  `migrator`; прочее — r16-правки сервисной сессии); чтением — `:226` несёт 🟢;
  `cargo` не запускался (D50; адресный `docs_journal` 14/0 — за `validator`).
  Новых Q/D/задач нет. Файлы: карточка T-15 (+ лента r16/память). Следующее:
  гейт пакета → `git` (develop) → `complete`.
- 03.10.2026, Q92 → D95 (service-mcp-ready-r17, фаза E — открытие): решение
  владельца — фаза E как **отдельная задача** `T-26-mcp-server-design` (⬜, P1;
  зависит от заморозки — фаза D T-15); предмет — **только process-MCP** (6–10
  инструментов: storage + validation + query; решений не принимает); git-MCP
  (Прил. A) — вне предмета. Создано: `docs/questions/Q92.md`,
  `docs/decisions/D95-phase-e-mcp-design.md`,
  `docs/tasks/T-26-mcp-server-design/README.md`; реестр `docs/tasks/README.md:59`;
  карточка T-15 — фаза E ссылается на `T-26` (`:165`); каталоги Q/D;
  `docs/TRACEABILITY.md:96` (Q92: D95, `in work`, `T-26` ⬜). Фича
  `agents-mcp-readiness` не тронута (правка заголовка отклонена правами
  `migrator`; требования не менялись — D95 поведение не меняет; имя `T-26` в
  фиче не требуется). Грабля: упёрся в лимит шагов (18) — отчёт в ленту и
  чекпойнт дозаписаны сервисной сессией; аудит P2 (чекпойнт отсутствовал) закрыт
  этой записью. Следующее: `validator` → гейт → `git`.
