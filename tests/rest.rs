//! Интеграционные тесты REST (Задачи 3/8 плана Q11/Q20/Q21/Q26):
//! 409/410/404, конверт ошибок Q23 и его отсутствие в старом плоском виде.

use axum::Router;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use credo2::core::{Rule, contract_from_rule, parse_rule};
use credo2::{
    AppState, ServiceCache, commit_tree, publish, rev_parse, update_ref, write_index_with_parent,
};
use serde_json::{Value, json};
use std::path::Path;
use std::sync::Arc;
use tower::ServiceExt;

const AGE_RULE: &str = r#"Правило CreditAgeMin { Если (Клиент.Возраст < 21) { Решение = Отказ; Причина = "Возраст меньше 21"; } }"#;

fn publish_and_merge(repo: &Path, source: &str, version: &str) {
    let rule: Rule = parse_rule(source).unwrap();
    let contract = contract_from_rule(&rule, version);
    let outcome = publish(repo, &rule, &contract, version, "test").unwrap();
    let tree = write_index_with_parent(repo, &outcome.branch, &[]).unwrap();
    let main = rev_parse(repo, "main").unwrap();
    let commit = commit_tree(repo, &tree, &main, "merge").unwrap();
    update_ref(repo, "refs/heads/main", &commit).unwrap();
}

struct Fixture {
    _dir: tempfile::TempDir,
    app: Router,
}

/// `specs`: (исходник правила, версия, помечать ли deprecated) — в порядке
/// публикации (для одного имени версии по возрастанию).
fn fixture(specs: &[(&str, &str, bool)], api_key: Option<&str>) -> Fixture {
    let dir = tempfile::tempdir().unwrap();
    let state =
        Arc::new(AppState::new(dir.path().to_path_buf(), api_key.map(str::to_owned)).unwrap());
    for (source, version, deprecated) in specs {
        let rule: Rule = parse_rule(source).unwrap();
        publish_and_merge(state.published_repo(), source, version);
        if *deprecated {
            credo2::deprecate(state.published_repo(), &rule.name, version, "устарела").unwrap();
        }
    }
    let cache = ServiceCache::load(state.published_repo().to_path_buf()).unwrap();
    Fixture {
        _dir: dir,
        app: credo2::rest::app(state, cache),
    }
}

async fn send(
    app: &Router,
    method: &str,
    uri: &str,
    body: Option<Value>,
    key: Option<&str>,
) -> (StatusCode, Value) {
    let mut req = Request::builder().method(method).uri(uri);
    if let Some(k) = key {
        req = req.header("x-api-key", k);
    }
    let body = match body {
        Some(v) => {
            req = req.header("content-type", "application/json");
            Body::from(v.to_string())
        }
        None => Body::empty(),
    };
    let resp = app.clone().oneshot(req.body(body).unwrap()).await.unwrap();
    let status = resp.status();
    let bytes = axum::body::to_bytes(resp.into_body(), usize::MAX)
        .await
        .unwrap();
    let value = if bytes.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&bytes).unwrap()
    };
    (status, value)
}

/// Проверяет конверт Q23 у error-ответа и возвращает `message`.
fn assert_error(body: &Value, expected_code: &str) -> String {
    let err = body
        .get("error")
        .unwrap_or_else(|| panic!("нет error в ответе: {body}"))
        .as_object()
        .unwrap_or_else(|| panic!("error — не объект (плоский конверт Q23?): {body}"));
    let mut keys: Vec<&str> = err.keys().map(|k| k.as_str()).collect();
    keys.sort_unstable();
    assert_eq!(
        keys,
        ["code", "message"],
        "конверт Q23: только code/message"
    );
    assert_eq!(err["code"], expected_code, "код ошибки: {}", err["code"]);
    let message = err["message"]
        .as_str()
        .unwrap_or_else(|| panic!("message — не строка: {body}"))
        .to_string();
    assert!(!message.is_empty(), "message пуст");
    assert!(
        message
            .chars()
            .any(|ch| ('А'..='я').contains(&ch) || matches!(ch, 'ё' | 'Ё')),
        "message не русский (Q11): {message}"
    );
    message
}

#[tokio::test]
async fn get_checks_returns_q21_manifest() {
    let f = fixture(
        &[
            (AGE_RULE, "1.0.0", false),
            (AGE_RULE, "1.0.1", false),
            (AGE_RULE, "1.1.0", true),
        ],
        None,
    );
    let (status, body) = send(&f.app, "GET", "/checks", None, None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["schema_version"], json!(1));
    assert_eq!(body["count"], json!(1));
    let entry = &body["checks"][0];
    assert_eq!(entry["name"], "CreditAgeMin");
    assert_eq!(entry["active"], "1.0.1");
    assert_eq!(entry["supported"], json!(["1.0.1", "1.0.0"]));
    assert_eq!(entry["deprecated"], json!(["1.1.0"]));
}

