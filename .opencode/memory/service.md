# Память: сервисная сессия (контур владельца)

- **Назначение:** сервисные операции без задачи (T-15 B0 «wave 0 фазы B», вне
  канона; MCP-ready, процесс агентов).
- **Канон:** `AGENTS.md` §«Память и почта», §«Сервисные операции»;
  `.opencode/rules/dispatch-loop.md` (hard rules).
- **Правило:** чекпойнт — волна/итерация, выполненные шаги, улики, что осталось;
  кратко, со ссылками.

## Чекпойнты

- 28.09.2026 · T-15 B0: старт (решения владельца — модель проб
  `opencode-go/deepseek-v4.1-flash`; пробы в temp; перенос зелёного —
  `.opencode/plugins` + `.opencode/package.json`; `state/current` не трогаем —
  находка в отчёт). Выполнен **B0-i1**: temp-проект
  `%TEMP%\opencode\wave0b-probe`; форма 1 — `.opencode/plugins/*.ts` + `npm install
  @opencode/plugin` (283 пакета, 60 с), маркер setup 13:53:02; форма 2 — config
  `plugins[]` (каталог), маркер 13:53:48; startup чистый (`failed to load plugin`
  нет). Аномалия: прогон 1 — CLI завис на `/wait`, сессия `succeeded` (kill).
  Улики — `target/wave0b-i1/`; протокол — `wave0b-probes.md`; лента — r3.
  Осталось: **B0-i2…i8** (i2 — Shell Strategy: `instructions` в V2 не загружается,
  контент — рекомендация в C).
- 28.09.2026 · **B0-i2** (Shell Strategy, v1.1.0, MIT): `instructions` в V2 не
  загружается (проба A: `ок`), `AGENTS.md` — рабочий путь (проба B: `ПАНТЕРА-9137`);
  сверка: `git --no-pager` в начале команды конфликтует с префиксными allowlist'ами,
  Linux-части вне pwsh; `GIT_TERMINAL_PROMPT=0` — учесть. Вердикт: 🔴 установка /
  🟢 контент → C. Улики — `target/wave0b-i2/`. Осталось: i3…i8.
- 28.09.2026 · **B0-i3** (Opencode Telemetry, v0.2.0, MIT): V1 Plugin API →
  WARN `failed to load plugin` в V2 («must export a default definition with an id
  and an effect or setup function»); config-пути (абс/отн/файл) не подхватываются;
  `.opencode/plugins/<name>/` грузится только с корневым `index.ts`. CLI `octm`
  работает (Bun 1.4.2), но без плагина пуст. Вердикт 🔴; D — нативные
  `stats`/`session export`. Улики — `target/wave0b-i3/`. Осталось: i4…i8.
- 28.09.2026 · **B0-i4** (Subagent Reporter + Agent Identity, v3.1.1): оба V1 API →
  WARN `failed to load plugin` (default-определение / `@opencode-ai/plugin`);
  нативный stdout субагента — только `✓ … General Agent`; вердикт 🔴×2;
  рекомендации C (`--format json`/свой V2-плагин; своя атрибуция). Улики —
  `target/wave0b-i4/`. Осталось: i5…i8.
- 28.09.2026 · **B0-i5** (CC Safety Net v2.4.11): 🟢 — V2-энтрипоинт работает;
  live: `.env` read BLOCKED (`secret.basename.env`), `git push --force` BLOCKED
  (`git.push-force`), `git status` прошёл; audit JSONL с sessionId (копия в
  `target/wave0b-i5/`); `reset --hard` BLOCKED в репо / ALLOWED в temp;
  `Remove-Item -Recurse -Force` — только paranoid. Перенос — решение владельца +
  auditor. Осталось: i6…i8.
- 28.09.2026 · **B0-i6** (snip): 🔴 — V1 API, CLI отсутствует на Windows (нет
  snip/go/brew); замер экономии невозможен; префикс `snip` конфликтует с
  allowlist'ами ролей. Политика C — B1/token-guard. Улики — `target/wave0b-i6/`.
  Осталось: i7, i8.
- 28.09.2026 · **B0-i7** (Handoff v0.5.0): 🔴 — V1 API + runtime `@opencode-ai/plugin`
  (`src/tools.ts`), без default; проба в temp зафиксировала WARN + повтор ~16 раз;
  данные C: resume по `sessionID` (полный контекст) vs continuation-сводка
  (дешевле) + `session export`. Улики — `target/wave0b-i7/`. Осталось: i8 (отчёт).
- 28.09.2026 · **B0-i8**: отчёт `wave0b-report.md` заполнен; вердикты: 🟢 — CC
  Safety Net (перенос — решение владельца + auditor), 🔴 — Telemetry/Reporter/
  Identity/snip/Handoff, Shell Strategy — 🔴 установка/🟢 контент; рекомендации
  C/D/E/F переданы; коммит-пакет ждёт подтверждения (develop, как c13_records).
  Пробы завершены; канон не тронут; состояние — у владельца.
- 28.09.2026 · решения владельца: **своя обвязка B0-own** (P1 observe, P2 guard,
  P3 checkpoint, P4 metrics, P5 attribution) — после коммита B0 (прототипы в temp,
  перенос после аудита); **перенос CC Safety Net — позже** (пресет standard);
  **S/M-прогон** — после B0-own. Отчёт дополнен (§4 «Своя обвязка», §6 «Решения
  владельца»); реестр — строка `B0-own`. Далее: пакет `wave0b_records` → роль git.
