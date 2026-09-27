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
