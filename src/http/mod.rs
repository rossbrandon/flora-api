mod clients;
mod errors;
mod flows;
mod metrics;
mod ping;

use crate::config::Config;
use crate::AppContext;
use anyhow::Context;
use axum::Router;
use clap::Parser;
use std::net::{Ipv4Addr, SocketAddr};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tower_http::validate_request::ValidateRequestHeaderLayer;

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
        .merge(ping::router())
        .merge(protected_router())
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .with_state(context)
}

fn protected_router() -> Router<AppContext> {
    dotenv::dotenv().ok();
    let config = Config::parse();

    Router::new()
        .merge(metrics::router())
        .nest("/api/v1", v1_router())
        .route_layer(ValidateRequestHeaderLayer::bearer(&config.api_token))
}

fn v1_router() -> Router<AppContext> {
    Router::new()
        .merge(clients::router())
        .merge(flows::router())
        .merge(errors::router())
}
