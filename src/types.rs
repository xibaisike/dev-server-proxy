use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Location matching rule type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleType {
    /// Exact match: = /path
    Exact,
    /// Prefix match with highest priority: ^~ /prefix
    PrefixStrict,
    /// Regex match (case-sensitive): ~ regex
    RegexSensitive,
    /// Regex match (case-insensitive): ~* regex
    RegexInsensitive,
    /// Default prefix match: /
    Prefix,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    /// Nginx-like matching rule (e.g., "= /", "^~ /api", "~ .*\.js$")
    pub rule: String,
    /// Target proxy URL
    pub target: String,
    /// Optional HTML content injection
    #[serde(default)]
    pub inject: Option<String>,
    /// Optional path rewrite rules (regex pattern -> replacement)
    #[serde(default)]
    pub path_rewrite: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketConfig {
    pub target: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Virtual server name (for identification)
    pub server_name: String,
    /// Default proxy target for unmatched requests
    pub default_target: String,
    /// Port to listen on
    #[serde(default = "default_port")]
    pub port: u16,
    /// Optional WebSocket proxy configuration
    #[serde(default)]
    pub websocket: Option<WebSocketConfig>,
    /// Location rules array
    #[serde(default)]
    pub locations: Vec<Location>,
}

fn default_port() -> u16 {
    8080
}

impl Config {
    pub fn load_from_file(path: &std::path::Path) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Config = serde_json::from_str(&content)
            .or_else(|_| toml::from_str(&content))?;
        Ok(config)
    }
}

/// Log event for tracking proxy operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEvent {
    pub timestamp: String,
    pub event: String,
    pub req_path: String,
    pub proxy_path: String,
    pub status: u16,
}
