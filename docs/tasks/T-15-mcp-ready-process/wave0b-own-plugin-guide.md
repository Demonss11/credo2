# T-15 · B0-own — шпаргалка: V2-плагин за шаги (для агентов)

- **Статус:** рабочая инструкция мини-волны B0-own (не канон). Проверено на
  OpenCode 2.0.18 (`opencode --version`); источник — проба BO-i1
  ([`wave0b-own.md`](wave0b-own.md)) + V2-доки.
- **Кому:** агентам, которые пишут/правят плагины P1–P5 или проверяют чужие
  V2-плагины. Цель — не разыскивать API заново.
- **Экономный маршрут:** §1 форма → §2 шаблон → §3 установка/проверка → §5
  таблица «задача → хук» → §6 формы данных → §8 грабли. Детали API — §4.

## 1. Форма установки (зафиксировано B0-i1)

| Что | Как |
|---|---|
| Локальный плагин проекта | файл `.opencode/plugins/<name>.ts` — автозагрузка, config не нужен |
| Зависимость типов | `.opencode/package.json`: `{"dependencies": {"@opencode/plugin": "^2.0.18"}}` + `npm install` в `.opencode/` (пустой каталог: 283 пакета, ~40–60 с) |
| Пакет/каталог | config `plugins[]` (`"opencode-acme-plugin@1.2.0"`, `"./plugins/local"`); каталог грузится только с корневым `index.ts` |
| CLI-плагины (TUI) | `cli.json` → `plugins[]`; в пробах не использовались |

- V2-плагин **обязан** экспортировать default с `id` + `setup` (или `effect`).
  V1-форма (функция/`server`): WARN `failed to load plugin` + cause «Plugin must
  export a default definition with an id and an effect or setup function».
- `instructions` в конфиге V2 **не загружается** — рабочий путь инструкций
  `AGENTS.md` (B0-i2).
- Пробы плагинов — в temp-проекте (%TEMP%), не в служебной зоне; перенос — после
  аудита.

## 2. Минимальный шаблон (копировать)

```ts
// .opencode/plugins/my-plugin.ts
import { Plugin } from "@opencode/plugin";

export default Plugin.define({
  id: "my-plugin",
  async setup(ctx) {
    // маркер: доказательство, что setup выполнен
    console.log(`[my-plugin] loaded in ${ctx.app.version} @ ${ctx.location.directory}`);

    // подписки/хуки (примеры — §4); всё можно снять через Registration.dispose()
    const reg = await ctx.tool.hook("execute.before", (event) => {
      console.log(`[my-plugin] tool=${event.tool}`);
    });

    return () => {
      void reg.dispose(); // cleanup при выгрузке/перезагрузке плагина
    };
  },
});
```

Всё покрывать try/catch: ошибка плагина не должна ломать сессию (hook с ошибкой
пропускается, результат остаётся как есть).

## 3. Установка и проверка (чек-лист)

1. **Атомарный write.** Файл плагина писать/перезаписывать **одним** write:
   watcher перезагружает плагин на каждое изменение и ловит промежуточные
   состояния (B0: гонки). Сбойная перезагрузка самоизлечится следующим
   изменением/рестартом, но лог замусорится.
2. **Startup-критерий.** В `~/.local/share/opencode/log/opencode.log`
   (Windows: `%USERPROFILE%\.local\share\opencode\log\opencode.log`):
   - ждём `msg="loading plugin" id="…<name>.ts"`;
   - НЕ ждём `level=WARN message="failed to load plugin" target=…`;
   - первая загрузка каталога-плагина бывает медленной (B0: до ~7 с).
3. **Маркер из setup** — файл в `ctx.location.directory` (JSON: версия, время,
   stage). Это единственное надёжное «setup выполнен», не зависящее от логов.
4. **Живая проба.** В каталоге проекта:
   `opencode run --auto --model <provider/model> "<короткий промпт>"`.
   Полезно: `--agent`, `--format json`, `--print-logs` (логи в stderr),
   `--session/-s`, `--continue/-c`, `--fork`.
5. **Аномалия.** Первый headless-прогон после подъёма проекта может не завершиться
   в CLI (сессия при этом `succeeded`); повторный прогон — норма (B0-i1 и BO-i1).
   Контроль таймаута (240–300 с), при повторе — `--print-logs`.
6. **Осторожно со счётчиками в логе.** Лог фиксирует spawn собственных команд:
   `rg "failed to load plugin"` попадёт в лог и «увеличит» счётчик (самореферентно).
   Проверять адресно — по имени/пути плагина.

## 4. Карта контекста `ctx` (V2 Plugin API)

