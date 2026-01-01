use std::future::Future;
use std::pin::Pin;
use std::thread;

use crossbeam_channel::Sender;
use tokio::task::JoinSet;
use tokio::time::Instant;

use tokio_util::sync::CancellationToken;

mod tokio_workers;
use tokio_workers::start_axum_server;
pub use tokio_workers::tokio_run_internal;

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
            println!("[Task] Monitoring service started.");

            let mut interval = tokio::time::interval(std::time::Duration::from_secs(5));

            loop {
                tokio::select! {
                    _ = token.cancelled() => {
                        println!("[Task] Monitoring service stopping...");
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

async fn ddd_rust_entry(token: CancellationToken) -> TaskResult {
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

pub fn create_signal_handler_factory(event_tx: Sender<SystemEvent>) -> TaskFactory {
    Box::new(move |_token| {
        let tx = event_tx.clone();
        Box::pin(async move {
            // Tokio's built-in signal listener
            if tokio::signal::ctrl_c().await.is_ok() {
                println!("\n[Signal] Ctrl+C detected");
                let _ = tx.send(SystemEvent::ShutdownTriggered);
            }
            Ok(())
        })
    })
}
