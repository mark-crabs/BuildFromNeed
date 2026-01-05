use crate::views::{get_user_by_email, get_users};
use actix_web::{Scope, web};

pub fn urls() -> Scope {
    web::scope("/users")
        .service(get_users)
        .service(get_user_by_email)
}
