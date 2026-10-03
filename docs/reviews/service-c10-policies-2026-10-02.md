# Приёмка: service-c10 — `policies`-страховки + перенос P2 `wave0-guard`

- **Задача:** сервисная операция `service-c10` (T-15 фаза C, пункт C10; перенос
  B0-own P2) — iteration 1.
- **Дата:** 02.10.2026.
- **Версия (снимок):** `develop` = `origin/develop` = `HEAD` = `dace3eb`
  (прямая правка без ветки) + рабочее дерево; сверка
  `git rev-parse HEAD develop origin/develop` ×3.
- **Вердикт:** **принято** (P1/P2/P3 нет).

## Что проверялось

1. Плагин `.opencode/plugins/wave0-guard.ts` (новый, служебная зона):
   id `wave0-guard`; якорные shell-правила; `read:*.env` deny; внешние правки —
   аудит; журнал `target/wave0-guard.jsonl` (bounded, ротация > 8 МБ).
2. Глобальный конфиг — копия `target/wave0b-own-i7/c10-final-global.jsonc`:
   ровно 4 правила.
3. Улики проб `c10-probes.md` (P1–P14) и `wave0-guard-journal-sample.jsonl`.
4. Записи: лента r6, память `service`, карточка T-15, журнал `wave0b-own.md`,
   отчёт `wave0b-own-report.md` §3.
5. Границы пакета и `git diff --check`.

## P1/P2/P3

- **P1:** — (критичных проблем нет)
- **P2:** —
- **P3:** —

## Проверки (команды → результат)

| Проверка | Команда | Результат |
|---|---|---|
| База/ветка | `git rev-parse HEAD develop origin/develop`; `git status -sb` | `dace3eb` ×3; `## develop...origin/develop`; ветки нет (прямая правка) |
| Снимок дерева | `git status --porcelain` | 6 `M` + 2 `??` (см. «Состав пакета») |
| Границы продукта | `git diff --stat -- src tests Cargo.toml AGENTS.md` | пусто |
| Границы канона | `git status --porcelain` (agents/rules/opencode.json/decisions/questions/TRACEABILITY не в статусе) | не затронуты |
| Пробелы | `git diff --check` | пусто |
| Права | `node .opencode/scripts/agents-perms.mjs` | `agents: 11 из 18` |
| Зависимость плагина | `.opencode/package.json` | `@opencode/plugin ^2.0.18` |
| gitignore журнала | `.gitignore:1` | `/target` — `target/wave0-guard.jsonl` вне git |

## Артефакт = заявленному

### Плагин `wave0-guard.ts`

- id — `Plugin.define({ id: "wave0-guard" })` (:23).
- Якорные shell-правила (проверяются по каждому ресурсу, :55–88):
  - `^git push…--force` → `git-push-force` (:57–60), плюс `-f` → `git-push-force-short`
    (:65–68);
  - `^git reset --hard` → `git-reset-hard` (:61–64);
  - shell-чтение `.env` (`Get-Content|gc|cat|type|more|less|head|tail|Select-String|rg|grep`),
    исключая `.env.example` → `secrets-env` (:69–76);
  - shell-чтение `/.ssh/` → `ssh-read` (:77–83);
  - `^Remove-Item … -Recurse … -Force` → `recursive-force-delete` (:84–87).
- `read` deny: `[\\/]?\.env(\..+)?$`, исключая `.env.example` (:89–95).
- **Внешние правки — аудит, не deny (P13):** `edit` + абсолютный путь вне
  `location` → `l({kind:"outside-edit"})` без установки `why` (:96–107); доступ —
  по default `external_directory`.
- Журнал: `target/wave0-guard.jsonl` (join `dir`), запись `appendFileSync`,
  ротация при `> MAX_BYTES = 8 МБ` каждые 200 appends (:20, :31–33); ошибки
  аудита подавляются (`catch`, :38–40) — плагин не ломает permission-поток.

### Глобальный конфиг (`c10-final-global.jsonc`)

- Ровно 4 правила (:10–13): `read:*.env`, `read:*/.ssh/*`,
  `shell:git push *--force*`, `shell:git reset --hard*` — совпадают с записью
  ленты r6:50–51, памятью `service:99–101` и карточкой T-15 C10.

### Улики проб

