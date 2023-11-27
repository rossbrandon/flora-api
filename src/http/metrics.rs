use crate::http::AppContext;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{
    headers::authorization::{Authorization, Bearer},
    Router, TypedHeader,
};
use clap::Parser;
use prometheus::{Encoder, TextEncoder};
use crate::config::Config;

pub(crate) fn router() -> Router<AppContext> {
    Router::new().route("/metrics", get(prometheus_metrics))
}

async fn prometheus_metrics(
    context: State<AppContext>,
    TypedHeader(authorization): TypedHeader<Authorization<Bearer>>,
) -> Response {
    dotenv::dotenv().ok();
    let config = Config::parse();
    if authorization.token().ne(&config.metrics_token) {
        return StatusCode::UNAUTHORIZED.into_response();
    }

    let encoder = TextEncoder::new();
    let metric_families = context.registry.gather();
    let mut result = Vec::new();
    encoder.encode(&metric_families, &mut result).unwrap();
    result.into_response()
}
