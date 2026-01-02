use std::time::Duration;

use tokio::time::Instant;
use tokio_util::sync::CancellationToken;

use futures::StreamExt;

use crate::domain;

pub async fn ddd_rust_entry(token: CancellationToken) -> domain::TaskResult {
    tracing::info!("[ddd_rust_entry] started");

    // Independent intervals for different business logic
    let mut batch_interval = tokio::time::interval(Duration::from_secs(10));
    let mut math_interval = tokio::time::interval(Duration::from_secs(5));

    loop {
        tokio::select! {
            // Global cancellation signal
            _ = token.cancelled() => {
                tracing::info!("[ddd_rust_entry] received shutdown signal, preparing to exit");
                break;
            }

            // Logic 1: 100,000 tasks as a single cancelable unit
            _ = batch_interval.tick() => {
                let start = Instant::now();

                // Create a child token to manage this specific batch of tasks
                let child_token = token.child_token();

                let error_target = rand::random_range(0..=100_000);

                let task_stream = futures::stream::iter(0..=100_000)
                        .map(|i| {
                            let task_token = child_token.clone();
                            tokio::spawn(async move {
                                tokio::select! {
                                    _ = task_token.cancelled() => Ok(()),
                                    _ = tokio::time::sleep(Duration::from_millis(1000)) => {
                                        if i % 10000 == 0 {
                                            tracing::info!(
                                                "[batch_task] [{}ms] | Task {} done | thread: {:?}",
                                                start.elapsed().as_millis(), i, std::thread::current().id()
                                            );
                                        }
                                        if i == error_target {
                                            return Err(format!("Task {} failed manually", i));
                                        }
                                        Ok(())
                                    }
                                }
                            })
                        })
                        .buffer_unordered(5000);

                // Wait for all tasks in the current batch to complete
                let mut stream = task_stream;
                while let Some(res) = stream.next().await {
                    match res {
                        Ok(_) => { /* Task finished successfully */ }
                        Err(e) => {
                            tracing::error!("[batch_task] Business error: {}", e);
                        }
                    }
                }

                tracing::info!(
                    "[batch_task] [{}ms] All batch tasks finished, thread: {:?}",
                    start.elapsed().as_millis(),
                    std::thread::current().id()
                );
            }

            // Logic 2: Math tasks with separate interval
            _ = math_interval.tick() => {
                let start = Instant::now();

                let async_add_futures: Vec<Pin<Box<dyn Future<Output = i32> + Send>>> = vec![
                    Box::pin(async_add(1, 2)),
                    Box::pin(get_async_add_future())
                ];

                // Wrap with select to ensure math tasks also respect the cancellation token
                tokio::select! {
                    _ = token.cancelled() => {
                        tracing::info!("[math_task] cancelled during execution");
                    }
                    results = futures::future::join_all(async_add_futures) => {
                        tracing::info!(
                            "[math_task] [{}ms] results: {:?}",
                            start.elapsed().as_millis(),
                            results
                        );
                    }
                }
            }
        }
    }

    tracing::info!("[ddd_rust_entry] dropped and cleanup finished");
    Ok(())
}

async fn async_add(a: i32, b: i32) -> i32 {
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    a + b
}

fn get_async_add_future() -> impl Future<Output = i32> + Send {
    async_add(1, 2)
}
