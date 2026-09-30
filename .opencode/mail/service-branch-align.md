# Сервисная лента: выравнивание веток master/develop (service-branch-align)

**Назначение:** служебная операция по решению владельца (01.10.2026): `develop`
выравнивается по `master` (развилка вскрыта волной `service-statuses-review`:
`develop` — предок `master`, но деревья разошлись из-за слияния `9173fc4`).

## сервисная сессия · 01.10.2026 · открытие и пакет (гейт — решение владельца)

- Решение владельца (01.10.2026, ответ на итог волны `service-statuses-review`):
  «выравниваем» — `develop` выравнивается по `master`. В `develop` приезжают
  T-03-код, R2-строка `AGENTS.md` и правки канона (дубли и §10-пункты сняты);
  из `develop` в `master` ничего не переносится — линия `develop` полностью
  содержится в `master`; дубль-абзац `migrator.md` и §10-пункт, остававшиеся в
  `develop`, исчезают вместе с выравниванием.
- База: `master` @ `3990700` (= `origin/master`); `develop` @ `3d572f3`
  (= `origin/develop`) — предок `master` (merge-base `3d572f3`).
- Пакет (санкция владельца — «выравниваем»):
  1. коммит записей (`chore(process)`: эта лента, `progress.yaml`, чекпойнт
     `memory/git.md`) в `master` + `git push origin master`;
  2. `git switch develop` → `git merge --no-ff master -m "Слияние master в develop
     (выравнивание веток)"` (канон: слияние с сохранением истории) →
     `git push origin develop`.
- Ожидание: деревья идентичны (`git diff develop master` — пусто); `develop`
  опережает `master` на merge-коммит выравнивания; `origin/develop` догоняет
  `origin/master`.

## git · 01.10.2026 · сверка и команды — готово

- Сделано (F43 — записи до `add`): сверка базы перед пакетом —
  `git status -sb` → `## master...origin/master`, дерево: 1 `M`
  (`progress.yaml`) + 1 `??` (лента) = ровно пакет; `git log -1 --oneline` →
  `3990700`. Отчёт роли и чекпойнт памяти — этими записями.
- Команды пакета: `git add` ровно 3 пути (`./.opencode/mail/service-branch-align.md`,
  `./.opencode/state/current/progress.yaml`, `./.opencode/memory/git.md`); сверка
  staged — `git diff --cached --name-status` → ожидается 1 `A` + 2 `M`;
  `git commit -m "chore(process): выравнивание develop по master — записи
  (service-branch-align)"`; `git push origin master`; `git switch develop` +
  `git status -sb`; `git merge --no-ff master -m "Слияние master в develop
  (выравнивание веток)"`; `git push origin develop`.
- Проверки: сверка базы — `git status -sb`, `git log -1 --oneline` (см. выше).
- Дальше / риски: расхождение снимка или конфликт merge — останов и возврат
  факта без частичного продолжения; после `push` в отслеживаемые файлы не
  писать (F43), хеши — в ответе `lead`.
