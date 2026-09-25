use crate::core::Value;
use crate::{AppState, Manifest, ServiceCache};
use axum::{
    Router,
    extract::{Path, Request, State},
    http::StatusCode,
    middleware::{self, Next},
    response::{Html, IntoResponse, Json, Response},
    routing::{get, post},
};
use serde_json::{Value as JsonValue, json};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;

pub async fn serve(
    state: Arc<AppState>,
    cache: Arc<ServiceCache>,
    addr: SocketAddr,
) -> anyhow::Result<()> {
    let app = Router::new()
        .route("/health", get(health))
        .route("/docs", get(docs))
        .route("/openapi.json", get(openapi))
        .route("/version", get(version))
        .route("/checks", get(list_checks))
        .route("/checks/{name}/versions", get(list_versions))
        .route("/checks/{name}/versions/{version}", get(get_version))
        .route("/checks/{name}/evaluate", post(eval_active))
        .route(
            "/checks/{name}/versions/{version}/evaluate",
            post(eval_version),
        )
        .layer(middleware::from_fn_with_state(state.clone(), auth))
        .with_state((state, cache));

    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!("REST on http://{addr}  Swagger: http://{addr}/docs");
    axum::serve(listener, app).await?;
    Ok(())
}

type Ctx = (Arc<AppState>, Arc<ServiceCache>);
type ApiErr = (StatusCode, Json<JsonValue>);

fn err(code: StatusCode, msg: impl Into<String>) -> ApiErr {
    (code, Json(json!({ "error": msg.into() })))
}

async fn auth(State(state): State<Arc<AppState>>, req: Request, next: Next) -> Response {
    let path = req.uri().path();
    if matches!(path, "/health" | "/docs" | "/openapi.json") {
        return next.run(req).await;
    }
    if let Some(expected) = state.api_key() {
        let provided = req.headers().get("x-api-key").and_then(|v| v.to_str().ok());
        if provided != Some(expected.as_str()) {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({ "error": "ошибка аутентификации: неверный или отсутствующий x-api-key" })),
            )
                .into_response();
        }
    }
    next.run(req).await
}

async fn health() -> Json<JsonValue> {
    Json(json!({ "status": "ok" }))
}

async fn version(State((_, cache)): State<Ctx>) -> Json<JsonValue> {
    let m = cache.manifest().await;
    Json(serde_json::to_value(&m).unwrap())
}

/// Канон Q21: `{schema_version, count, service_hash, checks}` — без
/// `generated_at` (он остаётся в `GET /version`). Чистая функция, чтобы
/// проверяться unit-тестом без HTTP.
fn manifest_response(m: &Manifest) -> JsonValue {
    json!({
        "schema_version": m.schema_version,
        "count": m.checks.len(),
        "service_hash": m.service_hash,
        "checks": m.checks,
    })
}

async fn list_checks(State((_, cache)): State<Ctx>) -> Json<JsonValue> {
    let m = cache.manifest().await;
    Json(manifest_response(&m))
}

async fn list_versions(
    State((_, cache)): State<Ctx>,
    Path(name): Path<String>,
) -> Result<Json<JsonValue>, ApiErr> {
    let e = cache.versions(&name).await.ok_or_else(|| {
        err(
            StatusCode::NOT_FOUND,
            format!("проверка '{name}' не найдена"),
        )
    })?;
    Ok(Json(serde_json::to_value(e).unwrap()))
}

async fn get_version(
    State((_, cache)): State<Ctx>,
    Path((name, version)): Path<(String, String)>,
) -> Result<Json<JsonValue>, ApiErr> {
    let c = cache.get(&name, &version).await.ok_or_else(|| {
        err(
            StatusCode::NOT_FOUND,
            format!("версия не найдена: {name}@{version}"),
        )
    })?;
    Ok(Json(json!({
        "name": c.name, "version": c.version,
        "rule": c.rule, "contract": c.contract,
        "published_at": c.meta.published_at,
        "published_by": c.meta.published_by,
        "checksum": c.meta.checksum,
        "deprecated_at": c.meta.deprecated_at,
        "deprecation_reason": c.meta.deprecation_reason,
    })))
}

async fn eval_active(
    State((_, cache)): State<Ctx>,
    Path(name): Path<String>,
    Json(input): Json<HashMap<String, Value>>,
) -> Result<Json<JsonValue>, ApiErr> {
    let active = cache.active_version(&name).await.ok_or_else(|| {
        err(
            StatusCode::NOT_FOUND,
            format!("проверка '{name}' не найдена"),
        )
    })?;
    eval_inner(&cache, &name, &active, input, true).await
}

async fn eval_version(
    State((_, cache)): State<Ctx>,
    Path((name, version)): Path<(String, String)>,
    Json(input): Json<HashMap<String, Value>>,
) -> Result<Json<JsonValue>, ApiErr> {
    eval_inner(&cache, &name, &version, input, false).await
}

async fn eval_inner(
    cache: &ServiceCache,
    name: &str,
    version: &str,
    input: HashMap<String, Value>,
    is_active: bool,
) -> Result<Json<JsonValue>, ApiErr> {
    let c = cache.get(name, version).await.ok_or_else(|| {
        err(
            StatusCode::NOT_FOUND,
            format!("версия не найдена: {name}@{version}"),
        )
    })?;
    // Q8/Q9: отсутствующее поле или несовместимые типы — 422, а не
    // молчаливое matched = false.
    let result = crate::core::evaluate_rule(&c.rule, &input)
        .map_err(|e| err(StatusCode::UNPROCESSABLE_ENTITY, e.to_string()))?;
    let deprecated = c.meta.deprecated_at.is_some();
    Ok(Json(json!({
        "check": name,
        "version": version,
        "active": is_active,
        "deprecated": deprecated,
        "deprecation_reason": c.meta.deprecation_reason,
        "result": result,
    })))
}

