use std::thread;

use crossbeam_channel::unbounded;

use tokio_util::sync::CancellationToken;

use crate::bootstrap::worker_factories;
use crate::domain;
use crate::infrastructure;

pub fn run_ddd_rust() -> Result<(), domain::AppError> {
    // 1. [Infrastructure] Create the Runtime at the very top of the stack
    // This ensures the runtime is the last thing to be dropped
    let rt = infrastructure::build_tokio_runtime()?;

    // 2. [Communication] Initialize Crossbeam for Sync-Async bridge
    let (event_tx, event_rx) = unbounded::<domain::SystemEvent>();
    let global_cancel_token = CancellationToken::new();

    // 3. [Task Definitions] Example: Multiple factories
    let factories: Vec<domain::TaskFactory> = vec![
        worker_factories::create_ddd_rust_entry_factory(),
        worker_factories::create_monitoring_factory(),
        worker_factories::create_axum_factory(9527),
        worker_factories::create_signal_handler_factory(event_tx.clone()),
    ];

    // 4. [Execution] Spawn the Manager Thread
    // The Runtime stays in main, the Handle goes into the thread
    let manager_thread = {
        let rt_handle = rt.handle().clone();
        let token = global_cancel_token.clone();
        let tx = event_tx.clone();

        thread::spawn(move || {
            // Transform this OS thread into a dedicated Runtime Worker
            rt_handle.block_on(async {
                if let Err(e) = infrastructure::tokio_run_internal(token, tx, factories).await {
                    // This is reached if tokio_run_internal hits a Fatal error
                    tracing::error!("[Runtime Host] Fatal error escalated: {}", e);
                    Err(e)
                } else {
                    Ok(())
                }
            })
        })
    };

    // 5. [Orchestration] Main Thread Loop (Reactive Controller)
    tracing::info!("[Main] System Controller started.");
    loop {
        crossbeam_channel::select! {
            // Listen for events from the Async world
            recv(event_rx) -> event => {
                match event {
                    Ok(domain::SystemEvent::TaskFatalError { task_name, error }) => {
                        tracing::error!("[Main] Critical failure in {}: {}. Initiating shutdown...", task_name, error);
                        global_cancel_token.cancel();
                        break;
                    }
                    Ok(domain::SystemEvent::ShutdownTriggered) => break,
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
