# D78: T-15 — программа «процесс, готовый к MCP» (фазы A–G)

- **Статус:** accepted
- **Дата:** 2026-09-30
- **Resolves:** [Q74](../questions/Q74.md)
- **Спека:** —
- **Affects:** карточка [`T-15`](../tasks/T-15-mcp-ready-process/README.md), сводка
  [`tasks/README.md`](../tasks/README.md) (источник задачи),
  [`TRACEABILITY.md`](../TRACEABILITY.md) (строка [Q74](../questions/Q74.md)/D78);
  входы — записка [`mcp-ready-process.md`](../tasks/T-15-mcp-ready-process/mcp-ready-process.md),
  [D45](D45-wave0-quality-config.md), D42–D48; согласовано с
  [D77](D77-tasks-visibility-completeness.md)
- **Tasks:** [`T-15`](../tasks/T-15-mcp-ready-process/README.md) (см. «Сверка с
  кодом»)

## Контекст

[`T-15`](../tasks/T-15-mcp-ready-process/README.md) — программа «процесс, готовый к
MCP»: по критерию готовности карточки «журнальные Q/D заведены и связаны с
карточкой»; реестр и карточка помечали источник как «Q/D — `migrator`». Часть
программы исполнена: волны W0 (A) — токен-гигиена, B0 — инструментальная обвязка
пилотов, C13 — хвостовые записи пакета (F43). Правило полноты задач —
[D77](D77-tasks-visibility-completeness.md). Полный контекст — [Q74](../questions/Q74.md);
содержание фаз — записка [`mcp-ready-process.md`](../tasks/T-15-mcp-ready-process/mcp-ready-process.md)
(фазы A–G).

## Решение

1. **Программа ведётся по проектной записке**
   [`mcp-ready-process.md`](../tasks/T-15-mcp-ready-process/mcp-ready-process.md) (фазы A–G);
   содержание фаз — **ссылкой**, не дублируется (Q41).
2. **Журнальная форма — одна пара на программу** ([Q74](../questions/Q74.md)/D78);
   фазовые решения — отдельными `Dn` по мере исполнения (D42–D48 — уже принятые
   входы, **не пересматриваются**).
3. **Источник задачи** в реестре [`tasks/README.md`](../tasks/README.md) и карточке
   [`T-15`](../tasks/T-15-mcp-ready-process/README.md) — **D78 (Q74)**.
4. **Ранее исполненные волны** (W0 A, B0, C13) считаются **частичным исполнением**
   программы и не переоткрываются.
5. **Прогресс и статусы программы** — в реестре
   [`tasks/README.md`](../tasks/README.md) и карточке
   [`T-15`](../tasks/T-15-mcp-ready-process/README.md) (не в
   [`TRACEABILITY.md`](../TRACEABILITY.md)); пустые статусы фаз — по карточке.

## Следствия

- [`T-15`](../tasks/T-15-mcp-ready-process/README.md) становится видимой в
  [`TRACEABILITY.md`](../TRACEABILITY.md) (**19/19**; правило
  [D77](D77-tasks-visibility-completeness.md)).
- Старт остатка фаз C–G легитимен по правилу D77: источник задачи зафиксирован.
- Находки F15/F26/F27 (привязаны к [`T-15`](../tasks/T-15-mcp-ready-process/README.md))
  остаются в реестре находок
  [`findings-registry.md`](../analysis/findings-registry.md).

## Сверка с кодом

Вердикт: ⚪ **не применимо** (документы/процесс) — процессная программа, продуктовый код не меняется. Факты по §5.3 (чтением, 30.09.2026):

- карточка [`T-15`](../tasks/T-15-mcp-ready-process/README.md) — фазы **C–G ⬜**, W0
  A/B0/C13 ✅ (принято 27–28.09.2026);
- записка [`mcp-ready-process.md`](../tasks/T-15-mcp-ready-process/mcp-ready-process.md) существует
  (разделы §5–§7, §10 «План работ (wave 0, фазы A–F)», приложение A);
- отчёты/квитанции W0 и `T-15-c13` — исполнено 27–28.09.2026;
- `cargo` не запускался (§5.3, [D50](D50-dod-by-package-scope.md)): кода прототипа
  запись не меняет.

**Задача —** [`T-15`](../tasks/T-15-mcp-ready-process/README.md): запись
легитимирует её источник; прогресс по фазам ведётся в карточке.

## Альтернативы

- **(б) Пара на каждую фазу сейчас** — отклонено: избыточно, большинство фаз ещё ⬜;
  прецедент — фазовые решения D42–D48 принимались по ходу.
- **(в) Не оформлять** — отклонено: нарушает правило [D77](D77-tasks-visibility-completeness.md)
  и критерий готовности карточки [`T-15`](../tasks/T-15-mcp-ready-process/README.md).

## Ссылки

- Вопрос: [Q74](../questions/Q74.md)
- Связанные: [Q73](../questions/Q73.md)/[D77](D77-tasks-visibility-completeness.md)
  (полнота задач), [D45](D45-wave0-quality-config.md) (W0 A),
  D42–D48 (входы программы)
- Артефакты: записка [`mcp-ready-process.md`](../tasks/T-15-mcp-ready-process/mcp-ready-process.md);
  карточка [`T-15`](../tasks/T-15-mcp-ready-process/README.md);
  реестр [`tasks/README.md`](../tasks/README.md);
  [`TRACEABILITY.md`](../TRACEABILITY.md)
