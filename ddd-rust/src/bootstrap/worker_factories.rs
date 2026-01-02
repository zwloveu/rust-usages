use tokio_util::sync::CancellationToken;

use crate::adapter::health_checks;
use crate::adapter::http as adapter_http;
use crate::adapter::http::RouterExt;
use crate::adapter::long_runnings;
use crate::domain;

pub fn create_axum_factory(port: u16) -> domain::TaskFactory {
    let router = adapter_http::new_router()
        .into_assembly()
        .apply(adapter_http::register_ping_routes)
        .register_middlewares()
        .finalize();

    Box::new(move |token| {
        let router_clone = router.clone();

        Box::pin(adapter_http::start_axum_server(token, port, router_clone))
    })
}

pub fn create_monitoring_factory() -> domain::TaskFactory {
    Box::new(move |token: CancellationToken| {
        Box::pin(async move {
            tracing::info!("[Task] Monitoring service started.");

            let mut interval = tokio::time::interval(std::time::Duration::from_secs(5));

            loop {
                tokio::select! {
                    _ = token.cancelled() => {
                        tracing::info!("[Task] Monitoring service stopping...");
                        break;
                    }
                    _ = interval.tick() => {
                        // Simulate a monitoring check
                        match health_checks::perform_health_check().await {
                            Ok(_) => tracing::info!("Health check passed"),
                            Err(e) => tracing::warn!("Health check failed: {}", e),
                        }
                    }
                }
            }
            Ok(())
        })
    })
}

pub fn create_ddd_rust_entry_factory() -> domain::TaskFactory {
    Box::new(move |token| Box::pin(long_runnings::ddd_rust_entry(token)))
}
