use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    pub state: String,
    pub code: String,
    pub scope: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Tokens {
    pub access: String,
}
