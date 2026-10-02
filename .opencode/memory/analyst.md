# Память: analyst (эфемерный решатель)

- **Канон:** `AGENTS.md` §Рабочая группа агентов;
  `.opencode/rules/dispatch-loop.md` (решение D39).
- **Одно решение за вызов:** свежий срез → досье
  `docs/analysis/<T-XX>-<дата>.md` + план
  `.opencode/state/current/next_action.yaml`; контекст не накапливается.
- **Правило:** план — до точки ветвления; scope-решение — Q/D через
  `migrator` до исполнения; чекпойнт здесь — только при обрыве планирования.

## Чекпойнты

- **02.10.2026 · T-20** (волна 2 `TRACEABILITY`): план обновлён, статус
  `awaiting_user` (гейт предложений). Досье `docs/analysis/T-20-2026-10-02.md`;
  расклад: 9 `done` (Q8–Q11, Q14, Q18, Q20, Q21, Q23), Q22 → `in work` + T-24
  (тестовая CLI-контур REST, `tester`, S), 16 `open` (v0.2), Q81 `open`;
  Q78/Q79 → `done`. Класс L. Грабли: `tests/rest.rs` строит `AppState` напрямую
  — CLI-флаги REST не покрыты; «Задач не требуется» в D-файлах — прямой признак
  кандидата в `done`/`open`.
- **02.10.2026 · T-20 · правки по проверке** (разбор не пересматривался):
  `done` = **9** (Q22 выведен в `in work`; 9+1+16=26); приёмка — адресная
  документная сверка **без cargo** (D50; решение владельца 02.10.2026), прогон
  `docs_journal.rs` — на следующем кодовом прогоне; в очередь добавлен
  `git branch_start` после owner-gate и до `migrator`; статусы T-20: migrator
  ⬜→🚧, validator 🚧→✅ до гейта пакета. Обновлены досье/next_action/state.
- **02.10.2026 · T-20 · re-plan после branch_start**: ветка
  `feature/T-20-traceability-wave2` создана от `develop` @ `d414998` (upstream,
  коммитов нет); в state зафиксированы `branch.base_head: d414998`,
  `branch.published: true`, `base.develop: d414998`; снимок дерева — 7 M + 2 ??
  (сверить `add_paths` на гейте пакета; в planned добавлен `memory/git.md`).
  Очередь не менялась — следующий `dispatch migrator`.
- **02.10.2026 · T-20 · правка перед migrator**: восстановлен хвост очереди
  (9 действий: … validator → migrator (закрытие: T-20 ✅ + Q78/Q79 done, D82) →
  surface_to_user (пакет) → git (branch_end) → complete); Q78/Q79 → done
  перенесено со шага применения на шаг закрытия (до ✅ T-20 — `in work`);
  статусы T-20 у `validator` убраны (нет прав на `docs/tasks/**` — делает
  `migrator`). Грабли: `validator` не правит `docs/tasks/**`; `done` для
  Q78/Q79 возможен только при отсутствии открытых задач (D82).
- **02.10.2026 · T-20 · участок №2 (rework)**: вердикт validator — rework (P2):
  битая ссылка `docs/tasks/T-24-rest-cli-contour-test/README.md:53`
  `../D50-…` → `../../decisions/D50-…`. `iteration: 2`, `rework: 1`,
  `session_index: 1`, маркер `t20_rework_link`, `status: in_progress`.
  Очередь: migrator (фикс одной строки) → validator (-r2, без cargo) →
  migrator (закрытие: T-20 ✅ + Q78/Q79 done) → surface_to_user → git
  (branch_end) → complete. `auditor` на №2 не требуется (одна ссылка).
- **02.10.2026 · T-20 · участок №2, правка**: класс L исключений не допускает —
  `auditor` возвращён в маршрут (после `migrator`-закрытия, до коммита);
  исключение `auditor` из `route_exclusions` снято. Грабли: на классе L
  `auditor` обязателен независимо от объёма правки (AGENTS.md §Размерные
  маршруты; `dispatch-loop.md` §«Маршрут и scope»).
- **02.10.2026 · T-20 · участок №3 (rework P1)**: вердикт validator `-r2`:
  `docs/TRACEABILITY.md:82,:83` (Q78/Q79) `[T-20] ⬜` против реестра `T-20 🚧`;
  тест `tests/docs_journal.rs:592` требует равенства статусных ячеек.
  `iteration: 3`, `rework: 2`, `session_index: 1`, маркер
  `t20_sync_status_cells`. Очередь: migrator (синхронизация ячеек `⬜`→`🚧`) →
  validator (-r3, без cargo) → migrator (закрытие: T-20 ✅ + Q78/Q79 → done
  **с видимой `[T-20] ✅`**, D82/D77) → auditor (L) → surface_to_user → git →
  complete. Грабли: ячейка «Задачи» в TRACEABILITY — представление реестра: её
  статус обязан совпадать с `docs/tasks/README.md` (проверка — тест T-18);
  при `done` строки Q78/Q79 задача должна оставаться видимой (`[T-20] ✅`, не
  `—`) — иначе D77-часть теста.
- **02.10.2026 · T-20 · гейт пакета**: `validator -r3` accepted, `auditor`
  финальный — P1/P2/P3 нет. Снимок `git status --porcelain` = **20 путей**
  (14 M + 6 ??), посторонних нет; `add_paths` = снимок точно. Обновлены
  `acceptance: accepted`, `phase: closing`, `status: awaiting_user`, маркер
  `t20_package_gate`. Порядок: коммит на ветке → merge --no-ff в develop →
  push origin develop → удаление ветки (local+origin); **master не трогаем**.
  База develop = origin/develop = d414998.
