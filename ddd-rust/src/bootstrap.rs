use std::sync::Arc;
use std::thread;

use crossbeam_channel::Sender;
use crossbeam_channel::unbounded;

use tokio_util::sync::CancellationToken;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::registry;
use tracing_subscriber::util::SubscriberInitExt;

use crate::domain;
use crate::infrastructure;

mod ddd_rust;
pub use ddd_rust::run_ddd_rust;

mod ddd_rust_sample_api;
pub use ddd_rust_sample_api::run_ddd_rust_sample_api;

mod ddd_rust_sample_api_client;
pub use ddd_rust_sample_api_client::run_ddd_rust_sample_api_client;

mod worker_factories;

pub fn register_tracing_subscriber() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let fmt_layer = fmt::layer()
        .with_target(true)
        .with_thread_ids(true)
        .with_line_number(true);

    registry().with(filter).with(fmt_layer).init();
}

fn run(tasks: Vec<domain::TaskDefinition>) -> Result<(), domain::errors::AppError> {
    // 1. [Infrastructure] Create the Runtime at the very top of the stack
    // This ensures the runtime is the last thing to be dropped
    let rt = infrastructure::build_tokio_runtime()?;

    // 2. [Communication] Initialize Crossbeam for Sync-Async bridge
    let (event_tx, event_rx) = unbounded::<domain::events::SystemEvent>();
    let global_cancel_token = CancellationToken::new();

    // 3. [Task Definitions] Example: Multiple factories
    let mut all_tasks = tasks;
    all_tasks.push(create_signal_handler_factory(event_tx.clone()));

    let mut current_active_count = all_tasks.len();

    // 4. [Execution] Spawn the Manager Thread
    // The Runtime stays in main, the Handle goes into the thread
    let manager_thread = {
        let rt_handle = rt.handle().clone();
        let token = global_cancel_token.clone();
        let tx = event_tx.clone();

        thread::spawn(move || {
            // Transform this OS thread into a dedicated Runtime Worker
            rt_handle.block_on(async move {
                match infrastructure::tokio_run_internal(token, tx, all_tasks).await {
                    Err(e) => {
                        tracing::error!("[Runtime Host] Fatal error escalated: {}", e);
                        Err(e)
                    }
                    Ok(()) => Ok(()),
                }
            })
        })
    };

    // 5. [Orchestration] Main Thread Loop (Reactive Controller)
    tracing::info!("[Main] System Controller started.");
    loop {
        if current_active_count <= 1 {
            tracing::info!("[Main] All tasks completed.");
            global_cancel_token.cancel();
            break;
        }

        crossbeam_channel::select! {
            // Listen for events from the Async world
            recv(event_rx) -> event => {
                match event {
                    Ok(domain::events::SystemEvent::TaskCompleted { task_name }) => {
                        tracing::info!("[Main] Task '{}' finished.", task_name);
                        current_active_count -= 1;
                    }

                    Ok(domain::events::SystemEvent::TaskFatalError { task_name, error }) => {
                        tracing::error!("[Main] Critical failure in {}: {}. Initiating shutdown...", task_name, error);
                        global_cancel_token.cancel();
                        break;
                    }

                    Ok(domain::events::SystemEvent::ShutdownTriggered) => break,

                    _ => {}
                }
            }

            // Non-blocking check for thread health
            default(std::time::Duration::from_millis(200)) => {
                if manager_thread.is_finished() {
                    tracing::info!("[Main] Manager thread exited unexpectedly.");
                    break;
                }
            }
        }
    }

    // 6. [Graceful Exit]
    tracing::info!("[Main] Cleaning up resources...");
    global_cancel_token.cancel();
    if let Err(panic_info) = manager_thread.join() {
        tracing::error!(
            "[Main] Manager thread panicked at the last moment: {:?}",
            panic_info
        );
    }

    // Once we exit main, 'rt' is dropped, closing all remaining async tasks
    tracing::info!("[Main] Shutdown complete.");
    Ok(())
}

fn create_signal_handler_factory(
    event_tx: Sender<domain::events::SystemEvent>,
) -> domain::TaskDefinition {
    domain::TaskDefinition {
        name: Arc::from("signal_listener"),
        factory: Box::new(move |_token| {
            let tx = event_tx.clone();
            Box::pin(async move {
                #[cfg(unix)]
                {
                    use tokio::signal::unix::{SignalKind, signal};
                    let mut sigterm = signal(SignalKind::terminate())?;
                    let mut sigint = signal(SignalKind::interrupt())?;

                    tokio::select! {
                        _ = sigterm.recv() => tracing::info!("[Signal] SIGTERM detected"),
                        _ = sigint.recv() => tracing::info!("[Signal] SIGINT (Ctrl+C) detected"),
                        _ = tokio::signal::ctrl_c() => tracing::info!("[Signal] Ctrl+C detected"),
                    };
                }

                #[cfg(not(unix))]
                {
                    let _ = tokio::signal::ctrl_c().await;
                    tracing::info!("[Signal] Ctrl+C detected");
                }

                let _ = tx.send(domain::events::SystemEvent::ShutdownTriggered);

                Ok(())
            })
        }),
    }
}
