# T-15 · B0-own — отчёт мини-волны (BO-i1…BO-i6)

- **Статус:** выполнен (28.09.2026 — BO-i1; 02.10.2026 — BO-i2…BO-i6).
  **Не канон**; перенос в служебную зону — после аудита `auditor` и решения
  владельца.
- **Рамка:** прототипы — в temp-полигоне `%TEMP%\opencode\wave0b-own`;
  репозиторий и канон не менялись; улики — `target/wave0b-own-i1…i6/`
  (вне git); рабочий журнал — [`wave0b-own.md`](wave0b-own.md);
  шпаргалка — [`wave0b-own-plugin-guide.md`](wave0b-own-plugin-guide.md).
- **Среда:** OpenCode 2.0.18 (BO-i1) → **2.0.22** (BO-i2…BO-i6); модель проб —
  `opencode-go/deepseek-v4.1-flash`; прогоны `opencode run --auto`.
- **Источник:** [`wave0b-report.md`](wave0b-report.md) §4/§6 (решение владельца:
  своя обвязка P1–P5 вместо чужих V1-плагинов).

## 1. Итоги итераций

| Итерация | Предмет | Вердикт |
|---|---|---|
| BO-i1 | каркас и разведка V2-API (хуки, события) | 🟢 форма установки, таблица «P→хук» |
| BO-i2 | P2 `wave0-guard`: плагин vs `policies` | 🟢 комбинация (плагин + policies + permissions) |
| BO-i3 | P1 `wave0-observe`: наблюдаемость субагентов | 🟢 журнал + агрегат + сводка родителю |
| BO-i4 | P4 `metrics-report.mjs`: отчёты из `stats`/`export` | 🟢 цепочки/роли/модели воспроизведены |
| BO-i5 | P3 `wave0-checkpoint`: сводка останова + resume | 🟢 сводка + `--session`; команда — форма подтверждена |
| BO-i6 | P5 `wave0-attribution`: онлайн-атрибуция | 🟢 **свернуть** (покрыто P1/P4 + context-хук) |

## 2. Ключевые факты по механизмам

- **P1 observe.** `ctx.event.subscribe()` — полный жизненный цикл сессии
  (`session.created` → `inbox.*` → `execution.*` → `step.*` → `reasoning/text.*`
  → `usage.*` → `succeeded`); дочерний `session.created` несёт `parentID` и
  `agent`; фильтр — `location.directory` + множество известных `sessionID`
  (поток серверный; `projectID` temp = `global` — фильтр по проекту ненадёжен);
  сводка родителю — `ctx.session.prompt` работает (доставка подтверждена);
  `ctx.session.list` у плагина нет.
- **P2 guard.** `permission.hook("evaluate")` — deny до запуска; **`ask` +
  `--auto` = выполнение** (защита — только `deny`); `experimental.policies` —
  hard-deny («Blocked by configuration policy») и применяется **после** хука;
  `permissions` deny — финален, хук не вызывается; шаблоны policies статические
  (`*--force*` не ловит `-Force`); scanner дробит составные команды.
- **P3 checkpoint.** `session-checkpoint.mjs` (export → цель/статус/файлы/
  маркеры/resume) пригоден для C9; `opencode run --session <id>` продолжает ту
  же сессию (2 промпта, succeeded); `command.transform` + `editor.add({name,
  description, template})` регистрирует команду (`command.list` видит);
  форма `{info, template}` ломает `command.list`.
- **P4 metrics.** `metrics-report.mjs` (CLI-only): цепочка прогона 02.10 —
  32 сессии, **$1.4133**, роли/модели 1:1 с разбором; `stats` без `--days`
  агрегирует всё; `opencode` в Node — npm-шим (нужен `shell: true`).
- **P5 attribution.** context-хук даёт `agent`/`model` для родителя и
  субагента; export: `info.agent` есть у детей, у root — нет; `session list`
  поля `agent` не содержит; хранилище top-level пишет `agent=build`
  (при `default_agent=lead`) — связка с F60.

