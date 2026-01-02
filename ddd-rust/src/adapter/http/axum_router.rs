use axum::{Router, routing::get};

pub fn new_router() -> Router {
    Router::new()
        .route(
            "/",
            get(|| async { "✅ Axum Worker is running (9527 port)" }),
        )
        .layer(axum::middleware::from_fn(simple_logging_middleware))
}

async fn simple_logging_middleware(
    req: axum::extract::Request,
    next: axum::middleware::Next,
) -> impl axum::response::IntoResponse {
    let path = req.uri().path().to_owned();
    let res = next.run(req).await;
    tracing::info!("request: {}, stats code: {}", path, res.status());
    res
}
