use crate::types::LogEvent;
use chrono::Local;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tracing_subscriber::filter::EnvFilter;

static LOG_FILE: once_cell::sync::Lazy<Arc<Mutex<Option<std::fs::File>>>> =
    once_cell::sync::Lazy::new(|| Arc::new(Mutex::new(None)));

pub fn init(log_level: &str) {
    let filter = match log_level.to_lowercase().as_str() {
        "trace" => EnvFilter::new("trace"),
        "debug" => EnvFilter::new("debug"),
        "info" => EnvFilter::new("info"),
        "warn" => EnvFilter::new("warn"),
        "error" => EnvFilter::new("error"),
        _ => EnvFilter::new("info"),
    };

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .init();

    // Initialize log file
    let log_path = PathBuf::from("proxy.log");
    if let Ok(file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)
    {
        let mut log_file = LOG_FILE.lock().unwrap();
        *log_file = Some(file);
    }
}

pub fn log_event(event: &LogEvent) {
    let line = format!(
        "[{}] {} | req: {} | proxy: {} | status: {}\n",
        event.timestamp, event.event, event.req_path, event.proxy_path, event.status
    );

    if let Ok(mut log_file) = LOG_FILE.lock() {
        if let Some(ref mut file) = *log_file {
            let _ = file.write_all(line.as_bytes());
            let _ = file.flush();
        }
    }

    // Also print to console via tracing
    tracing::info!(
        event = %event.event,
        req_path = %event.req_path,
        proxy_path = %event.proxy_path,
        status = event.status,
    );
}

pub fn format_timestamp() -> String {
    Local::now().to_rfc3339()
}
