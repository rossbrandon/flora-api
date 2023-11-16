mod config;
mod db;
mod http;
mod models;

use crate::config::Config;
use clap::Parser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    let config = Config::parse();

    let db = db::connect(&config.mongodb_uri, &config.db_name).await?;
    http::serve(db).await?;

    Ok(())
}
