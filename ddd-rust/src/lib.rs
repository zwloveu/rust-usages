use std::future::Future;
use std::pin::Pin;
use std::thread;
use std::time::Duration;

use axum::Router;
use axum::routing::get;
use tokio::runtime::Runtime;
use tokio::task::{JoinHandle, JoinSet};
use tokio::time::Instant;

use tokio_util::sync::CancellationToken;

pub type AnyError = Box<dyn std::error::Error + Send + Sync>;
pub type TaskResult = Result<(), AnyError>;
pub type BoxedFuture = Pin<Box<dyn Future<Output = TaskResult> + Send>>;
pub type TaskFactory = Box<dyn FnOnce(CancellationToken) -> BoxedFuture + Send>;
pub struct TaskDefinition {
    pub id: String,
    pub factory: TaskFactory,
}

pub fn tokio_run(task_definitions: Vec<TaskDefinition>) -> TaskResult {
    // 1, build multiple thread tokio runtime
    let rt: Runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;

    // 2, create cancellation token
    let cancel_token = CancellationToken::new();

    let mut handles: Vec<JoinHandle<()>> = Vec::new();

    // 3, spawn all tasks
    for task_definition in task_definitions {
        let task_token = cancel_token.clone();

        let task_future = (task_definition.factory)(task_token);

        handles.push(rt.spawn(async move {
            match run_worker(task_definition.id.to_owned(), task_future, 300).await {
                Ok(_) => println!(
                    "[{:?}] Task exited cleanly | thread: {:?}",
                    task_definition.id,
                    std::thread::current().id()
                ),
                Err(e) => eprintln!(
                    "[{:?}] Task error: {:?} | thread: {:?}",
                    task_definition.id,
                    e,
                    std::thread::current().id()
                ),
            }
        }));
    }

    // 4. main thread block here: listening CTRL+C
    println!(
        "[Main] Main thread waits for CTRL+C | thread: {:?}",
        std::thread::current().id()
    );
    rt.block_on(async {
        println!(
            "[Monitor] Press Ctrl+C to shut down | thread: {:?}",
            std::thread::current().id()
        );

        tokio::signal::ctrl_c()
            .await
            .map_err(|e| Box::new(e) as AnyError)?;

        println!(
            "\n[Monitor] Ctrl+C signal detected | thread: {:?}",
            std::thread::current().id()
        ); // This will print immediately upon signal
        Ok::<(), AnyError>(())
    })?;

    // 5. Received CTRL+C signals, send broadcast to every backgroud task to end
    // here main thread exists from the CTRL+C block_on
    println!(
        "[Main] received CTRL+C signals, send broadcast to every backgroud task | thread: {:?}",
        std::thread::current().id()
    );
    cancel_token.cancel();

    // 6. wait all tasks to complete, make sure they can really drop
    rt.block_on(async {
        println!(
            "[Main] Waiting for all background handles to finish | thread: {:?}",
            std::thread::current().id()
        );
        let _ = tokio::time::timeout(
            std::time::Duration::from_secs(5),
            futures::future::join_all(handles),
        )
        .await;
        println!(
            "[Main] All handles joined | thread: {:?}",
            std::thread::current().id()
        );
    });

    // 7. Hard Stop: Final cleanup of any hung or unmanaged tasks
    // This ensures the process definitely terminates.
    rt.shutdown_timeout(Duration::from_secs(1));

    println!(
        "[Main] main thread exited | thread: {:?}",
        std::thread::current().id()
    );
    Ok(())
}

pub async fn run_worker(worker_id: String, f: BoxedFuture, timeout_secs: u64) -> TaskResult {
    println!(
        "[{worker_id}] task begins to run，time limits {timeout_secs} seconds | thread: {:?}",
        std::thread::current().id()
    );

    let start = std::time::Instant::now();
    let res = tokio::time::timeout(std::time::Duration::from_secs(timeout_secs), f).await;

    let duration = start.elapsed();

    match res {
        Ok(Ok(_)) => {
            println!(
                "[{worker_id}] task completed，take: {:?} | thread: {:?}",
                duration,
                std::thread::current().id()
            );
            Ok(())
        }
        Ok(Err(e)) => {
            eprintln!(
                "[{worker_id}] task throws logic err: {}, take: {:?} | thread: {:?}",
                e,
                duration,
                std::thread::current().id()
            );
            Err(e)
        }
        Err(_) => {
            eprintln!(
                "[{worker_id}] task forced to shutdown due to timeout, take: {:?} | thread: {:?}",
                duration,
                std::thread::current().id()
            );
            Err("task timeout".into())
        }
    }
}
// fn do_something_wrong() -> Result<(), Box<dyn Error>> {
//     Err("something went wrong".into())
// }

pub async fn ddd_rust_entry(token: CancellationToken) -> TaskResult {
    println!("[ddd_rust_entry] started");

    let mut interval = tokio::time::interval(std::time::Duration::from_secs(10));

    loop {
        tokio::select! {
            // listening ctrl+c
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

pub async fn axum_worker_entry(token: CancellationToken) -> TaskResult {
    let app = Router::new().route("/health", get(|| async { "OK" }));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:9527").await?;

    println!("[axum_worker_entry] stars at 9527");

    axum::serve(listener, app)
        .with_graceful_shutdown(async move {
            token.cancelled().await;
            println!("[axum_worker_entry] is shutting down");
        })
        .await
        .map_err(|e| e.into())
}
