# language: ru
# Q20 (решено 2026-09-25): канон пути — /checks/{name}/versions/{version}/...;
# MVP-минимум REST: versioned evaluate, GET /checks (Q21), /health, /version,
# /openapi.json. Сценарии с active-evaluate и GET-деталями описывают
# эндпоинты вне MVP-минимума (реализованы в прототипе, судьба — пост-MVP).
# Q11: тексты ошибок — русские, статусы 409/410 (401 фиксируется в Q22).
# Q23 (решено 2026-09-25): конверт ошибки — {"error": {"code", "message"}};
# коды: check_not_found, version_not_found, version_deprecated,
# activation_unavailable, evaluation_failed, unauthorized.
Функция: REST API
  Как потребитель сервиса
  Я хочу читать версии и выполнять конкретные версии проверок
  Чтобы не быть привязанным к "последней"

  Контекст:
    Дано сервис запущен
    И манифест загружен

  Сценарий: GET /version возвращает отпечаток состояния
    Когда клиент вызывает GET /version
    Тогда ответ 200
    И тело содержит "schema_version", "generated_at", "service_hash", "checks"

  Сценарий: GET /checks возвращает манифест
    Когда клиент вызывает GET /checks
    Тогда ответ 200
    И тело содержит "schema_version", "count", "service_hash", "checks"

  Сценарий: GET /checks/:name/versions перечисляет версии
    Дано "CreditAgeMin" имеет supported ["1.0.1","1.0.0"] и deprecated ["1.1.0"]
    Когда клиент вызывает GET /checks/CreditAgeMin/versions
    Тогда ответ 200
    И поле "active" равно "1.0.1"
    И "supported" содержит "1.0.1" и "1.0.0"
    И "deprecated" содержит "1.1.0"

  Сценарий: GET конкретной версии
    Дано в main есть "CreditAgeMin" версии "1.0.0"
    Когда клиент вызывает GET /checks/CreditAgeMin/versions/1.0.0
    Тогда ответ 200
    И тело содержит "rule", "contract", "published_at", "checksum"

  Сценарий: GET несуществующей версии
    Когда клиент вызывает GET /checks/CreditAgeMin/versions/9.9.9
    Тогда ответ 404
    И тело содержит "error.code" равный "version_not_found" (конверт Q23)

  Сценарий: POST /checks/:name/evaluate использует active-версию
    Дано "CreditAgeMin" active "1.0.1"
    Когда клиент вызывает POST /checks/CreditAgeMin/evaluate
         с телом {"Клиент.Возраст": 19}
    Тогда ответ 200
    И поле "version" равно "1.0.1"
    И поле "active" равно true
    И поле "result.decision" непустое

  Сценарий: POST /checks/:name/versions/:version/evaluate использует конкретную
    Дано "CreditAgeMin" имеет версии "1.0.0" и "1.0.1"
    Когда клиент вызывает POST /checks/CreditAgeMin/versions/1.0.0/evaluate
    Тогда ответ 200
    И поле "version" равно "1.0.0"

  Сценарий: Выполнение deprecated-версии отклоняется
    Дано "CreditAgeMin" версии "1.1.0" помечена deprecated
    Когда клиент вызывает POST /checks/CreditAgeMin/versions/1.1.0/evaluate
    Тогда ответ 410
    И тело содержит "версия выведена из эксплуатации: CreditAgeMin@1.1.0"
    И поле "error.code" равно "version_deprecated"

  Сценарий: Нет active-версии — активация недоступна
    Дано все версии "CreditAgeMin" помечены deprecated
    Когда клиент вызывает POST /checks/CreditAgeMin/evaluate
    Тогда ответ 409
    И тело содержит "активация недоступна: CreditAgeMin"
    И поле "error.code" равно "activation_unavailable"

  Сценарий: OpenAPI отражает все версии
    Дано манифест содержит "CreditAgeMin" с версиями ["1.0.1","1.0.0"]
    Когда клиент вызывает GET /openapi.json
    Тогда paths содержит "/checks/CreditAgeMin/versions/1.0.1/evaluate"
    И paths содержит "/checks/CreditAgeMin/versions/1.0.0/evaluate"
    И paths содержит "/checks/CreditAgeMin/evaluate"
    И поле "info.version" равно "service_hash"

  Сценарий: REST не читает git на каждый запрос
    Дано сервис прочитал манифест один раз
    Когда приходит 100 запросов GET /checks
    Тогда git не вызывается ни разу
    И ответ формируется из кэша в памяти
