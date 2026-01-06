pub struct ENV {
    pub server: Server,
    pub database_url: String,
    pub oauth: Oauth2,
}

impl ENV {
    pub fn new(database_url: String, server: Server, oauth: Oauth2) -> Self {
        Self {
            server,
            database_url,
            oauth,
        }
    }
}

pub struct Oauth2 {
    pub client_id: String,
    pub client_secret: String,
    pub domain: String,
    pub region: String,
    pub pool_id: String,
}

impl Oauth2 {
    pub fn new(
        client_id: String,
        client_secret: String,
        domain: String,
        region: String,
        pool_id: String,
    ) -> Self {
        Self {
            client_id,
            client_secret,
            domain,
            region,
            pool_id,
        }
    }
}
pub struct Server {
    pub port: u16,
}

impl Server {
    pub fn new(port: u16) -> Self {
        Self { port }
    }
}
