mod cli;
mod config;
mod logger;
mod matcher;
mod proxy;
mod rewrite;
mod server;
mod types;

use anyhow::Result;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    let args = cli::Args::parse();
    
    // Initialize logging
    logger::init(&args.log_level);
    
    // Run CLI command
    cli::execute(args).await
}
