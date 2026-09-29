# Память: docs-writer (документация и статусы)

- **Канон:** `docs/tasks/README.md`, `docs/features/README.md`,
  `docs/BRIEF.md` §9.
- **Правило:** чекпойнт — какие файлы/статусы менялись, что осталось, ссылки.
  Кратко.

## Чекпойнты

- 2026-09-28 · операция `service-t11-closeout` · закрытие T-11: статус ✅ в
  карточке `docs/tasks/T-11-agent-cycle/README.md` и сводке `docs/tasks/README.md`;
  «Примечания» с уликами (r3 `reviews/T-11-2026-09-26-r3.md`, Run 4/T-03 `-r2`
  `reviews/T-03-2026-09-27*.md`, меморандумы W8 т.1/т.2, коммиты
  `0a5832f`/`5019c45`); `docs/features/README.md` — уточнены `agents-rework`
  (🟡, остаток D42 → T-15 B1-F15) и блок «Пилоты состоялись»;
  `docs/tasks/T-15-mcp-ready-process/README.md` — зависимость снята, H7 → T-16;
  `docs/CHANGELOG.md` — запись «Закрытие T-11 и задача T-16». Счётчики сценариев
  не менялись. Сводка T-15 синхронизирована — зависимость снята. P3 приёмки
  T-11-closeout закрыт: легенда блока «Пилоты состоялись» в
  `docs/features/README.md` приведена в соответствие со строкой `agents-rework`
  (🟡 — структурная готовность, подтверждено не полностью).
  Остаток: приёмка `validator` и пакет `git`.
- 2026-09-28 · операция `service-migration-q2q3` · сопутствующие документы к
  переносу Q2+Q3 → D16: `docs/features/README.md` :69 — ссылки `[Q3]`/`[D16]`
  в ноте Q3 (счётчики 47/278 не менялись); обратные ссылки `# D16 (Q2, Q3)` в
  шапках `parser.feature`/`lexer.feature`/`execution.feature`; `docs/CHANGELOG.md`
  :19–24 — запись «Перенос Q2, Q3 → D16». Ноты Q4/Q36/Q38 не трогал. Остаток:
  приёмка `validator` (docs) и пакет `git`.
- 2026-09-29 · операция `service-migration-q4` · сопутствующие к переносу Q4 → D17:
  `docs/features/README.md` :75–79 — ссылки `[Q4]`/`[D17]`, список фич уточнён
  (убран `explain.feature`, остались `parser`/`execution`/`editor`/`lsp`/
  `client_explanation`), счётчики 47/278 не менялись; обратные ссылки
  `# D17 (Q4)` в шапках `parser.feature`, `execution.feature`, `editor.feature`,
  `lsp.feature`; `client_explanation.feature` :8 `(Q4/Q36)` → `(D17/Q36)`;
  `docs/CHANGELOG.md` :25–28 — запись «Перенос Q4 → D17». Журнал/реестр/SPEC/
  tasks не трогал. Остаток: приёмка `validator` (docs) и пакет `git`.
- 2026-09-29 · операция `service-migration-q5q6` · сопутствующие к переносу
  Q5, Q6 → D19: `docs/features/README.md` :12–14 — в ноте «Решения Q5/Q40»
  обратная ссылка `[Q5]`→`[D19]`; `docs/CHANGELOG.md` §«Документация» :29–37 —
  запись «Перенос Q5, Q6 → D19». Шапки фич не трогал: D19 — канон статусов и
  приоритетов целиком, отдельных фич не адресует (в отличие от Q4 → D17).
  Счётчики 47/278 не менялись; журнал/реестр/SPEC/tasks не трогал.
  Остаток: приёмка `validator` (docs) и пакет `git`.
- 2026-09-29 · операция `service-dod-scope` · синхронизация `docs/BRIEF.md` §5.7
  (пункт DoD, строки 250–256): добавлена оговорка по составу пакета — без
  изменений `src/**`/`tests/**` cargo-прогоны не выполняются (адресная проверка;
  `review.md` §«Порог существенности», `D50`), исключение — правки
  счётчиков/состава сценариев `docs/features/**` → адресный
  `cargo test --test features_inventory`. Основание — Q55/D50. Другие файлы
  (канон, журнал, SPEC) не трогал; `cargo` не запускался. Остаток: `auditor`
  (аудит) → приёмка `validator` (docs) → пакет `git`.
