mod models;
mod http;
mod db;
mod config;

use clap::Parser;
use crate::config::Config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    let config = Config::parse();

    let db = db::connect(&config.mongodb_uri, &config.db_name).await?;
    http::serve(config, db).await?;

    Ok(())
}
