mod client;

use std::net::SocketAddr;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Json, Router};
use axum::routing::get;
use tower_http::cors::{Any, CorsLayer};
use crate::client::{Client, ClientType, DeviceType};

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new().allow_origin(Any);

    let app = Router::new()
        .route("/ping", get(ping))
        .route("/api/v1/clients", get(get_clients))
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("server listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn ping() -> &'static str {
    "pong"
}

async fn get_clients() -> impl IntoResponse {
    let clients = vec![
        Client {
            id: String::from("internal"),
            name: String::from("Internal Pipelines"),
            client_type: ClientType::Internal,
            description: String::from("Internal system-to-system data pipelines."),
            device_type: DeviceType::System,
            user_agent: String::from("grpc-rust/9.2.1"),
        },
        Client {
            id: String::from("3e7780b8-7b57-44d2-ab6c-163ee1e18bb3"),
            name: String::from("User iOS Device"),
            client_type: ClientType::External,
            description: String::from("End user device"),
            device_type: DeviceType::Mobile,
            user_agent: String::from("Mozilla/5.0 (X11; Linux i686 AppleWebKit/533.1.0 (KHTML, like Gecko) Chrome/36.0.811.0 Safari/533.1.0"),
        },
    ];

    (StatusCode::OK, Json(clients))
}