- 2026-09-29 · операция `service-dod-scope` (продолжение) · закрытие P3 аудита:
  в `docs/BRIEF.md` §5.7 перечень состава пакета расширен до `src/**`,
  `tests/**`, `Cargo.toml` (строки 253–256) — синхронно с `review.md:35–38`.
  Прочие файлы не трогал; `cargo`/git не запускались. Остаток: приёмка
  `validator` (docs) → пакет `git`.
- 2026-09-29 · операция `service-agent-tools` · закрытие P2-2 аудита
  (D51, экономия токенов): `docs/features/agents-cycle.feature:51` — команда
  `opencode debug agents` → `node .opencode/scripts/agents-perms.mjs`;
  `docs/features/agents-audit.feature:17` — то же в оговорке «первый прогон».
  Сценарии/структура и `docs/features/README.md` не тронуты: счётчики 47/278
  (276 `Сценарий:` + 2 «Структура сценария:»); `rg` — сырых упоминаний в фичах
  нет, путь скрипта существует. `cargo`/git не запускались. Остаток: P3
  (журнал/SPEC — не зона) → аудит → приёмка `validator` → пакет `git`.
- 2026-09-29 · операция `service-migration-q7` · сопутствующие к переносу Q7 → D52:
  `docs/features/publish.feature:7-11` — в шапочной пометке Q7 стал ссылкой
  (`[Q7](../questions/Q7.md) → [D52](../decisions/D52-glossary-terms-canon.md)`),
  Q13 оставлен текстом, дата 2026-09-26 сохранена; `docs/CHANGELOG.md` :38–46 —
  запись «Перенос Q7 → D52» (14 новых статей + уточнение `Черновик`/`Публикация`;
  сверка ⚪, расхождение `meta.json` покрыто T-07). Счётчики 47/278 не менялись;
  журнал/реестр/SPEC/tasks не трогал. Остаток: приёмка `validator` (docs) → пакет `git`.
- 2026-09-29 · операция `service-migration-q8q10q42` · сопутствующие к переносу
  Q8–Q10, Q42 → D21: обратные ссылки `# D21 (Q8–Q10, Q42): …` в шапках пяти фич
  `Affects` — `errors.feature:2-4`, `execution.feature:4-6` (после D16/D17),
  `explain.feature:2-4`, `explain_full.feature:2-4`, `test_draft.feature:7-9`;
  `docs/CHANGELOG.md:47-62` — запись «Перенос Q8–Q10, Q42 → D21» (строгие
  ошибки; словарь решений; схема объяснения; сверка ✅, задач не требуется).
  Нюанс: `errors.feature` сценарий типов на парсере (v0.2, 🟡) не трогал.
  Счётчики 47/278 не менялись; журнал/реестр/SPEC/tasks/analysis не трогал.
  Остаток: приёмка `validator` (docs) → пакет `git`.
- 2026-09-29 · операция `service-migration-q11` · сопутствующие к переносу Q11 → D53:
  обратные ссылки `# D53 (Q11): человекочитаемое — русский; машиночитаемое
  (коды, ключи, идентификаторы) — латиница` в шапках четырёх фич `Affects` —
  `evaluate.feature:9`, `rest_api.feature:10`, `rest_auth.feature:8`,
  `errors.feature:5` (после Q-блоков, формат BRIEF §5.6 п.2);
  `docs/CHANGELOG.md:63-76` — запись «Перенос Q11 → D53» (принцип «машина/человек»;
  канонические тексты REST 404/410/409/401; сверка ✅, задач не требуется,
  `version_deprecated` — резерв T-09/Q33). Нюанс: D53 «Следствия» называет три
  REST-фичи, а `errors.feature` — по «Affects» через ядро (Q8/Q9); ссылку добавил
  по BRIEF §5.2 п.6 и прецеденту D21 — снимается одной правкой при ином чтении.
  Счётчики 47/278 не менялись; журнал/реестр/SPEC/tasks не трогал; `cargo`/git не
  запускались. Остаток: приёмка `validator` (docs) → пакет `git`.
