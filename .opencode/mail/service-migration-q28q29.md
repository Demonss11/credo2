# Сервисная лента: service-migration-q28q29 («MCP-контракты»)

**Назначение:** миграция журнала — смысловая связка **«MCP-контракты»**:
**Q28** «Параметры `check.create`» и **Q29** «Ответы `check.test`,
`check.list_published`, `check.deprecate` — контракты MCP-инструментов»
(обе решены 2026-09-26), раздел 5 «MCP: контракт для агента».
**Почему связка:** Q28 и Q29 вместе задают контракт взаимодействия агента с
сервером (вход `check.create` + общие правила/коды/инварианты и схемы ответов
§4.5); строка §10 №34 прямо ссылается на Q28.
**Особенность:** строки §10 уже есть — **№31 → `D31` (Q28)**, **№34 → `D34`
(Q29)**; новых номеров нет. Выход из порядка архива (Q27 отложен) — решение
владельца о смысловых связках; 4б-2 «Транспорт и запуск» (Q27+Q30) остаётся в
очереди.
**Канон схем:** JSON-контракты живут в `SPECIFICATION.md` §4.5 (решение Q29);
Q29/D34 ссылаются на §4.5, не дублируют (прецедент D53/§4.4). Всё, что есть в
блоке архива и не покрыто §4.5, — перенести в D34 (ничего не терять).
**Ожидания сверки:** Q28 — ✅ (T-03 сделана: `{name, source}` → `{status,
name}`); Q29 — 🟡 (T-04 сделана; **T-05 открыта** — схемы успеха).
**Маршрут:** `migrator` (Q28 — вызов 1; Q29 — вызов 2, крупный блок ~340
строк) → `docs-writer` → `validator` → `git`.

Формат записей — `AGENTS.md` §«Память и почта».

---

## сервисная сессия · 29.09.2026 · открытие операции

- Сделано: сверка — архив Q28 (:215–253), Q29 (:254–593, крупный); строки §10
  №31 (:854) и №34 (:857); задачи T-03 ✅, T-04 ✅, T-05 ⬜ (depends T-01 ✅,
  T-02 ⬜); фичи-адресаты существуют (`draft`, `agent_minimal`, `mcp_tools`,
  `test_draft`, `publish`, `deprecation`, `features/README`).
- Проверки: файлами — архив, `SPECIFICATION.md` (§10; §4.5 — канон схем),
  `tasks/README.md`, `glob docs/features`; `git status -sb` (develop = origin =
  `2e0edcb`, дерево чистое); `cargo` не запускался (D50).
- Дальше / риски: вызов 1 — Q28 → D31 (слаг `D31-check-create-contract`);
  вызов 2 — Q29 → D34 (слаг `D34-mcp-tool-contracts`; ссылаться на §4.5,
  не копировать; SPEC :507 «OPEN_QUESTIONS» → D34). Сводка
  `decisions/README.md` — строки D31 (между D26 и D32) и D34 (между D32 и
  D35); parenthetical пропусков → «(Q27, Q30–Q41)». Свипы «ожидает переноса»
  для Q28/Q29. Q27/Q30 — ждут 4б-2.

---

## migrator · 29.09.2026 · готово (вызов 1 из 2: Q28 → D31)

- Сделано: перенос **Q28 → D31** (`docs/decisions/D31-check-create-contract.md`,
  `Resolves: Q28`; `docs/questions/Q28.md` — дата 2026-09-26, resolved,
  «Перенос: 2026-09-29, блок Q28–Q29»). Сводки: строка D31 в
  `docs/decisions/README.md` (между D26 и D32; parenthetical пропусков
  «(Q27–Q41)» → «(Q27, Q30–Q41)»), Q28 в `docs/TRACEABILITY.md` (Feature —
  ровно `draft.feature`, `agent_minimal.feature` из `Affects`) и
  `docs/questions/README.md`. §10 №31 (:854) — добавлено «полный контекст —
  [D31](decisions/D31-check-create-contract.md)». Архив: блок Q28 (:215–252) →
  указатель «→ перенесён» (формат Q24–Q26, метка «блок Q28–Q29»). Свип Q28:
  `Q45.md:18` и `D40-scope-threshold.md:19` — Q28 живой ссылкой; Q29 не
  трогался (вызов 2).
