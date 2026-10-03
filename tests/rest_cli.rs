//! T-24: интеграционный тест CLI-контура REST на реальном бинарнике
//! (Q22/[`D26`], Q27/[`D27`]).
//!
//! Существующие `tests/rest.rs` собирают `AppState`/`Router` напрямую и не
//! покрывают запуск бинарника: флаги `--rest`/`--addr`/`--no-rest` и способы
//! задания ключа (`--api-key` и env `CREDO_API_KEY`). Здесь поднимается
//! настоящий процесс `env!("CARGO_BIN_EXE_credo2")`, готовность REST
//! проверяется поллингом `GET /health`, а запросы идут по TCP.
//!
//! [`D26`]: ../docs/decisions/D26-rest-auth-x-api-key.md
//! [`D27`]: ../docs/decisions/D27-rest-launch-address.md

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::process::{Child, ChildStdin, Command, Stdio};
use std::time::{Duration, Instant};

const EXE: &str = env!("CARGO_BIN_EXE_credo2");
const KEY: &str = "s3cret-key";
/// Канонический текст 401 (Q22/D26, `rest_auth.feature`).
const UNAUTHORIZED_MSG: &str =
    "ошибка аутентификации: неверный или отсутствующий x-api-key";

/// Запущенный `credo2` с REST на свободном порту.
///
/// `stdin` остаётся открытым: MCP-транспорт читает stdio, и при EOF процесс
/// штатно завершается, гася REST. Дескриптор держим до `Drop`.
struct RestServer {
    child: Child,
    _stdin: Option<ChildStdin>,
    port: u16,
    _dir: tempfile::TempDir,
}

impl Drop for RestServer {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

impl RestServer {
    /// Поллинг `GET /health` до готовности (D26: путь открыт без ключа).
    fn wait_ready(&self) {
        let deadline = Instant::now() + Duration::from_secs(20);
        loop {
            if let Ok((200, body)) = request(self.port, "GET", "/health", None)
                && body.contains("ok")
            {
                return;
            }
            assert!(
                Instant::now() < deadline,
                "REST не ответил на GET /health за 20 с (порт {})",
                self.port
            );
            std::thread::sleep(Duration::from_millis(100));
        }
    }
}

/// Свободный loopback-порт: слушатель закрывается, номер отдаётся серверу.
fn free_port() -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind 127.0.0.1:0");
    listener.local_addr().expect("local_addr").port()
}

/// Поднимает REST: `--rest` + `--addr 127.0.0.1:<свободный порт>`; ключ —
/// флагом (`--api-key`) либо переменной окружения (`CREDO_API_KEY`), либо
/// отсутствует (режим демо).
fn start_rest(
    api_key_arg: Option<&str>,
    api_key_env: Option<&str>,
) -> RestServer {
    assert!(
        api_key_arg.is_none() || api_key_env.is_none(),
        "ключ задаётся либо флагом, либо env — не обоими"
    );
    let dir = tempfile::tempdir().expect("tempdir");
    let port = free_port();
    let mut cmd = Command::new(EXE);
    cmd.arg("--workspace")
        .arg(dir.path())
        .arg("--rest")
        .arg("--addr")
        .arg(format!("127.0.0.1:{port}"))
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .env_remove("CREDO_API_KEY");
    if let Some(key) = api_key_arg {
        cmd.arg("--api-key").arg(key);
    }
    if let Some(key) = api_key_env {
        cmd.env("CREDO_API_KEY", key);
    }
    let mut child = cmd.spawn().expect("spawn credo2");
    let stdin = child.stdin.take();
    let server = RestServer {
        child,
        _stdin: stdin,
        port,
        _dir: dir,
    };
    server.wait_ready();
    server
}

/// Один HTTP/1.1 запрос по TCP; возвращает `(статус, тело)`.
fn request(
    port: u16,
    method: &str,
    path: &str,
    api_key: Option<&str>,
) -> std::io::Result<(u16, String)> {
    let mut stream = TcpStream::connect(("127.0.0.1", port))?;
    stream.set_read_timeout(Some(Duration::from_secs(10)))?;
    stream.set_write_timeout(Some(Duration::from_secs(10)))?;
    let mut req = format!(
        "{method} {path} HTTP/1.1\r\nHost: 127.0.0.1:{port}\r\nConnection: close\r\n"
    );
    if let Some(key) = api_key {
        req.push_str(&format!("x-api-key: {key}\r\n"));
    }
    req.push_str("\r\n");
    stream.write_all(req.as_bytes())?;
    let mut raw = Vec::new();
    stream.read_to_end(&mut raw)?;
    let text = String::from_utf8_lossy(&raw);
    let status = text
        .lines()
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
        .and_then(|code| code.parse().ok())
        .unwrap_or(0);
    let body = text
        .split_once("\r\n\r\n")
        .map(|(_, body)| body.to_string())
        .unwrap_or_default();
    Ok((status, body))
}

