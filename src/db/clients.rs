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
        // let _id = result._id;
        // let id = result.id;
        // let name = result.name;
        // let client_type = result.client_type;
        // let description = result.description;
        // let device_type = result.device_type;
        // let user_agent = result.user_agent;
        // // transform ObjectId to String
        // let client_json = Client {
        //     _id: _id.to_string().parse()?,
        //     id: id.to_string(),
        //     name: name.to_string(),
        //     client_type,
        //     description: description.to_string(),
        //     device_type,
        //     user_agent: user_agent.to_string(),
        // };
        clients.push(result);
    }

    Ok(clients)
}