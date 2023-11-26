use crate::db::clients::{get_all_clients, get_client_by_id};
use crate::http::AppContext;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use opentelemetry::KeyValue;

pub(crate) fn router() -> Router<AppContext> {
    Router::new()
        .route("/api/v1/clients", get(all_clients))
        .route("/api/v1/clients/:client_id", get(client_by_id))
}

async fn all_clients(context: State<AppContext>) -> impl IntoResponse {
    let counter = context
        .meter
        .u64_counter("api.all_clients")
        .with_description("Get All Clients API call count")
        .init();
    counter.add(1, &[KeyValue::new("key", "value")]);

    let clients = get_all_clients(&context.db)
        .await
        .expect("clients could not be retrieved");
    (StatusCode::OK, Json(clients))
}

async fn client_by_id(context: State<AppContext>, client_id: Path<String>) -> impl IntoResponse {
    let counter = context
        .meter
        .u64_counter("api.client_by_id")
        .with_description("Get Client by ID API call count")
        .init();
    counter.add(1, &[KeyValue::new("key", "value")]);

    let client = get_client_by_id(&context.db, client_id.0)
        .await
        .expect("client could not retrieved");
    (StatusCode::OK, Json(client))
}
