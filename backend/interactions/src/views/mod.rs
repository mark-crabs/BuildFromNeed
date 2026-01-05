use actix_web::{HttpResponse, Responder, get, web};
use utils::config::AppState;

#[get("/problem-like")]
pub async fn get_problem_like(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok()
}
