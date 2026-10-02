# Память: сервисная сессия (контур владельца)

- **Назначение:** сервисные операции без задачи (T-15 B0 «wave 0 фазы B», вне
  канона; MCP-ready, процесс агентов).
- **Канон:** `AGENTS.md` §«Память и почта», §«Сервисные операции»;
  `.opencode/rules/dispatch-loop.md` (hard rules).
- **Правило:** чекпойнт — волна/итерация, выполненные шаги, улики, что осталось;
  кратко, со ссылками.

## Чекпойнты

- 02.10.2026, service-t15-run-review (открыта): ревизия прогона 02.10
  (`ses_f0494cafcffettc8f9kGoillc4`, верхний уровень `build` + выборка ролей
  analyst/tester/migrator/validator/git) — улики через
  `.opencode/scripts/session-analysis/`, отчёты в `docs/analysis/`; решения
  владельца: выборка, F26/F27 открыты до чистого S-прогона, форма — сервисная.
  Лента — `.opencode/mail/service-t15-run-review.md`.
- 02.10.2026, service-t15-run-review (разбор готов): 6 отчётов
  `docs/analysis/T-15-run-2026-10-02-*`; ключевое — 4 упора лимита lead 16 +
  пустой финал (5 прерываний владельца), F15 подтверждён (iteration=участок,
  -rN без инкремента), атрибуция build≠lead, прогон ≈ $1.41; кандидаты
  F58–F61 заведены `migrator`. Дальше: migrator → auditor → validator →
  гейт → git. Грабли: вывод `node` в консоли — mojibake (лечится UTF-8
  OutputEncoding); кириллические имена в путях — ок через `%TEMP%`.
- 02.10.2026, service-t15-run-review (закрыта): F15/F26/F27 уточнены,
  F58–F61 заведены; T-15 ⬜→🚧; аудит P1/P2 нет (P3 закрыт), приёмка
  принята (DoD 135/0, docs_journal 14/14); коммит `fed94d6`, merge master
  `46b98c2`; F26/F27 — открыты до чистого S-прогона (решение владельца).
  Дальше по T-15: B0-own BO-i2 / чистый S-прогон / фаза C (выбор владельца).
- 02.10.2026, BO-i2 `wave0-guard` (открыта): полигон
  `%TEMP%\opencode\wave0b-own`; план — A плагин `permission.hook` (standard/
  paranoid), B `experimental.policies`, C совмещение; сценарии: --force,
  reset --hard, .env, Remove-Item -Recurse -Force, запись вне проекта,
  allow-поток, `--auto`, аудит JSONL. Лента `service-mcp-ready-r5.md`;
  улики `target/wave0b-own-i2/`.
- 02.10.2026, BO-i2 (готово, вердикт 🟢): 13 проб — плагин `wave0-guard`
  (deny до запуска, режимы, аудит), `experimental.policies` («Blocked by
  configuration policy»), `permissions` deny (хук evaluate не вызывается),
  комбинация. Ключевое: `ask`+`--auto`=выполнение (нужен deny); шаблоны
  policies статические (`-Force` ≠ `--force`); policy применяется после хука;
  `opencode reload` в полигоне сбрасывает MCP-каталог общей сессии (дважды,
  восстановился). Журнал — `wave0b-own.md` §BO-i2. Дальше: BO-i3
  (P1 `wave0-observe`) — по подтверждению владельца.
- 02.10.2026, BO-i2 закоммичен: `85658fd` → origin/develop (8 путей).
  BO-i3 открыт: плагин `wave0b-own-observe` (события сессий → JSONL, сводка,
  опция родителю); улики `target/wave0b-own-i3/`.
- 02.10.2026, BO-i3 (готово, вердикт 🟢): журнал событий сессий + агрегат
  (parentID/agent/tools) + сводка родителю (`ctx.session.prompt ok:true`,
  доставка подтверждена экспортом). Факты: `ctx.session.list` у плагина нет;
  `projectID` temp = `global`; `sh_*` — не сессии; корневой agent — из
  step/экспорта. Журнал — `wave0b-own.md` §BO-i3. Дальше: BO-i4 — P4
  `metrics-report.mjs` (по подтверждению владельца).
- 02.10.2026, BO-i3 закоммичен: `9a811b4` → origin/develop (6 путей).
  BO-i4 открыт: скрипт `metrics-report.mjs` (stats/export → сводка + цепочки
  по ролям/моделям); пробы T1 (полигон), T2 (прогон 02.10, 31 ребёнок);
  улики `target/wave0b-own-i4/`.
