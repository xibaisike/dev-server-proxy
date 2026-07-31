use anyhow::Result;
use bytes::Bytes;
use http_body_util::Full;
use hyper::body::Incoming;
use hyper::service::service_fn;
use hyper::{Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;

use crate::config::Config;
use crate::logger::{format_timestamp, log_event};
use crate::matcher::find_match;
use crate::proxy::build_target_url;
use crate::types::LogEvent;

pub async fn start_server(config: Config) -> Result<()> {
    let config = Arc::new(config);
    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));

    tracing::info!(
        "Starting proxy server on {} for {}",
        addr,
        config.server_name
    );

    let listener = TcpListener::bind(&addr).await?;

    println!("Proxy server listening on port {}", config.port);

    loop {
        let (stream, _) = listener.accept().await?;
        let config = Arc::clone(&config);

        tokio::spawn(async move {
            let svc = service_fn(move |req| {
                handle_request(req, Arc::clone(&config))
            });

            if let Err(err) = hyper::server::conn::http1::Builder::new()
                .serve_connection(TokioIo::new(stream), svc)
                .await
            {
                tracing::error!("Failed to serve connection: {}", err);
            }
        });
    }
}

async fn handle_request(
    req: Request<Incoming>,
    config: Arc<Config>,
) -> Result<Response<Full<Bytes>>, hyper::Error> {
    let method = req.method().clone();
    let uri = req.uri().clone();
    let req_path = uri.path();

    tracing::debug!("Incoming request: {} {}", method, req_path);

    match find_match(&config.locations, req_path) {
        Some(location) => {
            match build_target_url(location, req_path) {
                Ok(_target_uri) => {
                    let response = Response::builder()
                        .status(StatusCode::OK)
                        .body(Full::new(Bytes::from("Proxied successfully")))
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
                        proxy_path: String::new(),
                        status: 500,
                    };
                    log_event(&event);

                    Ok(Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(Full::new(Bytes::from("Failed to proxy request")))
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
                proxy_path: String::new(),
                status: 502,
            };
            log_event(&event);

            Ok(Response::builder()
                .status(StatusCode::BAD_GATEWAY)
                .body(Full::new(Bytes::from("No proxy target matched")))
                .unwrap())
        }
    }
}
