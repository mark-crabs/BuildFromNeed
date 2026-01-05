use crate::models::Solution;
use actix_web::{HttpResponse, Responder, get, web};
use diesel::RunQueryDsl;
use serde::{Deserialize, Serialize};
use utils::{config::AppState, db::schema::solution};

#[get("")]
pub async fn get_solutions(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok()
}
