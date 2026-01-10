pub struct ENV {
    pub server: Server,
    pub database_url: String,
    pub oauth: Oauth2,
    pub jwt_secret: String,
}

impl ENV {
    pub fn new(database_url: String, server: Server, oauth: Oauth2, jwt_secret: String) -> Self {
        Self {
            server,
            database_url,
            oauth,
            jwt_secret,
        }
    }
}

#[derive(Debug)]
pub struct Oauth2 {
    pub client_id: String,
    pub client_secret: String,
    pub domain: String,
    pub redirect_url: String,
}

impl Oauth2 {
    pub fn new(
        client_id: String,
        client_secret: String,
        domain: String,
        redirect_url: String,
    ) -> Self {
        Self {
            client_id,
            client_secret,
            domain,
            redirect_url,
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
