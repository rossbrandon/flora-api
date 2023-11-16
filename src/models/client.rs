use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Client {
    pub client_id: String,
    pub name: String,
    pub client_type: ClientType,
    pub description: String,
    pub device_type: DeviceType,
    pub user_agent: String,
}

#[derive(Serialize, Deserialize, Display)]
pub enum ClientType {
    Unknown,
    Internal,
    External,
}

#[derive(Serialize, Deserialize, Display)]
pub enum DeviceType {
    Unknown,
    System,
    Web,
    Mobile,
    Desktop,
}
