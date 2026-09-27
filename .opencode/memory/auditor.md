# Память: auditor (аудит системы агентов)

- **Канон:** `AGENTS.md` §Рабочая группа агентов; чек-лист — в промпте роли.
- **Правило:** чекпойнт — что аудировано, находки P1–P3, что осталось. Кратко.

## Чекпойнты

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
