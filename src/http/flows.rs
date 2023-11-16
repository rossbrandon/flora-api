use crate::db::flows::{get_flow_by_client_id_upstream_id, get_flows_by_client_id};
use crate::http::ApiContext;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};

pub(crate) fn router() -> Router<ApiContext> {
    Router::new()
        .route("/api/v1/clients/:client_id/flows", get(all_client_flows))
        .route(
            "/api/v1/clients/:client_id/flows/:upstream_id",
            get(client_flows_by_upstream_id),
        )
}

async fn all_client_flows(
    context: State<ApiContext>,
    client_id: Path<String>,
) -> impl IntoResponse {
    let flows = get_flows_by_client_id(&context.db, client_id.0)
        .await
        .expect("flows could not be retrieved");
    (StatusCode::OK, Json(flows))
}

async fn client_flows_by_upstream_id(
    context: State<ApiContext>,
    Path((client_id, upstream_id)): Path<(String, String)>,
) -> impl IntoResponse {
    let flow = get_flow_by_client_id_upstream_id(&context.db, client_id, upstream_id)
        .await
        .expect("flow could not retrieved");
    (StatusCode::OK, Json(flow))
}
