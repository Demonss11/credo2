# D30: Единый механизм исполнения из редактора — MCP `check.test` на черновике

- **Статус:** accepted
- **Дата:** 2026-09-26
- **Resolves:** [Q33](../questions/Q33.md)
- **Спека:** [`SPECIFICATION.md`](../SPECIFICATION.md) §10, решение №30
- **Affects:** [`SPECIFICATION.md`](../SPECIFICATION.md) §2.2, §4.1, §4.2, §7 и §10;
  [`features/inline_execution.feature`](../features/inline_execution.feature),
  [`features/agent_minimal.feature`](../features/agent_minimal.feature),
  [`features/draft.feature`](../features/draft.feature),
  [`features/mcp_tools.feature`](../features/mcp_tools.feature) (целевое
  состояние; шапки/комментарии `# Dn (Qn) …` — зона `docs-writer`);
  [`features/README.md`](../features/README.md) (счётчики, заметки)
- **Tasks:** [T-08](../tasks/T-08-materialize-source-file/README.md) (открыта),
  [T-09](../tasks/T-09-check-run/README.md) (открыта) — см. «Сверка с кодом»

## Контекст

`inline_execution.feature` показывал кнопку «Выполнить» прямо под правилом;
`editor.feature` — диагностику из LSP; `agent_minimal.feature` — `check.test`
для черновика. LSP правила не исполняет, а источник данных для inline-запуска не
был определён: локальное ядро Tauri (`dar-core`) или MCP. Полный контекст —
[Q33](../questions/Q33.md).

## Решение

1. **Единый механизм исполнения:** кнопка «Выполнить» в редакторе и агент через
   чат — оба идут через MCP `check.test`. Локального исполнения через Tauri
   `dar-core` **нет**.
2. **Кнопка «Выполнить» активна только при наличии черновика.** Если черновик не
   создан/не сохранён — кнопка неактивна, пользователю предлагается сохранить
   правило (создать черновик).
3. **Поток исполнения:** (а) пользователь сохраняет правило → создаётся/
   обновляется черновик; (б) кнопка «Выполнить» активируется; (в) клик →
   `check.test` на черновике → `Explanation`.
4. **Источники данных для исполнения:** черновик — обязателен для `check.test`;
   опубликованная версия — `check.run` (не через черновик).
5. **Семантика «сохранить» (draft-first):** «Создать черновик» →
   `check.create(source = буфер)`, файл не создаётся; кнопка «Выполнить»
   активируется после создания черновика; файл `rules/{name}.dar` материализуется
   при публикации (Q12, [D54](D54-source-of-truth-flow.md)). Противоречия с Q12
   нет: [Q33](../questions/Q33.md) требует наличия черновика, а не его
   происхождения из файла.

## Следствия

- `SPECIFICATION.md` §10 (решение №30) — единый механизм, draft-first, граница
  «опубликованная версия — `check.run`»; §2.2/§4.1/§4.2/§7 (канон исполнения из
  редактора — `check.create` из буфера → `check.test` на черновике).
- Требования (целевое состояние; шапки — зона `docs-writer`):
  [`features/inline_execution.feature`](../features/inline_execution.feature)
  (кнопка неактивна без черновика, draft-first, исполнение через `check.test`),
  [`features/agent_minimal.feature`](../features/agent_minimal.feature) (агент —
  через `check.test` на черновике),
  [`features/draft.feature`](../features/draft.feature) (черновик из буфера без
  файла), [`features/mcp_tools.feature`](../features/mcp_tools.feature)
  (`check.run`), [`features/README.md`](../features/README.md) (счётчики,
  заметки).
- **`check.run`** (исполнение опубликованной версии, п. 4) в текущем `credo2`
  отсутствует — требование `mcp_tools.feature`, задача
  [T-09](../tasks/T-09-check-run/README.md) (вместе с семантикой
  `version_deprecated`/`version_not_found`).
- **Материализация `rules/{name}.dar`** при публикации (п. 5, Q12) в `credo2` не
  реализована — задача
  [T-08](../tasks/T-08-materialize-source-file/README.md) (источник Q12/Q33).
- Локальное исполнение отсутствует как класс: LSP не исполняет правила, `dar-core`
  — только через LSP; второго движка нет (согласуется с §10 решение №6, Q39).
- Инлайн-результаты в Notebook — [Q31](../questions/Q31.md)
  ([D12](D12-agent-chat-panel.md)).

## Сверка с кодом

Вердикт: 🟡 **расхождение** — реализованная часть (`check.test` на черновике,
draft-first, отсутствие локального исполнения) соответствует решению; не
реализованы `check.run` и материализация `rules/{name}.dar` при публикации —
покрыты задачами T-09 и T-08 (в периметре требований; кода прототипа решение в
этих частях не имеет).

