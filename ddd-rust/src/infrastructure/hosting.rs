use std::collections::HashMap;

use crossbeam_channel::Sender;
use tokio::task::JoinSet;
use tokio_util::sync::CancellationToken;

use crate::domain;

pub fn build_tokio_runtime() -> Result<tokio::runtime::Runtime, domain::errors::AppError> {
    Ok(tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .map_err(|e| domain::errors::AppError::Fatal {
            error: domain::errors::FatalError(e.to_string()),
        })?)
}

pub async fn tokio_run_internal(
    cancel_token: CancellationToken,
    event_tx: Sender<domain::events::SystemEvent>,
    tasks: Vec<domain::TaskDefinition>,
) -> Result<(), domain::errors::AppError> {
    let mut set = JoinSet::new();
    let mut task_names = HashMap::new();

    // 1. Initialize and spawn all background tasks
    for task in tasks {
        let token = cancel_token.clone();
        let handle = set.spawn((task.factory)(token));
        task_names.insert(handle.id(), task.name);
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
            Some(result) = set.join_next_with_id() => {
                match result {
                    Ok((id, task_result)) => {
                        let task_name = task_names.remove(&id).unwrap_or_else(|| "Unknown".into());

                        match task_result {
                            Ok(()) => {
                                tracing::info!(
                                    task = %task_name,
                                    "[Orchestrator] completed successfully. Remaining: {}", set.len());
                                let _ = event_tx.send(domain::events::SystemEvent::TaskCompleted {
                                    task_name: task_name.to_string(),
                                });
                            }

                            Err(domain::errors::AppError::Fatal{error}) => {
                                tracing::error!(
                                    task = %task_name,
                                    "Fatal error detected: {}. Escalating...", error);
                                let _ = event_tx.send(domain::events::SystemEvent::TaskFatalError {
                                    task_name: task_name.to_string(),
                                    error: error.clone(),
                                });
                                cancel_token.cancel(); // Trigger ripple shutdown
                                break;
                            }

                            Err(domain::errors::AppError::Recoverable{error}) => {
                                tracing::warn!(
                                    task = %task_name,
                                    "Recoverable error: {}", error)
                            }
                        }
                    }

                    Err(join_err) => {
                        let id = join_err.id();
                        let task_name = task_names.remove(&id).unwrap_or_else(|| "Unknown".into());
                        if join_err.is_panic() {
                            tracing::error!(
                                task = %task_name,
                                "panic detected! Escalating...");
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
