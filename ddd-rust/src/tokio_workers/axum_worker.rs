use axum::{Router, routing::get};
use tokio_util::sync::CancellationToken;

use crate::{AppError, TaskResult};

pub async fn start_axum_server(token: CancellationToken, port: u16) -> TaskResult {
    tracing::info!(
        "[axum_server] dependencies construction completed | thread :{:?}",
        std::thread::current().id()
    );

    let app = Router::new()
        .route(
            "/",
            get(|| async { "✅ Axum Worker is running (9527 port)" }),
        )
        .route("/ping", get(|| async { "pong" }));
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .map_err(|e| AppError::Fatal(format!("Port {} bound failed: {}", port, e)))?;

    tracing::info!("[axum_server] stars at 9527");

    axum::serve(listener, app)
        .with_graceful_shutdown(async move {
            token.cancelled().await;
            tracing::info!("[axum_server] received cancellation signal and begin to shutdown");
            // drop(db_conn_clone); // drop resources explicitly
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            tracing::info!("[axum_server] shutdown completed");
        })
        .await
        .map_err(|e| AppError::Fatal(format!("Axum runtime error: {}", e)))?;

    Ok(())
}
