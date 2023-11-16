use crate::db::clients::{get_all_clients, get_client_by_id};
use crate::http::ApiContext;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};

pub(crate) fn router() -> Router<ApiContext> {
    Router::new()
        .route("/api/v1/clients", get(all_clients))
        .route("/api/v1/clients/:client_id", get(client_by_id))
}

async fn all_clients(context: State<ApiContext>) -> impl IntoResponse {
    let clients = get_all_clients(&context.db)
        .await
        .expect("clients could not be retrieved");
    (StatusCode::OK, Json(clients))
}

async fn client_by_id(context: State<ApiContext>, client_id: Path<String>) -> impl IntoResponse {
    let client = get_client_by_id(&context.db, client_id.0)
        .await
        .expect("client could not retrieved");
    (StatusCode::OK, Json(client))
}