- 2026-09-29 · операция `service-migration-q12q15` · сопутствующие к переносу
  Q12–Q15 → D54/D14/D55/D56: обратные ссылки `# Dn (Qx): …` (BRIEF §5.6 п.2)
  в шапках 13 фич `Affects` — D54 (Q12): `draft.feature:10`,
  `test_draft.feature:10`, `publish.feature:13`, `notebook_ui.feature:10`,
  `editor.feature:3`, `file_management.feature:4`; D14 (Q13):
  `publish.feature:12`, `storage_paths.feature:8`, `publish_rules.feature:2`,
  `immutability.feature:6`, `mcp_tools.feature:7`; D55 (Q14):
  `publish.feature:14`, `publish_rules.feature:3`; D56 (Q15):
  `git_integration.feature:7`, `deferred.feature:2`, `publish_rules.feature:4`
  (в `publish.feature`/`publish_rules.feature` — по три строки, каждый D своей
  строкой). `features/README.md:145-149` — блок «Решение Q14» (ветка
  `publish/{name}-{version}`; `checks/...` — путь артефакта, не имя ветки).
  `docs/CHANGELOG.md:77-104` — запись «Перенос Q12–Q15 → D54, D14, D55, D56».
  Счётчики 47/278 не менялись (правки фич — только комментарии, 0 удалений);
  `cargo`/git не запускались (D50). Остаток: сплошной обход ссылок не делался
  (только точечный) → приёмка `validator` (docs) → пакет `git`.
- 2026-09-29 · операция `service-decisions-readme` · сопутствующие к сводке
  `decisions/README.md` (`migrator`, 29 решений D14–D58; не канон):
  `docs/README.md` :18–19 — в карте у `questions/` и `decisions/` указаны
  `README.md` (сводки), строка `decisions/` переформулирована в `Dn-….md +
  README.md (сводка решений)`; :48 — строка «Быстро обозреть принятые решения»
  → `decisions/README.md` (сводка, не канон); `docs/CHANGELOG.md` :131–136 —
  запись «Сводка решений `decisions/README.md`». Счётчики 47/278 не менялись
  (`features/README.md:309`); журнал/`TRACEABILITY`/`SPECIFICATION`/`tasks`/
  `BRIEF.md` не трогал; `cargo`/git не запускались (D50). Остаток: приёмка
  `validator` (docs) → пакет `git`.
- 2026-09-29 · операция `service-migration-q16q19` · сопутствующие к переносу
  Q16–Q19 → D32/D57/D35/D58: обратные ссылки `# Dn (Qx): …` (BRIEF §5.6 п.2) в
  шапках 7 фич `Affects` — D32 (Q16): `publish.feature:15`,
  `test_draft.feature:11`; D57 (Q17): `immutability.feature:7`; D35 (Q18):
  `semver.feature:7`, `deferred.feature:3`, `storage_paths.feature:9`; D58
  (Q19): `notebook_ui.feature:11`. `features/README.md:136` — канон `[D57]` в
  ноте Q17; `:143-144` — покрытие `[T-06]` (статус матрицы иммутабельности 🟡
  согласован). `docs/CHANGELOG.md:105-130` — запись «Перенос Q16–Q19 → D32,
  D57, D35, D58». Счётчики 47/278 не менялись; `cargo`/git не запускались
  (D50). Остаток: приёмка `validator` (docs) → пакет `git`.
- 2026-09-29 · операция `service-migration-q20q23` · сопутствующие к переносу
  Q20–Q23 → D22/D23/D26/D25: обратные ссылки `# Dn (Qx): …` (BRIEF §5.6 п.2) в
  шапках 8 фич `Affects` — D22 (Q20): `rest_api.feature:11`,
  `evaluate.feature:10`, `batch.feature:8`, `import_export.feature:6`;
  D23 (Q21): `rest_api.feature:12`, `evaluate.feature:11`,
  `manifest.feature:2`, `dashboard.feature:5`; D26 (Q22):
  `evaluate.feature:12`, `rest_auth.feature:9`; D25 (Q23):
  `rest_api.feature:13`, `evaluate.feature:13`, `rest_auth.feature:10`,
  `errors.feature:6` (в `rest_api`/`evaluate` — по три строки, каждый D своей;
  порядок по Q, после существующей D53). `docs/CHANGELOG.md:137-160` — запись
  «Перенос Q20–Q23 → D22, D23, D26, D25» (канон путей, схема `GET /checks`,
  `x-api-key`, конверт ошибок; задачи «не требуется»). Счётчики 47/278 не
  менялись (`features/README.md:309`, сверено чтением); numstat фич 14/0.
  Нюанс: прозаичные Q-строки в шапках (`# Q20 (решено …)` в `rest_api`/
  `evaluate`/`batch`/`import_export`, `# Q21: …` в `evaluate`) не удалял — по
  инструкции только добавление D-строк. `cargo`/git не запускались (D50);
  журнал/`TRACEABILITY`/`SPECIFICATION`/`tasks`/`decisions/README.md` не трогал.
  Остаток: приёмка `validator` (docs) → пакет `git`.
