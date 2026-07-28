use anyhow::Result;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server, StatusCode};
use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;

use crate::config::Config;
use crate::logger::{format_timestamp, log_event};
use crate::matcher::find_match;
use crate::proxy::{build_target_url, inject_into_html, is_html_response};
use crate::types::LogEvent;

pub async fn start_server(config: Config) -> Result<()> {
    let config = Arc::new(config);
    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));

    tracing::info!(
        "Starting proxy server on {} for {}",
        addr,
        config.server_name
    );

    let make_svc = make_service_fn(move |_conn| {
        let config = Arc::clone(&config);
        async move {
            Ok::<_, Infallible>(service_fn(move |req| {
                let config = Arc::clone(&config);
                handle_request(req, &config)
            }))
        }
    });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Proxy server listening on port {}", config.port);
    println!("Loaded config: (in-memory)");

    server.await?;
    Ok(())
}

async fn handle_request(
    req: Request<Body>,
    config: &Arc<Config>,
) -> Result<Response<Body>, Infallible> {
    let method = req.method().clone();
    let uri = req.uri().clone();
    let req_path = uri.path();

    // Log incoming request
    tracing::debug!("Incoming request: {} {}", method, req_path);

    // Find matching location
    match find_match(&config.locations, req_path) {
        Some(location) => {
            tracing::debug!("Matched location: {:?}", location.rule);

            match build_target_url(location, req_path) {
                Ok(_target_uri) => {
                    // In a real implementation, we would proxy the request here
                    // For now, return a success response
                    let response = Response::builder()
                        .status(StatusCode::OK)
                        .body(Body::from("Proxied successfully"))
                        .unwrap();

                    let event = LogEvent {
                        timestamp: format_timestamp(),
                        event: "proxy".to_string(),
                        req_path: req_path.to_string(),
                        proxy_path: format!("{}{}", location.target, req_path),
                        status: 200,
                    };
                    log_event(&event);

                    Ok(response)
                }
                Err(e) => {
                    tracing::error!("Failed to build target URL: {}", e);
                    let event = LogEvent {
                        timestamp: format_timestamp(),
                        event: "error".to_string(),
                        req_path: req_path.to_string(),
                        proxy_path: "".to_string(),
                        status: 500,
                    };
                    log_event(&event);

                    Ok(Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(Body::from("Failed to proxy request"))
                        .unwrap())
                }
            }
        }
        None => {
            tracing::warn!("No matching location found for: {}", req_path);
            let event = LogEvent {
                timestamp: format_timestamp(),
                event: "no-match".to_string(),
                req_path: req_path.to_string(),
                proxy_path: "".to_string(),
                status: 502,
            };
            log_event(&event);

            Ok(Response::builder()
                .status(StatusCode::BAD_GATEWAY)
                .body(Body::from("No proxy target matched"))
                .unwrap())
        }
    }
}
