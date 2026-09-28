# T-15 · Wave 0 фазы B — инструментальная обвязка пилотов (вне канона)

- **Тип:** подготовительный пакет фазы B задачи [T-15](README.md) («wave 0 B»);
  **вне канона** — `AGENTS.md`, `.opencode/rules/**` и права ролей не правятся.
- **Дата:** 2026-09-28 · **Автор:** сервисная сессия по поручению владельца.
- **Статус:** ✅ исполнено 28.09.2026 (B0-i1…B0-i8): план —
  [`wave0b-plan.md`](wave0b-plan.md), протоколы и вердикты —
  [`wave0b-probes.md`](wave0b-probes.md), итог и рекомендации —
  [`wave0b-report.md`](wave0b-report.md); перенос CC Safety Net — решение владельца
  + `auditor`; коммит документов — по подтверждению.
- **Основание:** разбор каталога
  [awesome-opencode](https://github.com/awesome-opencode/awesome-opencode#plugins)
  (136 плагинов, 28.09.2026) и решения владельца 28.09.2026: раскладка по фазам
  B0/C/D/E/F; пробы «temp-проект → служебная зона»; исполнитель — сервисная
  сессия вне канона.
- **Связано:** [карточка T-15](README.md) (B0/B1, C–F); wave 0 A
  ([`wave0-token-hygiene.md`](wave0-token-hygiene.md),
  [`wave0-report.md`](wave0-report.md)); записка
  [`../../analysis/mcp-ready-process.md`](../../analysis/mcp-ready-process.md) §10;
  [D44](../../decisions/D44-run5-refinements.md),
  [D45](../../decisions/D45-wave0-quality-config.md).

## 1. Цель и границы

Цель — без правок канона подготовить и проверить инструментальную обвязку
пилота (S/M, Run 6) и собрать проверенные кандидаты для фаз C–F: наблюдаемость,
метрики, состояние/восстановление, гигиена shell, уведомления.

**Входит:**

- пробы кандидатов в изолированном temp-проекте; перенос зелёных результатов
  в служебную зону — по подтверждению владельца и после `auditor`;
- протоколы, улики (`target/wave0b-*`, вне git) и вердикты по каждому кандидату;
- отчёт с рекомендациями: канон (C), верификация (D), MCP-дизайн (E),
  инструменты владельца (F).

**Не входит (границы):**

- правки канона (`AGENTS.md`, `.opencode/rules/**`, агенты/права) — фаза C;
- продуктовый код (`src/**`, `tests/**`) — не трогается; DoD не затрагивается;
- оркестраторы, «память-заменители», retired/V1-плагины — см. §6.

**Принцип:** сначала измерить в изоляции — потом переносить; откат каждой пробы
тривиален (удалить плагин/пакет).

## 2. Диагноз: пробелы пилота и чем закрываем

| # | Пробел (по фазам B/C/D) | Кандидат | Ожидаемый эффект |
|---|---|---|---|
| 1 | Метрики прогона собираются вручную (`session export`, таблицы) | `Opencode Telemetry` | локальная SQLite-история; отчёты CLI без расхода токенов; свёртка по цепочкам сессий |
| 2 | Схема состояния — без внешнего референса сходимости | `BRHP` (изучение, без установки) | идеи графа scopes/validation signals для `state-schema` и `validate-state.mjs` |
| 3 | Headless-прогоны ролей непрозрачны | `Subagent Reporter` | стрим шагов субагентов в stdout |
| 4 | Нет атрибуции «роль ↔ ответ/модель» | `Agent Identity` | атрибуция для метрик и аудита |
| 5 | Обрыв/лимит шагов: только resume по `sessionID` | `Handoff` | continuation-prompt — данные для правила останова |
| 6 | Shell-зависания (pager/editor/ввод) | `Shell Strategy` | инструкция non-interactive shell |
| 7 | Деструктивные команды ловятся только правилами ролей | `CC Safety Net` | блокировка до исполнения (git/fs/секреты) |
| 8 | Вывод shell дорог (продолжение B1 wave 0 A) | `snip` (опция) | −60–90% токенов shell-вывода |
| 9 | Уведомления владельцу на гейтах/завершении | нативные средства V2 (`cli.json`) | без плагина: `Opencode Notify` — retired V1 |

## 3. Состав волны

**Tier 1 — пилотируем:**

| Кандидат | Репозиторий | Роль в волне |
|---|---|---|
| Shell Strategy | `JRedeker/opencode-shell-strategy` | инструкция non-interactive shell; кандидат в канон C (B0-i2) |
| Opencode Telemetry | `agostinilabsrl/opencode-telemetry` | метрики прогонов; вход D (B0-i3) |
| Subagent Reporter | `raisbecka/opencode-subagent-output` | наблюдаемость субагентов (B0-i4) |
| Agent Identity | `gotgenes/opencode-agent-identity` | атрибуция `role`/модели (B0-i4) |
| CC Safety Net | `kenryu42/claude-code-safety-net` | защита от деструктивных команд (B0-i5) |
| Handoff | `joshuadavidthomas/opencode-handoff` | продолжение после обрыва/лимита (B0-i7) |

**Tier 2 — опции и референсы:**

| Кандидат | Репозиторий | Роль |
|---|---|---|
| snip | `VincentHardouin/opencode-snip` | shell-вывод; решение — токен-политика C (B0-i6, опция) |
| BRHP | `ZanzyTHEbar/brhp` | референс для C, без установки |
| Plannotator | `backnotprop/plannotator` | ревью планов владельцем — кандидат F |
| Opencode Sessions | `malhashemi/opencode-sessions` | форки сессий — идея для re-raise (C) |
| kibi / Semantic Anchors | `Looted/kibi`, `JensGrote/opencode-semantic-anchors` | референсы traceability/контрактов для E |

## 4. Критерии отбора кандидата

- совместимость с OpenCode 2.0.18 и V2-API плагинов (проверяется пробой);
- закрывает конкретный пробел T-15 (§2), не дублирует канон;
- не требует правок канона; откат тривиален;
- не создаёт «второго канона» (память/план/состояние — только источники идей);
- проект поддерживается (не retired/V1-only);
- цена (токены/латентность хуков) приемлема по замеру.

## 5. Протокол проб

1. Изоляция: temp-проект `%TEMP%\opencode\wave0b-probe` (вне репозитория);
   фиксируются версии OpenCode и плагина.
2. Установка по одному кандидату; критерий «startup без ошибок плагина»;
   при падении — фикс или красный вердикт (урок wave 0 A: три фикса API).
3. Сценарий: одна сессия с типовой задачей роли; замер (токены, время,
   артефакты); улики — `target/wave0b-*`.
4. Вердикт: 🟢/🔴 с фактом (команда → вывод); рекомендация C/D/E/F или
   «отклонить».
5. Перенос зелёного в служебную зону `.opencode/` — по подтверждению владельца,
   после `auditor`; коммит — пакетом (подтверждение один раз).
6. Откат: удаление плагина/пакета; для `.opencode` — revert коммита.

## 6. Закрытые направления (не планировать без нового решения владельца)

- оркестраторы и «свои процессы»: Swarm, CrewBee, Open Dynamic Workflows,
  Mission Control, Ralph Wiggum, Subtask2, Open Conclave, FlowDeck, ForLoop,
  Micode, GoopSpec, bmad-workflow, OpenSpec, Oh My Opencode, h2ai;
- «память-заменители»: Agent Memory, Harness Memory, oc-mnemoria и подобные —
  источник истины остаётся: `state/`, `memory/`, лента;
- retired/V1-only: `Opencode Notify`, `Opencode Worktree`, Dynamic Context
  Pruning (развитие — Sleev);
- `OpenCode Workaholic` — противоречит честной приёмке;
- worktree-плагины — только если фаза C введёт worktrees для `process/runN`.

## 7. Ссылки

- Карточка T-15: [`README.md`](README.md) (B0/B1, C–F)
- План итераций: [`wave0b-plan.md`](wave0b-plan.md)
- Протоколы и вердикты: [`wave0b-probes.md`](wave0b-probes.md)
- Отчёт: [`wave0b-report.md`](wave0b-report.md)
- Каталог-источник: <https://github.com/awesome-opencode/awesome-opencode#plugins>
