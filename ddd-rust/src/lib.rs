use std::future::Future;
use std::io::{self, BufRead};
use std::pin::Pin;
use std::thread;

use crossbeam_channel::Sender;
use tokio::task::JoinSet;
use tokio::time::Instant;

use tokio_util::sync::CancellationToken;

mod tokio_workers;
pub use tokio_workers::start_axum_server;
pub use tokio_workers::tokio_run;

pub type AnyError = Box<dyn std::error::Error + Send + Sync>;
pub type TaskResult = Result<(), AnyError>;
pub type BoxedFuture = Pin<Box<dyn Future<Output = TaskResult> + Send>>;
pub type TaskFactory = Box<dyn Fn(CancellationToken) -> BoxedFuture + Send>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WorkerId {
    AxumWorker,
}

#[derive(Debug, Clone)]
pub enum GlobalCommand {
    RestartWorker(WorkerId),
}

#[derive(Debug, Clone)]
pub enum MainThreadCommand {
    ShutdownFramework,
}

pub struct LongRunningWorker {
    pub id: String,
    pub factory: TaskFactory,
}

pub async fn ddd_rust_entry(token: CancellationToken) -> TaskResult {
    println!("[ddd_rust_entry] started");

    let mut interval = tokio::time::interval(std::time::Duration::from_secs(10));

    loop {
        tokio::select! {
            _ = token.cancelled() => {
                println!("[ddd_rust_entry] received ctrl_c, preparing drop and exit");
                break;
            }
            // business
            _ = interval.tick() => {
                let start = Instant::now();

                let mut set = JoinSet::new();

                for i in 0..=100_000 {
                    set.spawn(async move {
                        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
                        if i % 10000 == 0 {
                            println!(
                                "[ddd_rust_entry] [{}ms] | Task {} done | thread: {:?}",
                                start.elapsed().as_millis(),
                                i,
                                std::thread::current().id()
                            );
                        }
                    });
                }

                // Wait for all tasks in the set to finish
                while let Some(_) = set.join_next().await {}

                println!(
                    "[ddd_rust_entry] [{}ms] All tasks done, thread id: {:?}",
                    start.elapsed().as_millis(),
                    thread::current().id()
                );

                let async_add_futures: Vec<Pin<Box<dyn Future<Output = i32> + Send>>> =
                    vec![Box::pin(async_add(1, 2)), Box::pin(get_async_add_future())];
                let results: Vec<i32> = futures::future::join_all(async_add_futures).await;
                println!("[ddd_rust_entry] [{}ms] {:?}", start.elapsed().as_millis(), results);
            }
        }
    }

    println!("[ddd_rust_entry] dropped");

    Ok(())
}

async fn async_add(a: i32, b: i32) -> i32 {
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    a + b
}

fn get_async_add_future() -> impl Future<Output = i32> + Send {
    async_add(1, 2)
}

pub fn start_console_input_thread(
    main_cmd_sender: Sender<MainThreadCommand>,
    cmd_tx: Sender<GlobalCommand>,
) {
    thread::spawn(move || {
        println!("\n[Terminal] Type：");
        println!("[Terminal] shutdown - exit");
        println!("[Terminal] restart - restart axum server");
        println!("[Terminal] Please type order then press enter：");

        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            match line {
                Ok(input) => {
                    let input = input.trim().to_lowercase();
                    match input.as_str() {
                        "shutdown" => {
                            if let Err(e) =
                                main_cmd_sender.send(MainThreadCommand::ShutdownFramework)
                            {
                                eprintln!("[Terminal] failed to send exit command: {}", e);
                            } else {
                                println!("[Terminal] successfully sent exit command, exiting...");
                                break;
                            }
                        }
                        "restart" => {
                            if let Err(e) =
                                cmd_tx.send(GlobalCommand::RestartWorker(WorkerId::AxumWorker))
                            {
                                eprintln!("[Terminal] failed to send restart command: {}", e);
                            } else {
                                println!(
                                    "[Terminal] successfully sent exit command, restarting..."
                                );
                            }
                        }
                        _ => {
                            println!("[Terminal] command: shutdown or restart");
                        }
                    }
                }
                Err(e) => {
                    eprintln!("[Terminal] failed to read input: {}", e);
                    break;
                }
            }
        }
    });
}
