use crate::domain;
use std::sync::Arc;
use tokio::sync::{Semaphore, mpsc};
use tokio::task::JoinSet;
use tokio::time::Instant;
use tokio_util::sync::CancellationToken;

use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Cell, Color, Table};

pub async fn run_load_test(
    token: CancellationToken,
    url: String,
    concurrency: usize,
    rounds: usize,
    timeout: u64,
) -> Result<(), domain::errors::AppError> {
    tracing::info!(
        "[LoadTester] Start: URL={}, Concurrency={}, Round={}",
        url,
        concurrency,
        rounds
    );

    let client = reqwest::Client::builder()
        .pool_max_idle_per_host(concurrency)
        .connect_timeout(std::time::Duration::from_millis(timeout))
        .build()
        .map_err(|e| domain::errors::AppError::Fatal {
            error: domain::errors::FatalError(format!("failed to build reqwest client: {}", e)),
        })?;

    let semaphore = Arc::new(Semaphore::new(concurrency));

    let (tx, mut rx) = mpsc::channel(concurrency * 2);

    let stats_task = tokio::spawn(async move {
        let mut success_count = 0;
        let mut error_map = std::collections::HashMap::new();
        let mut latencies = Vec::new();

        while let Some((status, duration)) = rx.recv().await {
            if status >= 200 && status < 300 {
                success_count += 1;
                if let Some(d) = duration {
                    latencies.push(d);
                }
            } else {
                *error_map.entry(status).or_insert(0) += 1;
            }
        }
        (success_count, error_map, latencies)
    });

    let mut set = JoinSet::new();

    for i in 0..rounds {
        tokio::select! {
            _ = token.cancelled() => {
                tracing::warn!("[LoadTester] received stop signal, aborting dispatch.");
                break;
            }
            permit = semaphore.clone().acquire_owned() => {
                let permit = permit.map_err(|e| domain::errors::AppError::Fatal{error: domain::errors::FatalError(e.to_string())})?;
                let c = client.clone();
                let u = url.clone();
                let tx_clone = tx.clone();

                set.spawn(async move {
                    let _permit = permit;
                    let start = Instant::now();
                    let res = c.get(&u).send().await;
                    let elapsed = start.elapsed();

                    match res {
                        Ok(resp) => {
                            let status = resp.status().as_u16();
                            let _ = tx_clone.send((status, Some(elapsed))).await;
                            tracing::info!(
                                target: "load_tester_task",
                                index = i,
                                status = status,
                                elapsed_ms = elapsed.as_millis(),
                                "Request processed"
                            );
                        },
                        Err(e) => {
                            let _ = tx_clone.send((0, None)).await;
                            tracing::error!(
                                target: "load_tester_task",
                                index = i,
                                error = %e,
                                "Request failed"
                            );
                        }
                    }
                });
            }
        }
    }

    while let Some(_) = set.join_next().await {}

    drop(tx);

    let (success_count, error_map, latencies) =
        stats_task
            .await
            .map_err(|e| domain::errors::AppError::Fatal {
                error: domain::errors::FatalError(format!(
                    "failed to run stats_task in load test: {}",
                    e
                )),
            })?;

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