| Область | API | Назначение |
|---|---|---|
| Общее | `ctx.app.version`, `ctx.location.directory/project`, `ctx.options` | версия, каталог location, опции из config-объекта `plugins[]` |
| Хранилище | `ctx.storage.get/set(key, value)` | персистентный JSON плагина (счётчики; пример — `token-guard`) |
| Инструменты | `ctx.tool.hook("execute.before"\|"execute.after", cb)` | вход до исполнения (правим `input`), результат после (`event.result`, `status`) |
| Сессии | `ctx.session.hook(name, cb, {providerID?})` | `prompt` \| `context` \| `compaction` \| `generate` \| `title` \| `model.request` \| `http.request` \| `http.response` \| `retry` \| `experimental.ws.*` |
| Сессии (клиент) | `ctx.session.create/get/context/switchAgent/switchModel/prompt/generate/command/synthetic/interrupt/rename/wait` | работа с сессиями как клиент |
| Права | `ctx.permission.hook("evaluate", cb)` | **точка deny**: `event.effect = "deny"`; выполняется для `allow` и `ask` |
| Shell | `ctx.shell.hook("create.before", cb)` | правка `command/cwd/timeout/shell/env` перед запуском |
| Команды | `ctx.command.list/transform/reload` | `list()` → `{location, data:[{name, description}]}`; `transform(editor.add({...}))` — свои команды |
| Агенты | `ctx.agent.list/get/transform/reload` | `list()` → `{location, data:[Agent.Info]}` (mode, permissions, request) |
| События | `ctx.event.subscribe({signal})` | `AsyncIterable<OpenCodeEvent>` — публичный поток сервера (см. §6) |
| Трансформы | `ctx.provider/model/skill.transform`, `reload()` | реестры провайдеров/моделей/скиллов |
| Генерация | `ctx.generate.text({model, prompt})` | вспомогательный вызов модели (например, в permission-хуке) |

Регистрации возвращают `Registration` — снимать через `await reg.dispose()` или
cleanup из `setup`.

## 5. Таблица «задача → хук» (P1–P5, BO-i1)

| Задача | Основной механизм | Статус |
|---|---|---|
| P1 observe: события сессий/субагентов | `ctx.event.subscribe()`: `session.created` несёт `data.parentID`, `agent`, `model`; др. `session.*`-события — §6 | 🟢 проба |
| P2 guard: deny `--force`/`reset --hard`/секретов | `permission.hook("evaluate")` (effect=deny; конфиг-deny финален) + аудит `tool.execute.before`; альтернатива — `experimental.policies` | 🟢 проба (policies — доки, проба в BO-i2) |
| P3 checkpoint: сводка/восстановление | `session export` (транскрипт) + `session.hook("prompt")` + команды (`ctx.command.transform`, config `commands`, `.opencode/commands/*.md`) | list() 🟢; transform/запуск — не проверялся |
| P4 metrics: отчёты | нативно: `opencode stats --json`, `opencode session export` (§7) | 🟢 проба |
| P5 attribution: role/модель | нативно `Session.Info.agent/model`; онлайн — `session.hook("context")` (`event.agent`, `event.model`) | 🟢 проба |

## 6. Проверенные формы данных (снимок проб 28.09.2026)

**События** (`ctx.event.subscribe()`): объект с ключами `id, created, type,
durable?, location?, data`. Наблюдённые типы: `session.created`, `session.renamed`,
`session.inbox.enqueued/delivered`, `session.execution.started/succeeded`,
`session.instructions.updated`, `session.step.started/streamed/ended`,
`session.reasoning.started/delta/ended`, `session.text.started/delta/ended`,
`session.tool.input.started/ended`, `session.tool.called/progress/success`,
`session.usage.updated`; служебные реестровые: `model.updated`, `provider.updated`,
`agent.updated`, `command.updated`, `skill.updated`, `integration.updated`,
`websearch.updated`. `session.idle` в потоке не наблюдался (есть сообщение
`type:"idle"` в export).

- `session.created.data`: `{sessionID, projectID, location{directory}, subpath,
  parentID?, slug, title, agent?, model?, version}` — **источник связки
  родитель→субагент и атрибуции**.
- Поток **серверный**: события чужих проектов приходят с их `location.directory`;
  события `usage/execution` — с `location=null` и любым `sessionID`. Фильтровать
  по проекту/сессии, иначе журнал засорится.

**`permission.evaluate`**: `{sessionID, agent?, action, resources[], metadata?,
source{type,messageID,id}, effect, message?}`. Наблюдённые `action`: `subagent`
(resource — имя агента), `shell` (resource — команда), `read` (resource — путь).
`effect` менять можно (`allow/ask/deny`); явный `deny` из конфига финален — хук
не вызывается. `message` — причина отказа/эскалации.

**`tool.execute.*`**: before — `{tool, sessionID, agent, messageID, id, input}`;
after — то же + `{status: "completed"|"error", result|error}`. Reject-API нет:
отказ делать через permission-хук.

