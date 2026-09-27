# Память: git (git-операции)

- **Канон:** `.opencode/rules/git-workflow.md`.
- **Правило:** чекпойнт — пакет, выполненные шаги и хеши, что осталось
  (идемпотентность при обрыве). Кратко.

## Чекпойнты

> D41: оперативная хроника задач — в `state/` и ленте; здесь — знание роли и
> аварийные чекпойнты. Записи ниже — история.

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
- **2026-09-27 · service-w7-rollback (откат Run 3 + старт W7) — готово.**
  Подтверждение `lead` — лента `.opencode/mail/service-w7-rollback.md` («пакет
  отката подтверждён», указание владельца 27.09.2026). Сверка
  `git status --short` до шагов: ровно 3 `M` (`mail/T-03.md`,
  `memory/analyst.md`, `memory/git.md`) + 6 `??` — совпала. Выполнено:
  `restore` этих 3 путей → `switch develop` →
  `branch -d feature/T-03-check-create` (was `ecebdee`; warning: merged to
  upstream, not HEAD — ожидаемо) → `push origin --delete` (`- [deleted]`) →
  `switch -c feature/T-13-agent-hardening develop` → `push -u` (`* [new
  branch]`, upstream выставлен). Состояние: ветка `feature/T-13-agent-hardening`
  в синхроне с `origin/feature/T-13-agent-hardening`, HEAD `c212156`; `develop`
  не тронут; `feature/T-03-check-create` нет локально и на origin. Незакоммичено:
  `?? mail/service-w7-rollback.md`, `?? docs/analysis/T-03-run3-mail-archive.md`,
  `?? docs/analysis/W7-preRUN4_T-03.md`, `?? docs/analysis/memorandum-*` (×3).
  Осталось: правки канона W7 и коммиты — отдельными подтверждёнными пакетами.
- **2026-09-27 · service-w7-rollback (аварийный архив Run 3) — готово.**
  Подтверждение `lead` — лента `.opencode/mail/service-w7-rollback.md` («аварийный
  архив Run 3», указание владельца 27.09.2026). Пакет: тег на `ecebdee` + push
  тега. Шаг 1 (`git cat-file -t`) недоступен роли (нет в allowlist) → заменён
  read-only: `git rev-parse ecebdee` → `ecebdeedf856470f1a74b4599f264c07c0de7ae6`,
  `git log -1 --oneline ecebdee` → `ecebdee` (коммит жив). Выполнено:
  `git tag -l "archive/*"` (пусто) → `git tag archive/run3-T-03 ecebdee` →
  `git push origin archive/run3-T-03` (`* [new tag]`, timeout 360000 мс).
  Проверки: `git tag -l "archive/*"` → `archive/run3-T-03`;
  `git rev-parse archive/run3-T-03` → `ecebdee…`; `git log -1 --oneline
  archive/run3-T-03` → `ecebdee code(T-03): check.create {name, source}, …`.
  Состояние: ветка `feature/T-13-agent-hardening`, дерево как было (6 `M` +
  20 `??`); коммитов/слияний нет. Заметка: текст сообщения коммита — `{name,
  source}` (в плане `lead` было `{name, subject}`).
  Осталось: правки канона W7 и коммиты — отдельными подтверждёнными пакетами.
- **2026-09-27 · T-13, W4 — коммиты 1–2 готовы.** Подтверждение `lead` — лента
  `T-13.md` («пакет подтверждён пользователем», 2026-09-27). Сверка
  `git status --short`: 22 `M` + 17 новых (итого 39), лишнего нет; ветка
  `feature/T-13-agent-hardening`, HEAD `c212156`. Коммит 1 `e34772f`
  (`docs(Q45,Q46): D40/D41`, 9 файлов); коммит 2 `299fd0c`
  (`chore(T-13): W7`, 30 файлов). Дерево после коммита 2 чистое. Осталось:
  коммит 3 (лента+память), затем `switch develop` → `pull` → `merge --no-ff` →
  `push origin develop` → `branch -d` → `push origin --delete` (push —
  timeout ≥ 300000 мс).
- **2026-09-27 · T-13, W4 — завершено.** Коммиты `e34772f` (9 файлов),
  `299fd0c` (30), `84ae806` (2). Merge `--no-ff` в `develop` → `7f6d02a`
  «Слияние feature/T-13-agent-hardening в develop» (39 файлов, +2456/−23);
  `push origin develop` (`c212156..7f6d02a`). Ветка удалена: `git branch -d`
  отказал (не «полностью merged» в устаревший upstream `origin/...` — коммиты
  задачи не были запушены в ветку) → `git push origin --delete`
  (`- [deleted]`) → повтор `git branch -d` (`was 84ae806`); без `-D`/`--force`.
  Состояние: `develop` в синхроне с `origin/develop`, дерево чистое;
  `feature/T-13-agent-hardening` нет локально и на origin; тег
  `archive/run3-T-03` не задет. Осталось: запись «после» в ленте/памяти не
  закоммичена (рабочие данные; служебный коммит — по решению `lead`).
  Дальше: Run 4 (T-03 заново) — вне T-13.
- **2026-09-27 · T-13, служебный коммит записей после пакета — шаг 2 готов.**
  Подтверждение `lead` — лента `T-13.md` («пакет подтверждён (служебный
  коммит)», 2026-09-27). Сверка `git status --short`: ровно два пути
  (` M .opencode/mail/T-13.md`, ` M .opencode/memory/git.md`), лишнего нет;
  ветка `develop`. Отчёт `git` и этот чекпойнт дописаны **до** коммита (входят
  в него). Осталось: `git add` этими путями → `git commit -m "chore(T-13):
  служебные записи ленты и памяти после пакета"` → `git push origin develop`
  (timeout ≥ 300000 мс) → проверки `git status -sb`, `git log -2 --oneline`.
  Основание прямого коммита в `develop` — служебные рабочие данные (ветка
  задачи удалена).
