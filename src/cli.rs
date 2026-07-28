use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

use crate::config::Config;
use crate::server::start_server;
use crate::logger;

#[derive(Parser, Debug)]
#[command(name = "dsp")]
#[command(about = "Dev Server Proxy - A lightweight HTTP proxy for web development", long_about = None)]
struct CliArgs {
    #[command(subcommand)]
    command: Option<Command>,

    /// Path to config file (used when no subcommand is provided)
    #[arg(value_name = "CONFIG")]
    config: Option<PathBuf>,

    /// Port to listen on (overrides config file)
    #[arg(short, long, value_name = "PORT")]
    port: Option<u16>,

    /// Log level
    #[arg(short, long, value_name = "LEVEL", default_value = "info")]
    log_level: String,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Start proxy server with config file
    Start {
        /// Path to config file
        #[arg(value_name = "CONFIG")]
        config: PathBuf,

        /// Port to listen on
        #[arg(short, long, value_name = "PORT")]
        port: Option<u16>,
    },

    /// Show help information
    Help,

    /// View proxy logs
    Log {
        /// Number of lines to show
        #[arg(short, long, value_name = "LINES", default_value = "100")]
        lines: usize,
    },
}

pub struct Args {
    pub command: Option<Command>,
    pub config: Option<PathBuf>,
    pub port: Option<u16>,
    pub log_level: String,
}

impl Args {
    pub fn parse_from_clap() -> Self {
        let cli = CliArgs::parse();
        Args {
            command: cli.command,
            config: cli.config,
            port: cli.port,
            log_level: cli.log_level,
        }
    }
}

pub fn parse_args() -> Args {
    Args::parse_from_clap()
}

pub async fn execute(args: Args) -> Result<()> {
    match args.command {
        Some(Command::Start { config, port }) => {
            let mut config = Config::load_from_file(&config)?;
            if let Some(port) = port {
                config.port = port;
            }
            start_server(config).await
        }
        Some(Command::Help) => {
            print_help();
            Ok(())
        }
        Some(Command::Log { lines }) => {
            show_logs(lines)?;
            Ok(())
        }
        None => {
            // If no command, try to start with provided config file
            if let Some(config_path) = args.config {
                let mut config = Config::load_from_file(&config_path)?;
                if let Some(port) = args.port {
                    config.port = port;
                }
                start_server(config).await
            } else {
                print_help();
                Ok(())
            }
        }
    }
}

fn print_help() {
    println!(
        r#"dev-server-proxy (dsp) - Web Development Proxy Server

Usage:
  dsp <config.json>              Start proxy server with config file
  dsp start <config.json>        Start proxy server
  dsp log [--lines 100]          View proxy logs
  dsp help                       Show this help message

Options:
  --port <PORT>                  Override listening port
  --log-level <LEVEL>            Set log level (trace, debug, info, warn, error)

Examples:
  dsp proxy.config.json
  dsp start proxy.config.json --port 3000
  dsp log --lines 50
"#
    );
}

fn show_logs(lines: usize) -> Result<()> {
    let log_path = std::path::PathBuf::from("proxy.log");
    if !log_path.exists() {
        println!("Log file not found at {:?}", log_path);
        return Ok(());
    }

    let content = std::fs::read_to_string(&log_path)?;
    let log_lines: Vec<&str> = content.lines().collect();
    
    let start = if log_lines.len() > lines {
        log_lines.len() - lines
    } else {
        0
    };

    for line in &log_lines[start..] {
        println!("{}", line);
    }

    Ok(())
}
