# D58: Каталоги данных workspace — `.credo/` и `.dar-notebook/`

- **Статус:** accepted
- **Дата:** 2026-09-26
- **Resolves:** [Q19](../questions/Q19.md)
- **Спека:** [`SPECIFICATION.md`](../SPECIFICATION.md) §10, решение №58
- **Affects:** [`SPECIFICATION.md`](../SPECIFICATION.md) §4.3 (раскладка
  workspace), §10;
  [`../features/notebook_ui.feature`](../features/notebook_ui.feature) (шаг
  `.gitignore`; правки — зона `docs-writer`);
  [`../../src/lib.rs`](../../src/lib.rs) (путь `.credo/`)
- **Tasks:** — (см. «Сверка с кодом»)

## Контекст

SPEC §4.2 упоминал `.credo/sandbox.json`/`.credo/published-repo/`, §4.3 —
`.dar-notebook/`; в фичах фигурировал только `.dar-notebook/`. Владельцы и
раскладка двух служебных каталогов не были описаны единообразно. Полный
контекст — [Q19](../questions/Q19.md).

## Решение

Оба служебных каталога живут **в корне workspace** (там же, где `rules/`),
владельцы разные:

| Каталог | Владелец | Содержимое | В git? |
|---|---|---|---|
| `rules/*.dar` | пользователь / workspace | исходники правил — источник истины (Q12) | да |
| `.credo/` | `credo-server` | `sandbox.json` (черновики, Q12); `published-repo/` (bare-git публикаций, Q15/Q32) | нет (`.gitignore`) |
| `.dar-notebook/` | DAR Notebook | `layout.json`; `results-cache.json` («быстрые прогоны», Q34); `agent-history.json` (Q31) | нет (`.gitignore`) |

- **Почему `.credo/` в `.gitignore`:** каталог лежит внутри
  workspace-репозитория, а `sandbox.json` и вложенный bare-repo — служебные
  данные; без игнорирования загрязняют git-статус пользователя. `.gitignore`
  workspace создаётся с записями `.credo/` и `.dar-notebook/`
  (`notebook_ui.feature`).
- **Домены не смешиваются:** `.credo/` — серверный (черновики и публикации),
  `.dar-notebook/` — клиентский (UI-метаданные, кэш тестов, история чата);
  общий только корень workspace.
- **Диаграмма §4.3 исправлена:** `.dar-notebook/` больше не показан снаружи
  workspace (это противоречило `.gitignore`); оба каталога — внутри
  `consumer-credit/`.
- **Имя `.credo/` оставлено прототипным** (решение 2026-09-26); переименование
  в `.dar/` вместе с эволюцией в `credo-server`/`dar-core` (Q1) — вопрос
  пост-MVP, отдельным решением.

## Следствия

- SPEC §4.3 получает единую таблицу «каталог → владелец → содержимое → в git?»;
  диаграмма workspace и `.gitignore` согласованы.
- Разграничены серверный и клиентский домены; `.credo/` остаётся внутренней
  деталью сервера (в т.ч. `published-repo`).
- Изменения — документные; фичи и счётчики по числу сценариев не меняются
  (`notebook_ui.feature` уточняет только шаг `.gitignore`).

## Сверка с кодом

Вердикт: ⚪ **не применимо** (решение о раскладке документов/каталогов) —
проверены согласованность SPEC и фичи и след в коде прототипа.

Что проверено (чтением, 29.09.2026), чем подтверждено:

- **`.credo/` — серверный домен — ✅ (в прототипе):**
  [`src/lib.rs`](../../src/lib.rs) `AppState`/пути: `.credo/sandbox.json`
  (784–787) и `.credo/published-repo` (787); тесты сидируют
  `.credo/sandbox.json` (`tests/mcp_draft.rs:211-217`) и
  `.credo/published-repo` (`tests/mcp_errors.rs:33`). Перезапуск состояние не
  теряет (персистентность).
- **SPEC §4.3 — ✅ (таблица внесена):** [`SPECIFICATION.md`](../SPECIFICATION.md)
  §4.3: диаграмма workspace (348–360) и таблица «каталог → владелец →
  содержимое → в git?» (368–372), пометка «Q19 (решено 2026-09-26)» (362–366),
  `.gitignore` (358, 371–372).
- **`notebook_ui.feature` — ✅:** сценарий «Создание нового workspace» требует
  `.gitignore` с записями `.credo/` и `.dar-notebook/`
  ([`../features/notebook_ui.feature`](../features/notebook_ui.feature) 40).
- **`.dar-notebook/` — ⏸ контур Notebook (v0.2):** в этом репозитории
  (сервер `credo2`) каталога нет — зона UI/Notebook; решение носит
  документный характер (Q34/Q31).

`cargo` не запускался (§5.3, [D50](D50-dod-by-package-scope.md)): решение
документное, адресный прогон не требуется.

**Задач не требуется:** раскладка каталогов не порождает кода в серверном
периметре MVP; `.credo/` уже используется, `.dar-notebook/` — контур Notebook
v0.2.

## Альтернативы

- **Оставить упоминания разрозненными** — отклонено: неясны владельцы и
  попадание в git; риск загрязнения workspace-репозитория.
- **Держать оба каталога вне workspace** (рядом с ним) — отклонено: они
  относятся к конкретному workspace и должны быть под его `.gitignore`.
- **Переименовать `.credo/` в `.dar/` сейчас** — отклонено: имя прототипное,
  переименование — пост-MVP с эволюцией в `credo-server`/`dar-core` (Q1).
- **Хранить `sandbox.json` в git** — отклонено: служебные данные загрязняют
  историю; черновик — производное состояние (Q12).

## Ссылки

- Вопрос: [Q19](../questions/Q19.md)
- Связанные: [Q12](../questions/Q12.md) (песочница — черновики),
  [Q17](../questions/Q17.md) (published-repo), [Q1](../questions/Q1.md)
  (эволюция); [Q31](../questions/Q31.md); [Q34](../questions/Q34.md); Q35 (ожидает переноса)
- Задачи: — (см. «Сверка с кодом»)
- Краткий канон: [`SPECIFICATION.md`](../SPECIFICATION.md) §10, решение №58;
  §4.3
- Связи: [`TRACEABILITY.md`](../TRACEABILITY.md)
