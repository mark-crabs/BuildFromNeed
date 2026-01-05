use actix_web::{HttpResponse, Responder, get, web};
use utils::config::AppState;

#[get("")]
pub async fn get_users(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok()
}

#[get("/{email}")]
pub async fn get_user_by_email(path: web::Path<String>) -> impl Responder {
    HttpResponse::Ok()
}
