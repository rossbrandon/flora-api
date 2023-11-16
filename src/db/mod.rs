pub mod clients;

use mongodb::options::{ClientOptions, ResolverConfig};
use mongodb::{Client, Database};

pub async fn connect(uri: &String, db_name: &str) -> mongodb::error::Result<Database> {
    let options =
        ClientOptions::parse_with_resolver_config(uri, ResolverConfig::cloudflare()).await?;

    let client = Client::with_options(options)?;
    let database = client.database(db_name);

    Ok(database)
}
