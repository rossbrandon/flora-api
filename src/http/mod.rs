mod clients;
mod errors;
mod flows;
mod metrics;
mod ping;

use crate::AppContext;
use anyhow::Context;
use axum::Router;
use std::net::{Ipv4Addr, SocketAddr};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

pub async fn serve(context: AppContext) -> anyhow::Result<()> {
    let app = api_router(context);

    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 8080));
    tracing::debug!("http server listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .context("error starting http server")
}

fn api_router(context: AppContext) -> Router {
    let cors = CorsLayer::new().allow_origin(Any);
    Router::new()
        .merge(metrics::router())
        .merge(ping::router())
        .merge(clients::router())
        .merge(flows::router())
        .merge(errors::router())
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .with_state(context)
}
