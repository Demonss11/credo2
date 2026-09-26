# language: ru
# Q29 (решено 2026-09-26): контракт ответа — SPECIFICATION.md §4.5;
# reason обязателен и непуст; повторная депрекация — deprecation_conflict.
Функция: Deprecation версии
  Как владелец проверки
  Я хочу отозвать версию без удаления данных
  Чтобы клиенты получили сигнал, но история сохранилась

  Сценарий: MCP-инструмент помечает версию deprecated
    Дано в main есть "CreditAgeMin" версии "1.0.0"
    Когда MCP вызывает "check.deprecate" с name="CreditAgeMin", version="1.0.0",
         reason="заменена на 1.0.1"
    Тогда ответ содержит "status": "deprecated"
    И ответ содержит "name", "version", "reason", "deprecated_at" и "service_hash"
    И в main появляется коммит "deprecate CreditAgeMin@1.0.0"
    И в "meta.json" поле "deprecated_at" непустое
    И поле "deprecation_reason" равно "заменена на 1.0.1"

  Сценарий: check.deprecate с пустым reason отклоняется
    Дано в main есть "CreditAgeMin" версии "1.0.0"
    Когда MCP вызывает "check.deprecate" с name="CreditAgeMin", version="1.0.0", reason=""
    Тогда ответ содержит ошибку с кодом "validation_failed"
    И версия "1.0.0" не помечается deprecated

  Сценарий: Повторная депрекация отклоняется
    Дано "CreditAgeMin" версии "1.0.0" помечена deprecated
    Когда MCP вызывает "check.deprecate" с name="CreditAgeMin", version="1.0.0",
         reason="заменена на 1.0.2"
    Тогда ответ содержит ошибку с кодом "deprecation_conflict"
    И "deprecated_at" и "deprecation_reason" версии не меняются

  Сценарий: После deprecation манифест пересобирается автоматически
    Дано "CreditAgeMin" версии "1.0.0" помечена deprecated
    Когда MCP-инструмент "check.deprecate" завершился
    Тогда "service_hash" изменился
    И в манифесте "1.0.0" в разделе "deprecated"
    И "active" переключился на следующую supported

  Сценарий: Данные deprecated-версии сохраняются
    Дано "CreditAgeMin" версии "1.0.0" помечена deprecated
    Тогда путь "checks/CreditAgeMin/1/0/0/" всё ещё существует в main
    И файлы внутри доступны через GET /checks/CreditAgeMin/versions/1.0.0
