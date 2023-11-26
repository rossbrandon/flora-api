use crate::http::AppContext;
use axum::extract::State;
use axum::routing::get;
use axum::Router;
use opentelemetry::KeyValue;

pub(crate) fn router() -> Router<AppContext> {
    Router::new().route("/ping", get(ping))
}

async fn ping(context: State<AppContext>) -> &'static str {
    let counter = context
        .meter
        .u64_counter("api.ping")
        .with_description("Ping API call count")
        .init();
    counter.add(1, &[KeyValue::new("key", "value")]);

    "pong"
}
