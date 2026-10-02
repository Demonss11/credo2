use crate::core::{
    Semver, Value, condition_to_string, contract_from_rule, evaluate_rule,
    parse_rule,
};
use crate::{AppState, DeprecateError, Draft, ServiceCache};
use rmcp::{
    handler::server::ServerHandler,
    model::{
        CallToolRequestParams, CallToolResponse, CallToolResult, ErrorData,
        ListToolsResult, PaginatedRequestParams, ServerConfig, Tool,
    },
    service::{RequestContext, RoleServer},
};
use serde_json::{Map, Value as JsonValue, json};
use std::collections::HashMap;
use std::sync::Arc;

pub async fn run_stdio(
    state: Arc<AppState>,
    cache: Arc<ServiceCache>,
) -> anyhow::Result<()> {
    let server = McpServer { state, cache };
    let service = rmcp::serve_server(server, rmcp::transport::stdio()).await?;
    service.waiting().await?;
    Ok(())
}

#[derive(Clone)]
struct McpServer {
    state: Arc<AppState>,
    cache: Arc<ServiceCache>,
}

/// Стабильные коды ошибок MCP-инструментов (Q29, §4.5). `code` — латиница
/// `snake_case`, `message` — русский (Q11).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ErrorCode {
    ValidationFailed,
    DraftNotFound,
    EvaluationFailed,
    PublishFailed,
    VersionNotFound,
    /// Зарезервирован под исполнение deprecated-версии (`check.run`, Q33 —
    /// вне MVP): в текущих MCP-путях не возникает.
    #[allow(dead_code)]
    VersionDeprecated,
    DeprecationConflict,
    ManifestError,
    UnknownTool,
    InternalError,
}

impl ErrorCode {
    fn as_str(self) -> &'static str {
        match self {
            ErrorCode::ValidationFailed => "validation_failed",
            ErrorCode::DraftNotFound => "draft_not_found",
            ErrorCode::EvaluationFailed => "evaluation_failed",
            ErrorCode::PublishFailed => "publish_failed",
            ErrorCode::VersionNotFound => "version_not_found",
            ErrorCode::VersionDeprecated => "version_deprecated",
            ErrorCode::DeprecationConflict => "deprecation_conflict",
            ErrorCode::ManifestError => "manifest_error",
            ErrorCode::UnknownTool => "unknown_tool",
            ErrorCode::InternalError => "internal_error",
        }
    }
}

/// Структурная ошибка инструмента MCP: код + русский текст. Рендерится в
/// единый конверт `{"error":{"code","message"}}` (Q29, §4.5).
#[derive(Clone, Debug, PartialEq, Eq)]
struct ToolError {
    code: ErrorCode,
    message: String,
}

impl ToolError {
    fn new(code: ErrorCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }

    fn validation(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::ValidationFailed, message)
    }

    /// `draft_not_found`; текст содержит «черновик не найден»
    /// (`test_draft.feature`).
    fn draft_not_found(name: &str) -> Self {
        Self::new(
            ErrorCode::DraftNotFound,
            format!("черновик не найден: {name}"),
        )
    }

    fn evaluation(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::EvaluationFailed, message)
    }

    fn publish(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::PublishFailed, message)
    }

    fn version_not_found(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::VersionNotFound, message)
    }

    fn deprecation_conflict(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::DeprecationConflict, message)
    }

    fn manifest_error(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::ManifestError, message)
    }

    fn unknown_tool(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::UnknownTool, message)
    }

    fn internal(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::InternalError, message)
    }

    /// Единый конверт ошибки MCP (Q29, §4.5).
    fn to_json(&self) -> JsonValue {
        json!({ "error": { "code": self.code.as_str(), "message": self.message } })
    }
}

impl McpServer {
    async fn dispatch(
        &self,
        name: &str,
        args: JsonValue,
    ) -> Result<JsonValue, ToolError> {
        match name {
            "check.create" => self.create(args).await,
            "check.list_drafts" => self.list_drafts().await,
            "check.get_draft" => self.get_draft(args).await,
            "check.test" => self.test(args).await,
            "check.delete_draft" => self.delete_draft(args).await,
            "check.publish" => self.publish(args).await,
            "check.deprecate" => self.deprecate(args).await,
            "check.list_published" => self.list_published().await,
            "check.rebuild_manifest" => self.rebuild_manifest().await,
            _ => Err(ToolError::unknown_tool(format!(
                "неизвестный инструмент: {name}"
            ))),
        }
    }

