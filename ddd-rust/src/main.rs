use std::thread;

use crossbeam_channel::unbounded;
use ddd_rust::{
    AppError, SystemEvent, TaskFactory, create_axum_factory, create_ddd_rust_entry_factory,
    create_monitoring_factory, create_signal_handler_factory, tokio_run_internal,
};
use tokio::runtime::Runtime;
use tokio_util::sync::CancellationToken;

fn main() -> Result<(), AppError> {
    // 1. [Infrastructure] Create the Runtime at the very top of the stack
    // This ensures the runtime is the last thing to be dropped
    let rt: Runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .map_err(|e| AppError::Fatal(e.to_string()))?;

    // 2. [Communication] Initialize Crossbeam for Sync-Async bridge
    let (event_tx, event_rx) = unbounded::<SystemEvent>();
    let global_cancel_token = CancellationToken::new();

    // 3. [Task Definitions] Example: Multiple factories
    let factories: Vec<TaskFactory> = vec![
        create_ddd_rust_entry_factory(),
        create_monitoring_factory(),
        create_axum_factory(9527),
        create_signal_handler_factory(event_tx.clone()),
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
                if let Err(e) = tokio_run_internal(token, tx, factories).await {
                    // This is reached if tokio_run_internal hits a Fatal error
                    eprintln!("[Runtime Host] Fatal error escalated: {:?}", e);
                    Err(e)
                } else {
                    Ok(())
                }
            })
        })
    };

    // 5. [Orchestration] Main Thread Loop (Reactive Controller)
    println!("[Main] System Controller started.");
    loop {
        crossbeam_channel::select! {
            // Listen for events from the Async world
            recv(event_rx) -> event => {
                match event {
                    Ok(SystemEvent::TaskFatalError { task_name, error }) => {
                        eprintln!("[Main] Critical failure in {}: {}. Initiating shutdown...", task_name, error);
                        global_cancel_token.cancel();
                        break;
                    }
                    Ok(SystemEvent::ShutdownTriggered) => break,
                    _ => {}
                }
            }

            // Non-blocking check for thread health
            default(std::time::Duration::from_millis(200)) => {
                if manager_thread.is_finished() {
                    println!("[Main] Manager thread exited unexpectedly.");
                    break;
                }
            }
        }
    }

    // 6. [Graceful Exit]
    println!("[Main] Cleaning up resources...");
    global_cancel_token.cancel();
    let _ = manager_thread.join();

    // Once we exit main, 'rt' is dropped, closing all remaining async tasks
    println!("[Main] Shutdown complete.");
    Ok(())
}
