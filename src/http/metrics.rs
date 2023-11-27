use crate::http::AppContext;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use prometheus::{Encoder, TextEncoder};

pub(crate) fn router() -> Router<AppContext> {
    Router::new().route("/metrics", get(prometheus_metrics))
}

async fn prometheus_metrics(context: State<AppContext>) -> impl IntoResponse {
    let encoder = TextEncoder::new();
    let metric_families = context.registry.gather();
    let mut result = Vec::new();
    encoder.encode(&metric_families, &mut result).unwrap();
    result
}
