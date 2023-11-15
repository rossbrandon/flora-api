pub mod clients;

use mongodb::{Client, Database};
use mongodb::options::{ClientOptions, ResolverConfig};

pub async fn connect(uri: &String, db_name: &String) -> mongodb::error::Result<Database> {
    let options = ClientOptions::parse_with_resolver_config(
        uri,
        ResolverConfig::cloudflare()
    ).await?;

    let client = Client::with_options(options)?;
    let database = client.database(db_name);

    Ok(database)
}