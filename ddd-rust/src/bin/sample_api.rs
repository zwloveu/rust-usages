use ddd_rust::bootstrap;
use std::process::ExitCode;

fn main() -> ExitCode {
    bootstrap::register_tracing_subscriber();

    if let Err(err) = bootstrap::run_ddd_rust_sample_api() {
        tracing::error!(error = %err, "Application startup error");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
