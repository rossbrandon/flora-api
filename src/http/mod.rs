mod ping;
mod clients;

use std::net::{Ipv4Addr, SocketAddr};
use std::sync::Arc;
use anyhow::Context;
use axum::Router;
use mongodb::{Database};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use crate::config::Config;

#[derive(Clone)]
pub(crate) struct ApiContext {
    config: Arc<Config>,
    db: Database,
}

pub async fn serve(config: Config, db: Database) -> anyhow::Result<()> {
    let context = ApiContext {
        config: Arc::new(config),
        db
    };

    let app = api_router(context);

    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 8080));
    println!("server listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .context("error starting http server")
}

fn api_router(context: ApiContext) -> Router {
    let cors = CorsLayer::new().allow_origin(Any);
    Router::new()
        .with_state(context)
        .merge(ping::router())
        .merge(clients::router())
        .layer(TraceLayer::new_for_http())
        .layer(cors)
}