Что проверено (чтением кода и тестов, 29.09.2026), чем подтверждено:

- **`check.test` реализован.** [`src/mcp.rs`](../../src/mcp.rs) — диспетчер
  `:143`, `async fn test(...)` `:214–262`: черновик из состояния (`:226`),
  `evaluate_rule` (`:235`), объяснение на верхнем уровне ответа (`rule_name`,
  `condition`, `actual_value`, `matched`, `decision`, `reason` — `:250–261`),
  метки `last_test_checksum`/`tested_at` (`:238–248`). Движок —
  [`src/core.rs`](../../src/core.rs) (`Explanation`, `evaluate_rule` `:136`,
  `:153`).
- **Тесты `check.test`.** [`tests/mcp_draft.rs`](../../tests/mcp_draft.rs) —
  успешный прогон и метки (`:169–200`), границы входа (`input` обязателен,
  `:350–357`), `.dar`-файл не создаётся (draft-first, `:367`); юнит-тесты в
  [`src/mcp.rs`](../../src/mcp.rs) `:817–870` (`stale` не блокирует `check.test`,
  `test_valid` после смены текста).
- **Draft-first / нет локального исполнения.** [`src/mcp.rs`](../../src/mcp.rs)
  `stale_false_without_dar_file_q29_inv3` (`:771–781`) — `rules/{name}.dar` при
  создании черновика не создаётся; `check.test` работает по черновику
  (пп. 2–3, 5 решения).
- **`check.run` отсутствует.** Диспетчер [`src/mcp.rs`](../../src/mcp.rs)
  `:143–148` инструмент не обрабатывает; `tool_specs()` (`:470–530`) объявляет 9
  инструментов, `check.run` среди них нет. `ErrorCode::VersionDeprecated`
  зарезервирован под `check.run` и в текущих MCP-путях не возникает
  (`src/mcp.rs:43–46`, `#[allow(dead_code)]`); REST `version_deprecated`
  ([`src/rest.rs`](../../src/rest.rs) `:197`) относится к deprecated-версии при
  `evaluate`, не к `check.run`. → [T-09](../tasks/T-09-check-run/README.md).
- **Материализация `.dar` при публикации отсутствует.**
  [`src/mcp.rs`](../../src/mcp.rs) `publish(...)` (`:282–341`) вызывает
  `crate::publish(&repo, &rule, &contract, ...)` и файл `.dar` не пишет; тест
  [`src/mcp.rs`](../../src/mcp.rs) `:771–781` подтверждает «не создаётся».
  → [T-08](../tasks/T-08-materialize-source-file/README.md).
- **Карта зон** — [`features/README.md`](../features/README.md) «Соответствие
  коду»: MCP-инструменты → [`../../src/mcp.rs`](../../src/mcp.rs), домен/DSL →
  [`../../src/core.rs`](../../src/core.rs).

`cargo` не запускался (§5.3, [D50](D50-dod-by-package-scope.md)): решение не
меняет код; адресный прогон (если понадобится) — за `validator`.

**Задачи:** [T-08](../tasks/T-08-materialize-source-file/README.md) (⬜ P2,
источник Q12/Q33 — материализация `rules/{name}.dar`) и
[T-09](../tasks/T-09-check-run/README.md) (⬜ P3, источник Q33 — `check.run` и
семантика `version_deprecated`). Механизм `check.test`/draft-first уже
соответствует — правок не требует.

## Альтернативы

- **Локальное исполнение буфера через Tauri `dar-core`** (архивная
  рекомендация) — отклонено: два движка исполнения дают расхождение «редактор vs
  агент» и дублируют парсер/ядро; LSP не исполняет правила.
- **Обязательный файл на диске для исполнения** — отклонено: противоречит
  draft-first (Q12) и мешает агенту создавать черновики с нуля.
- **Кнопка «Выполнить» без черновика** (прямой прогон буфера) — отклонено:
  результат должен быть воспроизводим и совпадать с прогоном через MCP;
  черновик — обязательное условие.

## Ссылки

- Вопрос: [Q33](../questions/Q33.md)
- Связанные: [Q31](../questions/Q31.md)
  ([D12](D12-agent-chat-panel.md) — чат и инлайн-результаты); [Q12](../questions/Q12.md)
  ([D54](D54-source-of-truth-flow.md) — источник истины, draft-first);
  [Q32](../questions/Q32.md) (три уровня источника правды)
- Задачи: [T-08](../tasks/T-08-materialize-source-file/README.md),
  [T-09](../tasks/T-09-check-run/README.md)
- Краткий канон: [`SPECIFICATION.md`](../SPECIFICATION.md) §10, решение №30;
  §2.2, §4.1, §4.2, §7
- Связи: [`TRACEABILITY.md`](../TRACEABILITY.md)