    async fn create(&self, args: JsonValue) -> Result<JsonValue, ToolError> {
        // Q28: оба параметра обязательны; `name` сверяется с заголовком
        // `Правило {name}` (GRAMMAR.md).
        let name = required_str(&args, "name")?;
        let source = required_str(&args, "source")?;
        // Q29: невалидный `.dar`-текст — `validation_failed`.
        let rule = parse_rule(source).map_err(ToolError::validation)?;
        if rule.name != name {
            return Err(ToolError::validation(format!(
                "Имя '{name}' не совпадает с заголовком '{}'",
                rule.name
            )));
        }
        let existing = self.state.get_draft(&rule.name).await;
        let draft =
            self.state
                .make_draft(source.to_string(), rule, existing.as_ref());
        let name = draft.name.clone();
        self.state
            .upsert_draft(draft)
            .await
            .map_err(|e| ToolError::internal(e.to_string()))?;
        Ok(json!({ "status": "ok", "name": name }))
    }

    async fn list_drafts(&self) -> Result<JsonValue, ToolError> {
        let ds = self.state.list_drafts().await;
        Ok(json!({
            "status": "ok",
            "count": ds.len(),
            "drafts": ds.iter().map(|d| json!({
                "name": d.name,
                "updated_at": d.updated_at,
                "condition": condition_to_string(&d.rule.condition),
                "decision": d.rule.action.decision,
                "stale": self.state.is_stale(d),
            })).collect::<Vec<_>>(),
        }))
    }