- 2026-09-29 · операция `service-migration-q24q26` · сопутствующие к переносу
  Q24–Q26 → D36/D37/D24: обратные ссылки `# Dn (Qx): …` (BRIEF §5.6 п.2) в
  шапках трёх фич — D36 (Q24): `batch.feature:9`; D37 (Q25):
  `client_explanation.feature:10`; D24 (Q26): `import_export.feature:7`
  (прежние D22/Q20-строки в `batch`/`import_export` сохранены, новые —
  отдельными строками). Статусы ⏸/⏳ (`features/README.md` :274–276) на месте,
  правок не требовалось. `docs/CHANGELOG.md` :161–182 — запись «Перенос
  Q24–Q26 → D36, D37, D24» (связка «границы MVP», отложенные сценарии; ⚪,
  задач не требуется). Счётчики 47/278 не менялись (`features/README.md:309`,
  сверено чтением); ссылки D/Q живые (glob 6/6). `cargo`/git не запускались
  (D50); `src/**`, `tests/**`, журнал, `TRACEABILITY`/`SPECIFICATION`/`tasks`/
  `decisions/README.md` не трогал. Остаток: приёмка `validator` (docs) →
  пакет `git`.
- 2026-09-29 · операция `service-migration-q28q29` · сопутствующие к переносу
  Q28–Q29 → D31/D34: обратные ссылки `# Dn (Qx): …` (BRIEF §5.6 п.2) в шапках
  6 фич — D31 (Q28): `draft.feature:11`, `agent_minimal.feature:12`; D34 (Q29):
  `draft.feature:12` (рядом с D31), `mcp_tools.feature:7`,
  `test_draft.feature:11`, `publish.feature:15`, `deprecation.feature:4`
  (7 строк, 6 фич; прежние Q-строки и D-строки сохранены — только добавление).
  `docs/CHANGELOG.md:183–205` — запись «Перенос Q28–Q29 → D31, D34» (связка
  «MCP-контракты»; Q28 ✅ T-03; Q29 🟡 T-04 сделана/T-05 открыта — единственная
  открытая). Счётчики 47/278 не менялись (`features/README.md:309`, сверено
  чтением); ссылки записи живые (glob 7/7). `cargo`/git не запускались (D50);
  `src/**`, `tests/**`, журнал, `TRACEABILITY`/`SPECIFICATION`/`tasks`/
  `decisions/README.md` не трогал. Остаток: приёмка `validator` (docs) →
  пакет `git`.
- 2026-09-29 · операция `service-migration-q27q30` · сопутствующие к переносу
  Q27, Q30 → D27/D29 (связка 4б-2 «Транспорт и запуск»): обратная ссылка
  `# D29 (Q30): …` в шапке `agent_minimal.feature:13` (после D31:12; Q-блок
  :2–4 и строку D31 не трогал — прецедент сосуществования
  `batch.feature:2` + `:9`); `features/README.md:213` — заголовок ноты Q30
  дополнен `[Q30]`/`[D29]` (текст ноты не менял; `(Q27)` в теле осталась
  bare-упоминанием); `docs/CHANGELOG.md` :207–224 — запись «Перенос Q27, Q30 →
  D27, D29». Q27 фич не адресует (запуск — инфраструктура, фиксирует сам D27) —
  шапок не добавлял. Счётчики 47/278 не менялись (`features/README.md:309`);
  ссылки живые (glob 4/4). `cargo`/git не запускались (D50); журнал/SPEC/
  `TRACEABILITY`/tasks/`src`/`tests` не трогал. Остаток: приёмка `validator`
  (docs) → пакет `git`.
