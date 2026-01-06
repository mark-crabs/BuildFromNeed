use actix_web::{Scope, web};
use crate::views::get_tokens;


pub fn auth_urls() -> Scope {
    web::scope("/auth")
        .service(get_tokens)
}