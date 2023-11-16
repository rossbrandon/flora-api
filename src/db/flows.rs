use crate::models::flow::Flow;
use bson::doc;
use futures::TryStreamExt;
use mongodb::options::FindOptions;
use mongodb::Database;

const COLLECTION_NAME: &str = "flows";

pub async fn get_flows_by_client_id(
    db: &Database,
    client_id: String,
) -> mongodb::error::Result<Vec<Flow>> {
    let collection = db.collection::<Flow>(COLLECTION_NAME);
    let options = FindOptions::default();
    let filter = doc! {"clientId": client_id };
    let mut cursor = collection.find(filter, options).await?;

    let mut flows: Vec<Flow> = Vec::new();
    while let Some(result) = cursor.try_next().await? {
        flows.push(result)
    }

    Ok(flows)
}

pub async fn get_flow_by_client_id_upstream_id(
    db: &Database,
    client_id: String,
    upstream_id: String,
) -> mongodb::error::Result<Option<Flow>> {
    let collection = db.collection::<Flow>(COLLECTION_NAME);
    let filter = doc! {"clientId": client_id, "upstream.upstreamId": upstream_id };
    let flow = collection.find_one(filter, None).await?;

    if flow.is_none() {
        return Ok(None);
    }

    Ok(flow)
}
