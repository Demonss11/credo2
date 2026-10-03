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
- **02.10.2026 · T-24 · участок rework P1 (iteration 2, rework 1, D42):** вердикт
  validator iteration 1 — rework: `tests/rest_cli.rs:47` `clippy::collapsible_if`
  (rust-1.96.0, `-D warnings`); остальное ок (fmt pass, `cargo test --all` 141/0
  `rest_cli` 6/6, покрытие 6/6, границы чисты). Правка — `tests/**` (зона
  `tester`; clippy не в правах роли — дефект всплыл на приёмке). Очередь:
  `tester` (одна правка let-chain; **вызов новый** — sessionID tester не
  зафиксирован) → `validator` `-r2` (полный DoD; точка ветвления) → `docs-writer`
  (T-24 ✅) → `migrator` (Q22 → done, видимая `[T-24] ✅`) → `surface_to_user`
  (пакет) → `git` branch_end → `complete`. Маркер `t24_rework_clippy_p1`;
  `session_index 2`; досье/state/лента обновлены; `inherited_boundaries` без
  изменений. Новых Q/D нет.
- **02.10.2026 · T-24 · участок закрытия (iteration 2, rework 1, D42):** вердикт
  `validator -r2` — accepted (fmt pass; clippy pass — P1 закрыт; `cargo test
  --all` 141/0, `rest_cli` 6/6; покрытие 6/6; границы чисты; квитанция
  iteration 2 в `receipts.yaml`; отчёт `-r2`). `iteration` закрытия = 2 —
  iteration хранит rework-раунд, закрытие нового раунда не вводит (прецедент
  T-03: closing iteration 2; T-20: closing на текущем iteration). Очередь до
  гейта: `docs-writer` (T-24 ✅: карточка `✅ сделана` + `docs/tasks/README.md`
  `✅`; features/ и CHANGELOG не трогать) → `migrator` (TRACEABILITY Q22
  `in work` → `done`, видимая `[T-24] ✅`; D77/D82; F57; иных строк не трогать)
  → `surface_to_user` (pre_gate сверка `add_paths` со свежим снимком **до**
  гейта) → `git` branch_end → `complete`. Факт-снимок 10 M + 5 ??; ожидание
  гейта 15 M + 5 ?? = **20** (`package.add_paths`); маркер
  `t24_closeout_package`; `session_index 2`; `phase: closing`,
  `acceptance: accepted`. Новых Q/D нет.
- **02.10.2026 · T-24 · хвост closeout после частичного branch_end (iteration 2,
  rework 1, D42):** git branch_end исполнен частично — коммит `8b6c8b9` (20
  путей: 15 M + 5 A) + merge `--no-ff` `6c28e51` в develop + push
  `caac10d..6c28e51` origin develop (✅); удаление ветки НЕ выполнено:
  `git push origin --delete` заблокирован гардом CCSN (`git.push-delete`, ручное
  действие владельца); origin-ветка и локальная ветка остались (`branch -d` не
  выполнялся); обходов нет. Канон частичного исполнения —
  `dispatch-loop.md:80-89`: выполненная часть зафиксирована, остаток —
  возобновление НОВЫМ подтверждением; `complete` — только после полного
  closeout. Очередь остатка: `surface_to_user` (эскалация владельцу: удалить
  origin-ветку вручную; рекомендованный вариант — «Удалю вручную (Recommended)»,
  прецедент T-20) → `dispatch git` (локальное `branch -d` без force + проверка
  отсутствия ветки local+origin; предусловие — origin-ветка удалена; отказ
  `-d`/наличие — не обходить, re-plan → `wait_for_user`) → `complete`. Маркер
  `t24_closeout_branch_cleanup`; `status: awaiting_user`; `base.develop = 6c28e51`;
  нового коммита нет (post-package записи — F43-подхват). Урок: гард CCSN
  `git.push-delete` стабильно блокирует удаление origin-ветки — обход не искать
  (T-20, T-24). Новых Q/D нет.