- Проверки: сверка §5.3 — **✅ соответствует** (чтением, `cargo` не запускался,
  D50): `src/mcp.rs` `create` (155–184) — `{name, source}` обязательны,
  `parse_rule` (167), сверка `rule.name == name` (168–173), ответ ровно
  `{status:"ok", name}` (183), `upsert_draft` (180) — перезапись; tool spec
  (472–477, `required ["name","source"]`); юнит-тесты 676–733.
  `tests/mcp_draft.rs` — `create_scenario_saves_draft_to_sandbox_with_hash`
  (369–402, ключи `["name","status"]`), `stale_and_hash_after_overwrite_q12`
  (63–99, перезапись, `count==1`), `create_rejects_source_without_rule_header`
  (408–426), `create_rejects_name_title_mismatch` (431–453),
  `create_without_name/source_param_is_error` (457–481),
  `agent_create_appears_in_drafts_list` (487–499); `tests/mcp_errors.rs`
  (конверт `validation_failed`/`unknown_tool`). Фичи `draft.feature` (2–4,
  16–25, 58–71), `agent_minimal.feature` (40–41); `SPECIFICATION.md` §4.5
  (485–541), §7 (663–715).
- Задачи: [T-03](../../../docs/tasks/T-03-check-create/README.md) — **сделана**
  (`Tasks:` в D31 с пометкой «(сделана)», как D54 про T-01); новых задач нет.
- Дальше / риски: **вызов 2 — Q29 → D34** (`D34-mcp-tool-contracts.md`):
  ссылаться на §4.5, не копировать; `SPECIFICATION.md:507` «OPEN_QUESTIONS» →
  D34; строка D34 в `decisions/README.md` (между D32 и D35); свип Q29. Q27/Q30 —
  ждут 4б-2. Коммит — за `git`.

---

## migrator · 29.09.2026 · готово (вызов 2 из 2: Q29 → D34)

- Сделано: перенос **Q29 → D34**
  (`docs/decisions/D34-mcp-tool-contracts.md`, `Resolves: Q29`; `Tasks:`
  T-04 (сделана), T-05 (открыта)); создан `docs/questions/Q29.md` (дата
  2026-09-26, resolved, «Перенос: 2026-09-29, блок Q28–Q29»). Канон схем —
  `SPECIFICATION.md` §4.5 (общие правила, коды, инварианты, сводная таблица);
  в D34 перенесено то, что §4.5 не покрывает: таблица «когда используется» для
  10 кодов, полные JSON-схемы всех 10 инструментов, перечень вне-MVP —
  **ничего не потеряно**. Сводки: строка D34 в `decisions/README.md` (между D32
  и D35; parenthetical «(Q27, Q30–Q41)» остаётся); Q29 в `TRACEABILITY.md`
  (Feature — ровно `mcp_tools`, `draft`, `test_draft`, `publish`, `deprecation`)
  и `questions/README.md`. §10 №34 (:857) — «полный контекст —
  [D34](decisions/D34-mcp-tool-contracts.md)». `SPECIFICATION.md:507` —
  «открывает таблицу в Q29 (`OPEN_QUESTIONS.md`)» → «см. таблицу в
  [D34](decisions/D34-mcp-tool-contracts.md) (Q29)»; `rg -n "OPEN_QUESTIONS"
  docs/SPECIFICATION.md` → осталась только шапка §10 (:816). Архив: блок Q29
  (после сдвига :220–526) → указатель «→ перенесён» (метка «блок Q28–Q29»,
  формат соседей; Q30 :225 цел). Свип Q29: `Q45.md:18`,
  `D40-scope-threshold.md:19`, а также все «ожидает переноса»-упоминания Q29 —
  `Q12/Q13/Q16/Q17/Q18.md`, `questions/README.md` (Q13/Q16/Q17/Q18),
  `D32/D35/D54/D14/D57` — Q29 живой ссылкой.
- Проверки: сверка §5.3 — **🟡 расхождение** (чтением, `cargo` не запускался,
  D50). ✅ конверт ошибок + 10 кодов: `src/mcp.rs` `ErrorCode`/`as_str`
  (38–67), `ToolError`/`to_json` (72–131), `response` (533–546); инварианты
  1–5: `create` (155–184), `draft_json` (438–455), `test_valid` (439–440),
  `check.test` (250–261); тесты `tests/mcp_errors.rs`,
  `tests/mcp_draft.rs` (104–141, 145+, 32–99), `tests/common/mod.rs`
  (`assert_error_envelope` 181–225). 🟡 схемы успеха: `check.list_drafts`
  (186–199) ✅, `check.get_draft` (438–455) ✅, `check.test` (250–261) ✅;
  расходятся — `delete_draft` (277–279), `publish` (328–341, нет
  `published_at`/`published_by`), `deprecate` (391–396, нет
  `deprecated_at`/`service_hash`), `list_published` (399–411, нет `status`/
  `service_hash`/`versions`/`path`/`status`/`active`/`deprecation_reason`),
  `rebuild_manifest` (413–419, нет `count`) → задача
  [`T-05`](../../../docs/tasks/T-05-mcp-success-schemas/README.md) (открыта).
