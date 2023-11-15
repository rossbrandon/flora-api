use axum::Router;
use axum::routing::get;

pub(crate) fn router() -> Router {
    Router::new().route("/ping", get(ping))
}

async fn ping() -> &'static str {
    "pong"
}
