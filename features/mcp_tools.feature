# language: ru
Функция: MCP инструменты
  Как AI-агент
  Я хочу управлять версиями через MCP
  Чтобы публиковать и отзывать проверки без ручных git-операций

  Сценарий: check.publish возвращает нормализованную версию
    Когда вызывается "check.publish" с version="v1.0.0"
    Тогда ответ содержит "version": "1.0.0"
    И ответ содержит "branch": "publish/CreditAgeMin-1.0.0"
    И ответ содержит "path": "checks/CreditAgeMin/1.0.0"

  Сценарий: check.publish с невалидной версией отклоняется
    Когда вызывается "check.publish" с version="v.0.0.1"
    Тогда ответ содержит "error"
    И сообщение содержит "невалидная версия"

  Сценарий: check.deprecate доступен
    Когда MCP перечисляет tools
    Тогда в списке есть "check.deprecate"
    И в списке есть "check.rebuild_manifest"

  Сценарий: check.list_published показывает deprecated
    Дано "CreditAgeMin" версии "1.0.0" помечена deprecated
    Когда вызывается "check.list_published"
    Тогда в ответе для "CreditAgeMin" указано "deprecated_at"
