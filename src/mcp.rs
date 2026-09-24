use crate::core::{Value, contract_from_rule, evaluate_rule, parse_rule};
use crate::{AppState, ServiceCache};
use rmcp::{
    handler::server::ServerHandler,
    model::{
        CallToolRequestParams, CallToolResponse, CallToolResult, ErrorData, ListToolsResult,
        PaginatedRequestParams, ServerConfig, Tool,
    },
    service::{RequestContext, RoleServer},
};
use serde_json::{Map, Value as JsonValue, json};
use std::collections::HashMap;
use std::sync::Arc;

pub async fn run_stdio(state: Arc<AppState>, cache: Arc<ServiceCache>) -> anyhow::Result<()> {
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

impl McpServer {
    async fn dispatch(&self, name: &str, args: JsonValue) -> Result<JsonValue, String> {
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
            _ => Err(format!("unknown tool: {name}")),
        }
    }

    async fn create(&self, args: JsonValue) -> Result<JsonValue, String> {
        let source = args
            .get("source")
            .and_then(|v| v.as_str())
            .ok_or("Нужен параметр 'source'")?;
        let rule = parse_rule(source)?;
        let existing = self.state.get_draft(&rule.name).await;
        let draft = self.state.make_draft(rule, existing.as_ref());
        let name = draft.name.clone();
        let overwritten = existing.is_some();
        self.state
            .upsert_draft(draft)
            .await
            .map_err(|e| e.to_string())?;
        Ok(json!({
            "status": "ok",
            "draft": name,
            "overwritten": overwritten,
            "sandbox_file": self.state.sandbox_path().display().to_string(),
        }))
    }

    async fn list_drafts(&self) -> Result<JsonValue, String> {
        let ds = self.state.list_drafts().await;
        Ok(json!({
            "count": ds.len(),
            "drafts": ds.iter().map(|d| json!({
                "name": d.name, "updated_at": d.updated_at,
                "condition": format!("{} {} {}",
                    d.rule.condition.field, d.rule.condition.op,
                    d.rule.condition.value.display()),
                "decision": d.rule.action.decision,
            })).collect::<Vec<_>>(),
        }))
    }

    async fn get_draft(&self, args: JsonValue) -> Result<JsonValue, String> {
        let name = args
            .get("name")
            .and_then(|v| v.as_str())
            .ok_or("Нужен 'name'")?;
        let d = self
            .state
            .get_draft(name)
            .await
            .ok_or_else(|| format!("Черновик '{name}' не найден"))?;
        serde_json::to_value(&d).map_err(|e| e.to_string())
    }

    async fn test(&self, args: JsonValue) -> Result<JsonValue, String> {
        let name = args
            .get("name")
            .and_then(|v| v.as_str())
            .ok_or("Нужен 'name'")?;
        let input_json = args.get("input").ok_or("Нужен 'input'")?;
        let input: HashMap<String, Value> =
            serde_json::from_value(input_json.clone()).map_err(|e| format!("Ошибка входа: {e}"))?;
        let d = self
            .state
            .get_draft(name)
            .await
            .ok_or_else(|| format!("Черновик '{name}' не найден"))?;
        // Q8/Q9: ошибка исполнения (отсутствующее поле, несовместимые
        // типы) возвращается как ошибка инструмента MCP.
        let e = evaluate_rule(&d.rule, &input).map_err(|e| e.to_string())?;
        Ok(serde_json::to_value(e).unwrap())
    }

    async fn delete_draft(&self, args: JsonValue) -> Result<JsonValue, String> {
        let name = args
            .get("name")
            .and_then(|v| v.as_str())
            .ok_or("Нужен 'name'")?;
        let removed = self
            .state
            .delete_draft(name)
            .await
            .map_err(|e| e.to_string())?;
        Ok(json!({ "status": if removed { "deleted" } else { "not_found" }, "name": name }))
    }

    async fn publish(&self, args: JsonValue) -> Result<JsonValue, String> {
        let name = args
            .get("name")
            .and_then(|v| v.as_str())
            .ok_or("Нужен 'name'")?;
        let version_raw = args
            .get("version")
            .and_then(|v| v.as_str())
            .ok_or("Нужен 'version'")?;
        let by = args
            .get("published_by")
            .and_then(|v| v.as_str())
            .unwrap_or("ai-agent");

        let d = self
            .state
            .get_draft(name)
            .await
            .ok_or_else(|| format!("Черновик '{name}' не найден"))?;

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
        .map_err(|e| format!("join: {e}"))?
        .map_err(|e| format!("{e}"))?;

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

    async fn deprecate(&self, args: JsonValue) -> Result<JsonValue, String> {
        let name = args
            .get("name")
            .and_then(|v| v.as_str())
            .ok_or("Нужен 'name'")?;
        let version = args
            .get("version")
            .and_then(|v| v.as_str())
            .ok_or("Нужен 'version'")?;
        let reason = args.get("reason").and_then(|v| v.as_str()).unwrap_or("");

        let repo = self.state.published_repo().to_path_buf();
        let name_s = name.to_string();
        let version_s = version.to_string();
        let reason_s = reason.to_string();

        tokio::task::spawn_blocking(move || {
            crate::deprecate(&repo, &name_s, &version_s, &reason_s)
        })
        .await
        .map_err(|e| format!("join: {e}"))?
        .map_err(|e| format!("{e}"))?;

        // Пересобираем манифест
        self.rebuild_manifest_inner()
            .await
            .map_err(|e| format!("{e}"))?;

        Ok(json!({
            "status": "deprecated",
            "name": name,
            "version": version,
            "reason": reason,
        }))
    }

    async fn list_published(&self) -> Result<JsonValue, String> {
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

    async fn rebuild_manifest(&self) -> Result<JsonValue, String> {
        let (h, written) = self
            .rebuild_manifest_inner()
            .await
            .map_err(|e| format!("{e}"))?;
        Ok(json!({ "status": "ok", "service_hash": h, "written": written }))
    }

    async fn rebuild_manifest_inner(&self) -> anyhow::Result<(String, bool)> {
        self.cache.reload().await?;
        let manifest = self.cache.manifest().await;
        let repo = self.state.published_repo().to_path_buf();
        let m = manifest.clone();
        let written = tokio::task::spawn_blocking(move || crate::write_manifest_to_main(&repo, &m))
            .await??
            .is_some();
        Ok((manifest.service_hash, written))
    }
}

fn make_tool(name: &str, description: &str, properties: JsonValue, required: Vec<&str>) -> Tool {
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
            "Создать черновик. source: Правило Name { Если (Поле < 21) { Решение = Отказ; Причина = \"...\"; } }",
            json!({ "source": { "type": "string" } }),
            vec!["source"],
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

fn response(value: JsonValue, is_error: bool) -> Result<CallToolResponse, ErrorData> {
    let text = serde_json::to_string(&value)
        .map_err(|e| ErrorData::internal_error(format!("serialize: {e}"), None))?;
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
            Err(e) => response(json!({ "error": e }), true),
        }
    }
}
