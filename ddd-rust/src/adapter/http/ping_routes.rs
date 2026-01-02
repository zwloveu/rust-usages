use axum::{Router, routing::get};

pub fn register_ping_routes(router: Router) -> Router {
    router.route("/ping", get(ping_handler))
}

async fn ping_handler() -> &'static str {
    "pong"
}
