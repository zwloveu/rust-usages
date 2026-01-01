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

    for factory in factories {
        let token = cancel_token.clone();
        set.spawn(factory(token));
    }

    loop {
        tokio::select! {
            // Handle Global Cancellation
            _ = cancel_token.cancelled() => {
                set.abort_all();
                return Ok(());
            }

            // Monitor individual task results
            Some(result) = set.join_next() => {
                match result {
                    Ok(Ok(())) => {
                        tracing::info!("A task finished naturally.");
                    }
                    Ok(Err(err)) => {
                        match err {
                            AppError::Fatal(_) => {
                                // ESCALATE: Return error to break block_on and reach the thread exit
                                return Err(err);
                            }
                            AppError::Recoverable(msg) => {
                                // DIGEST: Report but don't stop the whole system
                                let _ = event_tx.send(SystemEvent::TaskRecovered { task_name: msg });
                            }
                        }
                    }
                    Err(join_err) => {
                        // PANIC: Treat as fatal to ensure system integrity
                        if join_err.is_panic() {
                            return Err(AppError::Fatal("Internal Task Panic".to_string()));
                        }
                    }
                }
            }
        }

        if set.is_empty() {
            break;
        }
    }

    Ok(())
}