- 2026-09-29 · операция `service-migration-q31q33` · сопутствующие к переносу
  Q31 → D12 и Q33 → D30 (связка «Notebook-функции»): обратные ссылки `# Dn (Qx): …`
  (BRIEF §5.6 п.2) в шапках 5 фич — D12 (Q31): `notebook_ui.feature:12`,
  `agent_minimal.feature:14`, `inline_execution.feature:10`; D30 (Q33):
  `agent_minimal.feature:15`, `inline_execution.feature:11`, `draft.feature:13`,
  `mcp_tools.feature:9` (7 строк; прежние Q-строки/D-строки сохранены — только
  добавление в конец шапочного блока). `features/README.md` — ноты Q33 (:229) и
  Q31 (:240): заголовок дополнен `[Qx]`/`[Dn]` по прецеденту Q30:213, текст нот
  не менял. `docs/CHANGELOG.md:224–246` — запись «Перенос Q31, Q33 → D12, D30».
  Счётчики 47/278 не менялись (`features/README.md:309`, сверено чтением); ссылки
  записи живые (glob 11/11). T-08/T-09 не трогал (обе ⬜). `cargo`/git не
  запускались (D50); журнал/SPEC/`TRACEABILITY`/tasks/`decisions/**`/`src`/`tests`
  не трогал. Остаток: приёмка `validator` (docs) → пакет `git`.
- 2026-09-29 · операция `service-migration-q32` · сопутствующие к переносу
  Q32 → D28 («Git-контур», слаг `D28-two-git-contours`): обратные ссылки
  `# D28 (Q32): …` (BRIEF §5.6 п.2) в шапках **8 фич** (по одной строке, в конец
  шапочного блока, до `Функция:`) — `storage_paths.feature:10`,
  `git_integration.feature:8`, `agent_minimal.feature:16`, `publish.feature:17`,
  `publish_rules.feature:5`, `immutability.feature:8`, `deprecation.feature:5`,
  `mcp_tools.feature:10` (прежние Q-/D-строки сохранены — только добавление;
  `# D28` в фичах ранее отсутствовал — дублей нет). Темы формулировок: путь
  реестра `checks/{name}/{X}/{Y}/{Z}/` (storage_paths/publish/publish_rules/
  immutability/deprecation/mcp_tools), workspace-only git-панель + read-only
  «Версии» + переключатель версий + хотфикс-линия (git_integration),
  буфер→черновик→`check.test`, файл — при публикации (agent_minimal),
  публикация только с новым номером/downgrade запрещён (publish/publish_rules/
  immutability/mcp_tools), pre-release вне MVP (storage_paths). `features/README.md`
  — заголовки двух нот Q32 (:98,:219) дополнены `[Q32]`/`[D28]` по прецеденту
  Q30:213/Q33:229, текст нот не менял; табличные «(Q32)» не трогал.
  `docs/CHANGELOG.md:247–263` — запись «Перенос Q32 → D28» (после записи
  Q31/Q33:224–246; три уровня источника правды, сверка 🟡 — путь реестра не
  реализован, T-06 открыта; статусы фич и счётчики 47/278 не меняются).
  Проверки: `rg -c "^# D28 \(Q32\)"` = 1 на файл (8/8, без дублей); ссылки
  живы (glob 3/3 — Q32, D28, T-06); `features/README.md:309` — «47 файлов,
  278 сценариев» не менялось; numstat моих файлов — только добавления
  (features 8×1/0, features/README 2/2, CHANGELOG 17/0). T-06 не трогал (⬜).
  `cargo`/git не запускались (D50); журнал/SPEC/`TRACEABILITY`/tasks/`src`/
  `tests` не трогал. Остаток: приёмка `validator` (docs) → пакет `git`.
- 2026-09-29 · операция `service-migration-q34` · сопутствующие к переносу Q34 → D32
  (особый случай — расширение существующего D32, `Resolves: Q16, Q34`): обратные
  ссылки `# D32 (Q34): …` (BRIEF §5.6 п.2) в шапках 4 фич (по одной строке, в конец
  шапочного блока, до `Функция:`) — `notebook_ui.feature:13` (без `tests/` в
  workspace; `.dar-notebook/` в `.gitignore`), `inline_execution.feature:12`
  («быстрые прогоны» — черновики тестов в кэше), `test_draft.feature:13` (метка
  теста — в черновике; `tests/*.тест` — вне MVP), `publish.feature:18` (готовность —
  факт `check.test`, не файл теста). Прежние Q-строки и `# D32 (Q16)` не трогал.
  `features/README.md` — заголовки двух нот Q34 (:130,:236) дополнены
  `[Q34]`/`[D32]` по прецеденту Q30:213/Q33:229, текст нот не менял; табличные
  «(Q34)» не трогал. `docs/CHANGELOG.md:264–272` — запись «Перенос Q34 → D32»
  (после записи Q32:247–263; сверка 🟡 — метка реализована, гейт публикации T-02
  открыта; счётчики 47/278 не меняются). Проверки: `rg -c "^# D32 \(Q34\)"` = 1 на
  файл (4/4); ссылки живы (glob 3/3 — Q34, D32, T-02); `features/README.md:309` —
  «47 файлов, 278 сценариев» не менялось; numstat — только добавления (features
  4×1/0, features/README 2/2, CHANGELOG 9/0). T-02 не трогал (⬜). `cargo`/git не
  запускались (D50); журнал/SPEC/`tasks`/`src`/`tests` не трогал. Остаток: приёмка
  `validator` (docs) → пакет `git`.
