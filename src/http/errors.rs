use crate::db::errors::get_errors_by_upstream_id_downstream_id;
use crate::http::AppContext;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use opentelemetry::KeyValue;

pub(crate) fn router() -> Router<AppContext> {
    Router::new().route(
        "/errors/upstreams/:upstream_id/downstreams/:downstream_id",
        get(errors_by_upstream_id_downstream_id),
    )
}

async fn errors_by_upstream_id_downstream_id(
    context: State<AppContext>,
    Path((upstream_id, downstream_id)): Path<(String, String)>,
) -> impl IntoResponse {
    let counter = context
        .meter
        .u64_counter("api.errors_by_upstream_id_downstream_id")
        .with_description("Get Errors for Upstream & Downstream API call count")
        .init();
    counter.add(1, &[KeyValue::new("key", "value")]);

    let flow = get_errors_by_upstream_id_downstream_id(&context.db, upstream_id, downstream_id)
        .await
        .expect("errors could not retrieved");
    (StatusCode::OK, Json(flow))
}
