# D56: Минимальный цикл публикации — явный шаг `credo merge`

- **Статус:** accepted
- **Дата:** 2026-09-25
- **Resolves:** [Q15](../questions/Q15.md)
- **Спека:** [`SPECIFICATION.md`](../SPECIFICATION.md) §10, решение №56
  (§7 — жизненный цикл публикации)
- **Affects:** [`SPECIFICATION.md`](../SPECIFICATION.md) §7;
  [`../features/git_integration.feature`](../features/git_integration.feature),
  [`../features/deferred.feature`](../features/deferred.feature),
  [`../features/publish_rules.feature`](../features/publish_rules.feature);
  [`../../src/lib.rs`](../../src/lib.rs) (примитивы ref),
  [`../../src/mcp.rs`](../../src/mcp.rs) (`next_step` ответа `check.publish`)
- **Tasks:** [T-17](../tasks/T-17-merge-command/README.md) (см. «Сверка с
  кодом»)

## Контекст

SPEC §7 показывал «git commit+branch» и сразу `POST /evaluate`, без шага
слияния; MCP-ответ предлагал ручную `git update-ref refs/heads/main ...`.
Было неясно, кто и как переводит ветку публикации в `main`. Полный контекст —
[Q15](../questions/Q15.md).

## Решение

1. **Минимальный цикл публикации — двухшаговый:**
   1. `check.publish` создаёт ветку `publish/{name}-{version}` от `main`
      (см. [D55](D55-publish-branch-name.md));
   2. отдельный явный шаг слияния в `main` — команда
      `credo merge {name} {version}`.
2. **Видимость только после слияния:** правило становится доступным
   `evaluate`, MCP и UI только после слияния в `main`; до этого ветка —
   заготовка.
3. **Канон MVP — «человек командой».** Кнопка «Смержить» в UI и MCP-инструмент
   `check.merge` — в планах; команда `credo merge` проектируется так, чтобы её
   позже мог вызывать MCP-инструмент без изменения семантики.
4. **Механика мержа в bare-репозитории:**
   - ветка обязана быть потомком `main` (`git merge-base --is-ancestor`); если
     нет — отказ **без авто-ребейза** с предложением перепубликовать;
   - иначе — атомарное обновление `main` через
     `git update-ref refs/heads/main <sha-ветки> <ожидаемый-sha-main>`
     (**CAS-проверка** против параллельных мержей);
   - после успеха ветка `publish/{name}-{version}` удаляется; артефакт
     неизменяемо живёт в `main`.
5. **«Четыре глаза»** (публикует один, мержит другой) в MVP не требуются, но
   закладываются архитектурно: шаг мержа отделён от шага публикации и допускает
   другого исполнителя.
6. **Настоящий PR через GitHub API** остаётся отложенным
   (`deferred.feature`).

## Следствия

- Публикация и доставка разделены: `check.publish` только создаёт ветку;
  отдельный шаг (команда/будущий `check.merge`) переводит её в `main`.
- Появляется CLI-команда `credo merge {name} {version}`; позже — кнопка
  «Смержить» в UI и MCP-инструмент с той же семантикой.
- Слияние защищено ancestor-проверкой (без авто-ребейза) и CAS; после успеха
  ветка удаляется, история не теряется.
- `next_step` ответа `check.publish` должен вести на `credo merge` (а не на
  ручной un-CAS `git update-ref`).
- Расхождение в периметре MVP (шага слияния в коде нет) покрыто
  [T-17](../tasks/T-17-merge-command/README.md).

## Сверка с кодом

Вердикт: 🟡 **расхождение** — шаг публикации (создание ветки) реализован, шаг
слияния в `main` (⬜) отсутствует: нет команды `credo merge`, ancestor-проверки,
CAS-обновления `main` и удаления ветки.

Что проверено (чтением кода и фич, 29.09.2026), чем подтверждено:

- **Создание ветки — ✅ (шаг 1):** [`src/lib.rs`](../../src/lib.rs)
  `publish(...)` (321–409) создаёт ветку `publish/{name}-{vstr}` (397) от `main`
  (396, 400); `main` не изменяется (`publish_rules.feature:11`).
- **Шаг слияния — ⬜ не реализовано:**
  - команды `credo merge {name} {version}` нет: поиск по `src/**` не находит
    `credo merge`/подкоманды `merge` (CLI — [`src/main.rs`](../../src/main.rs));
  - ancestor-проверки (`git merge-base --is-ancestor`) в коде нет;
  - CAS-обновления `main` нет: `update_ref` ([`src/lib.rs`](../../src/lib.rs)
    161–164) — три аргумента, без «ожидаемого» значения; атомарный
    `create_ref` (169–179) применяет zero-oid только к **созданию** ветки, не к
    обновлению `main`;
  - удаления ветки `publish/{name}-{version}` после слияния нет.
- **`next_step` — 🟡 расхождение:** [`src/mcp.rs`](../../src/mcp.rs)
  `publish(...)` (335–340) предлагает ручную команду
  `git -C {repo} update-ref refs/heads/main $(git -C {repo} rev-parse {branch})`
  — без ancestor-проверки, без CAS и без удаления ветки; канон п. 4 требует
  `credo merge` с этими проверками.
- **Фичи:** [`git_integration.feature`](../features/git_integration.feature)
  «Слияние ветки публикации в main» (42–48) ожидает `credo merge CreditAgeMin
  1.0.0`, атомарное обновление `refs/heads/main` (CAS) и удаление ветки — кода
  нет; [`deferred.feature`](../features/deferred.feature) — PR через GitHub API
  отложен (соответствует п. 6).

`cargo` не запускался (§5.3, [D50](D50-dod-by-package-scope.md)): решение сверено
чтением кода и фич; адресный прогон не требуется (правок код не порождает).

**Задача:** [T-17](../tasks/T-17-merge-command/README.md) (P2, приоритет — на
подтверждение владельца) — команда `credo merge` с ancestor-проверкой, CAS,
удалением ветки после успеха. Отложенное (кнопка «Смержить», `check.merge`, PR)
— вне MVP.

## Альтернативы

- **Сервер автоматически мержит после `check.publish`** (вариант (в)
  [Q15](../questions/Q15.md)) — лишает смысла явный шаг контроля публикации,
  ломает принцип «доставка отделена от публикации»; отклонено.
- **Notebook/UI мержит кнопкой** (вариант (б)) — UI ещё не в MVP-цикле; кнопка
  остаётся в планах; отклонено для MVP.
- **Оставить ручной `git update-ref` без CAS** — гонка при параллельных
  мержах и потеря истории; отклонено в пользу `credo merge` с CAS.
- **Авто-ребейз ветки на `main`** — искажает историю публикации; отклонено:
  при не-потомке — отказ с предложением перепубликовать.

## Ссылки

- Вопрос: [Q15](../questions/Q15.md)
- Связанные: [Q14](../questions/Q14.md) (имя ветки),
  [Q13](../questions/Q13.md) (артефакт), [Q12](../questions/Q12.md)
  (источник истины); [Q30](../questions/Q30.md) (транспорт MCP в Notebook),
  [Q32](../questions/Q32.md) (опубликованный репозиторий)
- Задача: [T-17](../tasks/T-17-merge-command/README.md)
- Краткий канон: [`SPECIFICATION.md`](../SPECIFICATION.md) §10, решение №56;
  §7
- Связи: [`TRACEABILITY.md`](../TRACEABILITY.md)
