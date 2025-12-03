//! Simple health check HTTP endpoints for Zebra.
//!
//! Provides `/healthy` and `/ready` endpoints for container orchestration.
//! Health server always listens on 0.0.0.0:8080.

use std::{convert::Infallible, sync::Arc};

use hyper::{
    body::Incoming,
    server::conn::http1,
    service::service_fn,
    Method, Request, Response, StatusCode,
};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

/// Hardcoded health server address
const HEALTH_LISTEN_ADDR: &str = "0.0.0.0:8080";

/// HTTP response body type
type Body = http_body_util::Full<hyper::body::Bytes>;

/// Handle incoming HTTP requests
async fn handle_request(req: Request<Incoming>) -> Result<Response<Body>, Infallible> {
    let response = match (req.method(), req.uri().path()) {
        (&Method::GET, "/healthy") => {
            Response::builder()
                .status(StatusCode::OK)
                .body(Body::from("healthy\n"))
                .unwrap()
        }
        (&Method::GET, "/ready") => {
            Response::builder()
                .status(StatusCode::OK)
                .body(Body::from("ready\n"))
                .unwrap()
        }
        _ => {
            Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from("not found\n"))
                .unwrap()
        }
    };
    Ok(response)
}

/// Start the health check server on port 8080.
///
/// Returns a task handle that runs the server.
pub async fn start() -> tokio::task::JoinHandle<()> {
    let listener = match TcpListener::bind(HEALTH_LISTEN_ADDR).await {
        Ok(listener) => {
            tracing::info!("health server listening on {}", HEALTH_LISTEN_ADDR);
            Arc::new(listener)
        }
        Err(e) => {
            tracing::error!("failed to bind health server to {}: {}", HEALTH_LISTEN_ADDR, e);
            // Return a dummy task that never completes
            return tokio::spawn(std::future::pending());
        }
    };

    tokio::spawn(async move {
        loop {
            let (stream, _addr) = match listener.accept().await {
                Ok(conn) => conn,
                Err(e) => {
                    tracing::warn!("health server accept error: {}", e);
                    continue;
                }
            };

            let io = TokioIo::new(stream);

            tokio::spawn(async move {
                if let Err(e) = http1::Builder::new()
                    .serve_connection(io, service_fn(handle_request))
                    .await
                {
                    tracing::debug!("health server connection error: {}", e);
                }
            });
        }
    })
}