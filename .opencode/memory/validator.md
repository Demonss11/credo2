# Память: validator (приёмка)

- **Канон:** `.opencode/rules/review.md`; `docs/BRIEF.md` §5.3/§7.
- **Правило:** чекпойнт **до** прогона тестов (что проверено, что запускаю) и
  **после** (результат, вердикт) — тесты прерывают сессию (R5). Кратко.

## Чекпойнты

- **2026-09-28 · приёмка «service-t11-closeout» (итог):** вердикт **принято с
  замечаниями**; P1/P2 нет; P3 — `docs/features/README.md:291-292` (легенда `🟡`
  «поведение не подтверждено» против строки `agents-rework` с фактом Run 4).
  **Критерий T-11 подтверждён 4/4:** права (`cargo test *` только `validator.md:20`;
  r3 + свежий аудит W8), память 11 ролей + `service.md`, resume `sessionID` 3×,
  пилот Run 4/T-03 (отчёты rework/`-r2`, квитанция `iteration 2`), `0a5832f`/
  `5019c45` в `develop`. Согласованность T-15/T-16/D38/фичи/счётчики 47-278 —
  ок; границы (`agents|rules/**`, `AGENTS.md`, `src`, `tests`) не тронуты;
  `rustfmt.toml` — только комментарий. DoD: `cargo fmt --check` pass,
  `cargo test --test features_inventory` 4/4; полный `--all` не гонялся
  (`src/tests` неизменны с W8-config). Отчёт
  `docs/reviews/T-11-closeout-2026-09-28.md`; квитанция `T-11-closeout` append.
  Урок: `git branch --contains` вне allow-list `validator` — заменять `git log`.
- **2026-09-28 · приёмка сервисного пакета «service-t11-closeout» (до прогона):**
  прочитаны лента операции, карточки T-11/T-15/T-16, `review.md`, отчёты r3/T-03,
  меморандумы W8 т.1/т.2 §0, реестр находок, `receipts.yaml`. Снимок: ветка
  `develop`, HEAD `bed3019` + рабочее дерево; изменены 10 файлов
  (`docs/**`, 2 памяти, `rustfmt.toml`) + новые `mail/service-t11-closeout.md`,
  `docs/tasks/T-16-stale-check-test/`; `.opencode/agents|rules/**`, `AGENTS.md`,
  `src/**`, `tests/**` не тронуты (`git status --porcelain`). `git diff -- src
  tests` пусто. Критерий T-11: права (debug agents, frontmatter — `cargo test *`
  только `validator`), память 11 ролей + `service.md`, пилот Run 4/T-03 (отчёты
  rework/`-r2`, квитанция iteration 2), коммиты `0a5832f`/`5019c45` в истории
  `develop` (`git log develop -50`), 6 шапок `agents-*.feature` с `# D38 (Q43)`,
  счётчики 47/278 (`features/README.md:300`). Далее: `cargo fmt --check` +
  `cargo test --test features_inventory` (README фич правлен текстом); полный DoD
  не гоняю — `src/**`/`tests/**` неизменны с W8-config. `git branch --contains`
  отклонён движком прав — заменил на `git log develop`.
- **2026-09-28 · приёмка сервисного пакета «C13 (F43) + T-15 + очистка логов»
  (в работе):** прочитаны `git-workflow.md`, `git.md`, `AGENTS.md` (§«Память и
  почта»), карточка T-15, `findings-registry.md`, `receipts.yaml`, `r3`-лента.
  Снимок `git status`: 9 удалений `mail/**`, 11 сбросов `memory/*.md`, правки
  канона (`AGENTS.md`, `git.md`, `git-workflow.md`), доков (реестр, T-15),
  новый `mail/service-mcp-ready-r3.md`; HEAD `9006ca1`. Далее: `opencode debug
  agents` ×2 (11 ролей, steps/права), сверка diff канона. Полный DoD не гоняю:
  `src/**`/`tests/**` с `W8-canon` не менялись — зафиксирую.
