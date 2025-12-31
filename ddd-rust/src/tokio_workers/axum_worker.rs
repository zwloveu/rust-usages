use axum::{Router, routing::get};
use tokio_util::sync::CancellationToken;

use crate::TaskResult;

pub async fn start_axum_server(id: String, token: CancellationToken) -> TaskResult {
    println!(
        "[{id}] dependencies construction completed | thread :{:?}",
        std::thread::current().id()
    );

    let app = Router::new()
        .route(
            "/",
            get(|| async { "✅ Axum Worker is running (9527 port)" }),
        )
        .route("/ping", get(|| async { "pong" }));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:9527").await?;

    println!("[{id}] stars at 9527");

    axum::serve(listener, app)
        .with_graceful_shutdown(async move {
            token.cancelled().await;
            println!("[{id}] received cancellation signal and begin to shutdown");
            // drop(db_conn_clone); // drop resources explicitly
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            println!("[{id}] shutdown completed");
        })
        .await
        .map_err(|e| e.into())
}
