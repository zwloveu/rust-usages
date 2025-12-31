use crossbeam_channel::Receiver;
use tokio::runtime::Handle;
use tokio_util::sync::CancellationToken;

use crate::{AnyError, GlobalCommand, LongRunningWorker, TaskResult, WorkerId};

mod axum_worker;
pub use axum_worker::start_axum_server;

pub fn tokio_run(
    rt_handle: Handle,
    global_cancel_token: CancellationToken,
    cmd_rx: Receiver<GlobalCommand>,
    lrws: Vec<LongRunningWorker>,
) {
    for lrw in lrws {
        let worker_global_token = global_cancel_token.clone();
        let worker_cmd_rx = cmd_rx.clone();
        let rt_handle_clone = rt_handle.clone();

        rt_handle_clone.spawn(long_running_worker_entry(
            worker_global_token,
            worker_cmd_rx,
            lrw,
        ));
    }

    loop {
        if global_cancel_token.is_cancelled() {
            break;
        }

        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

pub async fn long_running_worker_entry(
    global_token: CancellationToken,
    cmd_rx: Receiver<GlobalCommand>,
    lrw: LongRunningWorker,
) -> TaskResult {
    let mut service_token = global_token.child_token();
    let task_future = (lrw.factory)(service_token.clone());
    task_future.await?;

    loop {
        if global_token.is_cancelled() {
            break;
        }

        if let Ok(GlobalCommand::RestartWorker(WorkerId::AxumWorker)) = cmd_rx.try_recv() {
            service_token.cancel();
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

            service_token = global_token.child_token();
            let task_future = (lrw.factory)(service_token.clone());
            task_future.await?;
            println!("[{:?}] successfully restarted", lrw.id);
        }

        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }

    Ok::<(), AnyError>(())
}
