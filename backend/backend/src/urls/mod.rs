use crate::views::get_tokens;
use actix_web::{Scope, web};

pub fn auth_urls() -> Scope {
    web::scope("/auth").service(get_tokens)
}
