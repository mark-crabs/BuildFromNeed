pub mod types;
use dotenv::dotenv;
use std::{
    env,
    error::Error
};
use self::types::ENV;
use anyhow::Result;

pub fn load_env() -> Result<ENV> {
    dotenv().ok();
    Ok(ENV {
        server_port: env::var("SERVER_PORT")?.parse()?
    })
}