# Приёмка: service-b0-own-transfer (перенос P1/P3/P4) — 02.10.2026

**Проверка:** сервисная операция B0-own — перенос зелёных механизмов мини-волны
B0-own в служебную зону репозитория: P1 `wave0-observe` →
`.opencode/plugins/wave0-observe.ts`, P3 `wave0-checkpoint` →
`.opencode/scripts/session-checkpoint.mjs`, P4 `metrics-report` →
`.opencode/scripts/metrics-report.mjs`. Решение владельца «Без P2 до C10»
(лента `service-mcp-ready-r5.md` §«перенос P1/P3/P4 исполнен»).

**Версия:** `develop` = `origin/develop` = `HEAD` = `ac5d382` + рабочее дерево
(ветки нет, прямая правка; снимок 02.10.2026).

**Вердикт:** принято

**P1:** — критичных проблем нет.

**P2:** — критичных проблем нет.

**P3:** — нет (замечаний, требующих правки, нет).

## Проверки

| # | Проверка | Команда / источник | Результат |
|---|---|---|---|
| 1 | База/ветка | `git rev-parse HEAD develop origin/develop` | `ac5d382` ×3; ветки нет |
| 2 | Снимок пакета | `git status --porcelain` | 6 `M` + 3 `??` (ровно целевые; см. «Состав пакета») |
| 3 | Границы | `git status --porcelain` (полный список) | `src/**`, `tests/**`, `Cargo.toml`, `AGENTS.md`, `opencode.json`, `.opencode/agents|rules/**`, `docs/decisions|questions/**`, `docs/TRACEABILITY.md` — не в статусе |
| 4 | Пробелы диффа | `git diff --check` | пусто |
| 5 | Права агентов (×2) | `node .opencode/scripts/agents-perms.mjs` | `11 из 18` дважды |
| 6 | P1-плагин | `read .opencode/plugins/wave0-observe.ts` | id `wave0-observe`; ротация `.1` при `>8 МБ`; агрегат `target/wave0-observe-summary.json`; сводка только `WAVE0_OBSERVE_SUMMARY=1`; фильтр `location===dir`/known-sid |
| 7 | Улики P1 (in-repo) | `rg` по `target/wave0-observe.jsonl` | `plugin.start` (v2.0.22, directory = repo) ×N; `event` root `ses_f03ba898…`; агрегат жив |
| 8 | P3-скрипт | `read .opencode/scripts/session-checkpoint.mjs` | changed (`edit/write/patch` + `snapshot.files`) / read; resume-путь; `shell: win32` |
| 9 | Улика P3 | `read target/wave0b-own-transfer/checkpoint-tester-T18.md` | «Файлы изменённые (3)» / «прочитанные (44)» |
| 10 | P4-скрипт | `read .opencode/scripts/metrics-report.mjs` | `stats --json`, `session list`, `--chain` по маркеру `<subagent sessionID=…>`; `shell: win32` |
| 11 | Запись «в gitignore» | `git check-ignore -v target/wave0-observe.jsonl …` | `.gitignore:1:/target` — журнал/агрегат вне git |
| 12 | Записи | `git diff` ленты/памяти/карточки/журнала/отчёта | согласованы |
| 13 | DoD-код | `cargo` | **не запускался** — продукт не затронут (D50) |

## Что проверено и ок

- **А1. Артефакт P1** (`.opencode/plugins/wave0-observe.ts`, 182 строки): корень
  `Plugin.define({ id: "wave0-observe", … })` (:22–23); журнал
  `target/wave0-observe.jsonl` (:26) с ротацией в `.1` при `> MAX_BYTES`
  (`8*1024*1024`, :20; проверка `++appends % 200 === 0`, :33–35); агрегат
  `target/wave0-observe-summary.json` (:27, `flush` :50–67); сводка родителю —
  `ctx.session.prompt` только при `summaryEnabled` (:28, :69–107, :78); фильтр
  `e.location.directory === dir` / известные `sessionID` (:130–132); подписка
  `ctx.event.subscribe` (:119). Ошибки журнала/агрегата заглушены `catch` —
  плагин не ломает сессию (:40–42, :64–66).
- **А2. Артефакт P3** (`.opencode/scripts/session-checkpoint.mjs`, 123 строки):
  `CHANGE_TOOLS = {edit,write,patch}` (:48) → `changed`, прочие с путём →
  `readFiles` (:48–62); добавление `snapshot.files` в `changed` (:61); разделы
  «Файлы изменённые (N)» (:104) / «Файлы прочитанные (N)» (:107); resume-путь
  (:114–117); `shell: process.platform === "win32"` (:32). CLI-only,
  `--session`/`--out` (:14–26).
