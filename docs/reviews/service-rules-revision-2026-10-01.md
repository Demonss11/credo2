# Приёмка: service-rules-revision, волна 1 (ревизия `.opencode/rules/**`)

**Дата:** 01.10.2026 · **Проверяющий:** `validator` · **Итерация:** 1
**Версия:** `develop` = `origin/develop` = `HEAD` @ `3821811`
(`3821811b4be31479f6b6e2825357ea355518ad85`) + рабочее дерево
**Вердикт:** **принято** (P1/P2/P3 нет)

## Пакет

Документно-канонный, без `T-XX`, база `develop`. 10 `M` + 3 `??` (до записей
приёмки):

- `M` `.opencode/rules/{journal,review,dispatch-loop,git-workflow}.md`,
  `AGENTS.md` (§«Служебная зона и аудит» — норма-запрет, +4 строки);
- `M` `docs/TRACEABILITY.md`, `docs/questions/README.md`,
  `docs/decisions/README.md`;
- `M` `.opencode/memory/{auditor,migrator}.md`;
- `??` `docs/questions/Q80.md`, `docs/decisions/D84-rules-revision.md`,
  `.opencode/mail/service-rules-revision.md`.

## Проверки

- **Ветка/база:** `git rev-parse develop origin/develop HEAD` → все три
  `3821811…`; `git status -sb` → `## develop...origin/develop` — база верна.
- **Диффы rules:** `git diff` по 4 файлам — только целевые правки: сняты
  декоративные `Dn/Qn`-ссылки (D71/D63/D60/D65/D70/D50/D64/D77/D79/D46/D81/Q41/
  Q8/Q9/D38–D41 и др.), исторические приписки (даты, «пробы T-13»×3, «Run 3»,
  «BRIEF», «28.09.2026»×2, «T-12/T-13/T-04», «T-15 фаза C…»), F-метки-декор
  (F35/F22/F30); примеры веток/коммитов → шаблоны (`feature/Dn-<слаг>`,
  `docs(Qn): …`). Нормы сохранены, дубли сведены к ссылкам на дом.
- **Якоря:** `rg R2|R7|R5 dispatch-loop.md` → `R2` :39 (§Hard rules), `R7` :59;
  `rg F43 git-workflow.md` → **ровно один** :77 (§«Пакет и подтверждение»);
  `rg T-18|⬜ journal.md` → :103 «(задача `T-18`)» без `⬜`; `⬜` в rules —
  только легенды/схемы (:28, :72 и др.).
- **Дедуп-ссылки резолвятся:** `rg "^## "` — дома `dispatch-loop.md`
  §«Команды и права» (:66), §«Hard rules» (:37); `git-workflow.md`
  §«Пакет и подтверждение» (:59), §«Минимальный цикл `git`» (:86);
  `review.md` §«Хранение отчётов» (:71). `git-workflow.md:89,95,151–161`
  ссылаются на эти разделы; `review.md:106,109,128` — на
  `dispatch-loop.md` §«Команды и права».
- **Формы путей / `git -C`:** дом — `dispatch-loop.md:68–75` (`./`-форма,
  запрет `git -C`); в `review.md:106`, `git-workflow.md:95,151` — ссылки.
- **Ссылки на решения:** `rg \[D\d|d/decisions|основание` в rules → только
  несущие в формате «основание — `Dn`»: `D82` (journal:30), `D65`×2
  (journal:116, review:84, dispatch-loop:129), `D48` (journal:116,
  dispatch-loop:131). Иных `Dn/Qn`-ссылок нет.
- **Техчасть `review.md`:** «Доступные команды» (:88–101) и абзац «не проверено
  и почему» (:109–115) целы; список ролей совпадает с фронтматтерами.
- **Закрытие находок аудита:**
  - **P2-1** — `journal.md:103` без `⬜` (проверено `rg`);
  - **P2-2** — `D84:39–41,57–60,72,91–96` — `R2/R7` (`dispatch-loop.md`),
    `R5` помечен как метка ролей/висячий якорь (волна 2), «Сверка» и п.6
    обновлены;
  - **P3** — `D84:26–27,51` — «основание — `Dn`» (код-форма), без гиперссылки.
- **Q80/D84:** формы журнала (Статус/Дата/Resolves/…), `Resolves` Q80↔D84
  взаимны; `TRACEABILITY.md:84` — строка Q80/D84 = `done`, «Реализация» =
  `.opencode/rules/**`, `AGENTS.md` §«Служебная зона и аудит» (волна 1);
  каталоги: `questions/README.md:102`, `decisions/README.md:113` — по 1 строке.
- **Ссылки Q80/D84:** `git ls-files` — цели D65/D48/D82/D63/D50/T-18/
  `findings-registry.md` существуют.
- **Права:** `node .opencode/scripts/agents-perms.mjs` ×2 → **`agents: 11 из
  18`**, выводы идентичны; фронтматтеры волной 1 не тронуты
  (`git diff --stat -- ./.opencode/agents` пусто).
- **Границы:** `git status --porcelain` — состав ровно плана (10 `M` + 3 `??`);
  `git diff --stat -- src tests Cargo.toml` — **пусто**;
  `git diff --stat -- ./.opencode/agents ./.opencode/rules/workspace.md
  ./.opencode/scripts` — пусто; `git diff --check` — пусто.
- **DoD:** `cargo` **не запускался** (D50 — пакет без `src/**`, `tests/**`,
  `Cargo.toml`, счётчиков `features/`).

## Находки

- **P1:** — (критичных проблем нет)
- **P2:** — (нет; P2-1/P2-2 аудита закрыты)
- **P3:** — (нет; P3 аудита закрыт)

## Итог

Ревизия волны 1 выполнена по политике Q80/D84: шум снят без потери норм,
несущие якоря (R2/R7, F43×1, живая `T-18` без `⬜`, `./`-формы и запрет
`git -C` как дом `dispatch-loop.md`) сохранены, решения — только `D82`/`D65`/
`D48` в формате «основание — `Dn`». Находки аудита закрыты; `R5` — disclosed
предсуществующий висячий якорь, долг волны 2 (не блокирует волну 1).
Пакет **принят** к гейту.
