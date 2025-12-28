use std::future::Future;
use std::pin::Pin;
use std::thread;
use std::time::Duration;

use tokio::runtime::Runtime;
use tokio::task::{JoinHandle, JoinSet};
use tokio::time::Instant;

use tokio_util::sync::CancellationToken;

pub type AnyError = Box<dyn std::error::Error + Send + Sync>;
pub type TaskResult = Result<(), AnyError>;
pub type BoxedFuture = Pin<Box<dyn Future<Output = TaskResult> + Send>>;
pub type TaskFactory = Box<dyn FnOnce(CancellationToken) -> BoxedFuture + Send>;

pub fn tokio_run(task_factories: Vec<TaskFactory>) -> TaskResult {
    // 1, build multiple thread tokio runtime
    let rt: Runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;

    // 2, create cancellation token
    let cancel_token = CancellationToken::new();

    let mut handles: Vec<JoinHandle<()>> = Vec::new();

    // 3, spawn all tasks
    for factory in task_factories {
        let task_token = cancel_token.clone();

        let task_future = factory(task_token);

        handles.push(rt.spawn(async move {
            match run(task_future, 300).await {
                Ok(_) => println!(
                    "[Worker] Task exited cleanly | thread: {:?}",
                    std::thread::current().id()
                ),
                Err(e) => eprintln!(
                    "[Worker] Task error: {:?} | thread: {:?}",
                    e,
                    std::thread::current().id()
                ),
            }
        }));
    }

    if handles.len() > 1 {
        let err: AnyError = "many tasks".into();
        return Err(err);
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

pub async fn run(f: BoxedFuture, timeout_secs: u64) -> TaskResult {
    println!(
        "[Monitor] task begins to run，time limits {} seconds | thread: {:?}",
        timeout_secs,
        std::thread::current().id()
    );

    let start = std::time::Instant::now();
    let res = tokio::time::timeout(std::time::Duration::from_secs(timeout_secs), f).await;

    let duration = start.elapsed();

    match res {
        Ok(Ok(_)) => {
            println!(
                "[Monitor] task completed，take: {:?} | thread: {:?}",
                duration,
                std::thread::current().id()
            );
            Ok(())
        }
        Ok(Err(e)) => {
            eprintln!(
                "[Monitor] task throws logic err: {}, take: {:?} | thread: {:?}",
                e,
                duration,
                std::thread::current().id()
            );
            Err(e)
        }
        Err(_) => {
            eprintln!(
                "[Monitor] task forced to shutdown due to timeout, take: {:?} | thread: {:?}",
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
    println!("[Task] DDD Entry started");

    let mut interval = tokio::time::interval(std::time::Duration::from_secs(10));

    loop {
        tokio::select! {
            // listening ctrl+c
            _ = token.cancelled() => {
                println!("[Task] DDD Entry received ctrl_c, preparing drop and exit");
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
                                "[{}ms] | Task {} done | thread: {:?}",
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
                    "[{}ms] All tasks done, thread id: {:?}",
                    start.elapsed().as_millis(),
                    thread::current().id()
                );

                let async_add_futures: Vec<Pin<Box<dyn Future<Output = i32> + Send>>> =
                    vec![Box::pin(async_add(1, 2)), Box::pin(get_async_add_future())];
                let results: Vec<i32> = futures::future::join_all(async_add_futures).await;
                println!("[{}ms] {:?}", start.elapsed().as_millis(), results);
            }
        }
    }

    println!("[Task] DDD Entry dropped");

    Ok(())
}

async fn async_add(a: i32, b: i32) -> i32 {
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    a + b
}

fn get_async_add_future() -> impl Future<Output = i32> + Send {
    async_add(1, 2)
}
