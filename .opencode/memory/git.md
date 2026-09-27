# Память: git (git-операции)

- **Канон:** `.opencode/rules/git-workflow.md`.
- **Правило:** чекпойнт — пакет, выполненные шаги и хеши, что осталось
  (идемпотентность при обрыве). Кратко.

## Чекпойнты

- **2026-09-27 · T-12, шаг «Старт задачи» — готово.** Пакет (подтверждён `lead`
  в ленте `T-12.md`, запись от 2026-09-27): `switch develop` → `pull origin
  develop` → `switch -c feature/T-12-agent-loop develop` → `push -u`. Выполнено
  полностью; шаг 1 пропущен (уже был на `develop`).
  Состояние: HEAD `f06ff82` (= `develop`), ветка `feature/T-12-agent-loop`,
  upstream `origin/feature/T-12-agent-loop`; незакоммичен только
  `?? .opencode/mail/T-12.md`. Осталось: W4 (коммиты/merge/push/удаление ветки)
  — отдельный подтверждённый пакет. Заметка: `git branch --list` роли не разрешён,
  идемпотентность сверять через `git status -sb` и вывод `push`.
- **2026-09-27 · T-12, W4 — коммиты 1–2 готовы.** Пакет подтверждён `lead`
  (лента `T-12.md`, запись 2026-09-27). Сверка: 29 M + 9 новых, лишнего нет;
  исходно HEAD `f06ff82` на `feature/T-12-agent-loop`.
  Коммит 1 `1d2e052` (`docs(Q44): D39`, 6 файлов); коммит 2 `9bd77d0`
  (`chore(T-12): цикл агентов v3`, 32 файла). Осталось: коммит 3 (лента+память),
  затем `switch develop` → `pull` → `merge --no-ff` → `push` → `branch -d` →
  `push --delete`.
- **2026-09-27 · T-12, W4 — завершено.** Коммиты `1d2e052`, `9bd77d0`, `8bbe2a5`.
  Merge `--no-ff` в `develop` → `8015ddd`; `push origin develop`
  (`f06ff82..8015ddd`); remote+local ветка удалены (`branch -d` отказал до
  удаления remote из-за устаревшего upstream; удалил remote первым — без
  `-D`/`--force`). Состояние: `develop` в синхроне с `origin/develop`, дерево
  чистое; `feature/T-12-agent-loop` нет. Осталось: Run 3 (T-03) — вне T-12;
  «после» в ленте/памяти не закоммичены (рабочие данные).
- **2026-09-27 · T-12, служебный коммит записей после пакета — шаг 2 готов.**
  Пакет подтверждён `lead` (лента `T-12.md`, запись «пакет подтверждён
  (служебный коммит)», 2026-09-27). Сверка `git status --short`: ровно два пути
  (` M .opencode/mail/T-12.md`, ` M .opencode/memory/git.md`), лишнего нет;
  ветка `develop`. Отчёт `git` и этот чекпойнт дописаны **до** коммита (входят в
  него). Осталось: `git add` этими путями → `git commit -m "chore(T-12):
  служебные записи ленты и памяти после пакета"` → `git push origin develop`
  (timeout ≥ 300000 мс) → проверки `git status -sb`, `git log -2 --oneline`.
- **2026-09-27 · T-03, «Старт задачи» — до push.** Пакет подтверждён `lead`
  (лента `T-03.md`, запись `surface_to_user` 2026-09-27): `switch develop` →
  `pull origin develop` → `switch -c feature/T-03-check-create develop` →
  `push -u origin`. Шаг 1 пропущен (уже на `develop`); pull — Already up to
  date; ветка создана от `c212156` (`develop`). Отклонение: `git ls-remote`
  ролью не разрешён — состояние `origin` сверяется по выводу push.
  Осталось: push -u (timeout ≥ 300000 мс) + сверка `git status -sb`; затем
  «после» в ленту/память.
- **2026-09-27 · T-03, «Старт задачи» — готово.** Пакет выполнен полностью:
  `push -u origin feature/T-03-check-create` → `* [new branch]`, upstream
  выставлен. Коммитов/слияний нет. Состояние: HEAD `c212156` (= `develop`),
  текущая ветка `feature/T-03-check-create`,
  `## feature/T-03-check-create...origin/feature/T-03-check-create` (синхрон);
  незакоммичено: `M memory/analyst.md`, `M memory/git.md`, `?? mail/T-03.md`,
  `?? docs/analysis/T-03-2026-09-27.md`. Осталось: W-пакет задачи (коммиты по
  приёмке `validator`, merge `--no-ff` в `develop`, push, удаление ветки) —
  отдельное подтверждение. Наблюдение: `git ls-remote` роли запрещён
  (permission.rejected) — идемпотентность ветки на `origin` сверять по
  `git status -sb`/выводу push.
- **2026-09-27 · T-03, пакет завершения — до коммита.** Подтверждение `lead`
  (лента `T-03.md`, «git-пакет завершения подтверждён», «Подтверждаю пакет»);
  `package` в `next_action.yaml` прочитан. Сверка `git status --porcelain`:
  ровно 14 путей = `add_paths`, посторонних нет. Ветка
  `feature/T-03-check-create`, HEAD `c212156` (= `develop`), задача не
  закоммичена. Осталось: `add` 14 путями → `commit` `code(T-03)` →
  `switch develop` → `pull` → `merge --no-ff` → `push` → `branch -d` →
  `push --delete`. Записи ленты/памяти входят в коммит (дописаны до `add`).
