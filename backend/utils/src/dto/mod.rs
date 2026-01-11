use crate::db::schema::oauth_requests;
use crate::models::Role;
use actix_web::error;
use chrono::{Duration, Utc};
use derive_more::derive::{Display, Error};
use diesel::prelude::*;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode};
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct Pagination {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginationDb {
    pub offset: i64,
    pub limit: i64,
}

impl Pagination {
    pub fn limit_and_offset(&self) -> PaginationDb {
        let offset =
            (self.page.unwrap_or(1).saturating_sub(1) * self.page_size.unwrap_or(10)) as i64;
        let limit = self.page_size.unwrap_or(10) as i64;

        PaginationDb { offset, limit }
    }
}

#[derive(Debug, Display, Error, Serialize, Deserialize)]
#[display("Error: {message}")]
pub struct ErrorResponse {
    pub message: String,
}

impl ErrorResponse {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataResponse<T> {
    pub data: T,
}

impl<T> DataResponse<T> {
    pub fn new(data: T) -> Self {
        Self { data }
    }
}

impl error::ResponseError for ErrorResponse {}

#[derive(Deserialize, Insertable, Serialize)]
#[diesel(table_name = oauth_requests)]
pub struct AddOauthRequest {
    pub pkce_challenge: String,
    pub pkce_verifier: String,
    pub csrf_state: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub name: Option<String>,
    pub email: Option<String>,
    pub role: Role,
    pub picture: Option<String>,
    pub iss: String,
    pub iat: i64,
    pub exp: i64,
}

impl Claims {
    pub fn new(
        name: Option<String>,
        email: Option<String>,
        role: Role,
        picture: Option<String>,
        iss: &String,
    ) -> Self {
        let now = Utc::now();
        let iat = now.timestamp();
        let exp = (now + Duration::days(1)).timestamp();

        Self {
            name,
            email,
            role,
            picture,
            iss: iss.to_string(),
            iat,
            exp,
        }
    }

    pub fn get_jwt(&self, key: &String) -> String {
        let key = EncodingKey::from_secret(key.as_bytes());
        let token = encode(&Header::default(), self, &key).unwrap();
        token
    }
    pub fn decode_jwt(
        token: &str,
        key: &String,
    ) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
        let key = DecodingKey::from_secret(key.as_bytes());
        decode::<Claims>(token, &key, &Validation::default())
    }
}
