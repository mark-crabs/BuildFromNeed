use crate::views::{add_user, delete_user_by_id, get_user_by_id, get_users, update_user_by_id};
use actix_web::{Scope, web};

pub fn urls() -> Scope {
    web::scope("/users")
        .service(get_users)
        .service(get_user_by_id)
        .service(update_user_by_id)
        .service(delete_user_by_id)
        .service(add_user)
}
