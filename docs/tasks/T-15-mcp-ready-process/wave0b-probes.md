# T-15 · Wave 0 фазы B — протоколы и результаты проб

- **Назначение:** рабочий журнал к [`wave0b-plan.md`](wave0b-plan.md): по каждому
  кандидату — паспорт, установка, сценарий, факты, вердикт. Не канон.
- **Статус:** 🚧 заполняется (28.09: B0-i1 готов; i2…i7 в работе).
- **Правило записи:** только факты (команда → вывод); улики — `target/wave0b-*`;
  один кандидат — один раздел; вердикт 🟢/🔴 с обоснованием и рекомендацией
  (C/D/E/F или «отклонить»).

## Сводка

| Кандидат | Итерация | Вердикт | Рекомендация | Улики |
|---|---|---|---|---|
| Инфраструктура проб (формы установки V2) | B0-i1 | 🟢 | — (форма зафиксирована) | `target/wave0b-i1/` |
| Shell Strategy | B0-i2 | 🔴 установка / 🟢 контент | C (раздел канона) | `target/wave0b-i2/` |
| Opencode Telemetry | B0-i3 | 🔴 (V1 Plugin API) | D: нативные `stats`/`session export` | `target/wave0b-i3/` |
| Subagent Reporter | B0-i4 | 🔴 (V1 Plugin API) | C: `--format json`/свой V2-плагин | `target/wave0b-i4/` |
| Agent Identity | B0-i4 | 🔴 (V1 API + dep) | C: своя атрибуция | `target/wave0b-i4/` |
| CC Safety Net | B0-i5 | 🟢 | C: перенос (после решения владельца + auditor) | `target/wave0b-i5/` |
| snip | B0-i6 | 🔴 (V1 API; CLI нет на Windows) | C: политика — B1/token-guard | `target/wave0b-i6/` |
| Handoff | B0-i7 | 🔴 (V1 API + dep) | C: `session-checkpoint` (resume) | `target/wave0b-i7/` |

## Шаблон раздела (копировать на кандидата)

### <Кандидат> · <репозиторий> · <версия/коммит>

- **Паспорт:** лицензия, статус (V1/V2, retired?), зависимости.
- **Установка:** temp/служебная зона; форма; результат startup (команда → вывод).
- **Сценарий пробы:** сессия/роль/задача.
- **Факты:** метрики (токены/время), артефакты.
- **Вердикт:** 🟢/🔴; рекомендация.
- **Откат:** шаг.

## Пробы

### Инфраструктура проб · temp-проект · OpenCode 2.0.18

- **Паспорт:** не кандидат; изоляция и фиксация форм установки V2 (карточка §5).
  Temp: `%TEMP%\opencode\wave0b-probe` (`git init`, dummy `.env`, `hello.txt`).
- **Установка (форма 1 — локальный файл):** `.opencode/plugins/<name>.ts`
  (`Plugin.define({id, setup})`, импорт `@opencode/plugin`; зависимости —
  `.opencode/package.json` + `npm install` в `.opencode/`: `added 283 packages in 60s`).
  Факт: лог `loading plugin …wave0b-probe.ts` (13:47:18, 13:53:02); setup выполнен —
  маркер `.opencode/wave0b-probe-loaded.json` (13:53:02.507Z, `opencode: 2.0.18`).
- **Установка (форма 2 — config `plugins`):** каталог `.opencode/probe-plugins/hello/`
  (index.ts) + `opencode.json`: `"plugins": ["./.opencode/probe-plugins/hello"]`.
  Факт: маркер `.opencode/wave0b-probe-config-loaded.json` (13:53:48.073Z,
  `via: plugins[]`).
- **Сценарий пробы:** `opencode run --auto --model opencode-go/deepseek-v4.1-flash
  "Ответь одним словом: ок"` (temp, агент по умолчанию `build`).
  Факт: `> build · deepseek-v4.1-flash` → `ок` (прогоны 13:53 и 13:53:48).