- 2026-09-29 · операция `service-migration-q35` · сопутствующие к переносу Q35 → D59
  (слаг `D59-workspace-templates`): обратные ссылки `# D59 (Q35): …` (BRIEF §5.6 п.2)
  в шапках 2 фич (по одной строке, в конец шапочного блока, до `Функция:`) —
  `notebook_ui.feature:14` (новый workspace: `rules/Пример.dar` из `GRAMMAR.md` §1,
  `README.md`, `.gitignore`; шаблон нового правила парсится),
  `file_management.feature:5` (`rules/{Имя}.dar` — парсящийся шаблон, имя = имя файла;
  черновик — при сохранении, Q33); прежние Q-/D-строки сохранены (только добавление).
  `features/README.md` **не менял**: табличные упоминания Q35 (`:203` `Q19/Q34/Q35/Q37`,
  `:208` `(Q35)`) уже в стиле соседей-перенесённых Q (`:205`/`:206` — плейн-Q без
  D-ссылок), ноты-блока «Решение Q35» нет — не сочинял (прецедент Q19/Q37), табличные
  «(Q35)» не трогал. `docs/CHANGELOG.md:273–286` — запись «Перенос Q35 → D59» (после
  Q34 `:264–272`; workspace-шаблоны, сверка ⚪ — UI Notebook вне кода `credo2`, задач
  не требуется; статусы/счётчики 47/278 не меняются). Проверки: `rg "^# D59"` = 1 на
  файл (2/2, без дублей); ссылки записи живы (glob 3/3 — `GRAMMAR.md`, Q35, D59);
  `features/README.md:309` — «47 файлов, 278 сценариев» не менялось; правки только
  добавления. `cargo`/git не запускались (D50); журнал/SPEC/`TRACEABILITY`/`tasks`/
  `reviews`/`src`/`tests` не трогал. Остаток: приёмка `validator` (docs) → пакет `git`.
- 2026-09-29 · операция `service-migration-q36q38` · сопутствующие к переносу
  Q36+Q38 → D18 («Конвейеры + LSP-состав»): обратные ссылки `# D18 (Q36, Q38): …`
  (BRIEF §5.6 п.2) в конец шапочного блока (до `Функция:`) — `lsp.feature:6`
  (конвейеры — вне MVP; LSP-MVP — diagnostics/completion/hover/symbols/
  semanticTokens (SPEC §3.3), formatting/definition/references — v0.2),
  `graph_view.feature:2` (конвейеры и граф — вне MVP: ⏳, v0.2, вместе с
  `Приоритетом`, Q4); прежние `# D17 (Q4)`/`# Q37` целы. `features/README.md`
  :83–85 — заголовок ноты «Решения Q36/Q38» дополнен `[Q36]`/`[Q38]`/`[D18]`
  (образец — ноты Q3:71/Q4:77–79), текст не менял; табличные `:209`/`:210` не
  трогал (плейн-Q по стилю соседей). `docs/CHANGELOG.md:287–303` — запись
  «Перенос Q36, Q38 → D18» (после Q35 `:273–286`; сверка ⚪ — LSP/граф вне кода
  `credo2`, задач не требуется; счётчики 47/278 не меняются). Нюанс: первая
  вставка разорвала строку Q37-комментария `lsp.feature:5` — замечено чтением,
  исправлено. Счётчики `features/README.md` 47/278 не менялись (строка `:309`
  → `:311`). `cargo`/git не запускались (D50); журнал/SPEC/`tasks`/`reviews`/
  `src`/`tests` не трогал. Остаток: приёмка `validator` (docs) → пакет `git`.
