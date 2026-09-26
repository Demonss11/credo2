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
- **Корень проекта** — `prototypes/credo2` (`AGENTS.md`, `opencode.json`);
  MCP `credo` работает с рабочей директорией репозитория, данные — `.credo/` (вне git).

### Исправлено

- **Кириллические имена проверок не попадали в кэш.** `git ls-tree`
  экранировал не-ASCII пути (`core.quotepath`), и `list_from_ref` их
  пропускал: после merge проверка с русским именем не появлялась в
  манифесте/REST. Дерево читается с `-z` (регрессионный тест
  `cyrillic_check_name_is_listed`).

### Ломающие изменения

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
