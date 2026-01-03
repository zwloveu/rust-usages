use crate::{
    bootstrap::{self, worker_factories},
    domain,
};

pub fn run_ddd_rust_sample_api_client(
    args: &domain::args::ApiTestArgs,
) -> Result<(), domain::errors::AppError> {
    let factories: Vec<domain::TaskDefinition> = vec![worker_factories::create_load_test_factory(
        args.url.clone(),
        args.concurrency,
        args.rounds,
        args.timeout,
    )];

    bootstrap::run(factories)
}
