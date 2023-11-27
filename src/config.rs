#[derive(clap::Parser)]
pub struct Config {
    #[clap(long, env)]
    pub environment: String,

    #[clap(long, env)]
    pub mongodb_uri: String,

    #[clap(long, env)]
    pub db_name: String,

    #[clap(long, env)]
    pub loki_url: String,

    #[clap(long, env)]
    pub metrics_token: String,
}
