use crate::{
    bootstrap::{self, worker_factories},
    domain,
};

pub fn run_ddd_rust() -> Result<(), domain::errors::AppError> {
    let factories: Vec<domain::TaskDefinition> = vec![
        worker_factories::create_ddd_rust_entry_factory(),
        worker_factories::create_monitoring_factory(),
        worker_factories::create_axum_factory(9527),
    ];

    bootstrap::run(factories)
}
