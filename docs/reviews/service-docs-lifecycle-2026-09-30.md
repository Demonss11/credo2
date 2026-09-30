# Приёмка: service-docs-lifecycle, операция №1 (`research`)

**Проверка:** сервисная волна разбора `docs/research/` — обновление политики
D65, снятие 18 ссылок канона на удаляемый файл, синхронизация `docs/README.md`,
память `researcher`, удаление `doc-quality-checks-2026-09-29.md`.

**Версия:** база `develop` @ `9948ebf` (= `origin/develop`, `git log -1`) +
рабочее дерево 30.09.2026. `git status --porcelain`: 12 `M` + ` D docs/research/
doc-quality-checks-2026-09-29.md` + `?? .opencode/mail/service-docs-lifecycle.md`
(плюс сам этот отчёт и квитанция — после приёмки). Версия совпадает с
заявленной в брифе.

**Вердикт:** принято

**P1:** — (нет)
**P2:** — (нет)
**P3:** — (нет)

## Проверки

- `git status --porcelain` → ровно ожидаемый набор; каталожные и статусные
  файлы (`questions/README.md`, `decisions/README.md`, `TRACEABILITY.md`,
  `SPECIFICATION.md`, `CHANGELOG.md`, `features/README.md`,
  `analysis/findings-registry.md`) — без изменений.
- **Границы:** `git diff --stat -- src tests Cargo.toml` → пусто;
  `git status --porcelain .opencode/agents .opencode/rules AGENTS.md
  docs/features` → пусто. `cargo fmt|clippy|test` **не запускались** — кода нет
  (D50; `review.md` §«Порог существенности»: пакет без `src/tests/Cargo.toml`).
- **Ссылки:** `git grep -n "doc-quality-checks-2026-09-29" -- docs` → только
  `docs/reviews/doc-tools-d66d68-2026-09-29.md:17,69` (зона волны №2 — снимок,
  append-only); `git grep -n -e "\.\./research/" -- docs/questions docs/decisions
  docs/tasks docs/features docs/README.md` → пусто (exit 1). Остаточные
  упоминания `research/` — только зоны/скоуп: `docs/README.md:22,25`,
  `D65:89` (текст «Обновления»), `T-19:35` (scope-строка) — адреса файла нет.
- **Хвосты:** `git grep -n -e "Обзор:" -e "обзор-"` по правкам → только «обзор
  `glob …`» (факт scan'а) и «Опора: обзор doc-quality-проверок» (провенанс без
  адреса) — висячих адресов нет.
- **Каталоги/целостность:** `questions/README.md:84–86`, `decisions/README.md:
  95–97`, `TRACEABILITY.md:66–68` согласованы (Q62↔D66→T-19 ⬜, Q63↔D67, Q64↔D68);
  ID не переиспользованы; миграционных маркеров (`миграц`, `TODO`, `FIXME`,
  `XXX`) в правках нет (единственное совпадение `D68:33` — предсуществующий
  текст «миграционных правок не требуется»).
- **D65:** «Обновление 30.09.2026» на месте (`D65:88–99`), уточняет п.4 явно;
  п.4 (`D65:46–69`) не сломан, противоречий нет; внутренние ссылки живые —
  `../analysis/findings-registry.md` (файл есть), `D48`, `D64`, `T-18` (пути
  существуют). «Сверка с кодом» (`D65:101+`) сохранена.
- **Права:** `node .opencode/scripts/agents-perms.mjs` дважды → `agents: 11 из 18
  (--all — все)` оба раза, расхождений нет (агенты/правила не менялись).
- **Архив:** `git show develop:docs/research/doc-quality-checks-2026-09-29.md` —
  файл целиком в истории git (160 строк), удаление восстановимо.
- **Лента:** порядок записей (открытие → migrator → docs-writer → служебная
  зона → auditor) сходится с фактами дерева; расчёт «18 ссылок в 7 файлах»
  совпал с диффом (`Q62×2, Q63×2, Q64×1, D66×4, D67×4, D68×3, T-19×2`).

## Что проверено и ок

- Политика D65 (обновление п.4) — согласована, ссылки живые.
- 18 снятых ссылок в Q62/Q63/Q64/D66/D67/D68/T-19 — хвостов и битых путей нет.
- `docs/README.md:25–28` — формулировка синхронизирована, ссылка на D65 живая.
- Память `researcher.md:3–5` — «Файлы: …не канон», без расхождения с каноном
  роли (F35).
- Удаление `docs/research/doc-quality-checks-2026-09-29.md` — архив в git есть.
- Границы (`src/tests/Cargo.toml`, `.opencode/agents`, `.opencode/rules`,
  `AGENTS.md`, `docs/features`) — не тронуты; права — без расхождений.
- Каталоги, `TRACEABILITY.md`, ID, статусы — не затронуты.

**Технические проблемы:** команда `git rev-parse develop origin/develop HEAD`
отклонена движком прав; заменена на `git log --oneline -1` (база `9948ebf`
подтверждена, совпадает с заявленной `develop = origin/develop`). Иных сбоев
команд не было.
