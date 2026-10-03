# Приёмка: сервисная операция r15 (T-15, C3-остаток + C4)

- **Тип:** служебная операция (процесс/канон агентов)
- **Задача:** service-process-branch
- **Итерация:** 1
- **Дата:** 2026-10-03
- **Версия (снимок):** `develop` @ `f58e27c` + рабочее дерево (пакет не
  закоммичен)

**Проверка:** адресная документная/канонная сверка D93 ↔ факт + адресный
`cargo test --test docs_journal` (F74) + `node .opencode/scripts/agents-perms.mjs`.
Полный DoD не запускался (D50 — пакет процессный, продуктовый код CREDO не
затронут).

**Вердикт:** принято (P1/P2/P3 нет)

## P1

—

## P2

—

## P3

—

## Проверки

- `cargo test --test docs_journal` → **14 passed / 0 failed** (0.06s).
  Зелёные ключевые: `ids_are_unique_and_contiguous`,
  `questions_are_listed_in_catalog_and_traceability`,
  `decisions_are_listed_in_catalog`,
  `traceability_lifecycle_uses_canon_dictionary`,
  `traceability_lifecycle_matches_task_openness`,
  `traceability_tasks_exist_and_match_registry`,
  `traceability_rows_reference_existing_decisions`,
  `every_decision_has_code_review_verdict`,
  `no_migration_markers_in_live_journal`,
  `no_line_number_addresses_in_live_journal`,
  `no_addresses_to_removable_or_session_data`.
- `node .opencode/scripts/agents-perms.mjs` → `agents: 11 из 18`; `git` —
  `allow:node .opencode/scripts/session-checkpoint.mjs` (обе формы), список
  `review.md:104-109` синхронен фронтматтеру `git.md:34-35`.
- `git status --porcelain` → ровно периметр операции: 15 `M`
  (`.opencode/agents/{auditor,git}.md`, `.opencode/commands/git/status.md`,
  `.opencode/rules/{dispatch-loop,git-workflow,review}.md`,
  `.opencode/scripts/session-checkpoint.mjs`, `AGENTS.md`,
  `docs/{TRACEABILITY.md,decisions/README.md,features/README.md,features/agents-session-checkpoint.feature,questions/README.md}`,
  memory `auditor`/`migrator`) + 4 `??` (`commands/git/checkpoint.md`, лента r15,
  `D93-*.md`, `Q90.md`).
- `git diff --numstat -- src tests Cargo.toml` → пусто.
- `git diff --check` → пусто.
- `.opencode/state/snapshots/**` — отсутствует (временный каталог пробы удалён);
  временных файлов нет.

## Что проверено и ок

**D93 ↔ факт (10 пунктов).** Решение
`docs/decisions/D93-session-commit-process-branch.md` сверено с каноном:

1. Прогон = `T-XX`, сессия `sM` — D93 п.1 ↔ `git-workflow.md:215`.
2. Топология ветки сессии `process/<прогон>-s<M>` от текущего HEAD чекаутом
   `git switch -c`; накопительная ветка отклонена — D93 п.2 ↔
   `git-workflow.md:211-215`, `dispatch-loop.md:107-111`, `checkpoint.md:16`.
3. Live `state/current/*` — финальным process-коммитом задачи — D93 п.3 ↔
   `git-workflow.md:215-216`.
4. Снапшот `.opencode/state/snapshots/<прогон>-s<M>/` скриптом `--snapshot`,
   право у `git` — D93 п.4 ↔ `git-workflow.md:205-208`, `git.md:70-73`.
5. «Постоянный» пакет без изменения D89 P3 — D93 п.5 ↔ `git-workflow.md:217-219`.
6. Сервисные операции — без process-ветки — D93 п.6 ↔ `git-workflow.md:223-224`.
7. Фасад `/git/checkpoint` — 7 шагов, без shell-блоков, «нет записи — нет
   чекпойнта», идемпотентно — D93 п.7 ↔ `checkpoint.md` (шаги 1-7:
   снапшот → `switch -c` → `add` → `commit` → `tag` → `push` → `switch -`;
   `agent: git`, `subagent: true`; shell-блоков `!` нет).
