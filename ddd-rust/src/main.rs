use ddd_rust::{ddd_rust_entry, run};
use std::error::Error;
use tokio::runtime::Runtime;

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let runtime: Runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;

    let server_handle = runtime.spawn(run(ddd_rust_entry()));

    let result = runtime.block_on(server_handle)?;

    if let Err(e) = result {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }

    Ok(())
}
