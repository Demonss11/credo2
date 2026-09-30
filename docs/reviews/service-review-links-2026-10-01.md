# Приёмка: снятие ссылки на review из карточки T-03 (service-review-links)

**Проверка:** сервисная операция по запросу владельца — снятие отхождения от
канона: из карточки `docs/tasks/T-03-check-create/README.md` удалена строка
«Отчёт приёмки: …» со ссылкой на `../../reviews/T-03-2026-09-26.md` (канон —
`.opencode/rules/review.md` §«Хранение отчётов»; D65).

**Версия:** ветка `develop` @ `b9fd791` (= `origin/develop` = `HEAD`) +
рабочее дерево (01.10.2026).

**Вердикт:** принято.

## P1

—

## P2

—

## P3

—

## Проверки

| Команда | Результат |
|---|---|
| `git diff -- docs/tasks/T-03-check-create/README.md` | ровно один ханк: 1 удалённая строка (`- **Отчёт приёмки:** [.../reviews/T-03-2026-09-26.md]`), 0 добавлений; шапка карточки, включая `- **Источник:** Q28 (SPEC §10 №31).` (стк. 6), не тронута (D70) |
| `git diff --stat -- docs/tasks` | `1 file changed, 1 deletion(-)` |
| `git diff --numstat` | `0 1 docs/tasks/T-03-check-create/README.md`; `11 0 .opencode/memory/migrator.md`; `10 0 .opencode/memory/validator.md` |
| `git grep -n "reviews/" -- docs/tasks` | только упоминания папки-зоны: T-18 (47/64/75), T-19 (34) — адресных ссылок нет |
| `git grep -n "reviews/" -- docs/questions` | пусто (exit 1) |
| `git grep -n "reviews/" -- docs/decisions` | D64 (47/53), D65 (50/91), D66 (37) — упоминания зон `docs/reviews/**`, не адреса |
| `git grep -n "reviews/" -- docs/features` | пусто (exit 1) |
| `git grep -n "reviews/" --` по `docs/*.md` (SPEC, README, TRACEABILITY, CHANGELOG, GRAMMAR) | пусто, кроме `docs/README.md:21,25` — карта зоны/роль (упоминание папки, не адрес) |
| `git grep -n "reviews/T-" -- docs` | только внутри `docs/reviews/**` (улики-снимки), вне канона |
| `git grep "Отчёт приёмки" -- docs/tasks` | нет совпадений |
| `git status --porcelain` | ` M .opencode/memory/migrator.md`, ` M .opencode/memory/validator.md`, ` M docs/tasks/T-03-check-create/README.md`, `?? .opencode/mail/service-review-links.md` (до моих записей — 3 пути по плану + мой чекпойнт памяти) |
| `git diff --stat -- src` / `tests` / `Cargo.toml` / `AGENTS.md` | пусто (по каждому пути) |
| `git diff --stat -- ./.opencode/agents` / `./.opencode/rules` | пусто (по каждому пути) |
| `git status -sb` | `## develop...origin/develop` |
| `git log -1 --oneline` | `b9fd791 Слияние master в develop (выравнивание веток)` |
| `git rev-parse HEAD develop origin/develop` | все три = `b9fd791c2fdf0cc15d93d4ee64c5b1d1b7e2b16d` |

**DoD (D50):** пакет документный — изменений `src/**`, `tests/**`,
`Cargo.toml` нет; счётчики/состав сценариев `docs/features/**` не затронуты;
`cargo fmt/clippy/test` не запускались — обоснованно
(`.opencode/rules/review.md` §«Порог существенности»).

## Что проверено и ок

- **Правка карточки** — ровно одна удалённая строка, добавлений нет; остальной
  текст (`Статус`, `Приоритет`, `Зависит от`, `Источник: Q28 (SPEC §10 №31).` —
  историческая помета D70, «Что сделать», «Критерий готовности») не тронут.
- **Ссылочная политика (D65)** — адресных ссылок на `docs/reviews/**` в
  `docs/questions`, `docs/decisions`, `docs/features`, `docs/tasks`,
  `docs/*.md` не осталось; все совпадения — имена папок-зон, что канон
  допускает (T-18:49 «упоминания папок как зон — не ссылки»).
- **Улики сохранены** — `docs/reviews/T-03-2026-09-26.md` и `-r2.md` на месте
  (отчёт — улика, архив — git).
- **Границы пакета** — `src/tests/Cargo.toml/AGENTS.md`,
  `.opencode/agents`, `.opencode/rules` чисты; прочих правок нет.
- **Ветка/база** — `develop` синхронна с `origin/develop`, `HEAD` = `b9fd791`,
  соответствует плану.

## Примечания

- `git diff --stat -- src tests Cargo.toml AGENTS.md .opencode/agents
  .opencode/rules` движок отклонил (известные грабли: несколько dot-путей под
  `--`); заменено одиночными вызовами по каждому пути — все пусты.
