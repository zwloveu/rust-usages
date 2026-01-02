use clap::Parser;
use ddd_rust::bootstrap;
use ddd_rust::domain;
use std::process::ExitCode;

fn main() -> ExitCode {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    let args = domain::args::ApiTestArgs::parse();

    if let Err(err) = bootstrap::run_ddd_rust_sample_api_client(&args) {
        tracing::error!(error = %err, "Application startup error");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
