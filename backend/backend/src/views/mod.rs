use ::users::{dto::AddUser, models::User};
use actix_web::{HttpResponse, Responder, get, http::header::LOCATION, web};
use anyhow::Result;
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use oauth2::{CsrfToken, PkceCodeChallenge};
use reqwest::Client;
use utils::models::Role;
use utils::{
    auth::{GoogleClaims, GoogleTokenResponse},
    config::AppState,
    db::schema::{oauth_requests, users},
    dto::{AddOauthRequest, DataResponse},
    models::OauthRequests,
};

use crate::dto::{AuthRequest, Tokens};
use utils::dto::Claims;

#[get("/google/callback")]
pub async fn google_redirect(
    query: web::Query<AuthRequest>,
    state: web::Data<AppState>
) -> impl Responder {
    let oauth = &state.env.oauth;
    match state.db_pool.get() {
        Ok(mut connection) => {
            let state_code = query.state.clone();

            let request = oauth_requests::dsl::oauth_requests
                .filter(oauth_requests::csrf_state.eq(state_code))
                .first::<OauthRequests>(&mut connection)
                .unwrap();

            let claims = get_token(
                &oauth.redirect_url,
                &oauth.client_id,
                &oauth.client_secret,
                "https://oauth2.googleapis.com/token",
                &query.code,
            )
            .await;
            match claims {
                Ok(claims) => {
                    let email = claims.email.clone();
                    let users = users::dsl::users
                        .filter(users::email.eq(email.clone().unwrap()))
                        .load::<User>(&mut connection)
                        .unwrap();

                    let access: String;

                    if users.is_empty() {
                        let user_add: AddUser = claims.clone().into();

                        let user: User = diesel::insert_into(users::dsl::users)
                            .values(&user_add)
                            .get_result(&mut connection)
                            .unwrap();
                        access = Claims::new(
                            user.id,
                            claims.name,
                            email,
                            Role::Casual,
                            claims.picture,
                            &oauth.client_id,
                        )
                        .get_jwt(&state.env.jwt_secret);
                    } else {
                        access = Claims::new(
                            users.get(0).unwrap().id,
                            claims.name,
                            email,
                            users.get(0).unwrap().role,
                            claims.picture,
                            &oauth.client_id,
                        )
                        .get_jwt(&state.env.jwt_secret);
                    }

                    return HttpResponse::Ok().json(DataResponse::new(Tokens { access }));
                }
                Err(_) => {}
            }
        }
        Err(_) => {}
    }

    HttpResponse::Ok().finish()
}

#[get("/token")]
pub async fn get_tokens(
    state: web::Data<AppState>
) -> impl Responder {
    let oauth = &state.env.oauth;
    let csrf_state = CsrfToken::new_random();
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    let values = AddOauthRequest {
        pkce_challenge: pkce_challenge.as_str().into(),
        pkce_verifier: pkce_verifier.secret().into(),
        csrf_state: csrf_state.secret().into(),
    };

    match state.db_pool.get() {
        Ok(mut connection) => {
            diesel::insert_into(oauth_requests::dsl::oauth_requests)
                .values(&values)
                .execute(&mut connection)
                .unwrap();

            let oauth_login_url = format!(
                "{oauth_url}?client_id={client_id}&redirect_uri={redirect_url}&response_type=code&scope={scope}&prompt=select_account&pkce_challenge={pkce_challenge}&state={state}&access_type=offline",
                oauth_url = &oauth.domain,
                redirect_url = &oauth.redirect_url,
                client_id = oauth.client_id,
                scope = "profile email",
                pkce_challenge = pkce_challenge.as_str(),
                state = &csrf_state.secret()
            );

            let mut response = HttpResponse::Found();
            response.append_header((LOCATION, oauth_login_url));
            response.finish()
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_token(
    redirect_url: &str,
    client_id: &str,
    client_secret: &str,
    oauth_token_url: &str,
    authorization_code: &str,
) -> Result<GoogleClaims, String> {
    let client = Client::new();
    let params = [
        ("grant_type", "authorization_code"),
        ("redirect_uri", redirect_url),
        ("client_id", client_id),
        ("code", authorization_code),
        ("client_secret", client_secret),
    ];

    let response = client
        .post(oauth_token_url)
        .form(&params)
        .send()
        .await
        .unwrap();
    let data: GoogleTokenResponse = response.json().await.unwrap();

    if let Some(data) = data.id_token {
        let parts: Vec<&str> = data.split('.').collect();
        if parts.len() == 3 {
            let payload = parts[1];

            let decoded = URL_SAFE_NO_PAD.decode(payload).unwrap();
            let claims: GoogleClaims = serde_json::from_slice(&decoded).unwrap();
            return Ok(claims);
        } else {
            return Err(String::new());
        }
    }

    Err(String::new())
}