**`session.context`** (агентский цикл): `{sessionID, model{providerID,id,variant},
system[], messages[], options{}, agent, tools{}}` — `event.agent`/`event.model`
даже для субагента. `providerID`-скоуп для request-хуков; хуки каждого kind
(`context/compaction/generate/title`) регистрируются отдельно.

**`ctx.command.list()`**: `{location:{directory}, data:[{name, description}]}`
(встроенные `init`, `review`). Файловые команды — `.opencode/commands/**/*.md`
с фронтматтером (`description`, `agent`, `subagent`); пример в репо —
`.opencode/commands/git/status.md`.

**`ctx.agent.list()`**: `{location, data:[{id, name, mode, hidden, permissions[],
request, description}]}` — эффективные права агента видны без чтения конфигов.

## 7. Нативные источники (не писать плагин, если хватает CLI)

- `opencode stats --json [--days N|--year Y|--all] [--project .] [--models]
  [--tools] [--cost] [--full]` — sessions/subagents/prompts/steps/tokens/cost/
  tools(usage+reliability)/activeDays/streak/activity/models.
- `opencode session export [--sanitize] [<ses_id>]` — JSON: `info{id, parentID,
  projectID, agent, model, cost, tokens, outcome, time{created,updated,idle},
  title, location}` + `messages[]` (assistant: `agent`, `model`, `content`,
  `snapshot{start,end,files}`).
- `opencode session list` (в т.ч. субагенты в списке), `session import`.
- HTTP (experimental, V2-клиент): `GET /api/experimental/session/stats`,
  `GET /api/experimental/session/{id}/export`; фильтр родителя —
  `GET /session?parentID=null`.
- Управление плагинами: `opencode plugin add/list/check/update/remove`;
  `opencode reload` — конфиг; `opencode service restart` — рестарт сервера
  (перезагрузка кода плагинов при сомнении в watcher).

## 8. Грабли и правила (опыт)

1. **V1 ≠ V2.** Старый `@opencode-ai/plugin`, функции/`server`-экспорт и
   `tool.execute.before`-стиль из V1-плагинов не работают — падают в WARN при
   загрузке (B0: 7 из 8 кандидатов).
2. **Один атомарный write** на файл плагина; watcher перезагружает по каждому
   изменению.
3. **Ошибку плагина глушить** (результат/схемы не трогаем): `try/catch` вокруг
   тела хука, иначе шум в логе и потеря данных события.
4. **`prompt`-хук не exactly-once**: ретраи и конкурентные отправки могут вызвать
   его повторно; side-effect'ы делать идемпотентными.
5. **`context`-хук** — только агентский цикл; служебные запросы (`title`,
   `generate`, `compaction`) — свои хуки, у них нет `agent`/`tools`.
6. **codemode-нюанс** (из W0-i3/`token-guard`): при `codemode=true` MCP-схемы не
   являются ключами `event.tools` — срез схем по префиксам не срабатывает;
   включать при прямой экспозиции MCP.
7. **Права**: права не менять плагином «на ходу» — только через permission-хук
   или `permissions[]`/`experimental.policies` в конфиге (канон/service zone —
   решение владельца).
8. **Кириллица в консоли Windows**: JSON/JSONL-файлы писать/читать в UTF-8;
   mojibake в консольном выводе `Get-Content` — артефакт кодировки, не порча
   файла.
9. **Секреты**: `permission`-хук получает `resources` (команда/путь) — не
   логировать содержимое файлов/выводов без нужды; журналы проб — в temp.
10. **Проверка — до переноса**: вердикт и улики по протоколу B0
    ([`wave0b-probes.md`](wave0b-probes.md)); перенос в `.opencode/**` — после
    аудита `auditor`.

## 9. Ссылки

- Рабочий пример V2-плагина в репо: `.opencode/plugins/token-guard.ts`
  (хуки `tool.execute.after`/`session.context`, `ctx.storage`, счётчики).
- Протокол разведки и таблица: [`wave0b-own.md`](wave0b-own.md) §BO-i1;
  формы установки: [`wave0b-probes.md`](wave0b-probes.md) §B0-i1.
- Доки V2: <https://opencode.ai/v2/docs/build/plugins> (API, хуки, события),
  <https://opencode.ai/v2/docs/plugins> (установка/CLI), <https://opencode.ai/v2/docs/config>
  (config), <https://opencode.ai/v2/docs/api> (HTTP/схемы), <https://opencode.ai/v2/docs/build/client> (клиент).
- Шпаргалка — кандидат в канон: отдельный skill `.opencode/skills/**` или раздел
  AGENTS.md — решением владельца и по протоколу (журнал → аудит → приёмка).
