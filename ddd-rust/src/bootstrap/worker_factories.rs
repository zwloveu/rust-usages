use crossbeam_channel::Sender;
use tokio_util::sync::CancellationToken;

use crate::adapter::http as adapter_http;
use crate::adapter::long_runnings;
use crate::domain;

pub fn create_axum_factory(port: u16) -> domain::TaskFactory {
    let router = adapter_http::new_router();
    let router = adapter_http::register_ping_routes(router);

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
                        if let Err(e) = perform_health_check().await {
                            // Example of a recoverable error: log and continue
                            tracing::warn!("Minor monitoring glitch: {}", e);
                            // If this were a fatal error, we would 'return Err(AppError::Fatal(...))'
                        }
                    }
                }
            }
            Ok(())
        })
    })
}

async fn perform_health_check() -> Result<(), String> {
    // Logic for checking disk/mem/cpu...
    Ok(())
}

pub fn create_ddd_rust_entry_factory() -> domain::TaskFactory {
    Box::new(move |token| Box::pin(long_runnings::ddd_rust_entry(token)))
}

pub fn create_signal_handler_factory(event_tx: Sender<domain::SystemEvent>) -> domain::TaskFactory {
    Box::new(move |_token| {
        let tx = event_tx.clone();
        Box::pin(async move {
            // Tokio's built-in signal listener
            if tokio::signal::ctrl_c().await.is_ok() {
                tracing::info!("[Signal] Ctrl+C detected");
                let _ = tx.send(domain::SystemEvent::ShutdownTriggered);
            }
            Ok(())
        })
    })
}
