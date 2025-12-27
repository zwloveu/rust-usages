use ddd_rust::{TaskFuture, ddd_rust_entry, tokio_run};
// use ddd_rust::AnyError;

fn main() {
    let tasks = vec![
        Box::pin(ddd_rust_entry()) as TaskFuture,
        // add below task to check application error
        // Box::pin(async { Ok::<(), AnyError>(()) }) as TaskFuture,
    ];

    if let Err(e) = tokio_run(tasks) {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}
