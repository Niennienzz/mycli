use std::{net::SocketAddr, path::PathBuf, sync::Arc};

use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Router,
};
use serde::Serialize;
use tera::Context;
use tower_http::services::ServeDir;
use tracing::{info, warn};

use crate::utils::DIRECTORY_LISTING;

pub struct HttpServe;

impl HttpServe {
    pub async fn start(path: PathBuf, port: u16) -> Result<()> {
        let addr = SocketAddr::from(([0, 0, 0, 0], port));
        info!("Serving {:?} on {}", path, addr);

        let state = HttpServeState { path: path.clone() };

        let router = Router::new()
            .nest_service("/tower", ServeDir::new(path))
            .route("/*path", get(file_handler))
            .with_state(Arc::new(state));

        let listener = tokio::net::TcpListener::bind(addr).await?;
        axum::serve(listener, router).await?;
        Ok(())
    }
}

#[derive(Debug, Serialize)]
struct SimpleFile {
    name: String,
    path: String,
}

async fn file_handler(
    State(state): State<Arc<HttpServeState>>,
    Path(path): Path<String>,
) -> (StatusCode, String) {
    const TEMPLATE_NAME: &str = "directory_listing.html";

    let p = std::path::Path::new(&state.path).join(path);
    info!("Reading file {:?}", p);
    if !p.exists() {
        (
            StatusCode::NOT_FOUND,
            format!("File {} note found", p.display()),
        )
    } else {
        // Check if 'p' is a directory.
        if p.is_dir() {
            let entries = tokio::fs::read_dir(p.clone()).await;
            return match entries {
                Ok(mut entries) => {
                    let mut files: Vec<SimpleFile> = Vec::new();
                    while let Ok(Some(entry)) = entries.next_entry().await {
                        let entry_path = entry.path();
                        let file_name = entry
                            .file_name()
                            .into_string()
                            .unwrap_or_else(|_| String::from("Invalid UTF-8"));
                        let file = SimpleFile {
                            name: file_name,
                            path: entry_path.display().to_string(),
                        };
                        files.push(file);
                    }
                    let mut context = Context::new();
                    context.insert("files", &files);
                    match DIRECTORY_LISTING.render(TEMPLATE_NAME, &context) {
                        Ok(content) => {
                            info!("Read {} bytes", content.len());
                            (StatusCode::OK, content)
                        }
                        Err(err) => {
                            warn!("Rendering directory listing: {:?}", err);
                            (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
                        }
                    }
                }
                Err(err) => {
                    warn!("Reading directory: {:?}", err);
                    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
                }
            };
        }
        match tokio::fs::read_to_string(p).await {
            Ok(content) => {
                info!("Read {} bytes", content.len());
                (StatusCode::OK, content)
            }
            Err(err) => {
                warn!("Reading file: {:?}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
            }
        }
    }
}

#[derive(Debug)]
struct HttpServeState {
    path: PathBuf,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_file_handler() {
        let state = Arc::new(HttpServeState {
            path: PathBuf::from("."),
        });
        let (status, content) = file_handler(State(state), Path("Cargo.toml".to_string())).await;
        assert_eq!(status, StatusCode::OK);
        assert!(content.trim().starts_with("[package]"));
    }
}
