# Память: analyst (эфемерный решатель)

- **Канон:** `AGENTS.md` §Рабочая группа агентов;
  `.opencode/rules/dispatch-loop.md` (решение D39).
- **Одно решение за вызов:** свежий срез → досье
  `docs/analysis/<T-XX>-<дата>.md` + план
  `.opencode/state/current/next_action.yaml`; контекст не накапливается.
- **Правило:** план — до точки ветвления; scope-решение — Q/D через
  `migrator` до исполнения; чекпойнт здесь — только при обрыве планирования.

## Чекпойнты

- **2026-10-02 · T-18/T-21, участок №8 (re-plan №7, вердикт validator T-18 =
  rework).** Решено: 5 из 6 красных `docs_journal` — дефекты самого теста T-18
  (P1-1 `field_line` :109-113 leading space → Q1 :353; P1-2 `cell_task_statuses`
  :206-234 не понимает `T-XX`/узкое окно `.take(4)` → :535/:581; P1-3 `section()`
  :146-152 inline `## …` в D64:83 → :472; P1-4 нет нормализации `features/`
  :237-249/:285-297 → :613) — зона `tester`, только `tests/docs_journal.rs`, без
  ослабления; 1 — канон-дефект **D39** `:44-45,56-57,61` (адреса
  `.opencode/state/current/*.yaml`, D65 п.4) — зона `migrator`. Порядок:
  `migrator` (D39) → `tester` (4 дефекта) → `validator` T-18 **-r2** (полный DoD
  → accepted) → `validator` T-21 **-r3**. Совмещение -r3 и приёмки T-18
  отклонено (F43: разные ветки/квитанции/пакеты). `src/mcp.rs` (12/18) и
  `tests/mcp_draft.rs` (12/8) не переправлять. Обновлены: досье T-18/T-21
  (разделы «Участок №8»), `next_action.yaml` (task T-18, iteration 1, маркер
  `t18_test_defects_r2_then_t21_r3`, очередь 4), `current_state.yaml` (operation
  T-18, acceptance rework), ленты T-18/T-21, этот чекпойнт.

- **2026-10-02 · T-18, участок №4 (re-plan №3, блокер `src/mcp.rs`).** Вызов
  прерван лимитом шагов; основное сделано, остался append-отчёт в ленту.
  Обновлены: досье `docs/analysis/T-18-2026-10-02.md` (раздел «Участок №4 —
  re-plan №3»), `next_action.yaml` (iteration 4, маркер `t18_src_blocker_t21`,
  очередь 4: migrator → surface_to_user → git → coder), `current_state.yaml`
  (`gate_pending: owner_confirmation`).
  - Факты блокера (чтением, без cargo — R2): `src/mcp.rs` — `required_str:437`
    dead_code; `ToolError::Envelope:674` E0223; `ToolError::Message:678` E0599;
    `into_json:783` E0599 (есть `to_json`). Предсуществующие (в HEAD и базе
    `5786875`; `git diff 5786875..HEAD -- src/mcp.rs` пусто) → post-accept
    defect **T-04**; вне scope T-18 (`src/**` не трогать).
  - Ключ: по **D50** DoD по составу пакета; пакет T-18 меняет `tests/**` ⇒
    `cargo test --all` у validator компилирует `src/**` ⇒ **фикс `src/**` (T-21)
    обязателен ДО приёмки T-18**.
  - Маршрут: `migrator` (карточка T-21, источник T-04, P3, ⬜; реестр +
    TRACEABILITY; новых Q/D/F нет) → owner-gate (подтвердить T-21) → `git`
    (ветка `feature/T-21-<слаг>` от develop) → `coder` (struct-API +
    `required_str`) → `validator` (T-21) → возврат к T-18 (`tester` resume
    `ses_f045ce490ffe7XnmTS0eBleSkQ`: отчёт участка №3 → `validator` T-18).
  - Точка обрыва: append-отчёт в `.opencode/mail/T-18.md` не дописан.

- **2026-10-02 · T-21, участок №6 (re-plan №5 по rework validator).** Вызов
  прерван лимитом шагов; основное сделано, остались append-отчёты и этот
  чекпойнт (дописаны отдельным resume-вызовом).
  Обновлены: `next_action.yaml` (task T-21, iteration 1, маркер
  `t22_mcp_draft_fix`, очередь 7: migrator → surface_to_user → git → tester
  T-22 → validator T-21 -r2 → tester T-18 → validator T-18),
  `current_state.yaml` (`acceptance: rework`, `gate_pending:
  owner_confirmation`, ветки T-21/T-18/T-22, `route_t22`, `route_t18_return`),
  досье `docs/analysis/T-21-2026-10-02.md` (раздел «Участок №6 — re-plan №5»);
  дописаны ленты T-21 (решение участка №6) и T-18 (cross-ref).
  - Факты (чтением; `cargo` не запускался — R2/D50): красный полный DoD T-21 —
    только `tests/**`; диф T-21 (`src/mcp.rs`) корректен. `tests/mcp_draft.rs:543`
    E0425 (`Value` не в scope), `:603/:624/:647` E0061 (`mcp.create(SRC)` vs
    сигнатура `create(&mut,name,source)`, `tests/common/mod.rs:148`) —
    **предсуществующие** (tracked = HEAD/develop `5786875`); `tests/docs_journal.rs:168`
    dead_code `field d` — **введён T-18** (untracked).
  - Решение: **T-22** (post-accept дефект `tests/mcp_draft.rs`; класс S; tester;
    ветка от develop; источник Q29/D34 + T-04) через `migrator` — фикс `Value` +
    `create(NAME,SRC)`, тест не ослаблять; dead_code — фикс tester **в T-18**
    (resume). **Сужение DoD по D50 отклонено**: пакет T-21 меняет `src/**` ⇒
    п.3 D50 требует полный DoD; красное — чужой дефект, D50 на него не ложится.
  - Ключ (повтор T-18 уч. №4): по **D50** DoD по составу пакета; `src/mcp.rs`
    не переправлять; тесты не ослаблять; F43 — пакеты **T-18/T-21/T-22
    раздельны** (три ветки/коммита).
