# Приёмка: service-task-t27 — оформление задачи T-27 (реестр находок: место и раскол)

- **Проверка:** адресная документная сверка Q95 → D98 → T-27 + адресный
  `cargo test --test docs_journal` (F74) + `node .opencode/scripts/agents-perms.mjs`;
  полный DoD не требуется (D50 — документы/журнал; `src/**`, `tests/**`,
  `Cargo.toml` не тронуты).
- **Версия:** ветка `develop`, HEAD `0e5a158` + рабочее дерево (пакет операции
  не закоммичен; снимок 2026-10-03).
- **Вердикт:** **принято с замечаниями** (P1/P2 нет; P3 — один хвостовой
  whitespace в памяти `migrator`).
- **Отчёт:** `docs/reviews/service-task-t27-2026-10-03.md` (этот файл).

**P1:** — критичных проблем нет.

**P2:** — нет.

**P3:** `.opencode/memory/migrator.md:529` — `git diff --check` сообщает
`new blank line at EOF` (в файле два завершающих перевода строки, 529-я строка
пустая) → удалить хвостовую пустую строку (одна строка, память роли — не канон;
владелец — сервисная сессия, F35). Последствие — грязный `git diff --check` в
пакете операции; на семантику приёмки не влияет.

## Проверки

- `cargo test --test docs_journal` (exit 0) — **14 passed / 0 failed** (0.05s).
  Ожидание 14/0 подтверждено; прогон адресный — реестр/тест ещё не перенесены
  (это объём T-27).
- `node .opencode/scripts/agents-perms.mjs` (exit 0) — `agents: 11 из 18`;
  расхождений инструкция ↔ права нет.
- `git diff --numstat -- src tests Cargo.toml` — пусто (код не тронут).
- `git diff --name-only` — ровно 6 `M`:
  `.opencode/memory/auditor.md`, `.opencode/memory/migrator.md`,
  `docs/TRACEABILITY.md`, `docs/decisions/README.md`, `docs/questions/README.md`,
  `docs/tasks/README.md`. Новые (`??`): лента r20, `D98`,
  `Q95`, `docs/tasks/T-27-findings-registry-split/`. Канон
  (`AGENTS.md`, `.opencode/rules/**`, `.opencode/agents/**`), реестр
  (`docs/analysis/findings-registry.md`), `tests/docs_journal.rs`,
  `Cargo.toml`, `src/**` — **не тронуты**.
- `git diff --check` — единственное замечание: `migrator.md:529` (P3 выше).

## Что проверено и ок

**1. Адресный `docs_journal`.** 14 passed / 0 failed — приёмка касается только
журнала/задач; реестр/тест ещё не перенесены (объём T-27). Совпадает с ожиданием.

**2. D98 / Q95 / T-27.**
- Парность: Q95 `:3` «resolved by D98»; D98 `:5` «Resolves: Q95»;
  оба в каталогах (`questions/README.md:117`, `decisions/README.md:127`).
- D98 — 6 полей (Статус/Дата/Resolves/Спека/Affects/Tasks), разделы
  «Контекст / Решение / Следствия / Сверка с кодом (⚪ не применимо) /
  Альтернативы / Ссылки»; Q95 — 4 поля и разделы «Контекст / Вопрос / Варианты /
  Рекомендация».
- Класс T-27 = **L** везде: карточка `:7` (отдельное поле «Класс: **L**»),
  `:23`, `:69`, `:78`; D98 `:23` (Tasks ⬜, L), `:66` (п.6 «класс **L**»),
  `:128`, `:149`. Вхождений «M» для T-27 нет.
- Канон-часть — за сервисной сессией: D98 `:76–78` «**канон-часть (`AGENTS.md`,
  `.opencode/**`) правит сервисная сессия** (не `coder`/`docs-writer` — вне их
  прав); код-часть (тест) — `coder`/`tester`; маршрут L — полный + `auditor`
  до коммита»; карточка `:69–72,78–80` — тот же маршрут.
- Объём карточки `:29–55`: создать `docs/registry/registry.md` (14 открытых) +
  `docs/registry/archive.md` (70 закрытых), раскол 84 записей; правка
  `tests/docs_journal.rs` (`journal_files()` + whitelist `removable_addresses`);
  обновление канона/ссылок (`journal.md` §8, `AGENTS.md`, D48/D65, T-18,
  `TRACEABILITY.md`, `migrator.md` + право `edit docs/registry/**`); массовая
  замена ссылок `../analysis/findings-registry.md` → `docs/registry/…`;
  полный DoD (`validator`).
- Критерий готовности `:59–64`: `docs_journal` 14/0 + полный DoD; оба файла
  созданы, 84 записи с расколом, формулировки не переписаны; `docs/analysis/`
  не тронут; ссылки живые.
- «Работу не выполнять» зафиксировано: карточка `:68` «**Работу в операции
  оформления не выполнять**»; D98 `:131–132` «Работу в этой операции **не
  выполняем** — только оформление». Факт: `docs/registry/` отсутствует (glob),
  реестр/тест/канон не изменены.

**3. P1/P2/P3 аудита закрыты.**
- P1 (класс): класс **L** везде (см. п.2); «M» для T-27 — 0 вхождений.
- P2 (маршрут): D98 `:76–78` и карточка `:69–72,78–80` — канон-часть у
  сервисной сессии, код-часть у `coder`/`tester`, L + `auditor`.
- P3 (строка реестра задач): `docs/tasks/README.md:65` — строка `T-27` в блоке
  **P2** (после `T-14 :64`, до P3-блока `T-09 :66`); `rg T-27` по файлу — ровно
  1 строка.
- Память `migrator`: `:515` «(⬜, **L**, Источник D98/Q95)», `:516` «строка T-27
  `:65`, блок P2», `:527` «класс T-27 M→L» — исправлено (было M/`:72`).

**4. Журнал / связность.**
- Каталоги Q/D согласованы (по одной строке Q95/D98).
- `docs/TRACEABILITY.md:99` — Q95 → D98, `in work`, `[T-27] ⬜` (открытая задача
  ⇒ ⬜-статус) — ровно одна строка.
- `docs/tasks/README.md` — ровно одна строка T-27, блок P2.

**5. Границы.** Изменены только: Q95, D98, `T-27/**`, `docs/tasks/README.md`,
каталоги Q/D (`questions/README.md`, `decisions/README.md`), `TRACEABILITY.md`,
лента r20, память (`migrator.md`, `auditor.md`). Не тронуты:
`docs/analysis/findings-registry.md`, `tests/docs_journal.rs`, канон
(`.opencode/rules/**`, `AGENTS.md`, `.opencode/agents/migrator.md`), `src/**`,
`Cargo.toml` (`git diff --numstat -- src tests Cargo.toml` пусто; `git
diff --name-only` не содержит этих путей).

**6. Артефакты.** Отчёт — этот файл; квитанция — append в
`.opencode/state/current/receipts.yaml` (task `service-task-t27`, iteration 1);
отчёт в ленту r20 — append. Канон не правился; статусы не менялись.

**D65-гигиена.** В Q95/T-27 адресов с номерами строк нет; в D98 номера строк —
только в секции «Сверка с кодом» (исключена карваутом теста, T-18) — допустимо
как датированный снимок.