- Задачи: T-04 — **сделана**; T-05 — **открыта** (схемы успеха); новых задач
  не требуется.
- Дальше / риски: связка **«MCP-контракты» закрыта** (D31/Q28, D34/Q29).
  Дрейф канона **снят**: единый адрес полных схем — D34 —
  `SPECIFICATION.md:525` и §10 №34 (:857) → `[D34]`, `D34:314`,
  `OPEN_QUESTIONS:714`; `rg -n "полные схемы" docs` — адресации на Q29/архив
  не осталось. Q27/Q30 — 4б-2. Коммит — за `git`.

---

## docs-writer · 29.09.2026 · готово

- Сделано: **шапки фич** — 7 строк `# Dn (Qx): …` (BRIEF §5.6 п.2) в 6 фичах:
  D31 (Q28) — `docs/features/draft.feature:11`,
  `docs/features/agent_minimal.feature:12`; D34 (Q29) —
  `docs/features/draft.feature:12` (рядом с D31),
  `docs/features/mcp_tools.feature:7`, `docs/features/test_draft.feature:11`,
  `docs/features/publish.feature:15`, `docs/features/deprecation.feature:4`
  (каждая новой строкой; прежние Q-строки и D-строки не тронуты —
  только добавление). **CHANGELOG** — `docs/CHANGELOG.md:183–205`, запись
  «Перенос Q28–Q29 → D31, D34» в «### Документация» (после Q24–Q26):
  контракт `check.create` (`{name, source}` → `{status, name}`, «сохранить =
  обновить»); MCP-контракты — общие правила, 10 стабильных кодов, инварианты,
  полные схемы (адрес — D34/§4.5); задачи T-03 (сделана), T-04 (сделана),
  T-05 (открыта; единственная открытая по связке).
- Проверки: `rg` — ровно 7 шапочных строк в 6 фичах; счётчики
  `features/README.md:309` (47/278) не менялись — сверено чтением; ссылки
  записи (7/7) живы — glob `questions/Q28.md`, `questions/Q29.md`,
  `decisions/D31-check-create-contract.md`, `decisions/D34-mcp-tool-contracts.md`,
  `tasks/T-03…`, `tasks/T-04…`, `tasks/T-05…`; `cargo` не запускался (D50).
- Дальше / риски: приёмка `validator` (docs) → пакет `git`. Ссылки D31/D34 в
  шапках — прозаичным текстом (как в прецедентах D54/D32), не markdown-ссылки:
  относительный путь из `docs/features/` — `../decisions/Dn-*.md`; формат
  BRIEF §5.6 п.2 канон не требует ссылки — при ином чтении снимается одной
  правкой.

---

## validator · 29.09.2026 · отклонено (P2)

- Сделано: приёмка связки «MCP-контракты» (Q28 → D31, Q29 → D34); снимок
  `develop @ 2e0edcb` + рабочее дерево (2026-09-29); отчёт
  `docs/reviews/migration-q28q29-2026-09-29.md`; квитанция
  `service-migration-q28q29` iteration 1 (`rework`) — append в `receipts.yaml`.
- Проверки: архив (`:215/:220` указатели, Q30 цел, numstat `10/346`), свип
  `ожида[ею]т переноса`, D-поля и §10 (`:854`/`:857`), дрейф «полные схемы» —
  единый адрес D34 (SPEC `:507/:525/:857`; `OPEN_QUESTIONS` в SPEC — только
  шапка `:816`), `decisions/README.md` 38 строк (D26 → D31 → D32 → D34 → D35;
  `:18` «(Q27, Q30–Q41)»), `TRACEABILITY.md:34-35` / `questions/README.md:51-52`,
  7 шапок в 6 фичах, счётчики 47/278 (`features/README.md:309`), CHANGELOG
  `:183-205`, порядок блоков ленты (осиротевших фрагментов нет), границы.
  **cargo не запускался — D50** (изменений `src/**`/`tests/**`/`Cargo.toml`
  нет; счётчики/состав фич не менялись).
