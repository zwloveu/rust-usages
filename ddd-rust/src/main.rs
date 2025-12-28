use ddd_rust::{TaskDefinition, axum_worker_entry, ddd_rust_entry, tokio_run};

fn main() {
    let tasks: Vec<TaskDefinition> = vec![
        TaskDefinition {
            id: "ddd_entry_worker_task".to_owned(),
            factory: Box::new(|token| Box::pin(ddd_rust_entry(token))),
        },
        TaskDefinition {
            id: "axum_worker_entry".to_owned(),
            factory: Box::new(|token| Box::pin(axum_worker_entry(token))),
        },
    ];

    if let Err(e) = tokio_run(tasks) {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}
