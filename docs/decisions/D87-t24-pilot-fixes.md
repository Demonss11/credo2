# D87: Правки по итогам S-пилота T-24 — права ролей, CCSN-хвост ветки, state hygiene, модель `lead`

- **Статус:** accepted
- **Дата:** 2026-10-02
- **Resolves:** [Q84](../questions/Q84.md)
- **Спека:** — (SPEC §10 — указатель [D70](D70-spec-reduction.md); запись в
  `SPECIFICATION.md` не требуется)
- **Affects:** `.opencode/agents/lead.md`, `.opencode/agents/git.md`,
  `.opencode/agents/analyst.md`, `.opencode/agents/tester.md`,
  [`review.md`](../../.opencode/rules/review.md),
  [`git-workflow.md`](../../.opencode/rules/git-workflow.md),
  [`dispatch-loop.md`](../../.opencode/rules/dispatch-loop.md),
  [T-15](../tasks/T-15-mcp-ready-process/README.md)
- **Tasks:** [T-15](../tasks/T-15-mcp-ready-process/README.md) (фаза C; строка
  реестра `C14`)

## Контекст

S-пилот T-24 (02.10.2026) прошёл, но «чистым» не был: 4 упора лимита `lead`,
8 вызовов `analyst`, ~15 `permission.rejected`, stale state на старте,
модель `#max`. Находки — F67–F72 в
[`findings-registry.md`](../analysis/findings-registry.md). Владелец на
открытии сервисной операции 02.10.2026 (лента r9) выбрал «Сначала дешёвые
правки»: права (F69/F71), CCSN-процедура (F71), state hygiene (F67-часть),
модель `lead` (F70). Полный контекст — [Q84](../questions/Q84.md).

## Решение

1. **Права ролей — дешёвое расширение allowlist.**
   - `lead` + `git rev-parse --short HEAD`;
   - `git` + `git branch --list *`, `git ls-remote --heads origin *`;
   - `tester` + `cargo clippy *`;
   - `analyst` — **без расширения**: ветку/HEAD проверяет косвенно; заметка в
     теле роли, что `git branch`/`rev-parse` — вне прав.
2. **CCSN-процедура при блоке `git.push-delete`.** Origin-ветку удаляет
   владелец вручную; роль `git` затем выполняет `branch -d`. Шаг планируется
   заранее (гейт + ожидание подтверждения), **без re-plan-цикла** — шаблон
   «хвост closeout» (`git-workflow.md` §«Завершение задачи»). Основание —
   F71, [D47](D47-git-refinements-run5.md) (хвост closeout).
3. **State hygiene — признак stale state.** `progress = complete` при
   противоречащем плане → `lead` не разбирает состояние, первым действием
   вызывает `analyst` и передаёт признак (без разведки).
4. **Модель `lead` — `#default`.** `#max` — только особым решением владельца.
   Основание — F70: `#max` при `#default` у ролей даёт 43% стоимости прогона.

## Следствия

- Отклонённые команды в проверках сокращаются; `tester` ловит дешёвый класс
  дефектов (`clippy`) до приёмки `validator` (класс `clippy::collapsible_if`
  P1 в T-24).
- CCSN-хвост перестаёт порождать re-plan/эскалационный цикл: шаг идёт по
  «шаблону closeout», ожидание подтверждения планируется заранее.
- Stale-снимок закрытой задачи больше не тратит сегмент `lead` на разведку.
- Стоимость `lead` снижается переводом на `#default`.
- Правки процесса (`.opencode/**`) — зона сервисной сессии; продуктовый код
  `src/**` и контракты не затрагиваются.
- Пункты F67/F67-часть (цена шага `lead`), F68 (churn re-plan) и лимит 16
  (C9/C12) остаются **вне** этого решения — отдельным решением.

## Сверка с кодом

Вердикт: ⚪ **не применимо** (процесс/документы) — решение меняет канон
агентского процесса (права ролей, процедуры цикла), продуктовый код и
контракты не меняет.

Что проверено (чтением, 02.10.2026), чем подтверждено:

- **Свобода номеров:** `docs/questions/Q84.md` и `docs/decisions/D87-*`
  отсутствовали (поиск по дереву); последние занятые — Q83/D86; нумерация
  сквозная (§2 [`journal.md`](../../.opencode/rules/journal.md)).
- **Основания:** F67–F72 в
  [`findings-registry.md`](../analysis/findings-registry.md) — чтением.
- **Периметр правок:** `.opencode/agents/{lead,git,analyst,tester}.md`,
  `.opencode/rules/{review,git-workflow,dispatch-loop}.md` — зона сервисной
  сессии; правки вносятся отдельно, решение лишь канонизирует их состав.

`cargo` не запускался (§5.3, [D50](D50-dod-by-package-scope.md)): решение
документное, кода не касается; адресный прогон — `validator` на приёмке (R2).

**Задача —** [T-15](../tasks/T-15-mcp-ready-process/README.md): решение
легитимирует дешёвые правки по итогам S-пилота в рамках программы (фаза C,
строка реестра `C14`).

## Альтернативы

- **Расширить права `analyst`** (`git branch`/`rev-parse`) — отклонено:
  `analyst` проверяет ветку/HEAD косвенно, права не нужны; заметка в теле
  роли вместо allowlist.
- **Оставить CCSN-цикл** (re-plan + эскалация как в T-24) — отклонено:
  канон-«хвост closeout» дешевле и без churn (F71).
- **Не трогать модель `lead`** (оставить `#max`) — отклонено: `#max` даёт
  43% стоимости при `#default` у ролей (F70).

## Ссылки

- Вопрос: [Q84](../questions/Q84.md)
- Основания: F67–F72 в [`findings-registry.md`](../analysis/findings-registry.md);
  [D47](D47-git-refinements-run5.md) (хвост closeout);
  [D78](D78-t15-mcp-ready-program.md) (программа T-15)
- Артефакты: карточка [T-15](../tasks/T-15-mcp-ready-process/README.md)
  (фаза C, строка реестра `C14`)
- Сервисная операция 02.10.2026 — решения владельца (лента r9)
- Связи: [`TRACEABILITY.md`](../TRACEABILITY.md)