## 3. Рекомендации по переносу (после аудита; решение владельца)

| Артефакт | Куда | Доработки перед переносом |
|---|---|---|
| `wave0-observe.ts` (P1) | `.opencode/plugins/` | bounded-журнал/ротация; режим (вкл/выкл); при нужде — встроить атрибуцию (P5) |
| guard (P2) | `experimental.policies` — глобальный конфиг/служебная зона; `permissions` — ролевые правила репо; плагин — `.opencode/plugins/` | решение C10: что в policies, что в плагин; CC Safety Net — отдельным решением (пресет standard) — **исполнено 02.10.2026: перенесён, 4 точных правила глобально + плагин `wave0-guard` (якорные правила)** |
| `session-checkpoint.mjs` (P3) | `.opencode/scripts/` | разделять файлы «изменённые/прочитанные»; уточнить маркеры; обрезка статуса; команда `checkpoint` — при нужде |
| `metrics-report.mjs` (P4) | `.opencode/scripts/` | кэш экспортов; `--days`/`--json`; DEP0190 (shell:true) — косметика; **хрупкость поиска детей по маркеру `<subagent sessionID=…>`** (при смене формата экспорта) — кэшировать/резервировать формат |
| P5 | — | свернуть: покрыто P1/P4 (+ context-хук при онлайн-нужде) |

## 4. Наблюдения (кандидаты в реестр находок — вносит `migrator`)

1. `opencode reload` в одной локации сбрасывает MCP-каталог общей сессии
   (наблюдено дважды; восстановился) — осторожность в служебной зоне.
2. Атрибуция top-level: хранилище `agent=build` при `default_agent=lead`;
   export root без `agent` (связка F60).
3. Watcher повторяет трансформации/регистрации при перезагрузке (дубли
   `command.add`) — идемпотентность обязательна.
4. `stats` без фильтров агрегирует все проекты; `projectID` temp = `global`.
5. Скрипты на Windows: `opencode` — npm-шим (`.ps1/.cmd`), прямой spawn →
   ENOENT; DEP0190 при `shell: true`.
6. Модель может отказаться от деструктивной команды даже в песочнице —
   промпт пробы должен требовать выполнения («техническая проба»).
7. `sh_*` (shell-сессии) не считать сессиями в агрегатах; дельты
   (`reasoning/text.delta`) — семплировать.

## 5. Готовность к переносу (чек-лист)

- [x] по каждой P — улики в `target/wave0b-own-i1…i6/` и раздел журнала;
- [x] протокол B0 соблюдён (temp-полигон, вердикты, откат — удаление полигона);
- [x] репозиторий и канон не менялись (диффы — только docs/лента/память);
- [x] BO-i1…BO-i6 закоммичены в `develop` (серия process-коммитов);
- [ ] аудит `auditor` (независимая проверка мини-волны) — **перед переносом**;
- [ ] решение владельца: состав переноса и адреса (таблица §3);
- [ ] откат полигона (удалить `%TEMP%\opencode\wave0b-own`; улики — по решению).

**Обновление 02.10.2026:** аудит проведён (P1/P2 — расхождений нет; P3 закрыт);
решение владельца — «Без P2 до C10»: **P1/P3/P4 перенесены** в
`.opencode/plugins/wave0-observe.ts` и `.opencode/scripts/{session-checkpoint,metrics-report}.mjs`;
**P2 (guard) — после решения C10**; полигон сохранён до заморозки.

## 6. Открытые вопросы владельцу

1. Состав переноса: P1/P3/P4 (плагин + скрипты) — переносим? P2 (guard) —
   какие правила в policies (глобально), какие в `permissions` репо?
2. Когда: сейчас или после фазы C (C10 — решение по страховкам)?
3. Полигон: удалить после переноса или сохранить до заморозки?
