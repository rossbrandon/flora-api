use crate::db::errors::get_errors_by_upstream_id_downstream_id;
use crate::http::ApiContext;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};

pub(crate) fn router() -> Router<ApiContext> {
    Router::new().route(
        "/api/v1/errors/upstreams/:upstream_id/downstreams/:downstream_id",
        get(errors_by_upstream_id_downstream_id),
    )
}

async fn errors_by_upstream_id_downstream_id(
    context: State<ApiContext>,
    Path((upstream_id, downstream_id)): Path<(String, String)>,
) -> impl IntoResponse {
    let flow = get_errors_by_upstream_id_downstream_id(&context.db, upstream_id, downstream_id)
        .await
        .expect("errors could not retrieved");
    (StatusCode::OK, Json(flow))
}
