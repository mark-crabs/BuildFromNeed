use crate::views::{get_tokens, google_redirect};
use actix_web::{Scope, web};

pub fn auth_urls() -> Scope {
    web::scope("/auth")
        .service(get_tokens)
        .service(google_redirect)
}