- **Startup-критерий:** `failed to load plugin` для temp-плагинов за 28.09 после
  13:47 — нет; оба маркера setup на месте. 🟢
- **Наблюдение:** прогон 1 (13:47) не завершился в CLI (kill по таймауту 240 с):
  сессия `ses_f17bae37bffebBsy55l2h8lIkQ` — `outcome: succeeded` (in 6281 / out 11;
  cost 0.00099627), `/api/experimental/session/…/wait` → 499 только после kill.
  Повторные прогоны — норма; риск для headless-проб i4/i7 (контроль таймаута,
  при повторе — `--print-logs`).
- **Вердикт:** 🟢 — изоляция и обе формы установки V2 зафиксированы, startup чистый.
- **Откат:** удалить temp-проект (вне репозитория).

### Shell Strategy · JRedeker/opencode-shell-strategy · v1.1.0

- **Паспорт:** MIT; инструкционный файл `shell_strategy.md` (не V2-плагин, кода
  нет); README-способ установки — `instructions: [URL]`; зависимостей нет; §7
  ссылается на V1-доки.
- **Установка (temp):** A — config `"instructions": ["./probe-instructions.md"]` с
  маркер-правилом «отвечай ровно ПАНТЕРА-9137» → `opencode run … "Ответь одним
  словом: ок"` = `ок` (правило не загружено; V2-доки: «accepts but does not load»).
  B — контроль: то же правило в temp `AGENTS.md` → `ПАНТЕРА-9137` (загружается).
- **Сценарий пробы:** headless-сессия в temp (`--auto`, модель
  `opencode-go/deepseek-v4.1-flash`). Живые hang-пробы (pager/editor) не
  проводились: V2-формы установки у кандидата нет, а non-TTY и запрет составных
  команд уже держит harness.
- **Сверка с дисциплиной ролей:** совместимо — одиночные команды, `git commit -m`,
  `--no-edit`, fail-fast, prefer tools; не переносить — Linux-only части; `git
  --no-pager` конфликтует с префиксными allowlist'ами (`git log *`); учесть
  `GIT_TERMINAL_PROMPT=0`.
- **Вердикт:** 🔴 установка (README-способ в V2 не работает; кандидат V1-эпохи);
  🟢 контент → рекомендация C (сжатый раздел «non-interactive shell» под
  pwsh/allowlist).
- **Откат:** удалить temp-вложения (вне репозитория).

### Opencode Telemetry · agostinilabsrl/opencode-telemetry · v0.2.0

- **Паспорт:** MIT; локальная телеметрия сессий (SQLite + CLI `octm`); плагин
  **V1-формы** (`src/index.ts`: функция + `export const server`, V1-хуки
  `event`/`tool.execute.before|after`); рантайм-зависимостей нет (импорты
  `@opencode-ai/plugin` — `import type`); CLI требует Bun (`bun:sqlite`; в среде
  Bun 1.4.2).
- **Установка (temp, формы V2):** config-пути — абс. и отн. к каталогу, отн. к
  файлу `src/index.ts` — **не подхвачены** (записи в логе нет);
  `.opencode/plugins/<name>/` c `package.json`+`exports` без корневого `index.ts`
  — не подхвачен; с корневым `index.ts` → `loading plugin …/index.ts`, затем
  `failed to load plugin`: `Plugin must export a default definition with an id and
  an effect or setup function. (cause: SchemaError(Expected object at ["default"]))`.
- **CLI:** `bun run bin/cli.ts help` → «octm — opencode-telemetry CLI»
  (report/inspect/health/config/cache/sql). `report` не запускался: без плагина БД
  не наполняется, CLI пишет в `~/.local/share/opencode-telemetry/` (вне temp).
- **Сверка (нативные V2):** `opencode stats --days 1` — агрегаты без плагина и без
  расхода токенов (19 сессий, 59 промптов, 1.2k шагов, 207.7m токенов, 97.1% tool
  success); `opencode session list` — сессии; `session export` — JSON.
