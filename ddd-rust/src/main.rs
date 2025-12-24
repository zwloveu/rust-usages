use std::future::Future;
use std::pin::Pin;
use std::thread;
use tokio::{self, runtime::Runtime, time::Instant};

//#[tokio::main]
async fn async_main() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    let tasks: Vec<_> = (0..=100000)
        .map(|i| async move {
            tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;

            if i % 10000 == 0 {
                println!(
                    "Task {} done | thread id: {:?} | elapsed: {}ms",
                    i,
                    thread::current().id(),
                    start.elapsed().as_millis()
                );
            }
        })
        .collect();

    futures::future::join_all(tasks).await;
    println!(
        "[{}ms] All tasks done, thread id: {:?}",
        start.elapsed().as_millis(),
        thread::current().id()
    );

    let async_add_futures: Vec<Pin<Box<dyn Future<Output = i32>>>> =
        vec![Box::pin(async_add(1, 2)), Box::pin(get_async_add_future())];
    let results: Vec<i32> = futures::future::join_all(async_add_futures).await;
    println!("[{}ms] {:?}", start.elapsed().as_millis(), results);

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let runtime: Runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .build()?;

    runtime.block_on(async_main())?;

    Ok(())
}

async fn async_add(a: i32, b: i32) -> i32 {
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    a + b
}

fn get_async_add_future() -> impl Future<Output = i32> {
    async_add(1, 2)
}
