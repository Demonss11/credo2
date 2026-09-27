# Память: auditor (аудит системы агентов)

- **Канон:** `AGENTS.md` §Рабочая группа агентов; чек-лист — в промпте роли.
- **Правило:** чекпойнт — что аудировано, находки P1–P3, что осталось. Кратко.

## Чекпойнты

> D41: оперативная хроника задач — в `state/` и ленте; здесь — знание роли и
> аварийные чекпойнты. Записи ниже — история.

- 2026-09-27 · T-12 (D39), ветка `feature/T-12-agent-loop`. Аудит «инструкция ↔
  права» + дубли/пробелы нового канона (loop-диспетчер, `analyst`, состояние).
  Машинно: `opencode reload` → `opencode debug agents` ×2 — 18 агентов, состав
  стабилен; `lead.steps 16`, `analyst.steps 20`; `cargo test *` — ровно 1
  (`validator`, стр. 2506). Находки: P2 — review.md «Доступные команды» не
  покрывает `lead`/`auditor` (автопроверка неполна); P2 — `edit state/**` у
  `lead`/`validator`/`analyst` шире «один писатель на файл»; P3 — строка
  `validator` в `AGENTS.md:45` без `.opencode/state/**`. Осталось непроверенным:
  прогон `features_inventory` (зона `validator`), содержимое `state/` (каталога
  ещё нет — появится при первом планировании).
- 2026-09-27 · T-12 (D39), аудит **r2** (свежая сессия), ветка
  `feature/T-12-agent-loop`. Машинно: `reload` → `debug agents` ×2 — 18 агентов
  в обоих прогонах, состав стабилен; `lead.steps 16`, `analyst.steps 20`;
  `cargo test *` — ровно 1 (`validator`); `edit` состояния поэлементный
  (`lead` progress.yaml, `analyst` next_action+current_state, `validator`
  receipts.yaml), `state/**` в правах ролей нет; `lead.subagent` — `analyst` +
  все командные роли; `git` — `question: deny`, изменяющие команды `ask`.
  `review.md` «Доступные команды» ↔ фронтматтеры — совпадают по всем 11 ролям.
  Находки r1 закрыты (P2×2 — да). P3-2 r1 (сузить YAML `docs-writer`) отклонён
  обоснованно: allow `docs/**` + точечные deny, совпадает с «Пишет в»
  `AGENTS.md`. Новых P1/P2 нет; P3 — `D39:46` перечень shell `lead` без
  `git branch --show-current` (есть в `lead.md:18`, `review.md:86`).
  Осталось: `features_inventory` — за `validator`.
- 2026-09-27 · T-13 (D40/D41, W7), ветка `feature/T-13-agent-hardening`. Аудит
  «инструкция ↔ права» + целостность нового канона цикла. Машинно: `reload` →
  `debug agents` ×2 — 18 агентов (11 ролей + 7 встроенных), состав стабилен;
  `lead.steps 16`, `analyst.steps 20`; `cargo test *` — ровно 1 (`validator`,
  стр. 2511); `opencode reload` — только `auditor`; `opencode debug agents` —
  `auditor` + `validator`; `edit` состояния поэлементный (`lead` progress.yaml,
  `analyst` next_action + current_state, `validator` receipts.yaml);
  `auditor` — только своя память (mail/state нет); `lead.subagent` = 10 ролей,
  включая `analyst`; `git` — `question: deny`, изменяющие `ask`. `review.md`
  «Доступные команды» ↔ фронтматтеры — совпадают по всем 11 ролям.
  Целостность: D41-пометка в 11 файлах памяти; research §6 с дословным
  `git -C …` и выводом; `findings-registry.md`/`run-checklist.md` на месте;
  T-03 `../../GRAMMAR.md` резолвится; Q45/D40, Q46/D41, §10 №40/№41,
  TRACEABILITY, T-13/T-14 согласованы; `.opencode/.gitignore` существует.
  P1/P2 нет. P3 — `dispatch-loop.md:84-85`/`lead.md:67`: `channel: question|text`
  при «`surface_to_user` всегда `question`» — ветка `text` недостижима; P3 —
  `findings-registry.md:14`: F7 «закрывается в W2» (в ленте уже закрыт).
  Не проверено: карточки T-13/T-14, содержимое `state/*.yaml` (только факт
  наличия), `features_inventory` — за `validator`; коммит-протокол — W4.
- 2026-09-27 · T-13 (D40/D41), аудит **r2** (свежая сессия), ветка
  `feature/T-13-agent-hardening`. Точечный повтор после закрытия 2 P3 r1.
  `dispatch-loop.md` §«Владелец: канал и фиксация» и `lead.md` (блок «Чего ты не
  делаешь»): `channel: question` — норма, `channel: text` — нарушение
  (отклонение); согласовано между файлами, противоречия с «`surface_to_user`
  всегда `question`» нет. `findings-registry.md:14`: F7 → «закрыт W2»;
  T-03 `../../GRAMMAR.md` резолвится (README:11). `git diff` по двум tracked —
  только правки W2/r2, лишнего нет; findings-registry — untracked. Машинно:
  `debug agents` — 18 агентов; `lead.steps 16`, `analyst.steps 20`;
  `cargo test *` — ровно 1 (`validator`). P1/P2 нет, P3 r1 закрыты. Не проверено:
  карточки T-13/T-14, `features_inventory` — за `validator`; коммит-протокол — W4.