- **02.10.2026 · T-16 (свежий план, session_index 3)**: задача T-16 — `check.test`
  на stale-черновике должен исполнять текст файла `rules/{name}.dar` (Q12/D54;
  сценарий `features/test_draft.feature`; находка H7). Класс **M** (код
  `src/mcp.rs` `test()` + при необходимости `src/lib.rs`; тесты
  `tests/mcp_draft.rs`; есть сценарий `features/` → guard «не ниже M»). Факты:
  `test()` (`mcp.rs:208-256`) исполняет `&d.rule`; `is_stale` (`lib.rs:813-822`)
  читает `workspace/rules/{name}.dar`, но `workspace` приватный (нужен хелпер).
  База develop = origin/develop = `6c28e51`; ветка `feature/T-16-stale-check-test`
  ещё не создана. Маркер `t16_stale_file_source`; статус awaiting_user (гейт
  старта). Очередь: surface_to_user (ветка) → git branch_start → docs-writer 🚧 →
  migrator TRACEABILITY 🚧 → coder → tester → validator (точка ветвления) →
  docs-writer ✅ + CHANGELOG → migrator ✅ (Q12 остаётся in work — T-08 ⬜; H7) →
  surface_to_user (пакет) → git branch_end → complete. Грабли: worktree не чист —
  2 чужих M-файла сервисной сессии (`mail/service-mcp-ready-r10.md`,
  `memory/service.md`); в пакет T-16 не включать. Статус `test_draft.feature` не
  поднимать до ✅ без validator (часть сценариев зависит от T-02). Новых Q/D нет
  (уточнения реализации — в рамках D40).
- **02.10.2026 · T-16 · re-plan branch_start (`expect_mismatch`)**: git сообщил
  базу `develop = origin/develop = HEAD = 2958bf5` («T-15 r10»), а не `6c28e51`
  из плана; `6c28e51` — в истории (развилка на процессные коммиты T-15 `af53e23`
  r9, `2958bf5` r10, кода T-16 не касаются). Решение: база ветки — текущий
  `develop @ 2958bf5`; **повторный owner-гейт не требуется** (владелец
  подтвердил старт задачи; хэш — контекст плана). `re_raise: T-16-branch-base-drift`
  (`expect_mismatch`, blocking false, resolved true). Обновлены
  next_action/current_state (`base.develop`/`branch.base_head` = 2958bf5,
  `status: in_progress`, гейт старта снят из очереди, первый action —
  `dispatch git` branch_start) + досье. Урок: хэш базы в плане — снимок; перед
  branch_start сверять с фактическим `develop`, дрейф на процессные коммиты —
  не повод для нового owner-гейта.
- **02.10.2026 · T-16 · P2-контроль closeout (`add_paths` ↔ снимок):** вердикт
  validator — пакет совмещённого closeout T-16+T-25 рассинхронизирован с
  деревом: `.opencode/mail/T-16-r2.md` (том 2 ленты T-16, создан самим re-plan
  из-за порога ~300 строк) был `??` и на него ссылался `T-25.md:69`, но в
  `package.add_paths` и в раскладке process-коммита отсутствовал. Правка —
  только состояние: строка добавлена в `add_paths` и в process-note
  `next_action.yaml`, в `add_paths` `current_state.yaml`; `snapshot_check`
  дополнен (снимок 34 = 25 M + 9 ??; `add_paths` 32 = снимок минус 2 чужих
  сервисных M). Расхождений нет; git-операций нет. **Паттерн:** при
  совмещённом closeout общие файлы неразделимы пофайлово (CHANGELOG и
  tasks/README — вообще один хунк); посылка «пофайловый add разделит вклад»
  ложна, `chunk`-split недоступен. **Грабли:** артефакты, созданные самим
  re-plan'ом (новый том ленты и пр.), сразу попадают в снимок — включать их в
  `add_paths`/process-раскладку при re-plan, а не после вердикта validator.
  Новых Q/D нет.
