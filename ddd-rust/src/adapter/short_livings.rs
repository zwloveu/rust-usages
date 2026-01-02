use crate::domain;
use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;
use tokio_util::sync::CancellationToken;

use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Cell, Color, Table};

pub async fn run_load_test(
    token: CancellationToken,
    url: String,
    concurrency: usize,
    rounds: usize,
) -> Result<(), domain::errors::AppError> {
    tracing::info!(
        "[LoadTester] Start: URL={}, Concurrency={}, Round={}",
        url,
        concurrency,
        rounds
    );

    let client = reqwest::Client::builder()
        .pool_max_idle_per_host(concurrency)
        .build()
        .map_err(|e| domain::errors::AppError::Fatal {
            error: domain::errors::FatalError(format!("failed to build reqwest client: {}", e)),
        })?;

    let semaphore = Arc::new(Semaphore::new(concurrency));
    let mut set = JoinSet::new();

    let mut success_count = 0;
    let mut error_map = std::collections::HashMap::new();
    let mut latencies = Vec::with_capacity(rounds);

    for _ in 0..rounds {
        tokio::select! {
            _ = token.cancelled() => {
                tracing::warn!("[LoadTester] received stop signal, aborting dispatch.");
                break;
            }
            permit = semaphore.clone().acquire_owned() => {
                let permit = permit.map_err(|e| domain::errors::AppError::Fatal{error: domain::errors::FatalError(e.to_string())})?;
                let c = client.clone();
                let u = url.clone();

                set.spawn(async move {
                    let _permit = permit;
                    let start = std::time::Instant::now();
                    let res = c.get(u.as_str()).send().await;


                    match res {
                        Ok(resp) => {
                            let (status_code, elapsed) = (resp.status().as_u16(), start.elapsed());

                            tracing::info!(
                                            target: "load_tester_task",
                                            url = %u,
                                            status = status_code,
                                            elapsed_ms = elapsed.as_millis(),
                                            "Request processed"
                                        );

                            (status_code, elapsed)
                        },
                        Err(e) => {
                            let elapsed = start.elapsed();
                                        tracing::error!(
                                            target: "load_tester_task",
                                            url = %u,
                                            error = %e,
                                            elapsed_ms = elapsed.as_millis(),
                                            "Request failed"
                                        );
                                        (0, elapsed)
                        },
                    }
                });
            }
        }
    }

    while let Some(res) = set.join_next().await {
        if let Ok((status, duration)) = res {
            latencies.push(duration);
            if status >= 200 && status < 300 {
                success_count += 1;
            } else {
                *error_map.entry(status).or_insert(0) += 1;
            }
        }
    }

    print_report(success_count, error_map, latencies);
    tracing::info!("[LoadTester] completed");

    Ok(())
}

fn print_report(
    success: usize,
    errors: std::collections::HashMap<u16, usize>,
    mut latencies: Vec<std::time::Duration>,
) {
    if latencies.is_empty() {
        println!("No data collected.");
        return;
    }

    latencies.sort();
    let total = latencies.len();
    let sum: std::time::Duration = latencies.iter().sum();
    let avg = sum / total as u32;
    let p95 = latencies[(total as f64 * 0.95) as usize];
    let p99 = latencies[(total as f64 * 0.99) as usize];

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec![
            Cell::new("Metric").fg(Color::Cyan),
            Cell::new("Value").fg(Color::Cyan),
        ]);

    table.add_row(vec![
        Cell::new("Success (2xx)").fg(Color::Green),
        Cell::new(success.to_string()).fg(Color::Green),
    ]);

    for (code, count) in errors {
        let label = if code == 0 {
            "Error (network)".to_string()
        } else {
            format!("Error ({})", code)
        };
        table.add_row(vec![
            Cell::new(label).fg(Color::Red),
            Cell::new(count.to_string()),
        ]);
    }

    table.add_row(vec![
        Cell::new("Average Latency"),
        Cell::new(format!("{:.4?}", avg)).fg(Color::Yellow),
    ]);
    table.add_row(vec![
        Cell::new("P95 Latency"),
        Cell::new(format!("{:.4?}", p95)).fg(Color::Yellow),
    ]);
    table.add_row(vec![
        Cell::new("P99 Latency"),
        Cell::new(format!("{:.4?}", p99)).fg(Color::Yellow),
    ]);

    println!("{}", table);
}