    async fn get_draft(&self, args: JsonValue) -> Result<JsonValue, ToolError> {
        let name = args
            .get("name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ToolError::validation("Нужен 'name'"))?;
        let d = self
            .state
            .get_draft(name)
            .await
            .ok_or_else(|| ToolError::draft_not_found(name))?;
        Ok(json!({ "status": "ok", "draft": draft_json(&self.state, &d) }))
    }

    async fn test(&self, args: JsonValue) -> Result<JsonValue, ToolError> {
        let name = args
            .get("name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ToolError::validation("Нужен 'name'"))?;
        let input_json = args
            .get("input")
            .ok_or_else(|| ToolError::validation("Нужен 'input'"))?;
        let input: HashMap<String, Value> =
            serde_json::from_value(input_json.clone()).map_err(|e| {
                ToolError::validation(format!("Ошибка входа: {e}"))
            })?;
        let d = self
            .state
            .get_draft(name)
            .await
            .ok_or_else(|| ToolError::draft_not_found(name))?;
        // Q8/Q9: ошибка исполнения (отсутствующее поле, несовместимые
        // типы) возвращается как ошибка инструмента MCP с кодом
        // `evaluation_failed` (Q29).
        // Q29: `stale` не блокирует `check.test`.
        let e = evaluate_rule(&d.rule, &input)
            .map_err(|e| ToolError::evaluation(e.to_string()))?;

        // Q16/Q34/Q29: успешный тест фиксирует метку `last_test_checksum`
        // (= `source_hash` на момент теста) и `tested_at`.
        let tested_at = chrono::Utc::now().to_rfc3339();
        let checksum = d.source_hash.clone();
        let mut updated = d.clone();
        updated.last_test_checksum = Some(checksum.clone());
        updated.tested_at = Some(tested_at.clone());
        self.state
            .upsert_draft(updated)
            .await
            .map_err(|e| ToolError::internal(e.to_string()))?;

        Ok(json!({
            "status": "ok",
            "rule_name": e.rule_name,
            "condition": e.condition,
            "actual_value": e.actual_value,
            "matched": e.matched,
            "decision": e.decision,
            "reason": e.reason,
            "source_hash": d.source_hash,
            "tested_at": tested_at,
            "last_test_checksum": checksum,
        }))
    }

    async fn delete_draft(
        &self,
        args: JsonValue,
    ) -> Result<JsonValue, ToolError> {
        let name = args
            .get("name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ToolError::validation("Нужен 'name'"))?;
        let removed = self
            .state
            .delete_draft(name)
            .await
            .map_err(|e| ToolError::internal(e.to_string()))?;
        Ok(
            json!({ "status": if removed { "deleted" } else { "not_found" }, "name": name }),
        )
    }

    async fn publish(&self, args: JsonValue) -> Result<JsonValue, ToolError> {
        let name = args
            .get("name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ToolError::validation("Нужен 'name'"))?;
        let version_raw = args
            .get("version")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ToolError::validation("Нужен 'version'"))?;
        let by = args
            .get("published_by")
            .and_then(|v| v.as_str())
            .unwrap_or("ai-agent");

        // Q29/§4.5: невалидная версия — `validation_failed`
        // (mcp_tools.feature); проверяем до поиска черновика, чтобы код не
        // зависел от его наличия.
        Semver::parse(version_raw).map_err(|e| {
            ToolError::validation(format!(
                "невалидная версия {version_raw:?}: {e}"
            ))
        })?;

        let d = self
            .state
            .get_draft(name)
            .await
            .ok_or_else(|| ToolError::draft_not_found(name))?;

        let repo = self.state.published_repo().to_path_buf();
        let rule = d.rule.clone();
        // `publish` сам нормализует contract.version — не дублируем.
        let contract = contract_from_rule(&rule, version_raw);

        let version_s = version_raw.to_string();
        let by_s = by.to_string();

        let outcome = tokio::task::spawn_blocking(move || {
            crate::publish(&repo, &rule, &contract, &version_s, &by_s)
        })
        .await
        .map_err(|e| ToolError::internal(format!("join: {e}")))?
        .map_err(|e| ToolError::publish(e.to_string()))?;

        let branch = outcome.branch.clone();
        let repo_display = self.state.published_repo().display().to_string();
        Ok(json!({
            "status": "published",
            "name": outcome.name,
            "version": outcome.version,
            "branch": outcome.branch,
            "path": outcome.path,
            "commit_msg": outcome.commit_msg,
            "next_step": format!(
                "Ветка '{branch}' создана в {repo}. Смержить: \
                 `git -C {repo} update-ref refs/heads/main $(git -C {repo} rev-parse {branch})`. \
                 REST подхватит автоматически в течение 2 с.",
                repo = repo_display,
            ),
        }))
    }

    async fn deprecate(&self, args: JsonValue) -> Result<JsonValue, ToolError> {
        let name = args
            .get("name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ToolError::validation("Нужен 'name'"))?;
        let version = args
            .get("version")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ToolError::validation("Нужен 'version'"))?;
        let reason = args.get("reason").and_then(|v| v.as_str()).unwrap_or("");
        // Q29/deprecation.feature: `reason` обязателен и непуст.
        if reason.is_empty() {
            return Err(ToolError::validation("Нужен непустой 'reason'"));
        }
        // Q29: невалидная версия — `validation_failed`.
        Semver::parse(version).map_err(|e| {
            ToolError::validation(format!("невалидная версия {version:?}: {e}"))
        })?;

        let repo = self.state.published_repo().to_path_buf();
        let name_s = name.to_string();
        let version_s = version.to_string();
        let reason_s = reason.to_string();

        tokio::task::spawn_blocking(move || {
            crate::deprecate(&repo, &name_s, &version_s, &reason_s)
        })
        .await
        .map_err(|e| ToolError::internal(format!("join: {e}")))?
        .map_err(|e| {
            let message = e.to_string();
            match e {
                DeprecateError::AlreadyDeprecated { .. } => {
                    ToolError::deprecation_conflict(message)
                },
                DeprecateError::VersionNotFound { .. } => {
                    ToolError::version_not_found(message)
                },
                DeprecateError::Other(_) => ToolError::internal(message),
            }
        })?;

        // Пересобираем манифест
        self.rebuild_manifest_inner()
            .await
            .map_err(|e| ToolError::manifest_error(e.to_string()))?;

        Ok(json!({
            "status": "deprecated",
            "name": name,
            "version": version,
            "reason": reason,
        }))
    }

    async fn list_published(&self) -> Result<JsonValue, ToolError> {
        let checks = self.cache.checks().await;
        Ok(json!({
            "count": checks.len(),
            "checks": checks.iter().map(|c| json!({
                "name": c.name,
                "version": c.version,
                "published_at": c.meta.published_at,
                "published_by": c.meta.published_by,
                "deprecated_at": c.meta.deprecated_at,
            })).collect::<Vec<_>>(),
        }))
    }

    async fn rebuild_manifest(&self) -> Result<JsonValue, ToolError> {
        let (h, written) = self
            .rebuild_manifest_inner()
            .await
            .map_err(|e| ToolError::manifest_error(e.to_string()))?;
        Ok(json!({ "status": "ok", "service_hash": h, "written": written }))
    }

    async fn rebuild_manifest_inner(&self) -> anyhow::Result<(String, bool)> {
        self.cache.reload().await?;
        let manifest = self.cache.manifest().await;
        let repo = self.state.published_repo().to_path_buf();
        let m = manifest.clone();
        let written = tokio::task::spawn_blocking(move || {
            crate::write_manifest_to_main(&repo, &m)
        })
        .await??
        .is_some();
        Ok((manifest.service_hash, written))
    }
}

/// Обязательный непустой строковый параметр (`check.create`, Q28): отсутствие,
/// не-строка и пустая строка — ошибка валидации `validation_failed`.
fn required_str<'a>(
    args: &'a JsonValue,
    key: &str,
) -> Result<&'a str, ToolError> {
    match args.get(key) {
        Some(JsonValue::String(s)) if !s.trim().is_empty() => Ok(s),
        Some(JsonValue::String(_)) => Err(ToolError::validation(format!(
            "Параметр '{key}' не может быть пустым"
        ))),
        Some(_) => Err(ToolError::validation(format!(
            "Параметр '{key}' должен быть строкой"
        ))),
        None => Err(ToolError::validation(format!("Нужен параметр '{key}'"))),
    }
}

/// Канонический объект черновика для `check.get_draft` (Q29, §4.5):
/// только рендеренные строки, внутренний `Rule` не публикуется; `stale`
/// и `test_valid` — вычисляемые.
fn draft_json(state: &AppState, d: &Draft) -> JsonValue {
    let test_valid =
        d.last_test_checksum.as_deref() == Some(d.source_hash.as_str());
    json!({
        "name": d.name,
        "source": d.source,
        "source_hash": d.source_hash,
        "condition": condition_to_string(&d.rule.condition),
        "decision": d.rule.action.decision,
        "reason": d.rule.action.reason,
        "created_at": d.created_at,
        "updated_at": d.updated_at,
        "last_test_checksum": d.last_test_checksum,
        "tested_at": d.tested_at,
        "test_valid": test_valid,
        "stale": state.is_stale(d),
    })
}

fn make_tool(
    name: &str,
    description: &str,
    properties: JsonValue,
    required: Vec<&str>,
) -> Tool {
    serde_json::from_value(json!({
        "name": name, "description": description,
        "inputSchema": { "type": "object", "properties": properties, "required": required }
    }))
    .expect("valid tool")
}

fn tool_specs() -> Vec<Tool> {
    vec![
        make_tool(
            "check.create",
            "Создать черновик. name сверяется с заголовком в source: Правило Name { Если (Поле < 21) { Решение = Отказ; Причина = \"...\"; } }",
            json!({ "name": { "type": "string" }, "source": { "type": "string" } }),
            vec!["name", "source"],
        ),
        make_tool("check.list_drafts", "Список черновиков", json!({}), vec![]),
        make_tool(
            "check.get_draft",
            "Получить черновик",
            json!({ "name": { "type": "string" } }),
            vec!["name"],
        ),
        make_tool(
            "check.test",
            "Прогнать на входе",
            json!({ "name": {"type":"string"}, "input": {"type":"object"} }),
            vec!["name", "input"],
        ),
        make_tool(
            "check.delete_draft",
            "Удалить черновик",
            json!({ "name": { "type": "string" } }),
            vec!["name"],
        ),
        make_tool(
            "check.publish",
            "Опубликовать: создаёт ветку publish/{name}-{version}. Требует semver. \
             Downgrade и дубликаты отклоняются. Изменение входов требует MAJOR bump.",
            json!({
                "name": {"type":"string"},
                "version": {"type":"string", "description":"semver, напр. 1.0.0"},
                "published_by": {"type":"string"}
            }),
            vec!["name", "version"],
        ),
        make_tool(
            "check.deprecate",
            "Пометить версию deprecated (пишет напрямую в main)",
            json!({
                "name":{"type":"string"},
                "version":{"type":"string"},
                "reason":{"type":"string"}
            }),
            vec!["name", "version", "reason"],
        ),
        make_tool(
            "check.list_published",
            "Список из main (то, что видит REST)",
            json!({}),
            vec![],
        ),
        make_tool(
            "check.rebuild_manifest",
            "Пересобрать manifest.json в main и перезагрузить кэш",
            json!({}),
            vec![],
        ),
    ]
}

fn response(
    value: JsonValue,
    is_error: bool,
) -> Result<CallToolResponse, ErrorData> {
    let text = serde_json::to_string(&value).map_err(|e| {
        ErrorData::internal_error(format!("serialize: {e}"), None)
    })?;
    let r: CallToolResult = serde_json::from_value(json!({
        "content": [{ "type": "text", "text": text }],
        "isError": is_error
    }))
    .map_err(|e| ErrorData::internal_error(format!("build: {e}"), None))?;
    Ok(CallToolResponse::Complete(r))
}

impl ServerHandler for McpServer {
    fn get_info(&self) -> ServerConfig {
        serde_json::from_value(json!({
            "protocolVersion": "2024-11-05",
            "capabilities": { "tools": {} },
            "serverInfo": { "name": "credo", "version": env!("CARGO_PKG_VERSION") },
            "instructions": "CREDO. Работаем в песочнице. Публикация создаёт git-ветку, \
                после мержа вызовите check.rebuild_manifest."
        }))
        .expect("valid ServerConfig")
    }

