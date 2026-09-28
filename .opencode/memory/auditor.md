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
- 2026-09-28 · T-04 (L, Run 5), ветка `feature/T-04-mcp-errors` @0437799. Аудит
  перед коммитом (коммита нет, работа в рабочем дереве). **Инструкция ↔ права:**
  фронтматтеры ↔ `review.md` §«Доступные команды» — совпадение по 11 ролям; но
  **P2**: бриф lead (`next_action.yaml:36-39`) требует от auditor «append в
  `.opencode/mail/T-04.md`», а права auditor разрешают `edit` только своей памяти
  (`agents/auditor.md:8-9`) — запись в ленту отклонена (`permission.rejected`),
  аудиторский отчёт в ленте отсутствует. Факты: канон (`AGENTS.md`,
  `.opencode/agents|rules`, `SPECIFICATION`, `GRAMMAR`, `decisions`) не тронут;
  `src/rest.rs`/`src/core.rs` не тронуты (Q23); пути ролей = allowlist (coder
  `src/*`, tester `tests/*`, validator `docs/reviews`+`receipts`, docs-writer
  `docs/tasks`, analyst `docs/analysis`+`state`, lead `progress`+лента);
  память/почта — только добавления (numstat 0 удалений, `T-04.md` — append);
  отчёт `docs/reviews/T-04-2026-09-28.md` + квитанция T-04 iteration 1 accepted
  (`receipts.yaml:55-65`). Git: 17 путей (12 M + 5 ??), ветка в синхроне с origin.
  Автопроверка фронтматтеры ↔ `review.md` §«Доступные команды» — совпадение по
  11 ролям, `cargo test` только `validator`. Замечания (не находки): shell-отказ
  на `git … -- .opencode` — канон review.md:97-99; `git ls-files` у auditor нет.
- 2026-09-28 · канон-пакет W8-т2 (Run 5), `develop` @`a7eac82` + рабочее дерево.
  `reload` → `debug agents` ×2 — состав стабилен: `coder 52`, `tester 36`,
  `auditor 32`, `git 28`, `lead 16`, `analyst 20`; `edit .opencode/mail/**` у
  всех 11 ролей; `edit findings-registry.md` — у `migrator`; `execute: deny` —
  только `docs-writer`/`git`; `git branch -vv` — allow у `git`; `B2_PREFIXES={}`.
  Фронтматтеры ↔ `review.md` — совпадение по 11 ролям. Память ↔ канон (F35):
  дрейфа нет (`coder` исправлен, `tester` `git -C`, `analyst` R2-формулировка).
  P1/P2 нет. P3: `agents/auditor.md:158` — `opencode run` вне allowlist и вне
  списка review.md (наследие, не пакет); эскалация-затык размножена в 4 файлах
  (`dispatch-loop` + `AGENTS.md` + `coder` + `tester`, формулировки расходятся);
  `analyst.md:9` `edit docs/analysis/**` покрывает `findings-registry.md`
  (владелец — `migrator` по D48). Отчёт — в `service-mcp-ready-r2.md` (F16
  проверен на практике: append прошёл).
- 2026-09-28 · канон-пакет W8-т2, аудит **r2** (свежая сессия) после закрытия
  трёх P3 r1. `reload` → `debug agents` ×2 — состав стабилен, 11 ролей;
  `coder 52`, `tester 36`; `analyst` → `docs/analysis/findings-registry.md`
  `deny`, `migrator` → `allow` (last-match-wins подтверждён). Проверено:
  `auditor.md` п.6 — headless как процедура владельца, без предписания роли
  `opencode run` и без дубля модели; `coder.md:48`/`tester.md:56`/`AGENTS.md`
  §«Лимиты шагов» — короткие ссылки на `dispatch-loop.md` §«Hard rules», канон
  правила только там; `analyst.md:11` — точечный deny реестра. P1/P2 нет,
  P3 нет. Вердикт: инструкция ↔ права — расхождений нет. Не проверено:
  `features_inventory` — за `validator`. Отчёт — `service-mcp-ready-r2.md`.
