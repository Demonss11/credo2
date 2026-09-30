# language: ru
# T-15 (проект): процесс, готовый к MCP — классификация re-plan (re-raise).
# Основание: docs/tasks/T-15-mcp-ready-process/mcp-ready-process.md; W8 §3 (цель конвергенции).
# Процесс агентов: структура re-plan.
Функция: Классификация re-plan
  Чтобы метрика конвергенции отличала улучшение канона от переобучения,
  каждый re-plan получает категорию и структурированный объект re-raise.

  Сценарий: Категория обязательна
    Когда "analyst" выполняет re-plan
    Тогда он помечает причину одной из категорий: "expect_mismatch", "owner_override", "plan_gap" или "role_failure"
    И пустая категория не допускается

  Сценарий: Объект re-raise
    Дано "analyst" классифицировал re-plan
    Тогда он записывает объект с полями "origin", "failed_clause" ("expectation", "observation"), "fix", "blocking" и "resolved"
    И объект доступен из плана и досье

  Сценарий: owner override не искажает конвергенцию
    Дано re-plan вызван директивой владельца (категория "owner_override")
    Тогда он не учитывается в метрике конвергенции
    И повторный owner override на одном участке — сигнал к разбору плана

  Сценарий: Повторяющийся "expect_mismatch" — кандидат в находку
    Дано категория "expect_mismatch" повторилась
    Тогда "analyst" вносит кандидата в "findings-registry.md"
