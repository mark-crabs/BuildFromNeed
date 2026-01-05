use actix_web::{HttpResponse, Responder, get, web};
use diesel::RunQueryDsl;
use serde::{Deserialize, Serialize};
use utils::{config::AppState, db::schema::problem};

#[get("")]
pub async fn get_problems(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok()
}
