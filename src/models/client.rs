use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Client {
    pub client_id: String,
    pub name: String,
    pub client_type: ClientType,
    pub device_type: DeviceType,
    pub description: Option<String>,
    pub user_agent: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub enum ClientType {
    Unknown,
    Internal,
    External,
}

#[derive(Serialize, Deserialize)]
pub enum DeviceType {
    Unknown,
    System,
    Web,
    Mobile,
    Desktop,
}
