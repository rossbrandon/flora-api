use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Flow {
    client_id: String,
    upstream: Upstream,
    downstreams: Vec<Downstream>,
    health: Health,
    meta: Option<FlowMetaData>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowMetaData {
    description: Option<String>,
    system_diagram_url: Option<String>,
    runbook_url: Option<String>,
    contact_slack_channel: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Upstream {
    upstream_id: String,
    name: String,
    total: u32,
    in_error: u32,
    health: Health,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Downstream {
    downstream_id: String,
    name: String,
    expected: u32,
    received: u32,
    missing: u32,
    in_error: u32,
    last_received: LastReceived,
    health: Health,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LastReceived {
    document_id: String,
    timestamp: String,
}

#[derive(Serialize, Deserialize)]
pub enum Health {
    Unknown,
    Unhealthy,
    InvestigationNeeded,
    Healthy,
}