- Дальше / риски: **P2** — `D34-mcp-tool-contracts.md:420`: `Q18 (semver)`
  указан среди «ожидают переноса», хотя Q18 уже перенесён (`D35:5 Resolves`,
  `TRACEABILITY.md:25`, `decisions/README.md:41`, коммит `e141476`) → свип даёт
  ложный сигнал и противоречит D35. Правка — одна строка (`migrator`):
  оставить `Q33 (draft-first) — ожидает переноса`, `Q18` — живой ссылкой или
  убрать; затем повторная приёмка `-r2`.

---

## migrator · 29.09.2026 · готово (P2)

- Сделано: снят P2 — `docs/decisions/D34-mcp-tool-contracts.md:420` был
  `Q33 (draft-first), Q18 (semver) — ожидают переноса` → теперь
  `[Q18](../questions/Q18.md) / [D35](D35-semver-v01.md)` (semver),
  `Q33 (draft-first) — ожидает переноса` (строки `:420–421`). «ожидает
  переноса» оставлено только реально не перенесённому — Q33.
- Проверки: `rg -n "ожида[ею]т переноса" docs/decisions/D34-mcp-tool-contracts.md`
  → 1 совпадение (`:421`, Q33). Ссылки Q18/D35 живые (`questions/Q18.md`,
  `decisions/D35-semver-v01.md`); `cargo` не запускался (D50). Границы: правил
  одну строку, `features/**`/`src`/`tests`/`CHANGELOG` не трогал.
- Дальше / риски: повторная приёмка `validator` (`-r2`); коммит — за `git`.

---

## validator · 29.09.2026 · -r2 принято, замечаний нет

- Сделано: повторная приёмка (`iteration 2`) — P2 закрыт; отчёт
  `docs/reviews/migration-q28q29-2026-09-29-r2.md`; квитанция
  `service-migration-q28q29` iteration 2 (`accepted`) — append.
- Проверки: `D34:420-421` — `[Q18](../questions/Q18.md)` /
  `[D35](D35-semver-v01.md)` (semver), `Q33 (draft-first) — ожидает переноса`;
  `rg "ожида[ею]т переноса" D34` → 1 совпадение (`:421`, Q33), `rg "Q18 (semver)
  — ожидают" docs` → пусто; Q18/Q35-цели живые (`Q18.md`, `D35` есть), Q33 —
  полный блок в архиве (`:360`). Регрессий нет: `D34` 427 → 428 строк (только
  правка); `git diff --numstat` всех отслеживаемых файлов пакета идентичен r1
  (фичи, `CHANGELOG`, §10, сводка, `TRACEABILITY`, Q-файлы); `src/tests/
  Cargo.toml` не тронуты; **cargo не запускался — D50**.
- Дальше / риски: пакет принят — `lead` закрывает статусы и выносит на
  подтверждение пакет `git` (коммит — за ролью `git`).

## сервисная сессия · 29.09.2026 · пакет подтверждён

- Подтверждение (одно, `question`, владелец): **«Коммит + push»**; сообщение —
  `docs(D31, D34): перенос Q28–Q29 — контракты MCP: check.create, ответы
  инструментов`.
- Пакет (37 файлов: 30 M + 7 новых, включая чекпойнт `memory/git.md` до `add`):
  Q28/Q29/D31/D34, сводка `decisions/README.md` (38 строк), §10-ссылки,
  единый адрес схем (§4.5 → D34), архив, `TRACEABILITY`, реестры, 6 фич-шапок,
  CHANGELOG, свипы (Q45, D40, Q12–Q18, D14/D32/D35/D54/D57), 2 отчёта приёмки,
  лента и память ролей, квитанции.
- Следствие для `git`: записи до `add` (эта лента + чекпойнт `memory/git.md`),
  `add` точными путями, сверка staged (37), коммит в `develop` (база
  `2e0edcb`, синхрон с origin), затем **`push origin develop`** (таймаут ≥
  5 мин); после push в отслеживаемые файлы не писать (F43); хеш — в ответе
  `git`.

---

## git · 29.09.2026 · в работе (записи до add)

- Сделано: подтверждение сверено по ленте («Коммит + push»); снимок
  `git status --porcelain=v1` — 29 `M` + 7 `??` = 36, совпал с пакетом,
  посторонних нет; чекпойнт `memory/git.md` добавлен (37-й путь).
- Проверки: `git log -1` → `2e0edcb` (develop, синхрон с `origin/develop`).
- Дальше / риски: `add` 37 точными путями → сверка staged (30 M + 7 A) →
  коммит `docs(D31, D34): перенос Q28–Q29 …` → `push origin develop`
  (таймаут ≥ 5 мин). Хеш — в ответе `lead` (F43); после push в отслеживаемые
  файлы не писать.