- 02.10.2026, BO-i4 (готово, вердикт 🟢): `metrics-report.mjs` (CLI-only):
  T2 воспроизвёл метрики прогона 02.10 — 32 сессии, $1.4133, роли/модели 1:1
  с B1-разбором. Грабли: opencode — npm-шим (shell:true), stats.tokens —
  объект, stats без --days — всё, дети — по маркеру в экспорте root.
  Журнал — `wave0b-own.md` §BO-i4. Дальше: BO-i5 — P3 `wave0-checkpoint`
  (по подтверждению владельца).
- 02.10.2026, BO-i4 закоммичен: `11110bd` → origin/develop (6 путей).
  BO-i5 открыт: скрипт `session-checkpoint.mjs` (сводка останова: цель/статус/
  файлы/риски/resume), проба resume `--session`, попытка регистрации команды
  плагином; улики `target/wave0b-own-i5/`.
- 02.10.2026, BO-i5 (готово, вердикт 🟢): сводка останова (T2 на tester T-18 —
  цель/статус/файлы/маркеры/resume) + resume `--session` подтверждён;
  команда регистрируется (`editor.add({name,description,template})`,
  `command.list` видит), форма `{info,template}` ломает list; запуск команды
  из headless не проверялся. Журнал — `wave0b-own.md` §BO-i5. Дальше: BO-i6 —
  P5 attribution (решить: подтвердить/свернуть) или подготовка BO-i7.
- 02.10.2026, продолжение (решение владельца «продолжаем»): BO-i6 открыт —
  проба context-хука (атрибуция role/model родитель/субагент), сверка с
  export/session list (дефект root: agent=build в хранилище, export root — без
  agent); улики `target/wave0b-own-i6/`; далее BO-i7 (отчёт мини-волны).
- 02.10.2026, BO-i6 (готово, вердикт 🟢 «свернуть P5»): context-хук —
  родитель build, субагент general (онлайн-атрибуция работает); export:
  ребёнок с `info.agent`, root — без; `session list` без agent; хранилище
  top-level `build` (F60). P5 закрывается как покрытое P1/P4. Дальше BO-i7 —
  отчёт мини-волны (`wave0b-own-report.md`), ревью, аудит перед переносом.
- 02.10.2026, BO-i6/i7 закоммичены (`ac5d382`); аудит — P1/P2 чисто, P3
  закрыт; решение владельца «Без P2 до C10»: перенос **P1/P3/P4 исполнен** —
  `.opencode/plugins/wave0-observe.ts` (bounded `target/wave0-observe.jsonl`,
  сводка при `WAVE0_OBSERVE_SUMMARY=1`), `.opencode/scripts/session-checkpoint.mjs`
  (файлы изменённые/прочитанные), `.opencode/scripts/metrics-report.mjs`;
  P2 — после C10; полигон сохранён до заморозки; проверки — плагин пишет
  события, скрипты прогнаны. Дальше: validator → коммит переноса.
- 02.10.2026, B0-own завершён: перенос принят validator (P1/P2/P3 нет),
  коммит `dace3eb` (13 файлов: 9 M + 4 A) → origin/develop. Итог: P1
  `wave0-observe` (плагин, bounded-журнал), P3 `session-checkpoint.mjs`,
  P4 `metrics-report.mjs` — в служебной зоне; P2 — после C10; P5 — свёрнут;
  полигон `%TEMP%\opencode\wave0b-own` — до заморозки. Дальше по T-15:
  чистый S-прогон (F26/F27) / фаза C (C1, C10).
- 02.10.2026, C10 открыт (решение владельца «приступаем к C10»): ревизия
  списка `experimental.policies` (кандидаты wave0 §3.5 + BO-i2), проба,
  решение «включить/отклонить»; лента `service-mcp-ready-r6.md`; улики —
  `target/wave0b-own-i7/`; правка при включении — служебная зона владельца.
- 02.10.2026, C10 включено: глобально 4 точных правила (`read:*.env`,
  `read:*/.ssh/*`, `shell:git push *--force*`, `shell:git reset --hard*`) +
  плагин `wave0-guard` (якорные deny + аудит; P2 перенесён). Находки: P11/P12
  — широкие паттерны ловят упоминания (сужено); P13 — plugin outside-edit
  блокировал служебную запись (правило → аудит). Проверки: force/reset/env —
  блок; упоминания — разрешены. Дальше: validator → коммит.
- 02.10.2026, C10 завершён: приёмка validator (принято), коммит `142458d`
  (13 файлов) → origin/develop. Итог: 4 точных правила глобально + плагин
  `wave0-guard` (P2 перенесён; якорные deny + аудит). CC Safety Net — позже.
  Дальше по T-15: чистый S-прогон (F26/F27) / фаза C (C1).
