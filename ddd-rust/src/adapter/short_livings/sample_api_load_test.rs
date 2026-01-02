use futures::StreamExt;
use reqwest::Client;
use std::time::Duration;
use tokio::time::Instant;
use tokio_util::sync::CancellationToken;

use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Cell, Color, Table};

use crate::domain;

pub async fn run_load_test(
    token: CancellationToken,
    url: String,
    concurrency: usize,
    rounds: usize,
    timeout_ms: u64,
) -> Result<(), domain::errors::AppError> {
    // 1. Build a high-performance HTTP client
    let client = Client::builder()
        // Essential for Windows to bypass the 300ms Delayed ACK / Nagle algorithm latency
        .tcp_nodelay(true)
        .pool_max_idle_per_host(concurrency)
        .pool_idle_timeout(Duration::from_secs(90))
        .connect_timeout(Duration::from_millis(timeout_ms))
        .build()
        .map_err(|e| domain::errors::AppError::Fatal {
            error: domain::errors::FatalError(format!("Failed to build client: {}", e)),
        })?;

    // 2. Connection Pool Warm-up
    // Send initial requests to establish TCP/TLS handshakes before measuring
    // This prevents "cold start" handshakes from polluting P99 metrics
    tracing::info!("Warming up connection pool...");
    for _ in 0..10 {
        let _ = client.get(&url).send().await;
    }

    tracing::info!(
        "[LoadTester] Start: URL={}, Concurrency={}, Total Rounds={}",
        url,
        concurrency,
        rounds
    );

    let start_test = Instant::now();

    // 3. Distribute requests using Stream for efficient concurrency control
    // buffer_unordered is superior to manual Semaphore/JoinSet as it reduces task-switching overhead
    let stats = futures::stream::iter(0..rounds)
        .map(|_| {
            let client = client.clone();
            let url = url.clone();
            let token = token.clone();

            async move {
                if token.is_cancelled() {
                    return None;
                }

                let start = Instant::now();
                let res = client.get(&url).send().await;
                let elapsed = start.elapsed();

                match res {
                    Ok(resp) if resp.status().is_success() => Some(elapsed),
                    _ => {
                        // Failures are excluded from latency statistics but recorded as None
                        None
                    }
                }
            }
        })
        .buffer_unordered(concurrency)
        .collect::<Vec<Option<Duration>>>()
        .await;

    // 4. Data processing and metric aggregation
    let latencies: Vec<Duration> = stats.into_iter().flatten().collect();
    let success_count = latencies.len();
    let total_elapsed = start_test.elapsed();

    print_report(success_count, latencies, total_elapsed);

    Ok(())
}

fn print_report(success: usize, mut latencies: Vec<std::time::Duration>, sum: Duration) {
    if latencies.is_empty() {
        println!("No data collected.");
        return;
    }

    latencies.sort();
    let total = latencies.len();
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
