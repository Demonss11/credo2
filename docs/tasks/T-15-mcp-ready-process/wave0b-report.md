# T-15 · Wave 0 фазы B — отчёт

- **Статус:** 🚧 отчёт заполнен 28.09.2026 (B0-i1…i7); финализация — по
  подтверждению владельца (коммит документов) и отдельному решению о переносе
  зелёного (после `auditor`). Канон не правился, репозиторий проб не касался.
- **Назначение:** итог волны к карточке [`wave0b-plugins.md`](wave0b-plugins.md):
  что включено и как откатить, метрики, выводы, рекомендации фазам C/D/E/F,
  вопросы владельцу. Не канон.

## 1. Что проверено и с каким результатом

| # | Кандидат / проба | Вердикт | Рекомендация | Улики |
|---|---|---|---|---|
| B0-i1 | Инфраструктура проб (формы установки V2) | 🟢 | форма зафиксирована | `target/wave0b-i1/` |
| B0-i2 | Shell Strategy (v1.1.0) | 🔴 установка / 🟢 контент | C: раздел «non-interactive shell» | `target/wave0b-i2/` |
| B0-i3 | Opencode Telemetry (v0.2.0) | 🔴 | D: нативные `stats`/`session export` | `target/wave0b-i3/` |
| B0-i4 | Subagent Reporter | 🔴 | C: `--format json` / свой V2-плагин | `target/wave0b-i4/` |
| B0-i4 | Agent Identity (v3.1.1) | 🔴 | C: своя атрибуция | `target/wave0b-i4/` |
| B0-i5 | CC Safety Net (v2.4.11) | 🟢 | C: перенос в служебную зону | `target/wave0b-i5/` |
| B0-i6 | snip | 🔴 | C: политика — B1/token-guard | `target/wave0b-i6/` |
| B0-i7 | Handoff (v0.5.0) | 🔴 | C: `session-checkpoint` (resume) | `target/wave0b-i7/` |

**Формы установки V2 (B0-i1, факты):**

- локальный файл `.opencode/plugins/<name>.ts` — автозагрузка; зависимости —
  `.opencode/package.json` + `npm install`/`npm ci`;
- config `plugins[]` — пакет или каталог; каталог подхватывается при наличии
  корневого `index.ts`; произвольные пути вне `.opencode` и абсолютные пути в
  пробах не подхватывались (наблюдения, не догма);
- `instructions` в V2 не загружается (README-способ Shell Strategy не работает);
  рабочий путь инструкций — `AGENTS.md`;
- V2-плагин обязан экспортировать default c `id` + `setup`/`effect`; V1-форма
  (функция/`server`) отклоняется: `Plugin must export a default definition with an
  id and an effect or setup function`.

**Включено в результате волны:** ничего — канон и служебная зона не менялись.
**Кандидат на перенос (решение владельца + `auditor`):** CC Safety Net (🟢).
**Откат:** temp-проект `%TEMP%\opencode\wave0b-probe` удаляется целиком; улики —
`target/wave0b-i*/` (вне git); при отмене переноса ccsn — `opencode plugin remove
cc-safety-net`; side effect пробы — `~/.cc-safety-net/` (`logs/`, `compile-cache/`,
можно удалить; копия аудита — в `target/wave0b-i5/`).

## 2. Метрики волны

- Прогоны: ~10 headless-сессий в temp (`opencode run --auto --model
  opencode-go/deepseek-v4.1-flash` — модель по решению владельца, как у ролей).
- `opencode stats --days 1` (снимок B0-i8): 23 сессии · 33 субагента; 63 промпта ·
  1.2k шагов · 219.9m токенов; 97.1% tool success. Снимок B0-i3: 19 · 32 ·
  59 промптов · 207.7m — волна добавила ≈4 сессии и ≈12.2m токенов (сервисная
  работа + пробы).
- B1 (`token-guard`, wave 0 A) — рабочий механизм: 11 срезов · 156 831 байт
  (`target/wave0-i4-plugin-stats.json`).
- Аномалия: 1 из ~10 прогонов не завершился в CLI (сессия `succeeded`,
  `/wait` — 499 после kill, B0-i1); повторные — норма.
- Аудит CC Safety Net: 5 записей (2 deny) в `~/.cc-safety-net/logs/**`
  (копия — `target/wave0b-i5/`).
- Токен-цена проб: провалы плагинов — без вызовов модели; содержательные прогоны —
  короткие сессии; точные цифры — в нативных `stats`.

## 3. Выводы

1. Пул awesome-opencode — преимущественно V1-эпохи: 7 из 8 кандидатов не
   загружаются в OpenCode 2.0.18 (нет default/id+setup; runtime `@opencode-ai/*`).
   Рабочим оказался один — CC Safety Net (V2-энтрипоинт `./opencode/v2`).
2. Совместимость кандидата с V2 проверяется за минуты: startup + `loading plugin`/
   `failed to load plugin` в логе; пробы в temp дают чистые вердикты.
3. Нативные средства V2 закрывают часть пробелов без плагинов: метрики —
   `opencode stats`/`session export` (D), транскрипты — `session export` (C).
