# Приёмка: service-docs-lifecycle, операция №2 — служебная зона (канон агентов)

**Проверка:** блок приёмки операции №2 волны `service-docs-lifecycle` — правки
канона агентов и сопутствующие записи (пункты 1–7 брифа): `review.md`
§«Хранение отчётов» + §«Доступные команды» (`git rev-parse`); `journal.md` §8
(антипаттерн «время жизни адреса» + исключение реестра); `docs-writer.md` шаг
«после приёмки»; `validator.md` фронтматтер; памяти `validator`/`git`/
`researcher`/`migrator`/`auditor`; `D49` «Обновление 30.09.2026»; `Q54`
терминальная пунктуация (:12/:15).
**Версия:** `develop` @ `21c3c80` (= `origin/develop` = `HEAD`) + рабочее дерево
(2026-09-30).
**Вердикт:** принято

**P1:** —
**P2:** —
**P3:** —

Критичных проблем нет. Закрыты обе находки независимого аудита (P2 —
`journal.md` §8 ↔ D65 п.4; P3 — `Q54.md:12,15`).

## Проверки

- **База.** `git rev-parse develop origin/develop HEAD` →
  `21c3c80` / `21c3c80` / `21c3c80` (право работает; «ловушка» из памяти
  `validator` снята).
- **Границы.** `git status --porcelain` → в `.opencode/**` изменены ровно:
  `.opencode/agents/docs-writer.md`, `.opencode/agents/validator.md`,
  `.opencode/rules/journal.md`, `.opencode/rules/review.md`, память
  `auditor`/`git`/`migrator`/`validator`, лента `service-docs-lifecycle.md`.
  `git diff --stat -- src tests Cargo.toml` → **пусто**; `git diff --stat --
  AGENTS.md` → **пусто** (`git diff --numstat` подтверждает посрочно).
- **`review.md` (diff).** §«Хранение отчётов»: снят пример
  `(T-01-2026-09-26.md)`; фраза «при закрытии задачи `docs-writer` ставит ссылку
  на отчёт…» заменена на «ссылка на отчёт из карточки **не ставится**: ссылки на
  `docs/reviews/**` из вопросов, решений, задач, фич и кода не допускаются;
  отчёт остаётся уликой (архив — git), факты и статусы — в каноне (D65)» —
  совпадает с D65 «Обновление 30.09.2026». §«Доступные команды»: строка
  `validator` дополнена `git rev-parse` (`git branch --contains` на месте).
- **`journal.md` §8 (diff).** В перечень «время жизни адреса» добавлены
  `docs/research/**`, `docs/reviews/**`, `docs/analysis/**` + оговорка
  исключения: «исключение — живой реестр `findings-registry.md`, D48». Ссылки
  живые (`glob`: `docs/analysis/findings-registry.md`,
  `docs/decisions/D48-findings-registry-owner.md` существуют).
- **Инструкция ↔ права.** Фронтматтер `.opencode/agents/validator.md:29–30` —
  ровно строки `git branch --contains *` и `git rev-parse *`; совпадают с
  `review.md:88–89`. `node .opencode/scripts/agents-perms.mjs` ×2 → идентично,
  `agents: 11 из 18`, у `validator` — `allow:git rev-parse *` (и
  `allow:git branch --contains *`); прочие роли без сдвигов.
- **Согласованность канона.** `docs-writer.md:69–71` (шаг «после приёмки») —
  ссылка на отчёт не ставится, отсылка к `review.md` §«Хранение отчётов»; не
  противоречит `review.md`/D65. `D49` — «Обновление 30.09.2026 — право
  `git rev-parse *`» на месте (`D49:42–49`), синхронизация `review.md` + память
  `validator` указана. Ссылка `review.md` → `D65` (`../../docs/…`) резолвится.
- **`Q54` (diff `-U0`).** Добавлены ровно две точки: строка 12 (конец «Связано»,
  `…(`.opencode/mail/**`).`) и строка 15 (конец «Источник»,
  `…(`.opencode/mail/**`).`); прочие строки diff — прежние снятия ссылок
  итерации 1 (вне этой приёмки). P3 аудита закрыт.
- **`D49` (diff).** Кроме «Обновления» — снятие ссылок на удаляемый
  `docs/reviews/T-11-closeout-2026-09-28.md` (итерация 1, вне этой приёмки);
  `git grep -n "T-11-closeout-2026-09-28\|T-01-2026-09-26\|T-11-2026-09-26\|
  T-12-2026-09-27\|T-13-2026-09-27" -- docs/decisions docs/questions docs/tasks`
  → **пусто** (ссылок канона на удаляемые отчёты не осталось).
- **Память (F35).** `validator.md` — «ловушка» заменена правом
  `git rev-parse` (согласована с `review.md`/`validator.md`); `git.md` — урок
  quoting согласован с `git-workflow.md` (без противоречий); `auditor.md`,
  `migrator.md` — чекпойнты (append, факты сходятся); записи чужих ролей не
  переписаны (диффы — только целевые строки + блоки-чекпойнты).
- **Лента.** Записи волны по порядку (итерации 1–8, синхронизация канона,
  право `rev-parse`, аудит, P3 `Q54`); факты сходятся с деревом. Открытые хвосты
  заявлены: 68 ` D docs/reviews/**` и карточки T-01/T-11/T-12/T-13/T-16 — вне
  этой приёмки (волна продолжается).
- **DoD.** `cargo fmt|clippy|test` **не запускались** (D50; в пакете нет
  `src/**`/`tests/**`/`Cargo.toml`). Счётчики/состав `docs/features/**` не
  правились → `cargo test --test features_inventory` не требуется.

## Технические проблемы

Отклонённые движком прав команды (сужены и повторены):
`git diff -- .opencode/rules/review.md .opencode/rules/journal.md …` (несколько
dot-путей под `--`) и `git diff --numstat -- .opencode` — заменены на одиночный
путь с префиксом `./` и на `git diff --numstat` соответственно (квик описан в
`review.md` §«Доступные команды»). Срезов вывода не было.

## Что проверено и ок

- `review.md` §«Хранение отчётов» и §«Доступные команды» — соответствуют брифу и
  D65/D49.
- `journal.md` §8 — антипаттерн + оговорка реестра (D48), ссылки живые; P2
  аудита закрыт.
- `docs-writer.md` шаг «после приёмки» — согласован с `review.md`.
- `validator.md` фронтматтер ↔ `review.md` ↔ `agents-perms.mjs` ×2 — расхождений
  нет.
- `D49` «Обновление 30.09.2026» на месте; `Q54:12,15` — две точки; P3 закрыт.
- Памяти `validator`/`git`/`researcher`/`migrator`/`auditor` — F35, чужих
  записей не переписано.
- Границы: `src/**`, `tests/**`, `Cargo.toml`, `AGENTS.md` не тронуты.
