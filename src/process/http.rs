use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Router,
};
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tracing::{info, warn};

#[derive(Debug)]
struct HttpServeState {
    path: PathBuf,
}

pub async fn process_http_serve(path: PathBuf, port: u16) -> Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Serving {} on {}", path.display(), addr);

    let state = HttpServeState { path };
    // axum router code here
    let router = Router::new().route("/*pp", get(file_handler).with_state(Arc::new(state)));

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}

// with_state可以帮助handler线程安全地访问状态
// State(state) 采用 pattern-match写法
async fn file_handler(
    State(state): State<Arc<HttpServeState>>,
    Path(pp): Path<String>,
) -> (StatusCode, String) {
    let p = state.path.join(pp);
    info!("Reading file: {}", p.display());
    if !p.exists() {
        (
            StatusCode::NOT_FOUND,
            format!("File {} not found", p.display()),
        )
    } else {
        match tokio::fs::read_to_string(p).await {
            Ok(content) => {
                info!("Read {} bytes from file", content.len());
                (StatusCode::OK, content)
            }
            Err(e) => {
                warn!("Failed to read file: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            }
        }
    }
}
