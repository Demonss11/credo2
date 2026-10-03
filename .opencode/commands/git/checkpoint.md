---
description: Process-commit прогона (лента, память, снапшот state) по «постоянному» пакету
agent: git
subagent: true
---

Выполни чекпойнт сессии по «постоянному» пакету, подтверждённому `lead` (запись
в ленте задачи/`state`; без записи верни «нужно подтверждение», изменяющих
команд не выполняй). Значения `<прогон>`/`s<M>` прочитай из
`.opencode/state/current/current_state.yaml` (`task`, `session_index`).

Идемпотентно, по одной команде:

1. `node .opencode/scripts/session-checkpoint.mjs --snapshot` — снапшот состояния
   в `.opencode/state/snapshots/<прогон>-s<M>/`;
2. `git switch -c process/<прогон>-s<M>` — от текущего HEAD;
3. `git add` — только process-пути: изменённые файлы `.opencode/mail/**`,
   `.opencode/memory/**`, каталог снапшота;
4. `git commit -m "chore(process): <прогон> s<M>"`;
5. `git tag session/<прогон>-s<M>`;
6. `git push origin process/<прогон>-s<M> session/<прогон>-s<M>` (таймаут ≥ 5 мин);
7. `git switch -` — назад на рабочую ветку.

Уже сделанные шаги пропускай (ветка/коммит/тег существуют — успех без повтора).
Продуктовые изменения не входят; shell-блоки не использовать; после push в
отслеживаемые файлы не писать (F43). Отчёт — в ленту задачи и память; верни
хеши (коммит, тег, ветка).
