pub mod types;
use self::types::{
    Claims,
    TokenBody
};
use actix_web::{web::Json};
use jsonwebtoken::{decode, decode_header, DecodingKey, Validation, Algorithm, TokenData};
use oauth2::{
    AuthorizationCode,
    ClientId,
    ClientSecret,
    RedirectUrl,
    TokenUrl,
    PkceCodeVerifier,

};
use oauth2::basic::BasicClient;
use oauth2::reqwest::Client;
use crate::files::types::Oauth2;
use anyhow::Result;



pub async fn get_token(oauth: &Oauth2, token_body: &TokenBody) -> Result<Json<oauth2::StandardTokenResponse<oauth2::EmptyExtraTokenFields, oauth2::basic::BasicTokenType>>, actix_web::error::Error>{
    let http_client = Client::new();
    let client = BasicClient::new(
        ClientId::new(oauth.client_id.clone()),
    )
    .set_client_secret(ClientSecret::new(oauth.client_secret.clone()))
    .set_token_uri(TokenUrl::new(format!("{}/oauth2/token", oauth.domain))
        .map_err(actix_web::error::ErrorInternalServerError)?)
    .set_redirect_uri(RedirectUrl::new(token_body.redirect_uri.clone())
        .map_err(actix_web::error::ErrorBadRequest)?);

    let token = client
        .exchange_code(AuthorizationCode::new(token_body.code.clone()))
        .set_pkce_verifier(PkceCodeVerifier::new(token_body.code_verifier.clone()))
        .request_async(&http_client)
        .await
        .map_err(actix_web::error::ErrorBadRequest)?;

    Ok(Json(token))
}


pub async fn validate_auth0_token(oauth: &Oauth2, auth_header: &str) -> Result<Claims, actix_web::Error> {
    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Missing Bearer token"))?
        .trim();

    let jwks_url = format!("https://{}/.well-known/jwks.json", oauth.domain);
    let jwks: serde_json::Value = reqwest::get(&jwks_url)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to fetch JWKS"))?
        .json()
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Invalid JWKS"))?;

    let header = decode_header(token).map_err(|_| actix_web::error::ErrorUnauthorized("Invalid token"))?;
    let kid = header.kid.ok_or_else(|| actix_web::error::ErrorUnauthorized("Missing kid"))?;

    let key = jwks["keys"]
        .as_array()
        .and_then(|keys| keys.iter().find(|k| k["kid"] == kid))
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Key not found"))?;

    let n = key["n"].as_str().ok_or_else(|| actix_web::error::ErrorUnauthorized("Invalid key"))?;
    let e = key["e"].as_str().ok_or_else(|| actix_web::error::ErrorUnauthorized("Invalid key"))?;

    let decoding_key = DecodingKey::from_rsa_components(n, e).map_err(|_| actix_web::error::ErrorInternalServerError("Failed to construct key"))?;

    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_audience(&[&oauth.client_id]);
    validation.set_issuer(&[format!("https://{}/", oauth.domain)]);

    let token_data: TokenData<Claims> = decode::<Claims>(token, &decoding_key, &validation)
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid token"))?;

    Ok(token_data.claims)
}
