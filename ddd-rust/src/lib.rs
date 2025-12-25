use std::pin::Pin;
use std::thread;
use std::{error::Error, future::Future};

use tokio::task::JoinSet;
use tokio::time::Instant;

pub async fn run<F, E>(f: F) -> Result<(), Box<dyn Error + Send + Sync>>
where
    F: Future<Output = Result<(), E>> + Send,
    E: Into<Box<dyn Error + Send + Sync>>,
{
    f.await.map_err(|e| e.into())?;
    Ok(())
}

// fn do_something_wrong() -> Result<(), Box<dyn Error>> {
//     Err("something went wrong".into())
// }

pub async fn ddd_rust_entry() -> Result<(), Box<dyn Error + Send + Sync>> {
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

    Ok(())
}

async fn async_add(a: i32, b: i32) -> i32 {
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    a + b
}

fn get_async_add_future() -> impl Future<Output = i32> + Send {
    async_add(1, 2)
}