- **2026-09-28 · приёмка сервисного пакета «C13 (F43) + T-15 + очистка логов»
  (итог):** вердикт **принято с замечаниями**; P1/P2 нет; P3 — `T-15/README.md:126`
  («`git status` пуст» vs канон «правок роли `git` нет»). Проверено: `debug
  agents` ×2 (11 ролей, steps без изменений, права не тронуты), `git diff` канона
  (тело, не фронтматтеры), якорь §«Пакет и подтверждение» резолвится, `review.md`
  не менялся, F43/F15/F27 ↔ C13 согласованы, `mail`/`memory` очищены. `src/**`/
  `tests/**` == `W8-canon` (`22f7683..9006ca1` пусто) — DoD не перезапускался.
  Отчёт `docs/reviews/T-15-c13-2026-09-28.md`; квитанция `T-15-c13` append.
  Урок: бэкап `clean-logs` лежит в `%TEMP%` — роли недоступен
  (`external_directory: deny`); принимать только по строке вывода, писать это в отчёт.
- **2026-09-28 · адресная проверка `docs/questions/README.md`** (сводка журнала,
  `migrator`, untracked; HEAD `9006ca1`): полнота 12 строк ↔ 12 файлов `Q1`,
  `Q43–Q53`; шапка Q41 ok; ссылки живые; стиль ↔ `tasks/README.md` ok. Вердикт
  **принято с замечаниями**: P2 `questions/README.md:30` — Q47 «Связано: T-15»
  против `TRACEABILITY.md:13` (`—`) и `D42:10,54` («Задач не требуется»); P3
  `README.md:13–14` — указатель на `BRIEF.md` §5.1–§5.2 косвенный (сводка там не
  упомянута). Итог — в `mail/service-mcp-ready-r3.md`; не коммичено.
- Чекпойнтов ещё не было до этой записи.
- **2026-09-28 · приёмка сервисного пакета «service-permissions» (Q54/D49,
  чекпойнт до прогона):** прочитаны лента `service-t11-closeout.md`, отчёты
  `migrator`/`auditor`, канон `validator.md:28` и `review.md:81`,
  Q54/D49, SPEC §10 №49 (`:870`), `TRACEABILITY.md:20`,
  `questions/README.md:37`, `findings-registry.md:52` (F44). Снимок: `develop`,
  HEAD `6d4c840` + рабочее дерево (10 M + 2 `??`); `git diff -- ./.opencode` —
  ровно одна строка allowlist `validator` + синхронизация `review.md`;
  `git diff -- src tests` пусто; `AGENTS.md`/`opencode.json` не тронуты.
  Далее: `opencode debug agents` (один прогон, срез token-guard возможен) +
  смоук `git branch --contains 0a5832f`. Полный DoD не гоняю:
  `src/**`/`tests/**` неизменны (обоснование в отчёте).
- **2026-09-28 · приёмка «service-permissions» (Q54/D49, итог):** вердикт
  **принято**, P1/P2/P3 нет. Канон: `validator.md:28` — ровно одна строка
  allowlist `git branch --contains *`, `review.md:81` синхронизирован; иных
  правок канона нет. Машинный резолв (`opencode debug agents`, один прогон,
  без `reload`; срез token-guard ~36.9/51.4 КБ, блок `validator` прочитан) —
  право видно. **Смоук прошёл:** `git branch --contains 0a5832f` → `* develop`,
  `exp/agent-cycle-rerun`, `exp/agent-update-t15w0` (право подхватилось в
  свежей сессии). Журнал Q54/D49/SPEC §10 №49/TRACEABILITY:20/questions
  README:37/F44 согласованы, ID уникальны, ссылки живые. P3 аудитора закрыт —
  F44 ссылается на созданный `docs/reviews/service-permissions-2026-09-28.md`.
  DoD: `src/**`/`tests/**` неизменны с W8-config — `cargo test/fmt/clippy` не
  перезапускались (правка канона/прав → машинная сверка прав). Квитанция
  `service-permissions` iteration 1 — append. **Урок снят:** `git branch
  --contains` теперь штатное право роли — косвенную замену `git log` больше
  не применять.
