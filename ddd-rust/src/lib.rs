use std::future::Future;
use std::pin::Pin;

use crossbeam_channel::Sender;

use tokio_util::sync::CancellationToken;

mod tokio_workers;
pub use tokio_workers::tokio_run_internal;
use tokio_workers::{ddd_rust_entry, start_axum_server};

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Fatal system error: {0}")]
    Fatal(String),

    #[error("Recoverable task error: {0}")]
    Recoverable(String),
}

pub enum SystemEvent {
    TaskFatalError { task_name: String, error: AppError },
    TaskRecovered { task_name: String },
    ShutdownTriggered,
}

pub type TaskResult = Result<(), AppError>;
pub type BoxedFuture = Pin<Box<dyn Future<Output = TaskResult> + Send>>;
pub type TaskFactory = Box<dyn Fn(CancellationToken) -> BoxedFuture + Send>;

pub fn create_axum_factory(port: u16) -> TaskFactory {
    Box::new(move |token| Box::pin(start_axum_server(token, port)))
}

pub fn create_monitoring_factory() -> TaskFactory {
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

pub fn create_ddd_rust_entry_factory() -> TaskFactory {
    Box::new(move |token| Box::pin(ddd_rust_entry(token)))
}

pub fn create_signal_handler_factory(event_tx: Sender<SystemEvent>) -> TaskFactory {
    Box::new(move |_token| {
        let tx = event_tx.clone();
        Box::pin(async move {
            // Tokio's built-in signal listener
            if tokio::signal::ctrl_c().await.is_ok() {
                tracing::info!("[Signal] Ctrl+C detected");
                let _ = tx.send(SystemEvent::ShutdownTriggered);
            }
            Ok(())
        })
    })
}
