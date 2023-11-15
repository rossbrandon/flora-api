use axum::{Json, Router};
use axum::extract::State;
use axum::routing::get;
use crate::db::clients::get_all_clients;
use crate::http::ApiContext;
use crate::models::client::Client;

pub(crate) fn router() -> Router<ApiContext> {
    Router::new().route("/api/v1/clients", get(get_clients))
}

async fn get_clients(ctx: State<ApiContext>) -> Result<Json<Vec<Client>>, Box<dyn std::error::Error>> {
    let clients = get_all_clients(&ctx.db).await;
    Ok(Json(clients?))
}
