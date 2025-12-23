use tokio::{self, runtime::Runtime};

//#[tokio::main]
async fn async_main() {
    let tasks: Vec<_> = (0..=100000)
        .map(|i| async move {
            tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
            if i % 10000 == 0 {
                println!("Task {} done", i);
            }
        })
        .collect();

    futures::future::join_all(tasks).await;
    println!("All tasks done");
}

fn main() -> std::io::Result<()> {
    let runtime: Runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;

    runtime.block_on(async_main());

    Ok(())
}
