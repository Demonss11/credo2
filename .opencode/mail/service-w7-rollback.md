# service-w7-rollback — откат пилотной ветки Run 3 и старт ветки W7

- **Тип:** служебная операция (задача W7/T-13 оформляется `migrator` в W1).
- **Указание владельца (27.09.2026):** «приступай к реализации. Начнем с отката
  ветки. Либо можем ее оставить потом уберем и сразу начать работу с новой
  ветки от develop» → выбран **откат сразу**: работа Run 3 не мержится
  (`develop` не трогается), Run 4 повторит задачу по обновлённому канону.
- **Пакет (исполняет `git`, идемпотентно, по одной команде):**
  1. `git restore .opencode/mail/T-03.md .opencode/memory/analyst.md .opencode/memory/git.md` —
     снять незакоммиченные рабочие записи пилота;
  2. `git switch develop`;
  3. `git branch -d feature/T-03-check-create` (при отказе — стоп, `-D` не
     применять);
  4. `git push origin --delete feature/T-03-check-create` (timeout ≥ 5 мин);
  5. `git switch -c feature/T-13-agent-hardening develop`;
  6. `git push -u origin feature/T-13-agent-hardening` (timeout ≥ 5 мин).
- **Улики:** лента `.opencode/mail/T-03.md` (полная версия, включая записи
  второй сессии) архивирована `docs-writer` в
  `docs/analysis/T-03-run3-mail-archive.md` **до** отката.
- **Не входит:** правки канона W7 (после W1), коммиты (пакет W7), удаление
  архива.
- **ID:** T-13 — предварительный (реестр на 27.09.2026: свободен; подтверждает
  `migrator`).

## lead · 2026-09-27 · пакет отката подтверждён

- Подтверждение (указание владельца, 27.09.2026, см. выше): откат пилотной
  ветки Run 3 и создание ветки W7 — исполнять.
- Дальше / риски: `git` — по шагам пакета; при отказе `branch -d` (ветка не
  влита в `develop`) — остановка и эскалация владельцу (ручной `-D`).

## lead · 2026-09-27 · аварийный архив Run 3 (указание владельца)

- Указание владельца (27.09.2026, стоп-чек B): до любых работ закрепить коммит
  Run 3 `ecebdee` — единственный носитель кода пилота после удаления ветки.
  Проверено: `git cat-file -t ecebdee` → `commit` (объект жив локально).
- Пакет (исполняет `git`): `git tag archive/run3-T-03 ecebdee` →
  `git push origin archive/run3-T-03` (timeout ≥ 5 мин).
- Дальше / риски: тег делает коммит достижимым на `origin` (защита от gc);
  если push тега не пройдёт — эскалация владельцу.

## git · 2026-09-27 · готово

- Сделано (по одной команде, идемпотентно): сверка `git status --short` = ровно
  3 `M` + 6 `??` (совпала с ожидаемым) → `git restore .opencode/mail/T-03.md
  .opencode/memory/analyst.md .opencode/memory/git.md` → `git switch develop`
  → `git branch -d feature/T-03-check-create` (удалена, was `ecebdee`; warning:
  merged to upstream, not to HEAD — ожидаемо) → `git push origin --delete
  feature/T-03-check-create` (`- [deleted]`) → `git switch -c
  feature/T-13-agent-hardening develop` → `git push -u origin
  feature/T-13-agent-hardening` (`* [new branch]`, upstream выставлен).
- Проверки: `git status -sb` →
  `## feature/T-13-agent-hardening...origin/feature/T-13-agent-hardening`,
  tracked-правок нет (только 6 рабочих `??`); `git branch -a` —
  `feature/T-03-check-create` нет ни локально, ни на `origin`,
  `feature/T-13-agent-hardening` есть (local + `remotes/origin`);
  `git log -1 --oneline` → `c212156`; `develop` не тронут.
- Дальше / риски: пакет отката+старта выполнен полностью; коммитов/слияний нет.
  Записи ленты и памяти этой сессии оставлены незакоммиченными (рабочие данные,
  войдут в пакет W7). Осталось: правки канона W7 (после W1) и последующие
  коммиты — отдельными подтверждёнными пакетами.

## git · 2026-09-27 · аварийный архив Run 3 — готово

- Сделано (идемпотентно): шаг 1-эквивалент: `git cat-file` роли недоступен
  (нет в allowlist — deny `*`), заменён разрешённой read-only сверкой
  `git rev-parse ecebdee` → `ecebdeedf856470f1a74b4599f264c07c0de7ae6` и
  `git log -1 --oneline ecebdee` → `ecebdee` (объект жив, коммит) →
  `git tag -l "archive/*"` (пусто) → `git tag archive/run3-T-03 ecebdee` →
  `git push origin archive/run3-T-03` (timeout 360000 мс) →
  `* [new tag] archive/run3-T-03 -> archive/run3-T-03`.
- Проверки: `git tag -l "archive/*"` → `archive/run3-T-03`;
  `git rev-parse archive/run3-T-03` → `ecebdeedf856470f1a74b4599f264c07c0de7ae6`;
  `git log -1 --oneline archive/run3-T-03` → `ecebdee code(T-03): check.create
  {name, source}, сверка заголовка и тексты ошибок`. Тег на `origin` (push
  `[new tag]`). Рабочее дерево не тронуто (те же 6 `M` + 20 `??`).
- Замечание: тексты `lead` в ленте указывали `{name, subject}`, фактическое
  сообщение коммита — `{name, source}` (несоответствие в цитате плана, на
  результат не влияет).
- Дальше / риски: коммит `ecebdee` достижим на `origin` через тег (защита от
  gc). Коммитов/слияний нет; записи ленты и памяти оставлены незакоммиченными.
