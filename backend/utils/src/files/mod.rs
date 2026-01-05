pub mod types;
use self::types::{ENV, Server};
use anyhow::Result;
use dotenv::dotenv;
use std::env;

pub fn load_env() -> Result<ENV> {
    dotenv().ok();

    Ok(ENV::new(
        env::var("DATABASE_URL")?,
        Server::new(env::var("SERVER_PORT")?.parse()?),
    ))
}
