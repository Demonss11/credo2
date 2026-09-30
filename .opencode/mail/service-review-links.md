# Сервисная лента: снятие ссылки на review из карточки T-03 (service-review-links)

**Назначение:** служебная операция по запросу владельца (01.10.2026): карточка
`docs/tasks/T-03-check-create/README.md` содержит запрещённую ссылку на отчёт
приёмки (`../../reviews/T-03-2026-09-26.md`) — отхождение от канона
(`.opencode/rules/review.md` §«Хранение отчётов»: ссылки на `docs/reviews/**`
из вопросов, решений, задач, фич и кода не допускаются; D65).

## сервисная сессия · 01.10.2026 · открытие

- Запрос владельца: «правим отхождение от канона» — снять ссылку в карточке
  T-03; работа — на ветке `develop` (переход зафиксирован владельцем).
- Сверка (до правки): ссылок-адресов на `docs/reviews/**` в каноне — одна:
  `docs/tasks/T-03-check-create/README.md:7` («Отчёт приёмки: …»); остальные
  упоминания (`T-18`, `T-19`, `D64`, `D65`, `D66`) — папки-зоны, допустимы.
- Правка: удалить строку «Отчёт приёмки: …» целиком (паттерн почищенных
  карточек T-01/T-11/T-12/T-13/T-16 — строки нет; отчёт — улика, архив — git).
- Маршрут: `migrator` (правка карточки) → `validator` (приёмка) → гейт →
  `git` (коммит + push `develop`). Новых Q/D нет (политика — D65).

## migrator · 01.10.2026 · готово

- Сделано: из карточки `docs/tasks/T-03-check-create/README.md` удалена строка 7
  `- **Отчёт приёмки:** [.../reviews/T-03-2026-09-26.md]` целиком (основание —
  `review.md` §«Хранение отчётов», D65). Остальной текст карточки, включая
  `Источник: Q28 (SPEC §10 №31).`, не тронут (D70). Новых Q/D нет.
- Проверки:
  - `git diff -- docs/tasks/T-03-check-create/README.md` → ровно один ханк,
    1 удалённая строка (шапка карточки), добавлений нет.
  - `git diff --stat …` → `1 file changed, 1 deletion(-)`.
  - `git grep -n "reviews/" -- docs/tasks` → осталось 4 упоминания папок-зон
    (T-18: строки 47/64/75, T-19: строка 34); адресных ссылок нет.
  - `git status --porcelain` → ` M docs/tasks/T-03-check-create/README.md`,
    `?? .opencode/mail/service-review-links.md` (лента операции), плюс мой
    чекпойнт в памяти после записи — прочих путей нет.
- Дальше / риски: передать `validator` на приёмку (правка только в зоне
  карточки, новых Q/D и правок канона нет); затем гейт пакета → `git`.
  `cargo` не запускался (D50). Риск низкий: удаление одной декларативной
  строки, семантика карточки не меняется.

## validator · 01.10.2026 · чекпойнт ДО прогона

- Проверяю: `git diff -- docs/tasks/T-03-check-create/README.md` (1 удаление,
  0 добавлений; «Источник: Q28 …» цел — D70); `git grep -n "reviews/" -- docs/tasks`
  (только зоны T-18/T-19); адресные ссылки на `docs/reviews/**` по канону —
  пусто; `git status --porcelain` (3 пути); границы `git diff --stat -- src tests
  Cargo.toml AGENTS.md .opencode/agents .opencode/rules` — пусто; ветка/база
  `git status -sb` + `git log -1 --oneline`.
- План: `cargo` не запускаю (D50, пакет документный). Итог — после прогона.

## validator · 01.10.2026 · принято (P1/P2/P3 нет)

- Вердикт: **принято**. Отчёт `docs/reviews/service-review-links-2026-10-01.md`;
  квитанция `service-review-links` iteration 1 `accepted` — в
  `.opencode/state/current/receipts.yaml`. Версия: `develop` @ `b9fd791`
  (= `origin/develop` = `HEAD`) + рабочее дерево (01.10.2026).
