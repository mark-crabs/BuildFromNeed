use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub iss: String,
    pub sub: String,
    pub aud: Vec<String>,
    pub exp: usize,
    pub iat: usize,
    pub nonce: Option<String>,

    pub name: Option<String>,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub nickname: Option<String>,
    pub email: Option<String>,
    pub email_verified: Option<bool>,
    pub picture: Option<String>,

    pub scope: Option<String>,
    pub permissions: Option<Vec<String>>,
    pub client_id: Option<String>,

    #[serde(flatten)]
    pub custom: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TokenBody {
    pub code: String,
    pub code_verifier: String,
    pub redirect_uri: String
}