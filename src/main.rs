use anyhow::Result;
use clap::Parser;
use credo2::{mcp, publish, rest, service, store::AppState};
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

const DEFAULT_REST_ADDR: &str = "127.0.0.1:8080";

#[derive(Parser, Debug)]
#[command(name = "credo2", version, about = "CREDO: MCP-сервер с опциональным REST")]
struct Cli {
    /// Поднять REST (адрес по умолчанию 127.0.0.1:8080)
    #[arg(long)]
    rest: bool,

    /// Только MCP, без REST (перекрывает --rest и --addr)
    #[arg(long, conflicts_with_all = ["rest", "addr"])]
    no_rest: bool,

    /// Адрес REST-сервера (включает REST)
    #[arg(long, value_name = "ADDR")]
    addr: Option<SocketAddr>,

    /// Рабочая директория (по умолчанию — текущая)
    #[arg(long, value_name = "PATH")]
    workspace: Option<PathBuf>,

    /// API-ключ REST; можно не указывать, если задан CREDO_API_KEY
    #[arg(long, env = "CREDO_API_KEY", hide_env_values = true)]
    api_key: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .with_writer(std::io::stderr)
        .with_target(false)
        .init();

    let workspace = cli
        .workspace
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_default());

    // Приоритет: --no-rest > --addr > --rest > (ничего → MCP only)
    let rest_addr: Option<SocketAddr> = if cli.no_rest {
        None
    } else if let Some(addr) = cli.addr {
        Some(addr)
    } else if cli.rest {
        Some(DEFAULT_REST_ADDR.parse().expect("valid default addr"))
    } else {
        None
    };

    let state = Arc::new(AppState::new(workspace, cli.api_key)?);
    publish::ensure_repo(state.published_repo())?;

    let cache = service::ServiceCache::load(state.published_repo().to_path_buf())?;
    cache.start_watcher(Duration::from_secs(2));

    tracing::info!(sandbox = %state.sandbox_path().display(), "sandbox");
    tracing::info!(published = %state.published_repo().display(), "published repo");

    let rest_handle = if let Some(addr) = rest_addr {
        tracing::info!(rest = %addr, "CREDO starting (REST enabled)");
        let rest_state = state.clone();
        let rest_cache = cache.clone();
        Some(tokio::spawn(async move {
            if let Err(e) = rest::serve(rest_state, rest_cache, addr).await {
                tracing::warn!("REST stopped: {e}");
            }
        }))
    } else {
        tracing::info!("CREDO starting (REST disabled; use --rest or --addr to enable)");
        None
    };

    let mcp_result = mcp::run_stdio(state, cache).await;
    if let Some(h) = rest_handle {
        h.abort();
    }
    mcp_result
}