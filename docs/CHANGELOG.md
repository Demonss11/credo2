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
