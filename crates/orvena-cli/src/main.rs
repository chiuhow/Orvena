//! `orvena` — a thin CLI over `orvena-core`. All real logic lives in the core
//! library; these commands only parse arguments and print results.

mod cli;
mod commands;

#[tokio::main]
async fn main() {
    std::process::exit(cli::run().await);
}
