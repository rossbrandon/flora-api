#[derive(clap::Parser)]
pub struct Config {
    #[clap(long, env)]
    pub mongodb_uri: String,


    #[clap(long, env)]
    pub db_name: String,
}
