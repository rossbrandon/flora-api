use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Error {
    upstream_id: String,
    downstream_id: String,
    aggregates: Vec<FlowErrorAggregate>,
    errors: Option<Vec<FlowError>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowErrorAggregate {
    error_type: ErrorType,
    count: u32,
    message: String,
    log_link: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowError {
    upstream_id: String,
    downstream_id: String,
    document_id: String,
    error_type: ErrorType,
    message: String,
    log_link: Option<String>,
    timestamp: String,
}

#[derive(Serialize, Deserialize)]
pub enum ErrorType {
    Unknown,
    NetworkError,
    InvalidData,
    MissingData,
}
