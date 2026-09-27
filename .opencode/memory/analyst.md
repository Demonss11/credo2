# Память analyst (цикл CREDO, prototypes/credo2)

## Контекст
Роль — эфемерный решатель цикла (D39): одно решение за вызов — досье
`docs/analysis/<T-XX>-<дата>.md`, план `.opencode/state/current/next_action.yaml`,
сводка `current_state.yaml`. Канон не правлю; действия не исполняю (только план).

## Записи

### 2026-09-27 · T-03 `check.create: {name, source}` · планирование
- Класс **M**: код + интеграционные тесты + сценарии `features/` (guard).
- Источник: Q28 (SPEC §10 №31). Границы: `src/mcp.rs` (+`src/core.rs`),
  `tests/mcp_draft.rs`; конверт/код ошибки — T-04 (Q29).
- Находки: схема инструмента без `name`, `required=["source"]`; handler не
  сверяет `name` с заголовком; сообщение — «Не найдено имя правила».
- Маршрут: git → docs-writer(🚧) → coder → rust-expert → tester → validator →
  docs-writer → git.
- Риски: R1 граница с T-04; R2 смена сообщения в core.rs; R3 mismatch name.
- Следующее действие: вести план по `next_action.yaml`; при отчёте validator —
  re-plan. Ветка задачи: `feature/T-03-check-create`.

### 2026-09-27 · T-03 · re-plan (после ветки и 🚧)
- Ветка `feature/T-03-check-create` создана/published (HEAD c212156); 🚧
  проставлен (карточка + сводка). Фаза → `implementation`.
- Код (подтверждено): `create()` mcp.rs:44-60 читает только `source`, `name`
  игнорируется; схема mcp.rs:302-305 required=["source"]; parse_rule
  core.rs:444 → «Не найдено имя правила».
- **Решение:** `rust-expert` из маршрута убран — правка не идиоматическая
  (обязательный параметр + сравнение + текст сообщения); стиль держит clippy
  у validator. `tester` оставлен (M, покрытие features).
- Очередь: coder (name+source, сверка заголовка, текст «отсутствует заголовок
  правила», схема) → tester (кейсы draft.feature) → validator (DoD, приёмка,
  точка ветвления). После pass: docs-writer ✅ → git-пакет → complete.
- Границы: конверт `validation_failed` — T-04 (R1), в брифе coder «не делать».

### 2026-09-27 · T-03 · re-plan pass (приёмка → closeout)
- Приёмка: validator — **accepted** (iteration 1), DoD зелёный (fmt/clippy ok,
  `cargo test --all` 86/0), квитанция в `receipts.yaml`, отчёт
  `docs/reviews/T-03-2026-09-27.md`; P1/P2/P3 нет. Открытый вопрос tester
  (rmcp vs `required`) закрыт фактом: tool-result `isError`, не JSON-RPC error.
- Состояние: `next_action.yaml` — маркер `closeout_pass`, очередь
  docs-writer → surface_to_user → git → complete; `current_state.yaml` — фаза
  `closeout`, acceptance accepted, rework 0.
- **GRAMMAR.md §7 (стр. 56): НЕ включаю в T-03** — цитата «Не найдено имя
  правила» устарела (код: «отсутствует заголовок правила»); GRAMMAR — канон
  языка (правка → L + auditor), T-03 принят как M, границы — src/tests.
  Follow-up: отдельная docs-задача.
- git-пакет завершения: один коммит `code(T-03): check.create {name, source},
  сверка заголовка и тексты ошибок` (add_paths: src/mcp.rs, src/core.rs,
  tests/mcp_draft.rs, карточка+сводка tasks, review, analysis, mail, memory),
  затем merge --no-ff в develop → push → удаление ветки. `.opencode/state/**`
  вне git.
- Следующее: вести closeout по плану; после `complete` — задача закрыта.

### 2026-09-27 · T-03 · микро re-plan (расхождение add_paths)
- Факт: `git status --porcelain` (14 файлов) vs `package.add_paths` — не хватало
  `.opencode/memory/docs-writer.md` (чекпойнт 🚧 → ✅). Без правки `expect`
  git («рабочее дерево чисто») не выполнился бы.
- Действие: добавлен 1 путь (после `coder.md`); итог add_paths = 14 позиций,
  полное совпадение с деревом. Notes/steps не менял: `.opencode/state/**` вне
  git — верно. Прочие поля плана без изменений.
- Дальше: `surface_to_user` (пакет) → `git` → `complete`.
