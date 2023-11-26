mod config;
mod db;
mod http;
mod models;

use crate::config::Config;
use clap::Parser;
use mongodb::Database;
use opentelemetry::metrics::{Meter, MeterProvider as _};
use opentelemetry_sdk::metrics::MeterProvider;
use prometheus::Registry;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use url::Url;

pub const SERVICE_NAME: &str = "flora-api";

#[derive(Clone)]
pub struct AppContext {
    db: Database,
    registry: Registry,
    meter: Meter,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    let config = Config::parse();

    init_tracing_subscriber(config.loki_url);
    let (registry, meter) = init_prometheus();

    let db = db::connect(&config.mongodb_uri, &config.db_name).await?;
    let context = AppContext {
        db,
        registry,
        meter,
    };
    http::serve(context).await?;

    Ok(())
}

fn init_tracing_subscriber(loki_url: String) {
    let (loki_layer, loki_task) = tracing_loki::builder()
        .label("service_name", SERVICE_NAME).unwrap()
        .build_url(Url::parse(&loki_url).unwrap()).unwrap();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "flora_api=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .with(loki_layer)
        .init();

    tokio::spawn(loki_task);

    tracing::info!(
        task = "tracing_init",
        result = "success",
        "loki tracing initialized successfully",
    );
}

fn init_prometheus() -> (Registry, Meter) {
    let registry = Registry::new();
    let exporter = opentelemetry_prometheus::exporter()
        .with_registry(registry.clone())
        .build().unwrap();
    let provider = MeterProvider::builder().with_reader(exporter).build();
    let meter = provider.meter(SERVICE_NAME);
    (registry, meter)
}
