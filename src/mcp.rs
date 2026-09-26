use crate::core::{Value, condition_to_string, contract_from_rule, evaluate_rule, parse_rule};
use crate::{AppState, Draft, ServiceCache};
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
            _ => Err(format!("неизвестный инструмент: {name}")),
        }
    }

    async fn create(&self, args: JsonValue) -> Result<JsonValue, String> {
        let source = args
            .get("source")
            .and_then(|v| v.as_str())
            .ok_or("Нужен параметр 'source'")?;
        let rule = parse_rule(source)?;
        let existing = self.state.get_draft(&rule.name).await;
        let draft = self
            .state
            .make_draft(source.to_string(), rule, existing.as_ref());
        let name = draft.name.clone();
        self.state
            .upsert_draft(draft)
            .await
            .map_err(|e| e.to_string())?;
        Ok(json!({ "status": "ok", "name": name }))
    }

    async fn list_drafts(&self) -> Result<JsonValue, String> {
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
        Ok(json!({ "status": "ok", "draft": draft_json(&self.state, &d) }))
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
        // Q29: `stale` не блокирует `check.test`.
        let e = evaluate_rule(&d.rule, &input).map_err(|e| e.to_string())?;

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
            .map_err(|e| e.to_string())?;

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

/// Канонический объект черновика для `check.get_draft` (Q29, §4.5):
/// только рендеренные строки, внутренний `Rule` не публикуется; `stale`
/// и `test_valid` — вычисляемые.
fn draft_json(state: &AppState, d: &Draft) -> JsonValue {
    let test_valid = d.last_test_checksum.as_deref() == Some(d.source_hash.as_str());
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::source_hash;

    /// «Сторож» Q26 (Задача 6 плана): импорт/экспорт `.dar` — вне MVP,
    /// инструментов `check.import`/`check.export*` в `list_tools` быть не должно.
    #[test]
    fn no_import_export_tools_q26() {
        let names: Vec<String> = tool_specs().iter().map(|t| t.name.to_string()).collect();

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
        let cache = ServiceCache::load(state.published_repo().to_path_buf()).unwrap();
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
}