4. Эксплуатационные наблюдения: сбойные плагины сессии не ломают, но дают
   повторные WARN/перезагрузки — в служебной зоне не оставлять; первая загрузка
   каталога-плагина медленная (до ~7 с); гонки копирования ловятся watcher'ом
   (нужен атомарный write или малый состав копирования).
5. Пробы вне канона и репозитория отработали: репо чист, канон не тронут, улики —
   в `target/wave0b-i*/` (вне git); temp-полигон `%TEMP%\opencode\wave0b-probe`
   пригоден для следующих волн.

## 4. Рекомендации (C/D/E/F)

**C (канон/процесс):**

- Shell Strategy → сжатый раздел «non-interactive shell»: одиночные команды,
  `git commit -m`, `git merge --no-edit`, неинтерактивные формы, fail-fast, prefer
  tools; не переносить Linux-only (apt/sudo/ssh) и `git --no-pager` в командной
  форме (конфликт с префиксными allowlist'ами ролей).
- CC Safety Net → кандидат на перенос: `opencode plugin add cc-safety-net@latest`,
  `options.shell = "powershell"`, пресет standard; решить вопрос paranoid
  (`Remove-Item -Recurse -Force`); перенос — после `auditor`.
- Наблюдаемость субагентов (Subagent Reporter) → `--format json` (NDJSON) или
  собственный V2-плагин по образцу `token-guard`; атрибуция роль/модель (Agent
  Identity) → собственный V2-плагин либо нативные данные сессии (проверить в C).
- Handoff → правило останова `session-checkpoint`: resume по `sessionID`
  (`--continue|--session`, `--fork`) как основной путь; continuation-сводка —
  своими средствами (цель, решения, файлы, открытые вопросы); `read_session` →
  `session export`.
- snip → токен-политика на базе B1/token-guard; пересмотр при V2-порте +
  Windows-сборке.

**D (верификация/метрики):** чек-лист §8 — на нативных `opencode stats` /
`session export`; Opencode Telemetry — отклонить (V1), пересмотреть при V2-порте
upstream.

**E (MCP-дизайн):** изменений нет; референсы kibi/Semantic Anchors/BRHP — как были
(B0 их не касался).

**F (инструменты владельца):** CC Safety Net CLI (`status/doctor/explain/logs/gui`)
— готовый диагностический слой; Plannotator — не проверялся (tier 2, вне B0);
уведомления — нативные V2 (`cli.json`).

**Своя обвязка (B0-own, решение владельца 28.09.2026):** вместо чужих V1-плагинов —
собственные V2-плагины/скрипт по идеям кандидатов (все локальные; код — свой):

- **P1 `wave0-observe`** — субагент-наблюдаемость (идея Subagent Reporter): события
  субагентских сессий → `target/…jsonl` + опциональная сводка родителю; вход C/D.
- **P2 `wave0-guard`** — страховка (идея CC Safety Net): deny `--force`/
  `reset --hard`/секретов; решить на прототипе — плагин (`tool.execute.before`/
  `permission.hook`) или `experimental.policies` (C10).
- **P3 `wave0-checkpoint`** — структурная сводка останова/обрыва (идея Handoff):
  цель, решения, файлы, открытые вопросы; вход C9.
- **P4 `metrics-report.mjs`** — отчёты из `session export`/`stats` (замена Telemetry
  для D; свёртка по цепочкам, роль/модель).
- **P5 `wave0-attribution`** — атрибуция `role`/модель крючком (частично
  перекрывается P4).
- Формат: прототипы в temp-полигоне → вердикты по протоколу B0 → перенос в
  `.opencode/plugins/`/`.opencode/scripts/` после аудита; Shell Strategy — текстом
  в канон; snip — не берём.

## 5. Вопросы владельцу

1. **Перенос CC Safety Net** в служебную зону (`opencode plugin add` + `auditor`) —
   подключать сейчас?
2. **Пресет ccsn**: standard (по умолчанию) или paranoid (блокирует и
   `Remove-Item -Recurse -Force`)?
3. **S/M-прогон после B0** (B1-F26): на какой задаче запускать пилот с обвязкой?
4. **Коммит документов B0** — один пакет прямо в `develop` (как `c13_records`):
   `wave0b-plugins.md`, `wave0b-plan.md`, `wave0b-probes.md`, `wave0b-report.md`,
   README T-15, лента `service-mcp-ready-r3.md`, `.opencode/memory/service.md`.
5. **Мелочи:** удалить ли `~/.cc-safety-net/` (артефакт пробы); `state/current`
   (Run 5/T-04) устарел — чистить ли после B0; следить ли за V2-портами
   Telemetry/snip (приоритет C/D)?

## 6. Решения владельца (28.09.2026)

- **Своя обвязка (B0-own):** берём P1–P5 (§4); порядок — зафиксировать в отчёте →
  коммит B0 → мини-волна «B0-own» (прототипы в temp, вердикты по протоколу B0,
  перенос после аудита).
- **Перенос CC Safety Net:** позже, отдельным шагом; пресет — **standard**.
- **Коммит документов B0:** подтверждён после фиксации этой правки — пакет
  `wave0b_records` (один коммит, прямо в `develop`, затем `push origin develop`).
- **S/M-прогон (B1-F26):** после закрытия текущих вопросов (B0-own).