- Проверки:
  - `git diff -- docs/tasks/T-03-check-create/README.md` → один ханк,
    **1 удаление, 0 добавлений** (снята строка «Отчёт приёмки: …»); шапка,
    включая `Источник: Q28 (SPEC §10 №31).` (:6), цела (D70).
  - `git grep -n "reviews/" -- docs/tasks` → только зоны T-18 (47/64/75),
    T-19 (34); `docs/questions`/`docs/features` — пусто; `docs/decisions` —
    только зоны (D64/D65/D66); `docs/*.md` — только `README.md:21,25`;
    `git grep "reviews/T-" -- docs` → только внутри `docs/reviews/**`.
    Адресных ссылок на `docs/reviews/**` в каноне нет.
  - `git status -sb` → `## develop...origin/develop`; `git log -1 --oneline`
    → `b9fd791`; `git rev-parse HEAD develop origin/develop` → все
    `b9fd791` (= план).
  - Границы: `git diff --stat` по `src`/`tests`/`Cargo.toml`/`AGENTS.md`/
    `./.opencode/agents`/`./.opencode/rules` — пусто (многоточечный вызов
    отклонён движком, заменён одиночными).
- DoD: `cargo` не запускался — пакет документный, `src/tests/Cargo.toml`
  не тронуты (D50; `review.md` §«Порог существенности»).
- Состав пакета для гейта: `M docs/tasks/T-03-check-create/README.md`
  (содержательная правка); `M .opencode/memory/migrator.md`,
  `M .opencode/memory/validator.md` (чекпойнты F35); `?? docs/reviews/
  service-review-links-2026-10-01.md`, `?? .opencode/mail/service-review-links.md`,
  `M .opencode/state/current/receipts.yaml` (отчёт/лента/квитанция).
  Продуктовый код не менялся.
- Риски: нет.

## сервисная сессия · 01.10.2026 · подтверждение пакета (гейт)

- Гейт пройден: владелец подтвердил **коммит + push** (`question`, ответ
  «Коммит + push (Recommended)») — операция `service-review-links`.
- База: `develop` @ `b9fd791` (= `origin/develop`); режим — коммит в `develop`,
  затем `git push origin develop`.
- Пакет — 7 путей + запись роли `git` (F43, `memory/git.md`, новый до `add`) = 8:
  - `docs/tasks/T-03-check-create/README.md` (содержательная правка)
  - `.opencode/mail/service-review-links.md` (лента)
  - `.opencode/memory/git.md` (чекпойнт роли `git`)
  - `.opencode/memory/migrator.md`
  - `.opencode/memory/validator.md`
  - `.opencode/state/current/progress.yaml`
  - `.opencode/state/current/receipts.yaml`
  - `docs/reviews/service-review-links-2026-10-01.md` (отчёт приёмки)
- Сообщение коммита:
  `docs(T-03): снята запрещённая ссылка на отчёт приёмки из карточки (review.md §«Хранение отчётов»)`
- Состояние: приёмка `validator` — принято (P1/P2/P3 нет).

## git · 01.10.2026 · готово

- Сделано (до `add`, F43): сверка базы — `git status -sb` → `## develop...origin/develop`,
  дерево 5 `M` + 2 `??` = ровно пакет; `git log -1 --oneline` → `b9fd791`
  (совпало с планом). Записи роли `git` (эта + чекпойнт в `memory/git.md`) —
  в пакет до `add`. Далее: `git add` 8 путей → сверка staged (6 `M` + 2 `A`) →
  коммит `docs(T-03): снята запрещённая ссылка на отчёт приёмки из карточки
  (review.md §«Хранение отчётов»)» → `git push origin develop`.
- Проверки: `git status -sb`; `git log -1 --oneline` (расхождений нет).
- Дальше / риски: хеши — в ответе `lead` (F43, здесь не хранятся); после `push`
  в отслеживаемые файлы не писать.
