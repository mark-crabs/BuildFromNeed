use crate::views::get_problems;
use actix_web::{Scope, web};

pub fn urls() -> Scope {
    web::scope("/problems").service(get_problems)
}