- **2026-09-27 · T-03 Run 4, шаг «Старт задачи» — готово.** Подтверждение
  `lead` — лента `T-03.md` («пакет ветки подтверждён», 2026-09-27),
  `progress.yaml` (`surface_to_user` → `confirmed: «Подтверждаю пакет»`).
  Состояние до: `develop` @ `bf56a4a`, sync `origin/develop`, 2 `??` (рабочие
  данные). Шаг 1 пропущен (уже на `develop`); `pull origin develop` —
  `Already up to date`; `switch -c feature/T-03-check-create develop` →
  `Switched`; `push -u` (timeout 360000) → `* [new branch]`, upstream выставлен.
  Проверки после: `git status -sb` →
  `## feature/T-03-check-create...origin/feature/T-03-check-create`, те же 2 `??`;
  `git branch -a` — ветка локально + `remotes/origin/feature/T-03-check-create`;
  HEAD `bf56a4a`. Осталось: работа в ветке (docs-writer 🚧 → coder → tester →
  validator → re-plan → ✅), затем W4 — отдельным подтверждённым пакетом.
- **2026-09-27 · T-03 Run 4, суженный пакет `branch_commit_push` — готово.**
  Подтверждение `lead` — лента `T-03.md` («surface_to_user: пакет заменён
  владельцем (сужение)»): «доведи задачу до push в удаленную ветку
  feature/T-03-check-create, а в develop не надо мерджить и удалять ветки»;
  `progress.yaml` (`result: СУЖЕНИЕ`). Сверка `git status -sb` (13 M + 4 ?? =
  17) ↔ `add_paths` — 1:1. Выполнено: `add` 17 точными путями → `commit`
  `7e94c79` («code(T-03): check.create {name, source}», 17 files, +1190/−12) →
  `push origin feature/T-03-check-create` → `bf56a4a..7e94c79` (timeout 360000).
  Проверки: `git status -sb` — ветка в синхроне с origin, дерево чистое;
  `git log -1 --oneline develop` → `bf56a4a` (не тронут); `git branch -a` —
  ветка локально + `remotes/origin/feature/T-03-check-create`. Merge `--no-ff` в
  develop, `push origin develop`, удаление ветки — **отменены владельцем**, не
  выполнялись. Осталось: незамерженная ветка `feature/T-03-check-create`
  (develop @ bf56a4a без T-03) — решение о merge/удалении за владельцем.
- **2026-09-27 · T-03 Run 4, пакет `t03_merge` — СТОП на шаге 1.** Подтверждение
  `lead` — лента `T-03.md` («merge T-03 — пакет подтверждён владельцем», директива
  владельца «делаем merge»). Сверка `git status -sb`: изменённых tracked **4**
  вместо ожидаемых 3 — лишний ` M docs/features/README.md` (правка T-15:
  `agents-*.feature` + итог 47/278), помимо `docs/tasks/README.md`,
  `.opencode/mail/T-03.md`, `.opencode/memory/git.md`. Затронут и `restore`-шаг:
  пакет снимал только `docs/tasks/README.md`, судьба `features/README.md` не
  задана. По hard rule шага 1 — **СТОП**, изменяющих команд не было. Состояние:
  ветка `feature/T-03-check-create` (HEAD `7e94c79`) в синхроне с origin;
  `develop` @ `bf56a4a`; 10 `??` (T-15/service). Осталось: решение `lead` по
  `features/README.md`, затем шаги 2–8 пакета `t03_merge`.
- **2026-09-27 · T-03 Run 4, пакет `t03_merge` (повтор) — завершено.**
  Подтверждение `lead` — лента `T-03.md` («merge T-03 — пакет подтверждён
  владельцем» + «уточнение пакета»; директива владельца «делаем merge»).
  Сверка шага 1 совпала: `develop` @ `bf56a4a`; `git diff --name-only develop
  HEAD` → ровно 17 путей коммита `7e94c79` без `docs/features/README.md`.
  Выполнено: process-коммит `8d9c9dc` «chore: записи прогона Run 4 (T-03)»
  (2 файла, +131; `add` только `mail/T-03.md`, `memory/git.md`) → `restore
  docs/tasks/README.md` (только он; `features/README.md` оставлен рабочим) →
  `switch develop` → `pull` (`Already up to date`) → `merge --no-ff
  feature/T-03-check-create` → `89ebd42` (17 файлов, +1321/−12) → `push origin
  develop` (`bf56a4a..89ebd42`, timeout 360000) → удаление ветки.
  `git branch -d` отказал (локальный `8d9c9dc` не был в устаревшем upstream
  `origin/feature/T-03-check-create`; коммит уже вошёл в develop через
  `89ebd42`) → без `-D`/`--force`: `push origin --delete` (`- [deleted]`) →
  `fetch --prune` → `branch -d` (`was 8d9c9dc`). Порядок шагов 7↔8 поменян
  только ради отказа `-d`.
  Состояние: `develop` в синхроне с `origin/develop` @ `89ebd42`; `git log -3`
  → `89ebd42`, `8d9c9dc`, `7e94c79`; ветки `feature/T-03-check-create` нет
  локально и на origin; ` M docs/features/README.md` + 10 `??` (T-15/service).
  Отчёт в ленте/памяти дописан **после** пакета — не закоммичен (рабочие
  данные; служебный коммит — по решению `lead`). Заметка: `git branch --list`
  роли не разрешён — проверки через `git status -sb`/`git branch -a`.
