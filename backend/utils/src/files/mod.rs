pub mod types;
use crate::files::types::Oauth2;

use self::types::{ENV, Server};
use anyhow::Result;
use dotenv::dotenv;
use std::env;

pub fn load_env() -> Result<ENV> {
    dotenv().ok();

    Ok(ENV::new(
        env::var("DATABASE_URL")?,
        Server::new(env::var("SERVER_PORT")?.parse()?),
        Oauth2::new(
            env::var("OAUTH_CLIENT_ID")?,
            env::var("OAUTH_CLIENT_SECRET")?,
            env::var("OAUTH_DOMAIN")?,
            env::var("OAUTH_REDIRECT_URL")?,
        ),
        env::var("JWT_SECRET")?,
    ))
}
