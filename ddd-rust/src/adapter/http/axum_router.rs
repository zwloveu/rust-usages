use axum::{Router, routing::get};

pub fn new_router() -> Router {
    Router::new().route(
        "/",
        get(|| async { "✅ Axum Worker is running (9527 port)" }),
    )
}
