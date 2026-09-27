# service-run4-preflight — pre-flight перед Run 4 (T-03)

- **Дата:** 2026-09-27 · **Исполнитель:** сервисная сессия · **Основание:**
  указание владельца (пройти чек-лист и сбросить состояние Run 3).
- **Чек-лист** (`docs/analysis/run-checklist.md`):
  1. Ветка чистая — ✅ (`## develop...origin/develop`, дерево пусто).
  2. Прошлая задача (T-13) закрыта/смержена; `develop` в синхроне — ✅
     (`a4ffcd2`).
  3. `state/current/` переинициализирован — ✅ (см. ниже).
  4. `opencode debug agents` — ✅ (18 агентов; `lead` 16 / `analyst` 20;
     `cargo test` только у `validator`).
  5. `.opencode/.gitignore` — ✅ (`state/` игнорируется).
  6. Отчёты прошлых прогонов — ✅ (T-01/T-11/T-12/T-13 и service-*; отчёт T-03
     отсутствует намеренно — Run 3 откачен, улики в архиве/теге).
  7. Незакоммиченного канона нет — ✅.
  8. Карточка T-03 актуальна (⬜, ссылка `../../GRAMMAR.md` исправлена);
     сценарии `features/` не менялись — ✅.
- **Переинициализация `state/current/`:**
  - `next_action.yaml` → `status: idle` (плана нет — `lead` вызовет `analyst`);
  - `current_state.yaml` → `task: null`, `phase: idle`;
  - `progress.yaml` → очищен (оставлен формат-заголовок);
  - `receipts.yaml` → квитанция откаченного Run 3 (T-03) удалена (отчёт не
    существует; улики: `docs/analysis/T-03-run3-mail-archive.md`, тег
    `archive/run3-T-03`); квитанции T-12/T-13 сохранены.
- **Дальше:** Run 4 (T-03 заново) — новой сессией; `T-14` (GRAMMAR §7) — после
  T-03.
