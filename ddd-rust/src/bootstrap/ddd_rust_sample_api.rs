use crate::{
    bootstrap::{self, worker_factories},
    domain,
};

pub fn run_ddd_rust_sample_api() -> Result<(), domain::errors::AppError> {
    let factories: Vec<domain::TaskFactory> = vec![
        worker_factories::create_monitoring_factory(),
        worker_factories::create_axum_factory(9527),
    ];

    bootstrap::run(factories)
}
