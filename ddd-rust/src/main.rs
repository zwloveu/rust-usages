use ddd_rust::{TaskFactory, axum_worker_entry, ddd_rust_entry, tokio_run};

fn main() {
    let mut tasks: Vec<TaskFactory> = Vec::new();

    tasks.push(Box::new(|token| Box::pin(ddd_rust_entry(token))));
    tasks.push(Box::new(|token| Box::pin(axum_worker_entry(token))));

    if let Err(e) = tokio_run(tasks) {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}
