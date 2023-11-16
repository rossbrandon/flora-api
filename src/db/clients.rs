use crate::models::client::Client;
use bson::doc;
use futures::TryStreamExt;
use mongodb::options::FindOptions;
use mongodb::Database;

pub async fn get_all_clients(db: &Database) -> mongodb::error::Result<Vec<Client>> {
    let collection = db.collection::<Client>("clients");
    let options = FindOptions::default();
    let mut cursor = collection.find(None, options).await?;
    let mut clients: Vec<Client> = Vec::new();
    while let Some(result) = cursor.try_next().await? {
        clients.push(result)
    }

    Ok(clients)
}

pub async fn get_client_by_id(
    db: &Database,
    client_id: String,
) -> mongodb::error::Result<Option<Client>> {
    let collection = db.collection::<Client>("clients");
    let client = collection
        .find_one(doc! {"clientId": client_id }, None)
        .await?;
    if client.is_none() {
        return Ok(None);
    }

    Ok(client)
}
