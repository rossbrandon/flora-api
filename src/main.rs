mod config;
mod db;
mod http;
mod models;

use crate::config::Config;
use clap::Parser;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing_subscriber();

    dotenv::dotenv().ok();
    let config = Config::parse();

    let db = db::connect(&config.mongodb_uri, &config.db_name).await?;
    http::serve(db).await?;

    Ok(())
}

fn init_tracing_subscriber() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "flora_api=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}
