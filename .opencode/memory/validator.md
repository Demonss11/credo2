# Память: validator (приёмка)

- **Канон:** `.opencode/rules/review.md`; `.opencode/rules/journal.md` §5.3/§7.
- **Правило:** чекпойнт **до** прогона тестов (что проверено, что запускаю) и
  **после** (результат, вердикт) — тесты прерывают сессию (R5). Кратко.
  Квитанция — append в `state/current/receipts.yaml`. Отчёт —
  `docs/reviews/<тип>-<id>-<дата>.md`.

## Чекпойнты

- **02.10.2026 · T-15/C1 (сервисная операция r8), до прогона.** Проверено
  чтением: состав пакета (`git status` — 10 `M` + 4 `??`, `src/tests/Cargo.toml`
  не тронуты); `state-schema.md` (4 артефакта, поля/типы/обязательность,
  писатель/читатель, инварианты, `session_index`, `owner_response`);
  `dispatch-loop.md` ссылается на схему; D86 `Affects` включает analyst/lead;
  Q83↔D86 парны; каталоги/TRACEABILITY синхронны; карточка C1 🚧; роль
  `lead.md:88` = `idle` (P3 закрыт). Запускаю: `cargo fmt --check`,
  `cargo clippy --all-targets -- -D warnings`, `cargo test --all` (R2/D50).
  Ориентир 135 passed / 0 failed.
- **02.10.2026 · T-15/C1 — после прогона: принято.** fmt pass; clippy pass;
  `cargo test --all` — 135 passed / 0 failed (docs_journal 14/14,
  features_inventory 4/4). P1/P2/P3 нет. Отчёт
  `docs/reviews/T-15-c1-2026-10-02.md`; квитанция `T-15-c1` (iteration 1,
  accepted) в `state/current/receipts.yaml`. F15 не закрыт — `migrator`.
- **02.10.2026 · T-15/C1 адресная переприёмка (r2), без cargo (D50).**
  Пакет не закоммичен (`deferred_by_owner`); после приёмки владелец вернул на
  доработку канон (порог `review.md` §«Возврат на доработку»: канон/права →
  новый отчёт + машинная сверка прав + адресные проверки). Дельта: сужено
  «Когда читать» `state-schema.md` (analyst+validator, auditor по потребности;
  lead/git не читают); `lead.md` — inline `session_index` + `channel`/
  `owner_response`/`result`, отсылка к схеме снята; в силе `wait_for_user`
  (:98), `idle` (:49), D86 `Affects` с ролями (:11), шаблон `lead.md` `idle`.
  Проверки: `agents-perms.mjs` — 11 из 18, команды = `review.md`
  §«Доступные команды»; `git status` — пакет + `memory/service.md` + r8;
  `git diff --numstat -- src tests Cargo.toml` пусто; `git diff --check` пусто.
  `cargo` не запускался (D50). Вердикт: принято, P1/P2/P3 нет. Отчёт
  `docs/reviews/T-15-c1-2026-10-02-r2.md`; квитанция `T-15-c1` iteration 1,
  `report -r2`.
