mod clients;
mod ping;

use anyhow::Context;
use axum::Router;
use mongodb::Database;
use std::net::{Ipv4Addr, SocketAddr};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

#[derive(Clone)]
pub struct ApiContext {
    db: Database,
}

pub async fn serve(db: Database) -> anyhow::Result<()> {
    let context = ApiContext { db };

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
        .merge(ping::router())
        .merge(clients::router())
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .with_state(context)
}
