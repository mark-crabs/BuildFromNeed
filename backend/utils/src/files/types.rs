pub struct ENV {
    pub server: Server,
    pub database_url: String,
}

impl ENV {
    pub fn new(database_url: String, server: Server) -> Self {
        Self {
            server,
            database_url,
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
