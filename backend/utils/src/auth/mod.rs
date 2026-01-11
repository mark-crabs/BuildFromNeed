pub mod types;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct GoogleTokenResponse {
    pub access_token: String,
    pub expires_in: i64,
    pub refresh_token: Option<String>,
    pub scope: String,
    pub token_type: String,
    pub id_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GoogleClaims {
    pub iss: Option<String>,
    pub aud: Option<String>,
    pub sub: Option<String>,
    pub email: Option<String>,
    pub email_verified: Option<bool>,
    pub name: Option<String>,
    pub picture: Option<String>,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub exp: Option<u64>,
    pub iat: Option<u64>,
}
