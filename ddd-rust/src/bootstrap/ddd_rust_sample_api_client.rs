use crate::{
    bootstrap::{self, worker_factories},
    domain,
};

pub fn run_ddd_rust_sample_api_client(
    args: &domain::args::ApiTestArgs,
) -> Result<(), domain::errors::AppError> {
    let url = args.url.clone();
    let concurrency = args.concurrency;
    let rounds = args.rounds;

    let factories: Vec<domain::TaskFactory> = vec![worker_factories::create_load_test_factory(
        url,
        concurrency,
        rounds,
    )];

    bootstrap::run(factories)
}
