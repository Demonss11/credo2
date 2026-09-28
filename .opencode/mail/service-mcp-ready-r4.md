# Сервисная лента: service-mcp-ready (том 4)

**Назначение:** продолжение линии `service-mcp-ready` — сервисные операции без
задачи (MCP-ready, процесс агентов). Том 3 —
[`service-mcp-ready-r3.md`](service-mcp-ready-r3.md) (322 строки, закрыт по
порогу ~300); тома 1–2 и ленты прошлых прогонов очищены 28.09.2026
(`node .opencode/scripts/clean-logs.mjs`), содержимое — в истории git.

Формат записей — `AGENTS.md` §«Память и почта».

---

## сервисная сессия · 28.09.2026 · T-15 · BO-i1: каркас и разведка V2-API (B0-own)

- Сделано: свежий temp-полигон `%TEMP%\opencode\wave0b-own` (форма B0:
  `.opencode/plugins/*.ts` + `@opencode/plugin ^2.0.18`; `npm install` — 283
  пакета за 42 с); probe-плагин `wave0b-own-probe.ts` — маркер из setup
  (stage=minimal 14:49:56Z; stage=recon 14:55:55Z после атомарного write);
  startup 🟢 (`loading plugin`, WARN для плагина нет); разведочный прогон:
  субагент (`ses_f177bdce2ffe…`, agent `general`), shell, read. Таблица «P1–P5
  → хук/источник», находки и предложение BO-i2 — `docs/tasks/T-15-mcp-ready-process/wave0b-own.md`;
  улики — `target/wave0b-own-i1/` (журнал 87 events + 4 context + 4 permission +
  4/4 tool + command/agent.list, экспорт дочерней сессии, снимок `stats --json`,
  вырезка лога). Реестр T-15: B0-i8 → ✅ (`cafc4c4`), `B0-own` → `B0-own-i1` ✅ +
  `B0-own-P1…P5` ⬜.
- Проверки: `git log -1`/`git status` — база `cafc4c4`, чисто; адресный поиск в
  `~/.local/share/opencode/log/opencode.log` по `wave0b-own-probe` — только
  `loading plugin`/watcher (счётчик `failed to load plugin` самореферентный:
  лог пишет spawn собственных `rg`-команд, содержащих шаблон); `opencode stats
  --days 0 --json`, `opencode session export` — источники P4/P5 подтверждены.
- Дальше / риски: BO-i2 (`wave0-guard`: плагин vs `experimental.policies`) —
  старт по подтверждению владельца, далее BO-i3…BO-i7; журнал probe в temp
  продолжает пополняться (поток серверный) — в BO-i2 ограничить фильтром;
  аномалия B0-i1 (первый CLI-прогон после подъёма проекта зависает, сессия
  `succeeded`) повторилась; коммит документов — один пакет по подтверждению
  владельца (C13).

---

## сервисная сессия · 28.09.2026 · шпаргалка «V2-плагин за шаги» (B0-own, по запросу владельца)

- Сделано: `docs/tasks/T-15-mcp-ready-process/wave0b-own-plugin-guide.md` —
  инструкция для агентов по разработке V2-плагинов из опыта BO-i1: форма
  установки и шаблон, чек-лист startup/reload, карта `ctx`, таблица «задача →
  хук» (P1–P5), проверенные формы событий/полей, нативные `stats`/`export`,
  грабли (атомарный write, самореферентный лог, аномалия первого прогона,
  codemode-нюанс). Ссылки: карточка волны, `token-guard`, V2-доки.
- Проверки: сверка с `wave0b-own.md` §BO-i1 и уликами `target/wave0b-own-i1/`;
  новых кодовых проб нет (документный артефакт).
- Дальше / риски: кандидат в канон (skill `.opencode/skills/**` или раздел
  AGENTS.md) — решением владельца и по протоколу; входит в коммит-пакет BO-i1.

---

## сервисная сессия · 28.09.2026 · подтверждение владельца: пакет `wave0b_own_i1`

- Владелец подтвердил (2026-09-28, чат сервисной сессии): коммит и push
  документов BO-i1 одним пакетом **прямо в `develop`** (как `wave0b_records`;
  C13, `.opencode/rules/git-workflow.md` §«Пакет и подтверждение»).
- Пути пакета: `docs/tasks/T-15-mcp-ready-process/wave0b-own.md` (new),
  `docs/tasks/T-15-mcp-ready-process/wave0b-own-plugin-guide.md` (new),
  `docs/tasks/T-15-mcp-ready-process/README.md`,
  `.opencode/mail/service-mcp-ready-r4.md`, `.opencode/memory/service.md`; по C13
  в пакет входят отчёт роли `git` (эта лента) и `.opencode/memory/git.md` —
  записи формируются до `git add`.
- Сообщение коммита: `chore(process): T-15 B0-own BO-i1 — карточка мини-волны,
  разведка V2-API, шпаргалка, реестр`. Затем `git push origin develop`.
- База — `develop @ cafc4c4`, дерево до правок чистое; иных правок, кроме
  перечисленных, нет (`git status --porcelain` сверен 28.09.2026).

---

## сервисная сессия · 28.09.2026 · ревью BO-i1 и гигиена полигона

- Сделано: независимое ревью BO-i1 (раздел «Ревью BO-i1» в `wave0b-own.md`) —
  вердикт подтверждён: реестр/память/лента (r3 = 322 → r4), маркер `stage=recon`
  (14:55:55Z), комплект улик, сессии субагента/родителя проверены по API
  (`agent=general`, `parentID`, `succeeded`). Гигиена: probe-плагин погашен —
  `wave0b-own-probe.ts` убран из `.opencode/plugins/` полигона в `_off/`
  (журнал рос после итерации: 87 → 123 строки, 55.7 КБ; остановлен 15:10:57Z;
  контрольный прогон — без роста); репозиторий, канон и
  `.opencode/**` репо не тронуты.
- Проверки: `git diff` (только рабочие файлы мини-волны), API сессий,
  `Get-Item` журнала, счётчики строк, `Select-String` по логу.
- Дальше / риски: в BO-i2 первым шагом — «прогрев» полигона (аномалия 2/2: первый
  CLI-прогон висит, сессия `succeeded`) и фильтр журнала по `project`/`session`;
  находка-кандидат (F/D) — в отчёт BO-i7.

---

## git · 28.09.2026 · готово (пакет `wave0b_own_i1`)

- Сделано (записи до `add`, C13/F43): сверка `git status -sb`/`git log -1` —
  ветка `develop`, синхрон с `origin/develop`, база `cafc4c4`, ровно 5 правок
  пакета (2 ` M` + 3 `??`), чужой грязи нет. `git switch develop` (already),
  `git pull origin develop` (Already up to date). Сформированы записи до `add`:
  этот отчёт и чекпойнт `memory/git.md`. Далее: `git add` 6 путей → сверка
  `git diff --cached --name-only` (6, посторонних нет) → commit → push.
- Проверки: `git status -sb`, `git log -1 --oneline`, `git diff --cached --name-only`.
- Дальше / риски: после `push` в отслеживаемые файлы не пишу (F43); хеш коммита
  возвращаю `lead` ответом. Расширение пакета/посторонние правки — не коммичу,
  отмечаю в ответе.

---