async fn openapi(State((_, cache)): State<Ctx>) -> Json<JsonValue> {
    let m = cache.manifest().await;
    let checks = cache.checks().await;

    let mut paths = serde_json::Map::new();
    paths.insert("/version".into(), json!({
        "get": { "summary": "Отпечаток состояния", "responses": { "200": { "description": "OK" } } }
    }));
    paths.insert(
        "/checks".into(),
        json!({
            "get": { "summary": "Манифест", "responses": { "200": { "description": "OK" } } }
        }),
    );

    for e in &m.checks {
        let name = &e.name;
        paths.insert(
            format!("/checks/{name}/versions"),
            json!({
                "get": { "summary": format!("Версии {name}"),
                         "responses": { "200": { "description": "OK" } } }
            }),
        );

        for v in e.supported.iter().chain(e.deprecated.iter()) {
            let c = match checks.iter().find(|c| &c.name == name && &c.version == v) {
                Some(c) => c,
                None => continue,
            };
            let props: serde_json::Map<String, JsonValue> = c
                .contract
                .inputs
                .iter()
                .map(|f| (f.name.clone(), json!({ "type": f.type_ })))
                .collect();

            let summary = format!("{name}@{v}");
            let deprecated = c.meta.deprecated_at.is_some();
            let mut ops = json!({
                "get": { "summary": format!("Контракт {summary}"),
                         "responses": { "200": { "description": "OK" }, "404": { "description": "Not found" } } },
                "post": {
                    "summary": format!("Выполнить {summary}"),
                    "description": c.contract.description,
                    "requestBody": {
                        "required": true,
                        "content": { "application/json": {
                            "schema": { "type": "object", "properties": props, "additionalProperties": true }
                        }}
                    },
                    "responses": { "200": { "description": "OK" }, "404": { "description": "Not found" } }
                }
            });
            if deprecated {
                ops["post"]["deprecated"] = json!(true);
            }
            paths.insert(
                format!("/checks/{name}/versions/{v}/evaluate"),
                json!({ "post": ops["post"].clone() }),
            );
            paths.insert(
                format!("/checks/{name}/versions/{v}"),
                json!({ "get": ops["get"].clone() }),
            );
        }

        // /checks/:name/evaluate — active-версия
        if let Some(active) = checks
            .iter()
            .find(|c| &c.name == name && c.version == e.active)
        {
            let props: serde_json::Map<String, JsonValue> = active
                .contract
                .inputs
                .iter()
                .map(|f| (f.name.clone(), json!({ "type": f.type_ })))
                .collect();
            paths.insert(format!("/checks/{name}/evaluate"), json!({
                "post": {
                    "summary": format!("Выполнить {name}@{active_version} (active)",
                        active_version = e.active),
                    "requestBody": {
                        "required": true,
                        "content": { "application/json": {
                            "schema": { "type": "object", "properties": props, "additionalProperties": true }
                        }}
                    },
                    "responses": { "200": { "description": "OK" }, "404": { "description": "Not found" } }
                }
            }));
        }
    }

    Json(json!({
        "openapi": "3.0.3",
        "info": {
            "title": "CREDO Published Checks",
            "version": m.service_hash,
            "description": "Только main. service_hash = info.version."
        },
        "paths": paths
    }))
}

async fn docs() -> Html<&'static str> {
    Html(SWAGGER_HTML)
}

const SWAGGER_HTML: &str = r#"<!DOCTYPE html><html lang="ru"><head><meta charset="utf-8"/>
<title>CREDO API</title>
<link rel="stylesheet" href="https://unpkg.com/swagger-ui-dist@5/swagger-ui.css"/>
<style>body{margin:0}</style></head><body>
<div id="swagger-ui"></div>
<script src="https://unpkg.com/swagger-ui-dist@5/swagger-ui-bundle.js" crossorigin></script>
<script>window.onload=()=>{window.ui=SwaggerUIBundle({url:'/openapi.json',dom_id:'#swagger-ui',deepLinking:true,presets:[SwaggerUIBundle.presets.apis]});};</script>
</body></html>"#;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ManifestEntry;

    fn entry(name: &str) -> ManifestEntry {
        ManifestEntry {
            name: name.into(),
            active: "1.0.1".into(),
            supported: vec!["1.0.1".into(), "1.0.0".into()],
            deprecated: vec![],
        }
    }

    #[test]
    fn manifest_response_has_canonical_keys_q21() {
        let m = Manifest {
            schema_version: 1,
            generated_at: "2026-09-25T00:00:00Z".into(),
            service_hash: "sha256:x".into(),
            checks: vec![entry("CreditAgeMin")],
        };
        let v = manifest_response(&m);
        let obj = v.as_object().unwrap();

        // Верхний уровень — ровно канон Q21, без generated_at.
        let mut keys: Vec<&str> = obj.keys().map(|k| k.as_str()).collect();
        keys.sort_unstable();
        assert_eq!(keys, ["checks", "count", "schema_version", "service_hash"]);
        assert_eq!(obj["schema_version"], json!(1));
        assert_eq!(obj["count"], json!(1));

        // Элемент checks — ровно name/active/supported/deprecated.
        let first = obj["checks"][0].as_object().unwrap();
        let mut fields: Vec<&str> = first.keys().map(|k| k.as_str()).collect();
        fields.sort_unstable();
        assert_eq!(fields, ["active", "deprecated", "name", "supported"]);
    }
}
