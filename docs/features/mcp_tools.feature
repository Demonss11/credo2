# language: ru
# Q29 (решено 2026-09-26): контракты MCP-инструментов — SPECIFICATION.md §4.5;
# ключи — snake_case латиницей, текстовые значения — русские; ошибки —
# {"error": {"code", "message"}} со стабильными кодами.
# Q33 (решено 2026-09-26): check.run — исполнение опубликованной версии
# (не через черновик); в credo2 пока не реализован.
Функция: MCP инструменты
  Как AI-агент
  Я хочу управлять версиями через MCP
  Чтобы публиковать и отзывать проверки без ручных git-операций

  Сценарий: check.publish возвращает нормализованную версию
    Дано черновик "CreditAgeMin" успешно протестирован
    Когда вызывается "check.publish" с name="CreditAgeMin", version="v1.0.0"
    Тогда ответ содержит "status": "published"
    И ответ содержит "version": "1.0.0"
    И ответ содержит "branch": "publish/CreditAgeMin-1.0.0"
    И ответ содержит "path": "checks/CreditAgeMin/1/0/0/"

  Сценарий: check.publish с невалидной версией отклоняется
    Когда вызывается "check.publish" с version="v.0.0.1"
    Тогда ответ содержит ошибку с кодом "validation_failed"
    И сообщение содержит "невалидная версия"

  Сценарий: MCP перечисляет инструменты MVP
    Когда MCP перечисляет tools
    Тогда в списке есть "check.create", "check.list_drafts" и "check.get_draft"
    И в списке есть "check.test" и "check.delete_draft"
    И в списке есть "check.publish", "check.deprecate" и "check.list_published"
    И в списке есть "check.rebuild_manifest" и "check.run"

  Сценарий: check.deprecate с пустым reason отклоняется
    Дано в main есть "CreditAgeMin" версии "1.0.0"
    Когда вызывается "check.deprecate" с reason=""
    Тогда ответ содержит ошибку с кодом "validation_failed"
    И версия "1.0.0" не помечается deprecated

  Сценарий: check.list_published возвращает контракт ответа
    Дано в main опубликована "CreditAgeMin" версии "1.0.1"
    И "CreditAgeMin" версии "1.0.0" помечена deprecated
    Когда вызывается "check.list_published"
    Тогда ответ содержит "status": "ok", "count" и "service_hash"
    И элемент списка содержит "name", "version", "path", "status" и "active"
    И элемент списка содержит "published_at" и "published_by"
    И для "1.0.0" указаны "deprecated_at" и "deprecation_reason"

  Сценарий: check.list_published сортирует версии по semver
    Дано в main опубликованы "CreditAgeMin" версии "1.0.0", "1.0.10" и "1.0.2"
    Когда вызывается "check.list_published"
    Тогда имена идут по возрастанию
    И версии идут по убыванию по семантике semver: "1.0.10", "1.0.2", "1.0.0"

  Сценарий: check.rebuild_manifest возвращает count
    Дано в main опубликованы две проверки
    Когда вызывается "check.rebuild_manifest"
    Тогда ответ содержит "status": "ok", "service_hash" и "written": true
    И ответ содержит "count", согласованный с числом проверок в "GET /checks"

  Сценарий: check.run исполняет опубликованную версию
    Дано в main опубликована "CreditAgeMin" версии "1.0.0"
    Когда вызывается "check.run" с name="CreditAgeMin", version="1.0.0"
    И поле "input" содержит "Клиент.Возраст": 19
    Тогда ответ содержит "status": "ok", "name" и "version"
    И ответ содержит поля объяснения (Q42): "rule_name", "condition",
       "actual_value", "matched", "decision" и "reason"
    И черновик не используется

  Сценарий: check.run отклоняет deprecated-версию
    Дано "CreditAgeMin" версии "1.0.0" помечена deprecated
    Когда вызывается "check.run" с name="CreditAgeMin", version="1.0.0"
    Тогда ответ содержит ошибку с кодом "version_deprecated"

  Сценарий: check.run требует опубликованную версию
    Дано "CreditAgeMin" существует только черновиком
    Когда вызывается "check.run" с name="CreditAgeMin", version="1.0.0"
    Тогда ответ содержит ошибку с кодом "version_not_found"