fn json_body(body: &str) -> serde_json::Value {
    serde_json::from_str(body)
        .unwrap_or_else(|e| panic!("тело не JSON ({e}): {body}"))
}

/// Конверт Q23/D26 для 401: `code = unauthorized`, канонический русский текст.
fn assert_unauthorized(body: &str) {
    let value = json_body(body);
    assert_eq!(
        value["error"]["code"], "unauthorized",
        "конверт Q23: {value}"
    );
    assert_eq!(value["error"]["message"], UNAUTHORIZED_MSG, "текст 401 Q22");
}

/// Карточка T-24, «Запуск и готовность»: `--rest` + `--addr` поднимают REST;
/// `GET /health` отвечает 200.
#[test]
fn rest_starts_on_addr_and_health_is_open() {
    let server = start_rest(None, None);
    let (status, body) =
        request(server.port, "GET", "/health", None).expect("GET /health");
    assert_eq!(status, 200, "/health: {body}");
    assert_eq!(json_body(&body)["status"], "ok", "/health: {body}");
}

/// Карточка T-24, «Ключ через `--api-key`»: без заголовка и с неверным
/// заголовком — 401 (конверт Q23), с корректным — 200.
#[test]
fn api_key_flag_401_without_header_and_200_with_it() {
    let server = start_rest(Some(KEY), None);

    let (status, body) =
        request(server.port, "GET", "/checks", None).expect("GET /checks");
    assert_eq!(status, 401, "без x-api-key: {body}");
    assert_unauthorized(&body);

    let (status, body) = request(server.port, "GET", "/checks", Some("wrong"))
        .expect("GET /checks (неверный ключ)");
    assert_eq!(status, 401, "неверный x-api-key: {body}");
    assert_unauthorized(&body);

    let (status, body) = request(server.port, "GET", "/checks", Some(KEY))
        .expect("GET /checks (верный ключ)");
    assert_eq!(status, 200, "верный x-api-key: {body}");
    assert_eq!(json_body(&body)["schema_version"], 1, "тело: {body}");
}

/// Карточка T-24, «Ключ через env `CREDO_API_KEY`»: тот же контракт, что и
/// у флага (401 без заголовка, 200 с ним).
#[test]
fn api_key_env_follows_same_contract() {
    let server = start_rest(None, Some(KEY));

    let (status, body) =
        request(server.port, "GET", "/checks", None).expect("GET /checks");
    assert_eq!(status, 401, "env-ключ, без x-api-key: {body}");
    assert_unauthorized(&body);

    let (status, body) = request(server.port, "GET", "/checks", Some(KEY))
        .expect("GET /checks (верный ключ)");
    assert_eq!(status, 200, "env-ключ, верный x-api-key: {body}");
}

/// Карточка T-24, «Открытый режим»: без ключа (ни флага, ни env) API открыт
/// (демо) — запрос без заголовка отвечает 200.
#[test]
fn without_key_api_is_open_demo_mode() {
    let server = start_rest(None, None);
    let (status, body) =
        request(server.port, "GET", "/checks", None).expect("GET /checks");
    assert_eq!(status, 200, "режим демо без ключа: {body}");
    // Пустой `main` в свежем workspace — манифест без проверок.
    assert_eq!(json_body(&body)["count"], 0, "тело: {body}");
}

/// D26 (`rest_auth.feature`): `/health`, `/docs`, `/openapi.json` доступны
/// без ключа даже при заданном `CREDO_API_KEY`.
#[test]
fn open_paths_need_no_key() {
    let server = start_rest(Some(KEY), None);
    for path in ["/health", "/docs", "/openapi.json"] {
        let (status, body) = request(server.port, "GET", path, None)
            .expect("GET открытого пути");
        assert_eq!(status, 200, "{path} без ключа (D26): {body}");
    }
}

/// Карточка T-24, «Конфликт флагов»: `--no-rest` вместе с `--rest`/`--addr`
/// отклоняется `clap` (D27: приоритет, конфликт объявлен явно).
#[test]
fn no_rest_conflicts_with_rest_flags() {
    for conflicting in ["--rest", "--addr"] {
        let mut cmd = Command::new(EXE);
        cmd.arg("--no-rest");
        if conflicting == "--addr" {
            cmd.arg("--addr").arg("127.0.0.1:0");
        } else {
            cmd.arg("--rest");
        }
        let out = cmd.output().expect("spawn credo2");
        assert!(
            !out.status.success(),
            "--no-rest + {conflicting} должен быть отклонён clap"
        );
        let stderr = String::from_utf8_lossy(&out.stderr);
        assert!(
            stderr.contains("--no-rest") && stderr.contains(conflicting),
            "отказ clap не упоминает конфликтующие флаги: {stderr}"
        );
    }
}
