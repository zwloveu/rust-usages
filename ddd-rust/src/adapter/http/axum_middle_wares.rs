pub async fn simple_logging(
    req: axum::extract::Request,
    next: axum::middleware::Next,
) -> impl axum::response::IntoResponse {
    let path = req.uri().path().to_owned();
    let res = next.run(req).await;
    tracing::info!("request: {}, stats code: {}", path, res.status());
    res
}
