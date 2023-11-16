use crate::http::ApiContext;
use axum::routing::get;
use axum::Router;

pub(crate) fn router() -> Router<ApiContext> {
    Router::new().route("/ping", get(ping))
}

async fn ping() -> &'static str {
    "pong"
}