- **Вердикт:** 🔴 для OpenCode 2.0.18 (V1 Plugin API; не загружается); рекомендация
  D: метрики — нативные `stats`/`session export`; кандидат — при V2-порте upstream.
- **Откат:** удалить каталог в temp (выполнено).

### Subagent Reporter · raisbecka/opencode-subagent-output · файл `subagent-reporter.ts`

- **Паспорт:** один файл; V1-форма — `export default async function` → объект
  V1-хуков; пишет в `process.stdout` (рассчитан на процесс CLI V1); режимы
  `default/json` (argv/env).
- **Проба (temp):** файл в `.opencode/plugins/` → `loading plugin …subagent-reporter.ts`
  → WARN `failed to load plugin`: «Plugin must export a default definition with an
  id and an effect or setup function» (SchemaError `["default"]`).
- **Вердикт:** 🔴 для V2 (V1 Plugin API; в V2 stdout — канал `serve --stdio`).
  Рекомендация C: наблюдаемость субагентов — `--format json` (NDJSON) или
  собственный V2-плагин по образцу `token-guard`.
- **Откат:** удалить файл в temp (выполнено).

### Agent Identity · gotgenes/opencode-agent-identity · v3.1.1

- **Паспорт:** MIT; npm-пакет; V1 API: `import { tool } from "@opencode-ai/plugin"`
  (runtime), именованные экспорты `AgentAttributionToolPlugin`/`AgentSelfIdentityPlugin`
  (V1-функции; хуки `experimental.chat.*`); default-экспорта нет.
- **Проба (temp):** `.opencode/plugins/agent-identity/` (src + root `index.ts`) →
  `loading plugin …agent-identity/index.ts` → WARN `failed to load plugin`:
  `Cannot find package '@opencode-ai/plugin' imported from …agent-attribution-tool.ts`.
- **Вердикт:** 🔴 для V2 (V1 API + рантайм V1-пакета). Рекомендация C: атрибуция
  `role`/модель — собственный V2-плагин; нативные данные сессии — проверить в C.
- **Откат:** удалить каталог в temp (выполнено).

### Baseline: нативный stdout субагента (вход C)

- `opencode run --auto … "Используй субагента … 'ПОДТВЕРЖДЕНО-42' …"` → stdout:
  `> build · deepseek-v4.1-flash`, `✓ Reply with exact token General Agent`,
  `> build · deepseek-v4.1-flash`, ответ-цитата. Внутренний стрим субагента
  (текст/тулы/thinking) не виден — пробел, который закрывал бы кандидат.
- Сбои плагинов сессию не ломают (только WARN в логе на каждый reload); в temp
  token-guard не подключён — срезы B1 не влияют.

### CC Safety Net · kenryu42/claude-code-safety-net · v2.4.11

- **Паспорт:** MIT; npm-пакет с V2-энтрипоинтом `./opencode/v2` (default-экспорт;
  peer `@opencode/plugin ^2.0.6`); один рантайм V1/V2; CLI `ccsn`; audit в
  `~/.cc-safety-net/`; поддержка OpenCode 1.18.29+/2.0.6+ (Windows заявлен).
- **Установка (temp):** локальная копия `dist/` + корневой `index.ts`; config
  `shell: pwsh` + object-form `plugins` с `options.shell = "powershell"`. Факт:
  первая загрузка — гонка копирования (`Cannot find module './dist/index.js'`),
  автоповтор — успех (`loading plugin …cc-safety-net` без WARN).
- **Сценарий (headless, temp; файлы менять запрещено):** (1) read `.env`;
  (2) `git push --force origin main`; (3) `git status --short`.
- **Факты:** (1) `✗ Read .env failed — Error: BLOCKED by CC Safety Net … Rule:
  secret.basename.env`; (2) `✗ git push --force … failed — BLOCKED … Rule:
  git.push-force … Use --force-with-lease` (сработало через обёртку `2>&1`);
  (3) `git status --short` выполнен. Audit JSONL (5 записей, `sessionId`,
  `ruleId`, `intent`) — копия в `target/wave0b-i5/`.