8. `/git/status` + теги сессий; `.opencode/commands/**` в аудит `auditor` —
   D93 п.8 ↔ `status.md:11` (`git tag -l "session/*"`), `auditor.md:59-61,:95-96`.
9. Фича — сценарий 1 к принятому имени (`process/<прогон>-s<M>`,
   `chore(process): <прогон> s<M>`), счётчик 4 — D93 п.9 ↔
   `agents-session-checkpoint.feature:13-14` (4 `Сценарий:`).
10. Идемпотентность/catch-up — как D89 P3 — D93 п.10 ↔ `checkpoint.md:24`,
    `git-workflow.md:220-222`.

Строка «остаток C3 — отдельным решением» снята из §«Session-commit» (проверено
чтением §:200-227).

**P2-фиксы аудита закрыты.**

- P2-1: `review.md:104-109` — у `git` в «Доступные команды» есть
  `node .opencode/scripts/session-checkpoint.mjs`; совпадает с фронтматтером
  `git.md:34-35`; автопроверка `agents-perms.mjs` расхождений не даёт.
- P2-2: `docs/features/README.md:299` — строка `agents-session-checkpoint`
  описывает ветку сессии/теги/снапшот/фасад (D93-именование), `process/runN`
  отсутствует; соседние строки `:298`/`:300` не задеты.

**Скрипт `session-checkpoint.mjs`** (проверка чтением, не R2): режим
`--snapshot` (`:25-53`) читает `task`/`session_index` из `current_state.yaml`
(не найдены → `exit 2`), создаёт каталог (`--dir` override), копирует 4
артефакта (`next_action`/`current_state`/`progress`/`receipts`) + `meta.md`,
печатает путь, `exit 0`. Запись прогона сервисной сессией в ленте r15:
каталог + 5 файлов; временный каталог удалён до пакета.

**Журнал.** Q90 ↔ D93 (`resolved by`/`Resolves`); `questions/README.md:112`,
`decisions/README.md:122`; `TRACEABILITY.md:94` (Q90: D93 / `in work` /
T-15 🚧); `Q90.md`/`D93-session-commit-process-branch.md`; D93 «Сверка с кодом»
— ⚪ не применимо (процесс/служебная зона), обосновано; номеров строк канона нет.

**Границы.** Изменены только заявленные `.opencode/**`, `AGENTS.md`, `docs/**`
(журнал/TRACEABILITY/фичи), лента/память; `src/**`, `tests/**`, `Cargo.toml` не
тронуты; `git diff --check` чисто; временных файлов и каталога снапшота нет.

## Техническое замечание (не находка)

- Права `validator` на прогон `session-checkpoint.mjs` нет (в
  `review.md`/фронтматтере роли не предусмотрено) — проверка скрипта выполнена
  **чтением** кода и записи прогона в ленте, а не R2-тестом. Для процессного
  пакета это достаточная замена; при желании право можно рассмотреть отдельно.

## Заметка к закрытию (hand-off, не блокер)

- Карточка `docs/tasks/T-15-mcp-ready-process/README.md` хранит
  `process/runN` (`:108` — состав фазы C, `:216` — строка реестра `C3`,
  `:276-277` — черновик команды). D93 п.1-2 канонизировал `T-XX` /
  `process/<прогон>-s<M>`. Карточка вне периметра правки r15 (статусы и
  черновик закрывает `migrator`); при закрытии C3/C4 строку реестра `C3` и
  черновик `:261-280` привести к D93-именованию, статусы `C3`/`C4` → ✅.

## Артефакты

- Квитанция: `.opencode/state/current/receipts.yaml` (`task:
  service-process-branch`, iteration 1, verdict accepted).
- Отчёт в ленте r15: `.opencode/mail/service-mcp-ready-r15.md`.
- Канон не правился; статусы карточки не менялись.
