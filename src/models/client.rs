use serde::{Deserialize, Serialize};
use strum_macros::Display;
use mongodb::bson::oid::ObjectId;

#[derive(Serialize, Deserialize)]
pub struct Client {
    pub _id: ObjectId,
    pub id: String,
    pub name: String,
    pub client_type: ClientType,
    pub description: String,
    pub device_type: DeviceType,
    pub user_agent: String
}

#[derive(Serialize, Deserialize, Display)]
pub enum ClientType {
    Unknown,
    Internal,
    External
}

#[derive(Serialize, Deserialize, Display)]
pub enum DeviceType {
    Unknown,
    System,
    Web,
    Mobile,
    Desktop
}