#[tokio::test]
async fn active_version_is_evaluated_ok() {
    let f = fixture(
        &[(AGE_RULE, "1.0.0", false), (AGE_RULE, "1.0.1", false)],
        None,
    );
    let (status, body) = send(
        &f.app,
        "POST",
        "/checks/CreditAgeMin/evaluate",
        Some(json!({"Клиент.Возраст": 19})),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["version"], "1.0.1");
    assert_eq!(body["active"], json!(true));
    assert_eq!(body["result"]["decision"], "Отказ");
}

#[tokio::test]
async fn supported_version_after_deprecation_still_ok() {
    let f = fixture(
        &[(AGE_RULE, "1.0.0", false), (AGE_RULE, "1.1.0", true)],
        None,
    );
    let (status, body) = send(
        &f.app,
        "POST",
        "/checks/CreditAgeMin/versions/1.0.0/evaluate",
        Some(json!({"Клиент.Возраст": 19})),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["version"], "1.0.0");
}

#[tokio::test]
async fn deprecated_version_is_gone_410() {
    let f = fixture(
        &[(AGE_RULE, "1.0.0", false), (AGE_RULE, "1.1.0", true)],
        None,
    );
    let (status, body) = send(
        &f.app,
        "POST",
        "/checks/CreditAgeMin/versions/1.1.0/evaluate",
        Some(json!({"Клиент.Возраст": 19})),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::GONE);
    let msg = assert_error(&body, "version_deprecated");
    assert_eq!(msg, "версия выведена из эксплуатации: CreditAgeMin@1.1.0");
}

#[tokio::test]
async fn all_deprecated_gives_409_activation_unavailable() {
    let f = fixture(&[(AGE_RULE, "1.0.0", true)], None);
    let (status, body) = send(
        &f.app,
        "POST",
        "/checks/CreditAgeMin/evaluate",
        Some(json!({"Клиент.Возраст": 19})),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::CONFLICT);
    let msg = assert_error(&body, "activation_unavailable");
    assert_eq!(msg, "активация недоступна: CreditAgeMin");
}

