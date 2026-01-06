use utils::auth::{get_token, types::TokenBody};
use actix_web::{HttpResponse, Responder, post, web};



#[post("/token")]
pub async fn get_tokens(info: web::Json<TokenBody>) -> impl Responder {
    HttpResponse::Ok().json(info)
}