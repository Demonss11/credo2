# Сервисная лента: service-hygiene-ruff (.ruff_cache в .gitignore)

**Назначение:** микроправка гигиены по запросу владельца (30.09.2026):
в корне репозитория появился `.ruff_cache/` (создаёт внешний инструмент,
ruff 0.15.22); добавлен в игнор, кэш удалён.

**Границы:** корневой `.gitignore`, `.opencode/scripts/pm/.gitignore`, лента;
код и канон агентов не затрагиваются. Приёмка `validator` не привлекалась:
правка не касается кода/канона, проверка — прямой тест игнорирования
(`git check-ignore`); при необходимости владелец может запросить приёмку.

---

## сервисная сессия · 30.09.2026 · готово

- Сделано: `.gitignore` (корень, строка 5) и `.opencode/scripts/pm/.gitignore`
  (строка 5) — `.ruff_cache/`; каталог `.ruff_cache/` в корне удалён (кэш,
  пересоздаётся; ruff дополнительно пишет собственный внутренний `.gitignore`).
- Проверки: `git check-ignore -v .ruff_cache/CACHEDIR.TAG` →
  `.gitignore:5:.ruff_cache/`; `git check-ignore -v
  .opencode/scripts/pm/.ruff_cache/x` → `pm/.gitignore:5`; `git status` — только
  ожидаемые `M` (`.gitignore`, `pm/.gitignore`, `progress.yaml` — висящая запись
  волны `service-pm-numbering`); `Test-Path .ruff_cache` → False.
- Дальше / риски: пакет `git` (коммит + push) по гейту владельца; при следующем
  запуске ruff кэш появится снова — уже вне git.

---

## сервисная сессия · 30.09.2026 · гейт пакета — подтверждено

- Подтверждение владельца (`question`): **коммит + push** — микропакет `service-hygiene-ruff`.
- База: `develop` @ `03c20d8` (= `origin/develop`); режим — коммит прямо в `develop`, затем `git push origin develop`.
- Пакет (5 путей: 4 волны + `memory/git.md` по F43):
  - `.gitignore`, `.opencode/scripts/pm/.gitignore` (строка `.ruff_cache/`);
  - `.opencode/mail/service-hygiene-ruff.md` (новая лента);
  - `.opencode/state/current/progress.yaml` (висящая запись `service-pm-numbering`).
- Сообщение коммита: `chore: .ruff_cache в .gitignore (корень и pm)`.
- Запись роли `git` (лента + `memory/git.md`) формируется до `add` и входит в коммит (F43); после `push` в отслеживаемые файлы не пишем.

---

## git · 30.09.2026 · готово

- Сделано: подтверждение сверено по ленте (запись «гейт пакета — подтверждено»);
  база `develop` = `origin/develop` = `03c20d8` (синхрон); дерево до записей —
  ровно 4 пути пакета (`M` 3, `??` 1), посторонних нет. Записи F43 (эта лента и
  `memory/git.md`) сделаны до `add`; пакет — 5 точных путей, один коммит
  `chore: .ruff_cache в .gitignore (корень и pm)` прямо в `develop`, затем
  `git push origin develop` (таймаут ≥ 5 мин).
- Проверки: `git status -sb` → `## develop...origin/develop` без расхождения;
  `git log -1 --oneline` → `03c20d8`; дальше staged-сверка
  `node .opencode/scripts/git-check.mjs --staged`.
- Дальше / риски: хеши коммита и `push` — в ответе `lead` (F43, post-push
  git-команд нет).
