# CHANGELOG — credo2

## 0.1.0 (в разработке)

> Прототип до релиза API v0.1: перечисленные изменения — намеренные,
> потребителей в проде нет.

### Документация

- **Документация перенесена в `docs/`**: `SPECIFICATION.md`, `GRAMMAR.md`,
  `CHANGELOG.md`, `OPEN_QUESTIONS.md`, `features/`, `tasks/`; относительные
  ссылки и `tests/features_inventory.rs` обновлены.
- **Введён журнал вопросов и решений**: `docs/questions/Qn.md`,
  `docs/decisions/Dn-<слаг>.md` (Dn = номер решения в SPEC §10), сводка связей —
  `docs/TRACEABILITY.md`, процесс — `docs/BRIEF.md`. Пилот миграции — Q1 → D15.
- **Процесс переноса дополнен сверкой с кодом**: у каждого решения — вердикт
  «Сверка с кодом» и решение о задаче (`docs/BRIEF.md` §5.3); стратегия
  «миграция идёт блоками параллельно кодингу» — §8.

### Процесс

- **Рабочая группа агентов** (`.opencode/agents/`): `lead` (оркестратор,
  `default_agent`) + `migrator`, `docs-writer`, `coder`, `tester`, `validator`,
  `git`; маршруты и права ролей — `AGENTS.md` §Рабочая группа агентов.
- **Ревизия рабочей группы** (2026-09-26): `migrator` ведёт журнал целиком
  (перенос + новые Q/D); маршрут кода замкнут через `docs-writer` (закрытие
  статусов задачи); исправлен порядок прав `docs-writer` (журнал защищён);
  `lead` запускает только роли команды; merge веток публикаций CREDO — за
  человеком; удалён неиспользуемый комплект `.opencode/rules/doc-*.md`.
- **Усиление ролей по образцу проекта ex1** (2026-09-26): правила
  `.opencode/rules/workspace.md` (гигиена поиска и чтения) и
  `.opencode/rules/review.md` (методика ревью: что искать, блокеры, версия
  артефакта); в промпты добавлены методика приёмки, работа с глоссарием
  (`SPECIFICATION.md` §11), «Полезные вызовы» у `lead` и чтение больших
  файлов по карте заголовков.
- **Хранимые отчёты приёмки и роль `researcher`** (2026-09-26): `validator`
  сохраняет отчёты в `docs/reviews/` (единственная его зона записи; улики,
  не канон); новая роль `researcher` — внешние обзоры в `docs/research/`
  (`webfetch`/`websearch` разрешены только ей); карта — `docs/README.md`.
- **Роль `rust-expert` и разгрузка кода** (2026-09-26): код проходит
  идиоматическую вычитку у `rust-expert` (skill `rust-skills`, бюджет чтения,
  правки без изменения поведения); у `coder` и `tester` skill `rust-skills`
  отключён; маршрут кода — `coder` → `rust-expert` → `tester` → `validator` →
  `docs-writer` → `git`.
- **Primary-агент `auditor`** (2026-09-26): аудит системы агентов и
  мета-документации (дубли, противоречия, пробелы, ясность, экономия токенов);
  правит `AGENTS.md`, `.opencode/agents/**`, `.opencode/rules/**`; вне маршрутов
  команды, запускается владельцем отдельной сессией.
- **Корень проекта** — `prototypes/credo2` (`AGENTS.md`, `opencode.json`);
  MCP `credo` работает с рабочей директорией репозитория, данные — `.credo/` (вне git).
- **Цикл агентов v2** (T-11, 2026-09-26): Agile-петля с размерными маршрутами
  S/M/L (`coder → rust-expert → tester → validator → docs-writer → git`);
  `validator` — единственная роль с `cargo test` (полный DoD-прогон), роли,
  работающие с кодом, ограничиваются компиляцией (`fmt`/`check`/`clippy`).
  Память роли — `.opencode/memory/<роль>.md`, лента задачи —
  `.opencode/mail/T-XX.md` (рабочие данные в git, не канон, Q41); чекпойнт
  до/после тяжёлых операций, лимиты `steps` и продолжение по `sessionID`;
  git — пакетное подтверждение после приёмки и идемпотентность при обрыве;
  канон агентов правит `auditor` (`mode: all`), до коммита — аудит
  «инструкция ↔ права». Целевые сценарии — `features/agents-*.feature`
  (6 файлов / 27 сценариев; статусы — [`features/README.md`](features/README.md)),
  задача — [T-11](tasks/T-11-agent-cycle/README.md), решение —
  [D38](decisions/D38-agent-cycle.md) (Q43).

### Добавлено

- **Черновик хранит исходник (T-01).** `Draft` (`src/lib.rs`) хранит `source`
  и `source_hash` (sha256 текста), а не только разобранное правило;
  `check.get_draft`/`check.list_drafts` отдают рендеренные строки, внутренний
  `Rule` не публикуется. Введены вычисляемые `stale` и `test_valid` (Q29,
  инварианты 2–5 §4.5); `check.test` фиксирует `last_test_checksum`/`tested_at`.
  Карточка задачи — [`tasks/T-01-draft-source-hash/README.md`](tasks/T-01-draft-source-hash/README.md),
  отчёт приёмки — [`reviews/T-01-2026-09-26.md`](reviews/T-01-2026-09-26.md);
  автотесты — [`../tests/mcp_draft.rs`](../tests/mcp_draft.rs) (реальный stdio-MCP).

### Исправлено

- **Кириллические имена проверок не попадали в кэш.** `git ls-tree`
  экранировал не-ASCII пути (`core.quotepath`), и `list_from_ref` их
  пропускал: после merge проверка с русским именем не появлялась в
  манифесте/REST. Дерево читается с `-z` (регрессионный тест
  `cyrillic_check_name_is_listed`).

### Ломающие изменения

- **Ответ `check.create` (T-01).** Формат ответа изменён на `{status: "ok", name}`
  (канон Q28, `SPECIFICATION.md` §4.5) — ломающее для MCP-клиентов, читавших
  прежний ответ. Остаток T-03 (обязательный `name`, сверка с заголовком) не
  входит в это изменение.
- **Тело ошибок REST (Q23).** Все ответы с ненулевым статусом переведены на
  единый конверт `{"error": {"code", "message"}}` (было `{"error": "<строка>"}`).
  Коды: `check_not_found`, `version_not_found`, `version_deprecated`,
  `activation_unavailable`, `evaluation_failed`, `unauthorized`; `message` —
  русский текст Q11.
- **Семантика версий (Q11/Q21).** Исполнение deprecated-версии —
  `410 Gone` (`version_deprecated`); active-эндпоинт при отсутствии
  active-версии — `409 Conflict` (`activation_unavailable`); 404-текст
  проверки — `проверка не найдена: {name}` (без кавычек).
- **`GET /checks` (Q21).** В ответ добавлен `schema_version`; `active` —
  максимальная supported-версия, `""` если все версии deprecated;
  `supported`/`deprecated` — по убыванию.
