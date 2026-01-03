use std::sync::Arc;

use tokio_util::sync::CancellationToken;

use crate::adapter::health_checks;
use crate::adapter::http as adapter_http;
use crate::adapter::http::RouterExt;
use crate::adapter::long_runnings;
use crate::adapter::short_livings;
use crate::domain;

pub fn create_axum_factory(port: u16) -> domain::TaskDefinition {
    let router = adapter_http::new_router()
        .into_assembly()
        .apply(adapter_http::register_ping_routes)
        .register_middlewares()
        .finalize();

    let task_name = Arc::from("task_long_running_axum_web_api");
    domain::TaskDefinition {
        name: Arc::clone(&task_name),
        factory: Box::new(move |token| {
            let router_clone = router.clone();

            Box::pin(adapter_http::start_axum_server(
                Arc::clone(&task_name),
                token,
                port,
                router_clone,
            ))
        }),
    }
}

pub fn create_monitoring_factory() -> domain::TaskDefinition {
    let task_name = Arc::from("task_long_running_health_check");
    domain::TaskDefinition {
        name: Arc::clone(&task_name),
        factory: Box::new({
            // clone here to move the Arc to closure
            let task_name = Arc::clone(&task_name);

            move |token: CancellationToken| {
                // clone here again to move the Arc to Future
                let task_name = Arc::clone(&task_name);

                Box::pin(async move {
                    tracing::info!(
                    task = %task_name,
                    "service started.");

                    let mut interval = tokio::time::interval(std::time::Duration::from_secs(5));

                    loop {
                        tokio::select! {
                            _ = token.cancelled() => {
                                tracing::info!(
                                    task = %task_name,
                                    "service stopping...");
                                break;
                            }
                            _ = interval.tick() => {
                                // Simulate a monitoring check
                                match health_checks::perform_health_check().await {
                                    Ok(_) => tracing::info!(
                                        task = %task_name,
                                        "Health check passed"),
                                    Err(e) => tracing::warn!(
                                        task = %task_name,
                                        "Health check failed: {}", e),
                                }
                            }
                        }
                    }
                    Ok(())
                })
            }
        }),
    }
}

pub fn create_ddd_rust_entry_factory() -> domain::TaskDefinition {
    let task_name = Arc::from("task_long_running_ddd_entry");
    domain::TaskDefinition {
        name: Arc::clone(&task_name),
        factory: Box::new(move |token| {
            Box::pin(long_runnings::ddd_rust_entry(Arc::clone(&task_name), token))
        }),
    }
}

pub fn create_load_test_factory(
    url: String,
    concurrency: usize,
    rounds: usize,
    timeout: u64,
) -> domain::TaskDefinition {
    domain::TaskDefinition {
        name: Arc::from("task_short_living_load_test"),
        factory: Box::new(move |token| {
            let u = url.clone();
            Box::pin(short_livings::run_load_test(
                token,
                u,
                concurrency,
                rounds,
                timeout,
            ))
        }),
    }
}