- **Анализатор (`explain`):** cwd=репо — `git reset --hard` → BLOCKED
  («…destroys all uncommitted changes permanently. Use 'git stash' first»);
  cwd=temp — ALLOWED (политика temp-root, by design); `Remove-Item -Recurse
  -Force target/…` → ALLOWED на standard (правило
  `powershell.remove-item-recursive-force-paranoid` выключено пресетом).
- **Сосуществование с permissions:** typed-deny через `tool.execute.before` поверх
  permissions: явный `deny` конфига финален, `allow`/`ask` может быть сужен;
  легитимные команды проходят; диалект shell проверяется (`cmd.exe` отклоняется;
  в репо уже `shell: pwsh`).
- **Вердикт:** 🟢 — перенос в служебную зону (решение владельца + `auditor`);
  форма: `opencode plugin add cc-safety-net@latest` + `options.shell=powershell`;
  пресет standard; вопрос paranoid — в C.
- **Откат:** temp-копия удалена; side effect — `~/.cc-safety-net/` (logs + cache).

### snip · VincentHardouin/opencode-snip · (пакет `opencode-snip`)

- **Паспорт:** MIT; V1-зависимость `@opencode-ai/plugin ^1.0.0`; префикс команд
  `snip` (внешний CLI `edouard-claude/snip`, Go; engines `node ^24`).
- **CLI (Windows):** `snip --version` → CommandNotFound; `go version` →
  CommandNotFound; brew нет — документированная установка неприменима.
- **Лоад-тест (temp):** `loading plugin …opencode-snip.ts` → WARN `failed to load
  plugin` («must export a default definition with an id and an effect or setup
  function»).
- **Замер экономии:** не проводился (CLI недоступен); цифры README (60–90%) —
  заявление вендора.
- **Конфликты:** префикс `snip` ломает префиксные allowlist'ы ролей
  (`git status *`); рассчитан на пайпы/POSIX, а дисциплина ролей запрещает
  составные команды; вывод `cargo test --all` (validator) ≈7 КБ < порога B1.
- **Вердикт:** 🔴; рекомендация C: токен-политика на базе B1 (`token-guard`) и
  harness-лимитов; snip — пересмотр при V2-порте, Windows-сборке и форме без
  префикса команд.
- **Откат:** temp-файл удалён.

### Handoff · joshuadavidthomas/opencode-handoff · v0.5.0

- **Паспорт:** MIT; V1-плагин (`export const HandoffPlugin: Plugin = async (ctx) =>
  …`, без default; `@opencode-ai/plugin`, `@opencode-ai/sdk`, `zod` в runtime;
  README — OpenCode v1.2.15+; конфиг `"plugin"`). Идея: `/handoff <goal>` →
  continuation-prompt + новая сессия; `read_session` — транскрипт прошлой сессии.
- **Проба (temp):** `src` + root `index.ts`, доустановлены `zod`/`@opencode-ai/sdk`
  → WARN `failed to load plugin`: (1) гонка копирования (`Cannot find module
  './src/plugin.ts'`); (2) `Cannot find package '@opencode-ai/plugin'` из
  `src/tools.ts` (runtime V1), повтор ~16 раз.
- **Сравнение (вход C):** resume по `sessionID` (`--continue|--session`, есть
  `--fork`) — полный контекст, дороже; continuation-prompt — сжатая сводка,
  дешевле, устойчив к обрыву; нативные аналоги — `session export` + самоотчёт роли.
- **Вердикт:** 🔴; рекомендация C: `session-checkpoint` — resume основным путём +
  опционально своя continuation-сводка (цель, решения, файлы, открытые вопросы).
- **Откат:** temp-каталог удалён; наблюдение — сломанный плагин даёт повторные
  WARN-перезагрузки (в служебной зоне не оставлять).
