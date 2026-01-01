use crossbeam_channel::Sender;
use tokio::task::JoinSet;
use tokio_util::sync::CancellationToken;

use crate::{AppError, SystemEvent, TaskFactory};

mod axum_worker;
pub use axum_worker::start_axum_server;

pub async fn tokio_run_internal(
    cancel_token: CancellationToken,
    event_tx: Sender<SystemEvent>,
    factories: Vec<TaskFactory>,
) -> Result<(), AppError> {
    let mut set = JoinSet::new();

    // 1. Initialize and spawn all background tasks
    for factory in factories {
        let token = cancel_token.clone();
        set.spawn(factory(token));
    }

    // 2. Primary Event Loop (Orchestration Phase)
    loop {
        tokio::select! {
            // Monitor global shutdown signal from main thread or signal handler
            _ = cancel_token.cancelled() => {
                tracing::info!("[Orchestrator] Shutdown signal received. Transitioning to Draining Phase.");
                break;
            }

            // Monitor task execution status and fatal errors
            Some(result) = set.join_next() => {
                match result {
                    Ok(Ok(())) => tracing::debug!("Task completed successfully."),
                    Ok(Err(AppError::Fatal(e))) => {
                        tracing::error!("Fatal error detected: {}. Escalating...", e);
                        let _ = event_tx.send(SystemEvent::TaskFatalError {
                            task_name: "Service".into(),
                            error: AppError::Fatal(e.clone())
                        });
                        cancel_token.cancel(); // Trigger ripple shutdown
                        break;
                    }
                    Ok(Err(AppError::Recoverable(msg))) => tracing::warn!("Recoverable error: {}", msg),
                    Err(join_err) => {
                        if join_err.is_panic() {
                            tracing::error!("Task panic detected! Escalating...");
                            cancel_token.cancel();
                            break;
                        }
                    }
                }
            }
        }
    }

    // 3. Draining Phase with Hard Timeout (The Architect's Safety Net)
    // We give downstream tasks a window (e.g., 10s) to shut down gracefully.
    tracing::info!(
        "[Orchestrator] Draining {} remaining tasks with 10s timeout...",
        set.len()
    );

    let shutdown_timeout = tokio::time::sleep(std::time::Duration::from_secs(10));
    tokio::pin!(shutdown_timeout);

    loop {
        if set.is_empty() {
            tracing::info!("[Orchestrator] All tasks drained gracefully.");
            break;
        }

        tokio::select! {
            // Continue collecting finished tasks
            Some(res) = set.join_next() => {
                if let Ok(Err(e)) = res {
                    tracing::error!("[Cleanup] Task exited with error: {:?}", e);
                }
            }
            // If tasks take too long, force abort everything
            _ = &mut shutdown_timeout => {
                tracing::warn!("[Orchestrator] Shutdown timeout reached. Forcing abort on {} tasks.", set.len());
                set.abort_all();
                break;
            }
        }
    }

    tracing::info!("[Orchestrator] Runtime orchestrator exited.");
    Ok(())
}
