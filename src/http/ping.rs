use axum::Router;
use axum::routing::get;
use crate::http::ApiContext;

pub(crate) fn router() -> Router<ApiContext> {
    Router::new().route("/ping", get(ping))
}

async fn ping() -> &'static str {
    "pong"
}
