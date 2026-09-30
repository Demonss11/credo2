# D49: Право `git branch --contains` для `validator`

- **Статус:** accepted
- **Дата:** 2026-09-28
- **Resolves:** [Q54](../questions/Q54.md)
- **Спека:** [`SPECIFICATION.md`](../SPECIFICATION.md) §10, решение №49
- **Affects:** [`validator.md`](../../.opencode/agents/validator.md) (права);
  [`review.md`](../../.opencode/rules/review.md) §«Доступные команды»;
  [`findings-registry.md`](../analysis/findings-registry.md) (F44)
- **Tasks:** — (см. «Сверка с кодом»)

## Контекст

При приёмке service-t11-closeout (28.09.2026) проверялось утверждение
«коммиты `0a5832f`/`5019c45` влиты в `develop`».
Прямая команда `git branch --contains <коммит>` была отклонена движком прав —
у роли `validator` нет паттернов `git branch`; проверка заменена косвенной
`git log develop -50`. Полный контекст — [Q54](../questions/Q54.md).

Смежный отказ `git diff -- .opencode/...` — не пробел прав, а известный квик
dot-пути после `--` ([`rights-matching-2026-09-27.md`](../analysis/rights-matching-2026-09-27.md)
§5; `review.md` §«Доступные команды»); каноническая форма `./…` и правило
«сузь и повтори» применяются штатно.

## Решение

1. **Добавить в allowlist `validator` ровно одну строку:**
   `- { action: shell, resource: "git branch --contains *", effect: allow }`
   (read-only; иных ролей не касается).
2. **Синхронизировать** строку списка роли в
   [`review.md`](../../.opencode/rules/review.md) §«Доступные команды»
   (`git branch --contains`).

## Следствия

- Вхождение коммита в историю ветки подтверждается машинно (`git branch
  --contains <коммит>`), а не косвенно через `git log`.
- Автопроверка `auditor` («список команд ролей ↔ фронтматтеры») учитывает
  новую строку; ручного расхождения не возникает.
- Приёмка (по запросу) проверяется смоуком `git branch --contains <коммит>` на
  ближайшем прогоне; сама приёмка права — только в периметре этой строки.
- **Обновление 30.09.2026 — право `git rev-parse *` (дополняет п.1):** в
  allowlist `validator` добавлена ещё одна read-only строка — `git rev-parse *`:
  точная сверка базы приёмки (`git rev-parse develop origin/develop HEAD`).
  Повод — повторяющиеся отказы движка при проверке базы
  (`W8-config` 28.09.2026; `service-docs-lifecycle` №1 30.09.2026 — обход
  `git log -1`); право read-only, иных ролей не касается. Синхронизация:
  [`review.md`](../../.opencode/rules/review.md) §«Доступные команды», память
  `validator` (ловушка снята).
- **Обновление 30.09.2026 — права `git ls-files *` и `git check-ignore *`
  (дополняет п.1):** в allowlist `validator` добавлены read-only строки
  `git ls-files *`, `git check-ignore *`; в allowlist `auditor` —
  `git check-ignore *` (класс «границы пакета / gitignore»). Поводы — отказы
  движка: `W8-config` 28.09.2026 (`ls-files`, обход rg+log);
  `service-pm-numbering` 30.09.2026 (`check-ignore`, «на усмотрение владельца»
  → решено владельцем). Синхронизация:
  [`review.md`](../../.opencode/rules/review.md) §«Доступные команды».

## Сверка с кодом

Вердикт: ⚪ **не применимо** — решение процессно-правовое, продуктовое
поведение не меняет (образец — [D44](../decisions/D44-run5-refinements.md)).
Что проверено (28.09.2026): до правки паттерна нет —
`rg "git branch" .opencode/agents/validator.md` → пусто (allowlist содержит
только `git status|diff|log|show|grep`); после правки — резолв прав движком
(`opencode debug agents`) и смоук `git branch --contains <коммит>` на приёмке.
`cargo` не запускался. Задач не требуется: канон вносит сервисная сессия
владельца; сверка с кодом применима к правам, а не к продуктовому коду.

## Альтернативы

- **Обходиться `git log`** (вариант (а) [Q54](../questions/Q54.md)) — прямой
  вопрос «входит ли коммит в ветку» не проверяется. Отклонено.
- **Расширить весь read-only набор `git branch`** (вариант (в)) — массовое
  расширение allow-листа без подтверждённой потребности; принцип D44 —
  одна доказанная команда. Отклонено.

## Ссылки

- Вопрос: [Q54](../questions/Q54.md)
- Краткий канон: [`SPECIFICATION.md`](../SPECIFICATION.md) §10, решение №49
- Основание: [`rights-matching-2026-09-27.md`](../analysis/rights-matching-2026-09-27.md)
  §5; лента `service-t11-closeout.md` (`.opencode/mail/**`)
- Связи: [`TRACEABILITY.md`](../TRACEABILITY.md)
