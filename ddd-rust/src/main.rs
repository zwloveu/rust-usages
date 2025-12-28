use ddd_rust::{TaskFactory, ddd_rust_entry, tokio_run};

fn main() {
    let mut tasks: Vec<TaskFactory> = Vec::new();

    tasks.push(Box::new(|token| Box::pin(ddd_rust_entry(token))));
    // add another task to check Application error
    // tasks.push(Box::new(|token| Box::pin(ddd_rust_entry(token))));

    if let Err(e) = tokio_run(tasks) {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}
