use std::thread;

use crossbeam_channel::{RecvError, unbounded};
use ddd_rust::{
    GlobalCommand, LongRunningWorker, MainThreadCommand, TaskResult, ddd_rust_entry,
    start_axum_server, start_console_input_thread, tokio_run,
};
use tokio::runtime::Runtime;
use tokio_util::sync::CancellationToken;

fn main() -> TaskResult {
    let global_cancel_token = CancellationToken::new();
    let (cmd_tx, cmd_rx) = unbounded::<GlobalCommand>();
    let (main_cmd_sender, main_cmd_receiver) = unbounded::<MainThreadCommand>();

    let rt: Runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;
    let rt_handle = rt.handle().clone();

    let tasks: Vec<LongRunningWorker> = vec![
        LongRunningWorker {
            id: "ddd_entry_worker_task".to_owned(),
            factory: Box::new(|token| Box::pin(ddd_rust_entry(token))),
        },
        LongRunningWorker {
            id: "axum_worker_entry".to_owned(),
            factory: Box::new(|token| {
                Box::pin(start_axum_server("axum_worker_entry".to_owned(), token))
            }),
        },
    ];

    let global_cancel_token_clone = global_cancel_token.clone();
    thread::spawn(move || {
        tokio_run(rt_handle, global_cancel_token_clone, cmd_rx, tasks);
    });

    start_console_input_thread(main_cmd_sender.clone(), cmd_tx.clone());

    loop {
        match main_cmd_receiver.recv() {
            Ok(MainThreadCommand::ShutdownFramework) => {
                global_cancel_token.cancel();
                break;
            }
            Err(RecvError) => {
                global_cancel_token.cancel();
                break;
            }
        }
    }

    Ok(())
}