#[tokio::test]
async fn unknown_check_is_404_check_not_found() {
    let f = fixture(&[(AGE_RULE, "1.0.0", false)], None);
    let (status, body) = send(
        &f.app,
        "POST",
        "/checks/Nope/evaluate",
        Some(json!({"Клиент.Возраст": 19})),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(
        assert_error(&body, "check_not_found"),
        "проверка не найдена: Nope"
    );

    let (status, body) = send(&f.app, "GET", "/checks/Nope/versions", None, None).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(
        assert_error(&body, "check_not_found"),
        "проверка не найдена: Nope"
    );
}

#[tokio::test]
async fn unknown_version_is_404_version_not_found() {
    let f = fixture(&[(AGE_RULE, "1.0.0", false)], None);
    let (status, body) = send(
        &f.app,
        "POST",
        "/checks/CreditAgeMin/versions/9.9.9/evaluate",
        Some(json!({"Клиент.Возраст": 19})),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(
        assert_error(&body, "version_not_found"),
        "версия не найдена: CreditAgeMin@9.9.9"
    );

    let (status, body) = send(
        &f.app,
        "GET",
        "/checks/CreditAgeMin/versions/9.9.9",
        None,
        None,
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(
        assert_error(&body, "version_not_found"),
        "версия не найдена: CreditAgeMin@9.9.9"
    );
}

#[tokio::test]
async fn missing_field_is_422_evaluation_failed() {
    let f = fixture(&[(AGE_RULE, "1.0.0", false)], None);
    let (status, body) = send(
        &f.app,
        "POST",
        "/checks/CreditAgeMin/versions/1.0.0/evaluate",
        Some(json!({})),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    let msg = assert_error(&body, "evaluation_failed");
    assert!(msg.contains("Неизвестное поле"), "msg = {msg}");
}

#[tokio::test]
async fn unauthorized_401_uses_q23_envelope() {
    let f = fixture(&[(AGE_RULE, "1.0.0", false)], Some("test-key"));

    let (status, body) = send(&f.app, "GET", "/checks", None, None).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert_eq!(
        assert_error(&body, "unauthorized"),
        "ошибка аутентификации: неверный или отсутствующий x-api-key"
    );

    // Открытые пути (Q22) и доступ с верным ключом.
    let (status, _) = send(&f.app, "GET", "/health", None, None).await;
    assert_eq!(status, StatusCode::OK);
    let (status, _) = send(&f.app, "GET", "/checks", None, Some("test-key")).await;
    assert_eq!(status, StatusCode::OK);
}

/// Задача 4 (Q20/Q21/Q23): канонический путь, схема запроса из контракта,
/// конверт `ErrorResponse`, ответы 409/410; «сторож» Q26 — импорта/экспорта нет.
#[tokio::test]
async fn openapi_has_canonical_paths_error_schema_and_no_import_export() {
    let f = fixture(
        &[(AGE_RULE, "1.0.0", false), (AGE_RULE, "1.1.0", true)],
        None,
    );
    let (status, body) = send(&f.app, "GET", "/openapi.json", None, None).await;
    assert_eq!(status, StatusCode::OK);

    // Канонический путь и схема запроса из контракта.
    let op = &body["paths"]["/checks/CreditAgeMin/versions/1.0.0/evaluate"]["post"];
    assert!(op.is_object(), "нет канонического пути: {op}");
    let props = &op["requestBody"]["content"]["application/json"]["schema"]["properties"];
    assert!(
        props["Клиент.Возраст"].is_object(),
        "нет схемы запроса: {props}"
    );

    // Q23: общий конверт и его привязка к ответам.
    let error_schema = &body["components"]["schemas"]["ErrorResponse"];
    assert_eq!(
        error_schema["properties"]["error"]["required"],
        json!(["code", "message"])
    );
    for code in ["404", "422", "401"] {
        assert_eq!(
            op["responses"][code]["content"]["application/json"]["schema"]["$ref"],
            "#/components/schemas/ErrorResponse",
            "ответ {code} без конверта Q23"
        );
    }

    // deprecated-версия: 410 описан; active-путь: 409 описан.
    let dep = &body["paths"]["/checks/CreditAgeMin/versions/1.1.0/evaluate"]["post"];
    assert_eq!(dep["deprecated"], json!(true));
    assert!(dep["responses"]["410"].is_object(), "нет 410: {dep}");
    assert!(
        body["paths"]["/checks/CreditAgeMin/evaluate"]["post"]["responses"]["409"].is_object(),
        "нет 409 у active-пути"
    );

    // info.version = service_hash манифеста.
    let (_, checks) = send(&f.app, "GET", "/checks", None, None).await;
    assert_eq!(body["info"]["version"], checks["service_hash"]);

    // Q26: импорта/экспорта нет — не развиваем в MVP.
    let paths = body["paths"].as_object().unwrap();
    assert!(
        paths
            .keys()
            .all(|p| !p.contains("export") && !p.contains("import")),
        "в OpenAPI появились import/export: {:?}",
        paths.keys().collect::<Vec<_>>()
    );
    assert!(!body.to_string().contains("text/dar"));
}

/// «Сторож» Q23: ни один error-ответ не возвращает старый плоский вид
/// `{"error": "<строка>"}`.
#[tokio::test]
async fn error_envelope_is_never_flat_q23() {
    let f = fixture(&[(AGE_RULE, "1.0.0", true)], Some("test-key"));
    let cases: &[(&str, &str, Option<Value>, Option<&str>)] = &[
        // 401 (ключ не передан)
        ("GET", "/checks", None, None),
        // 404 check_not_found
        ("GET", "/checks/Nope/versions", None, Some("test-key")),
        // 404 version_not_found
        (
            "POST",
            "/checks/CreditAgeMin/versions/9.9.9/evaluate",
            Some(json!({"Клиент.Возраст": 19})),
            Some("test-key"),
        ),
        // 410 version_deprecated
        (
            "POST",
            "/checks/CreditAgeMin/versions/1.0.0/evaluate",
            Some(json!({"Клиент.Возраст": 19})),
            Some("test-key"),
        ),
        // 409 activation_unavailable
        (
            "POST",
            "/checks/CreditAgeMin/evaluate",
            Some(json!({"Клиент.Возраст": 19})),
            Some("test-key"),
        ),
    ];
    for (method, uri, body, key) in cases {
        let (status, value) = send(&f.app, method, uri, body.clone(), *key).await;
        assert!(status.is_client_error(), "{method} {uri}: {status}");
        assert!(
            value["error"].is_object(),
            "{method} {uri}: плоский конверт Q23: {value}"
        );
        assert!(
            value["error"]["code"].is_string(),
            "{method} {uri}: нет code"
        );
        assert!(
            value["error"]["message"].is_string(),
            "{method} {uri}: нет message"
        );
    }
}
