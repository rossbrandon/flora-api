use axum::{Json, Router};
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::http::StatusCode;
use crate::db::clients::get_all_clients;
use crate::http::ApiContext;

pub(crate) fn router() -> Router<ApiContext> {
    Router::new().route("/api/v1/clients", get(get_clients))
}

pub async fn get_clients(context: State<ApiContext>) -> impl IntoResponse {
    let clients = get_all_clients(&context.db).await.expect("clients could not be retrieved");
    (StatusCode::OK, Json(clients))
}