- `c10-probes.md`: P1 force → **Blocked**; P2 `reset --hard` → **Blocked**;
  P3 `.env` → **Blocked**, `.env.example` → **allowed**; P4 — shell-обход
  `read`-политики `.ssh` (ключевая находка); P5 allow-поток — без ложных;
  P6 `external_directory` — не блокируется; P7/P8 `shell:*/.ssh/*` — forward/backslash;
  P9/P10 — цена `shell:*.env*` (блокирует и `.env.example`); P11/P12 — ложные
  срабатывания широких паттернов (сужено); P13 — `outside-location` deny ломал
  служебную запись, переведён в аудит; P14 — итог: force/reset/env — блок,
  упоминания — разрешены.
- `wave0-guard-journal-sample.jsonl` подтверждает механику: `plugin.start`
  (v2.0.22, directory = repo, :1–2), deny force (:7 `why:"force-flag"`),
  allow-упоминания `echo "check .env mention"` (:9, :34), `:19` deny
  `outside-location` (до правки P13), `:29` `outside-edit` аудит (после правки
  P13), `:30` allow записи конфига, :39 deny `secrets-env`, :40 deny
  `git-reset-hard`. P13 подтверждён уликой до/после.

### Записи

- Лента `service-mcp-ready-r6.md` §«C10 включено — готово» (:44–64): состав
  4 правил, плагин, проверки P14, грабли P11–P13, улики, «дальше validator».
- Память `service.md:95–104` — C10 открыт/включено.
- Память `migrator.md:9–20` — две правки карточки.
- Карточка T-15 `README.md`: C10 `⬜` → `✅ 02.10 — 4 точных правила… + плагин`;
  B0-own-P2 — хвост `· перенесён 02.10 (C10)` (текст BO-i2 сохранён) —
  ровно 2 строки (`git diff -- README.md`: 1+/1− ×2).
- Журнал `wave0b-own.md:3–10` — статус: P2 перенесён 02.10 в рамках C10.
- Отчёт `wave0b-own-report.md:58` §3 — строка guard: `**исполнено 02.10.2026:
  перенесён, 4 точных правила глобально + плагин wave0-guard (якорные правила)**`.

## Находки и замечания

- **Не мои (в пакете):** `.opencode/mail/service-mcp-ready-r5.md` (+20) —
  F43-остаток предыдущей операции `service-b0-own-transfer`: git по F43 после
  `push` не пишет в отслеживаемые файлы, поэтому доехали две хвостовые записи
  («пакет переноса B0-own» `dace3eb`, «B0-own завершён»). Содержание
  согласовано с уже приёмкой (`dace3eb` в логе). В отчёте фиксирую как «не мои»,
  из пакета не исключаю (прецедент — `service-t15-run-review`).
- **P3 (одна строка, очевидна), информационно:** плагин пишет аудит-запись
  `evaluate` на **каждую** permission-проверку (`:114–120`), включая allow;
  при интенсивной сессии журнал растёт быстро, но ротация `>8 МБ` и `catch`
  закрывают риск. Правку не требую.

## Состав пакета для гейта (снимок `git status --porcelain`)

```
 M .opencode/mail/service-mcp-ready-r5.md
 M .opencode/memory/migrator.md
 M .opencode/memory/service.md
 M docs/tasks/T-15-mcp-ready-process/README.md
 M docs/tasks/T-15-mcp-ready-process/wave0b-own-report.md
 M docs/tasks/T-15-mcp-ready-process/wave0b-own.md
?? .opencode/mail/service-mcp-ready-r6.md
?? .opencode/plugins/wave0-guard.ts
```

- **Целевой пакет C10** (7 путей):
  - `.opencode/plugins/wave0-guard.ts` (новый, A);
  - `.opencode/mail/service-mcp-ready-r6.md` (новый, A);
  - `.opencode/memory/migrator.md`, `.opencode/memory/service.md`,
    `docs/tasks/T-15-mcp-ready-process/{README.md,wave0b-own.md,wave0b-own-report.md}` (M).
- **Сверх цели (F43-остаток, «не мои»):** `.opencode/mail/service-mcp-ready-r5.md`
  — предыдущая операция (закрыта коммитом `dace3eb`).
- **Мои записи (вне пакета git, но в дереве после приёмки):** этот отчёт
  `docs/reviews/service-c10-policies-2026-10-02.md`, секция validator в
  `service-mcp-ready-r6.md`, квитанция в `receipts.yaml`, чекпойнт
  `memory/validator.md`.
- **Глобальный конфиг `~/.config/opencode/opencode.jsonc` — вне репозитория**
  (служебная зона владельца; in-repo копия — улика `target/**`, gitignore).

## Хранение

- Отчёт: `docs/reviews/service-c10-policies-2026-10-02.md`.
- Квитанция: `accepted`, task `service-c10`, iteration 1 —
  `.opencode/state/current/receipts.yaml`.
