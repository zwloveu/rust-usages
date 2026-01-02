use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about = "client to test axum interface")]
pub struct ApiTestArgs {
    #[arg(short, long)]
    pub url: String,

    #[arg(short, long, default_value_t = 10)]
    pub concurrency: usize,

    #[arg(short, long, default_value_t = 10000)]
    pub rounds: usize,

    #[arg(short, long, default_value_t = 1000)]
    pub timeout: u64,
}
