use futures::TryStreamExt;
use mongodb::Database;
use mongodb::options::FindOptions;
use crate::models::client::Client;

pub async fn get_all_clients(db: &Database) -> mongodb::error::Result<Vec<Client>> {
    let collection = db.collection::<Client>("clients");

    let options = FindOptions::default();
    let mut cursor = collection.find(None, options)
        .await?;

    let mut clients: Vec<Client> = Vec::new();
    while let Some(result) = cursor.try_next().await? {
        clients.push(result)
    }

    Ok(clients)
}