- **А3. Артефакт P4** (`.opencode/scripts/metrics-report.mjs`, 175 строк):
  `opencode stats --json` (:56), `session list --format json` с фильтром по
  `cwd` (:64, :69), `--chain` — экспорт root + детей по маркеру
  `<subagent sessionID="…">` (:84–96), таблицы роль/модель/сессия (:153–168),
  `shell: win32` (:27).
- **А4. Улики смоука (в репозитории, `target/**` вне git).**
  `target/wave0-observe.jsonl`: `kind:"plugin.start"` (version `2.0.22`,
  `directory` = корень репозитория) и `kind:"event"` с `sid`
  `ses_f03ba898cffel2M28KDlhJPlX4` (текущая сессия репозитория);
  `target/wave0-observe-summary.json` — агрегат с сессиями root + дети
  (`migrator` `ses_f035c6e8…`, `validator` `ses_f035ba85…`, оба с `parentID` =
  root) и типами/инструментами. Это независимо подтверждает и загрузку плагина,
  и запись событий, и работу агрегата.
  `target/wave0b-own-transfer/checkpoint-tester-T18.md`: заголовок seссии
  `ses_f045ce49…`, «Файлы изменённые (3)» (:39) / «прочитанные (44)» (:44) —
  разделение P3 подтверждено на реальной сессии `tester`.
- **А5. Записи согласованы с фактами.** Лента
  `.opencode/mail/service-mcp-ready-r5.md:321–335` (§«перенос P1/P3/P4
  исполнен»); журнал `docs/tasks/T-15-mcp-ready-process/wave0b-own.md:3–9`
  (статус: перенос P1/P3/P4 исполнен, P2 — после C10); отчёт
  `wave0b-own-report.md:89–92` («Обновление 02.10.2026»); карточка
  `README.md:194/:196/:197` — хвост «перенесён 02.10 →
  `.opencode/plugins/wave0-observe.ts` / `.opencode/scripts/session-checkpoint.mjs`
  / `.opencode/scripts/metrics-report.mjs`» (существующий текст сохранён,
  по 1+/1−).
- **А6. Границы пакета.** Диффы записей — только: лента (+27), память
  `migrator` (+14), память `service` (+7), карточка README (3 замены),
  `wave0b-own.md` (+9/−3), `wave0b-own-report.md` (+5). Запрещённые зоны
  (`src/**`, `tests/**`, `Cargo.toml`, `AGENTS.md`, `opencode.json`,
  `.opencode/agents|rules/**`, `docs/decisions|questions/**`,
  `docs/TRACEABILITY.md`) в `git status` отсутствуют; `git diff --check` пусто.
  Три новых файла в `.opencode/plugins|scripts` — `??`, каталоги трекаются,
  это легитимный перенос служебной зоны.
- **А7. Права.** `agents-perms.mjs` ×2 → `11 из 18` (полный состав), изменений
  прав нет (плагины/скрипты — не агенты). `.opencode/package.json` объявляет
  `@opencode/plugin ^2.0.18` — импорт плагина резолвится, что подтверждено
  живым `plugin.start`.

## Не проверено и почему

- **Лог `~/.local/share/opencode/log/opencode.log`** (улика «`loading plugin …
  wave0-observe.ts`»): путь вне репозитория — команда отклонена движком
  (`permission.rejected: external_directory`). Компенсировано in-repo уликами:
  `kind:"plugin.start"` с `version`/`directory` в `target/wave0-observe.jsonl`
  доказывает фактическую загрузку плагина в этой сессии.
- **Прогон `session-checkpoint.mjs` / `metrics-report.mjs`** (`node`): у роли
  `validator` нет прав на `node` (кроме `agents-perms.mjs`) — смоук выполнен
  сервисной сессией; проверены артефакты, улики и соответствие заявленному.
- **`cargo`** — не запускался: продуктовый код (`src/**`, `tests/**`,
  `Cargo.toml`) не затронут (D50; `review.md` «Порог существенности»).

## Состав пакета для гейта (`git status --porcelain`, снимок)

```
 M .opencode/mail/service-mcp-ready-r5.md
 M .opencode/memory/migrator.md
 M .opencode/memory/service.md
 M docs/tasks/T-15-mcp-ready-process/README.md
 M docs/tasks/T-15-mcp-ready-process/wave0b-own-report.md
 M docs/tasks/T-15-mcp-ready-process/wave0b-own.md
?? .opencode/plugins/wave0-observe.ts
?? .opencode/scripts/metrics-report.mjs
?? .opencode/scripts/session-checkpoint.mjs
```

Сверх снимка приёмки в пакет коммита войдут записи `validator`
(`docs/reviews/service-b0-own-transfer-2026-10-02.md`,
`.opencode/state/current/receipts.yaml`, `.opencode/memory/validator.md`, лента
`service-mcp-ready-r5.md`). Ожидаемо: 6 `M` + 3 `??` + 2 `??`/`M` записей
валидатора (лента — уже в снимке).
