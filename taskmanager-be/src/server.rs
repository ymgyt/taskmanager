use std::{
    future::Future,
    net::{SocketAddr, TcpListener},
};

use axum::{routing::get, Router};
use tracing::info;

pub async fn run(
    addr: impl Into<SocketAddr>,
    shutdown: impl Future<Output = ()>,
) -> Result<(), anyhow::Error> {
    let addr = addr.into();
    let listener = std::net::TcpListener::bind(addr)?;

    info!(?addr, "listening");

    run_with_listener(listener, shutdown).await
}

pub async fn run_with_listener(
    listener: TcpListener,
    shutdown: impl Future<Output = ()>,
) -> Result<(), anyhow::Error> {
    let app = Router::new().route("/healthz", get(|| async { "OK" }));

    axum::Server::from_tcp(listener)?
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown)
        .await
        .map_err(anyhow::Error::from)
}
