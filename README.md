# dev-server-proxy (Rust Rewrite)

A lightweight HTTP proxy server for web development, written in Rust. This is a rewrite of the original Node.js/TypeScript version for better performance and minimal dependencies.

## Features

- **Nginx-like location matching** - Support for exact, prefix, and regex-based URL matching
- **Path rewriting** - Flexible path transformation using regex patterns
- **HTML content injection** - Inject scripts or styles into HTML responses
- **WebSocket support** - Optional WebSocket proxy configuration
- **Logging** - Comprehensive request logging to file and console
- **Configuration** - JSON or TOML configuration files

## Building

```bash
cargo build --release
```

The binary will be located at `target/release/dsp`

## Installation

```bash
cargo install --path .
```

## Usage

### Start proxy server

```bash
dsp proxy.config.json
dsp start proxy.config.json --port 3000
```

### View logs

```bash
dsp log
dsp log --lines 50
```

### Help

```bash
dsp help
```

## Configuration

Create a `proxy.config.json` file:

```json
{
  "server_name": "https://dev-example.com",
  "default_target": "http://localhost:8080",
  "port": 8000,
  "websocket": {
    "target": "wss://ws.example.com"
  },
  "locations": [
    {
      "rule": "= /",
      "target": "http://localhost:8080/index.html",
      "inject": "<script>window.API_BASE='/api'</script>"
    },
    {
      "rule": "^~ /api",
      "target": "http://localhost:5000",
      "path_rewrite": {
        "^/api": ""
      }
    },
    {
      "rule": "~* \\.(js|css)$",
      "target": "http://localhost:8080"
    },
    {
      "rule": "/",
      "target": "http://localhost:8080"
    }
  ]
}
```

### Configuration Options

- **server_name** - Virtual server name (for identification)
- **default_target** - Default proxy target URL
- **port** - Port to listen on (default: 8080)
- **websocket** - Optional WebSocket proxy configuration
- **locations** - Array of location matching rules

### Location Matching Rules

Rules are processed in order following nginx priority:

1. **Exact match**: `= /path` - Match exact path
2. **Prefix strict**: `^~ /prefix` - Match prefix with highest priority
3. **Regex sensitive**: `~ pattern` - Match regex (case-sensitive)
4. **Regex insensitive**: `~* pattern` - Match regex (case-insensitive)
5. **Prefix match**: `/path` or `/` - Match prefix (lowest priority)

## Development

### Run tests

```bash
cargo test
```

### Run with debug logging

```bash
RUST_LOG=debug cargo run -- proxy.config.json
```

## Performance

Compared to the Node.js version:
- ✅ Significantly lower memory footprint
- ✅ Better throughput with async/await using tokio
- ✅ Single binary - no runtime dependencies
- ✅ Cross-platform compilation support

## Architecture

- **main.rs** - Entry point and CLI setup
- **cli.rs** - Command-line interface and argument parsing
- **config.rs** - Configuration loading and parsing
- **matcher.rs** - Location matching engine
- **proxy.rs** - Proxy request building and response handling
- **rewrite.rs** - Path rewriting logic
- **server.rs** - HTTP server implementation
- **logger.rs** - Logging and event tracking
- **types.rs** - Shared data structures

## License

MIT