    async fn list_tools(
        &self,
        _r: Option<PaginatedRequestParams>,
        _c: RequestContext<RoleServer>,
    ) -> Result<ListToolsResult, ErrorData> {
        Ok(ListToolsResult {
            result_type: None,
            meta: None,
            next_cursor: None,
            ttl_ms: None,
            cache_scope: None,
            tools: tool_specs(),
        })
    }

    async fn call_tool(
        &self,
        req: CallToolRequestParams,
        _c: RequestContext<RoleServer>,
    ) -> Result<CallToolResponse, ErrorData> {
        let name = req.name;
        let args = req
            .arguments
            .map(JsonValue::Object)
            .unwrap_or_else(|| JsonValue::Object(Map::new()));
        match self.dispatch(&name, args).await {
            Ok(v) => response(v, false),
            Err(e) => response(e.to_json(), true),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::source_hash;

    /// «Сторож» Q26 (Задача 6 плана): импорт/экспорт `.dar` — вне MVP,
    /// инструментов `check.import`/`check.export*` в `list_tools` быть не должно.
    #[test]
    fn no_import_export_tools_q26() {
        let names: Vec<String> =
            tool_specs().iter().map(|t| t.name.to_string()).collect();

        assert!(!names.is_empty(), "list_tools пуст");
        for name in &names {
            assert!(
                !name.contains("import") && !name.contains("export"),
                "появился import/export инструмент (Q26): {name}"
            );
        }
        // Санитарная проверка: список читается и содержит канонические имена.
        assert!(
            names.contains(&"check.publish".to_string()),
            "names = {names:?}"
        );
    }

    // ---------- T-01: source/source_hash/stale/test_valid ----------

    const SRC: &str = "Правило МинимальныйВозраст { Если (Клиент.Возраст < 21) { \
                       Решение = Отказ; Причина = \"Возраст меньше 21\"; } }";

    fn new_server(dir: &std::path::Path) -> McpServer {
        let state = Arc::new(AppState::new(dir.to_path_buf(), None).unwrap());
        let cache =
            ServiceCache::load(state.published_repo().to_path_buf()).unwrap();
        McpServer { state, cache }
    }

    async fn create(srv: &McpServer, source: &str) -> JsonValue {
        srv.dispatch(
            "check.create",
            json!({ "name": "МинимальныйВозраст", "source": source }),
        )
        .await
        .unwrap()
    }

    async fn get_draft(srv: &McpServer) -> JsonValue {
        srv.dispatch("check.get_draft", json!({ "name": "МинимальныйВозраст" }))
            .await
            .unwrap()
    }

    // ---------- T-03: check.create {name, source} (Q28) ----------

    async fn create_named(
        srv: &McpServer,
        name: &str,
        source: &str,
    ) -> Result<JsonValue, ToolError> {
        srv.dispatch("check.create", json!({ "name": name, "source": source }))
            .await
    }

    /// Разворачивает ошибку в сообщение, требуя код `validation_failed`.
    fn validation_message(err: ToolError) -> String {
        assert_eq!(err.code, ErrorCode::ValidationFailed);
        err.message
    }

    #[tokio::test]
    async fn create_name_matches_header_ok_t03() {
        let t = tempfile::tempdir().unwrap();
        let srv = new_server(t.path());
        let ok = create_named(&srv, "МинимальныйВозраст", SRC).await.unwrap();
        assert_eq!(ok, json!({ "status": "ok", "name": "МинимальныйВозраст" }));
    }

    #[tokio::test]
    async fn create_name_mismatch_is_validation_failed_t03() {
        let t = tempfile::tempdir().unwrap();
        let srv = new_server(t.path());
        let msg = validation_message(
            create_named(&srv, "ДругоеИмя", SRC).await.unwrap_err(),
        );
        assert!(msg.contains("ДругоеИмя"), "нет входного name: {msg}");
        assert!(
            msg.contains("МинимальныйВозраст"),
            "нет имени из source: {msg}"
        );
        // Черновик не создан.
        assert!(
            srv.dispatch("check.get_draft", json!({ "name": "ДругоеИмя" }))
                .await
                .is_err()
        );
    }

    #[tokio::test]
    async fn create_missing_or_empty_name_is_validation_failed_t03() {
        let t = tempfile::tempdir().unwrap();
        let srv = new_server(t.path());

        let missing = srv
            .dispatch("check.create", json!({ "source": SRC }))
            .await
            .unwrap_err();
        assert!(validation_message(missing).contains("name"));

        let empty = create_named(&srv, "", SRC).await.unwrap_err();
        assert!(validation_message(empty).contains("name"));

        let non_string = srv
            .dispatch("check.create", json!({ "name": 42, "source": SRC }))
            .await
            .unwrap_err();
        assert!(validation_message(non_string).contains("name"));
    }

    #[tokio::test]
    async fn create_missing_source_is_validation_failed_t03() {
        let t = tempfile::tempdir().unwrap();
        let srv = new_server(t.path());
        let err = srv
            .dispatch("check.create", json!({ "name": "МинимальныйВозраст" }))
            .await
            .unwrap_err();
        assert!(validation_message(err).contains("source"));
    }

    #[tokio::test]
    async fn create_invalid_source_is_validation_failed_t03() {
        let t = tempfile::tempdir().unwrap();
        let srv = new_server(t.path());
        let bad = "Если (Клиент.Возраст < 21) { Решение = Отказ; }";
        let msg = validation_message(
            create_named(&srv, "МинимальныйВозраст", bad)
                .await
                .unwrap_err(),
        );
        assert!(msg.contains("отсутствует заголовок правила"), "{msg}");
        // Инвариант 1 (§4.5): черновик с невалидным текстом не создаётся.
        assert!(srv.state.get_draft("МинимальныйВозраст").await.is_none());
        assert_eq!(
            srv.dispatch("check.list_drafts", json!({})).await.unwrap()["count"],
            json!(0)
        );
    }

    #[tokio::test]
    async fn create_repeat_overwrites_draft_t03() {
        let t = tempfile::tempdir().unwrap();
        let srv = new_server(t.path());
        create(&srv, SRC).await;

        // «Сохранить = обновить»: тот же name, новый source.
        let new_src = SRC.replace("< 21", "< 18");
        let again = create(&srv, &new_src).await;
        assert_eq!(
            again,
            json!({ "status": "ok", "name": "МинимальныйВозраст" })
        );

        let d = &get_draft(&srv).await["draft"];
        assert_eq!(d["source"], new_src);
        assert_eq!(
            srv.dispatch("check.list_drafts", json!({})).await.unwrap()["count"],
            json!(1)
        );
    }

    #[test]
    fn validation_error_json_envelope_t03() {
        let j = ToolError::validation("нет заголовка").to_json();
        assert_eq!(j["error"]["code"], "validation_failed");
        assert_eq!(j["error"]["message"], "нет заголовка");
    }

    #[tokio::test]
    async fn create_stores_source_and_hash_q12() {
        let t = tempfile::tempdir().unwrap();
        let srv = new_server(t.path());

        let created = create(&srv, SRC).await;
        assert_eq!(created["status"], "ok");
        assert_eq!(created["name"], "МинимальныйВозраст");

        let got = get_draft(&srv).await;
        assert_eq!(got["status"], "ok");
        let d = &got["draft"];
        // Текст хранится без изменений; есть хэш и рендеренные строки.
        assert_eq!(d["source"], SRC);
        assert_eq!(d["source_hash"], source_hash(SRC));
        assert_eq!(d["condition"], "Клиент.Возраст < 21");
        assert_eq!(d["decision"], "Отказ");
        assert_eq!(d["reason"], "Возраст меньше 21");
        // Инвариант 2: внутренний `Rule` не публикуется.
        assert!(d.get("rule").is_none(), "rule не должен публиковаться: {d}");
        // Инвариант 5: `size`/`format` не вводятся.
        assert!(d.get("size").is_none() && d.get("format").is_none());
        assert!(d["created_at"].is_string() && d["updated_at"].is_string());
        assert!(d["last_test_checksum"].is_null() && d["tested_at"].is_null());
        assert_eq!(d["test_valid"], json!(false));
        assert_eq!(d["stale"], json!(false));
    }

    // ---------- T-03/Q28: {name, source} обязательны, name ↔ заголовок ----------

    #[tokio::test]
    async fn create_requires_name_param() {
        let t = tempfile::tempdir().unwrap();
        let srv = new_server(t.path());

        let err = srv
            .dispatch("check.create", json!({ "source": SRC }))
            .await
            .unwrap_err();
        assert_eq!(err.code, ErrorCode::ValidationFailed);
        assert_eq!(err.message, "Нужен параметр 'name'");
    }

    #[tokio::test]
    async fn create_rejects_name_title_mismatch() {
        let t = tempfile::tempdir().unwrap();
        let srv = new_server(t.path());

        let err = srv
            .dispatch(
                "check.create",
                json!({ "name": "ДругоеИмя", "source": SRC }),
            )
            .await
            .unwrap_err();
        assert_eq!(err.code, ErrorCode::ValidationFailed);
        assert!(err.message.contains("не совпадает"), "err = {err:?}");
        assert!(err.message.contains("МинимальныйВозраст"), "err = {err:?}");
        // Черновик при расхождении не создаётся.
        let drafts =
            srv.dispatch("check.list_drafts", json!({})).await.unwrap();
        assert_eq!(drafts["count"], json!(0));
    }

    #[tokio::test]
    async fn create_returns_status_and_name() {
        let t = tempfile::tempdir().unwrap();
        let srv = new_server(t.path());

        let created = srv
            .dispatch(
                "check.create",
                json!({ "name": "МинимальныйВозраст", "source": SRC }),
            )
            .await
            .unwrap();
        assert_eq!(
            created,
            json!({ "status": "ok", "name": "МинимальныйВозраст" })
        );
    }

    #[test]
    fn check_create_tool_spec_requires_name_and_source() {
        let tool = tool_specs()
            .into_iter()
            .find(|t| t.name == "check.create")
            .expect("check.create должен быть в tool_specs");
        let v = serde_json::to_value(&tool).unwrap();

        let props = &v["inputSchema"]["properties"];
        assert!(props.get("name").is_some(), "нет свойства name: {v}");
        assert!(props.get("source").is_some(), "нет свойства source: {v}");
        let required = v["inputSchema"]["required"]
            .as_array()
            .expect("required должен быть массивом");
        for key in ["name", "source"] {
            assert!(
                required.iter().any(|x| x == key),
                "required не содержит {key}: {v}"
            );
        }
    }

    #[tokio::test]
    async fn list_drafts_has_canonical_fields_q29() {
        let t = tempfile::tempdir().unwrap();
        let srv = new_server(t.path());
        create(&srv, SRC).await;

        let resp = srv.dispatch("check.list_drafts", json!({})).await.unwrap();
        assert_eq!(resp["status"], "ok");
        assert_eq!(resp["count"], json!(1));
        let item = &resp["drafts"][0];
        for key in ["name", "updated_at", "condition", "decision", "stale"] {
            assert!(item.get(key).is_some(), "нет поля {key}: {item}");
        }
        assert_eq!(item["condition"], "Клиент.Возраст < 21");
        assert_eq!(item["decision"], "Отказ");
        assert_eq!(item["stale"], json!(false));
        assert!(
            item.get("size").is_none() && item.get("format").is_none(),
            "size/format не вводятся: {item}"
        );
    }

    #[tokio::test]
    async fn stale_false_without_dar_file_q29_inv3() {
        let t = tempfile::tempdir().unwrap();
        let srv = new_server(t.path());
        create(&srv, SRC).await;

        // rules/МинимальныйВозраст.dar не создаётся (draft-first, Q33).
        assert!(!t.path().join("rules/МинимальныйВозраст.dar").exists());
        let d = &get_draft(&srv).await["draft"];
        assert_eq!(d["stale"], json!(false));
    }

    #[tokio::test]
    async fn stale_false_when_dar_hash_matches_q29_inv3() {
        let t = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(t.path().join("rules")).unwrap();
        let file = t.path().join("rules/МинимальныйВозраст.dar");
        std::fs::write(&file, SRC).unwrap();

        let srv = new_server(t.path());
        create(&srv, SRC).await;

        let d = &get_draft(&srv).await["draft"];
        assert_eq!(d["stale"], json!(false));
        // Исходный файл не изменяется (Q12).
        assert_eq!(std::fs::read_to_string(&file).unwrap(), SRC);
    }

    #[tokio::test]
    async fn stale_true_when_dar_hash_differs_q29_inv3() {
        let t = tempfile::tempdir().unwrap();
        let srv = new_server(t.path());
        create(&srv, SRC).await;

        // Файл изменён после создания черновика (Q12).
        std::fs::create_dir_all(t.path().join("rules")).unwrap();
        std::fs::write(
            t.path().join("rules/МинимальныйВозраст.dar"),
            "Правило МинимальныйВозраст { Если (Клиент.Возраст < 18) { \
             Решение = Отказ; Причина = \"Возраст меньше 18\"; } }",
        )
        .unwrap();

        let d = &get_draft(&srv).await["draft"];
        assert_eq!(d["stale"], json!(true));

        // Инвариант 3: `stale` не блокирует `check.test`.
        let tested = srv
            .dispatch(
                "check.test",
                json!({ "name": "МинимальныйВозраст", "input": { "Клиент.Возраст": 19 } }),
            )
            .await
            .unwrap();
        assert_eq!(tested["status"], "ok");
    }

    #[tokio::test]
    async fn successful_test_records_checksum_and_reason_q29() {
        let t = tempfile::tempdir().unwrap();
        let srv = new_server(t.path());
        create(&srv, SRC).await;

        let tested = srv
            .dispatch(
                "check.test",
                json!({ "name": "МинимальныйВозраст", "input": { "Клиент.Возраст": 19 } }),
            )
            .await
            .unwrap();
        assert_eq!(tested["status"], "ok");
        assert_eq!(tested["rule_name"], "МинимальныйВозраст");
        assert_eq!(tested["condition"], "Клиент.Возраст < 21");
        // `actual_value` — это `Value::Number(f64)`, поэтому 19 сериализуется как 19.0.
        assert_eq!(tested["actual_value"], json!(19.0));
        assert_eq!(tested["matched"], json!(true));
        assert_eq!(tested["decision"], "Отказ");
        assert_eq!(tested["reason"], "Возраст меньше 21");
        assert_eq!(tested["source_hash"], source_hash(SRC));
        assert_eq!(tested["last_test_checksum"], tested["source_hash"]);
        assert!(tested["tested_at"].is_string());
        // Q42: объяснение на верхнем уровне, без вложенного `explanation`.
        assert!(tested.get("explanation").is_none());

        let d = &get_draft(&srv).await["draft"];
        assert_eq!(d["last_test_checksum"], source_hash(SRC));
        assert_eq!(d["test_valid"], json!(true));
        assert_eq!(d["tested_at"], tested["tested_at"]);
    }

    #[tokio::test]
    async fn test_valid_false_after_source_change_q29_inv4() {
        let t = tempfile::tempdir().unwrap();
        let srv = new_server(t.path());
        create(&srv, SRC).await;
        srv.dispatch(
            "check.test",
            json!({ "name": "МинимальныйВозраст", "input": { "Клиент.Возраст": 19 } }),
        )
        .await
        .unwrap();

        // Перезапись черновика новым текстом («сохранить = обновить»).
        let new_src = "Правило МинимальныйВозраст { Если (Клиент.Возраст < 18) { \
                       Решение = Отказ; Причина = \"Возраст меньше 18\"; } }";
        create(&srv, new_src).await;

        let d = &get_draft(&srv).await["draft"];
        assert_eq!(d["source"], new_src);
        assert_eq!(d["source_hash"], source_hash(new_src));
        assert_ne!(d["source_hash"], source_hash(SRC));
        // last_test_checksum сохранился, но метка недействительна.
        assert_eq!(d["test_valid"], json!(false));
    }

    // ---------- T-04/Q29: единый конверт ошибок и стабильные коды ----------

    #[test]
    fn error_codes_are_latin_snake_case_q29() {
        let cases = [
            (ErrorCode::ValidationFailed, "validation_failed"),
            (ErrorCode::DraftNotFound, "draft_not_found"),
            (ErrorCode::EvaluationFailed, "evaluation_failed"),
            (ErrorCode::PublishFailed, "publish_failed"),
            (ErrorCode::VersionNotFound, "version_not_found"),
            (ErrorCode::VersionDeprecated, "version_deprecated"),
            (ErrorCode::DeprecationConflict, "deprecation_conflict"),
            (ErrorCode::ManifestError, "manifest_error"),
            (ErrorCode::UnknownTool, "unknown_tool"),
            (ErrorCode::InternalError, "internal_error"),
        ];
        assert_eq!(cases.len(), 10);
        for (code, expected) in cases {
            assert_eq!(code.as_str(), expected);
            assert!(
                code.as_str()
                    .chars()
                    .all(|c| c.is_ascii_lowercase() || c == '_'),
                "код не snake_case латиницей: {}",
                code.as_str()
            );
        }
    }

    #[test]
    fn error_envelope_has_error_code_and_message_q29() {
        for err in [
            ToolError::validation("плохой вход"),
            ToolError::draft_not_found("Нет"),
            ToolError::evaluation("Неизвестное поле: X"),
            ToolError::publish("дубль версии"),
            ToolError::version_not_found("версия не найдена"),
            ToolError::deprecation_conflict("уже помечена"),
            ToolError::manifest_error("манифест не записан"),
            ToolError::unknown_tool("неизвестный инструмент"),
            ToolError::internal("внутренняя ошибка"),
        ] {
            let v = err.to_json();
            assert_eq!(v["error"]["code"], err.code.as_str());
            assert_eq!(v["error"]["message"], err.message);
        }
    }

    #[tokio::test]
    async fn unknown_tool_is_unknown_tool_code_q29() {
        let t = tempfile::tempdir().unwrap();
        let srv = new_server(t.path());

        let err = srv.dispatch("check.nope", json!({})).await.unwrap_err();
        assert_eq!(err.code, ErrorCode::UnknownTool);
        assert!(
            err.message.contains("неизвестный инструмент"),
            "message = {}",
            err.message
        );
    }

    #[tokio::test]
    async fn manifest_rebuild_failure_is_manifest_error_q29() {
        let t = tempfile::tempdir().unwrap();
        let srv = new_server(t.path());
        // Портим репозиторий публикаций: `.git` делает его рабочим —
        // `ensure_repo` отказывает, `rebuild_manifest` не проходит.
        std::fs::create_dir_all(srv.state.published_repo().join(".git"))
            .unwrap();

        let err = srv
            .dispatch("check.rebuild_manifest", json!({}))
            .await
            .unwrap_err();
        assert_eq!(err.code, ErrorCode::ManifestError);
    }

    #[tokio::test]
    async fn sandbox_write_failure_is_internal_error_q29() {
        let t = tempfile::tempdir().unwrap();
        let srv = new_server(t.path());
        // `sandbox.json` — каталог: запись песочницы падает.
        std::fs::create_dir_all(srv.state.sandbox_path()).unwrap();

        let err = srv
            .dispatch(
                "check.create",
                json!({ "name": "МинимальныйВозраст", "source": SRC }),
            )
            .await
            .unwrap_err();
        assert_eq!(err.code, ErrorCode::InternalError);
    }
}
