use crate::views::get_problem_like;
use actix_web::{Scope, web};

pub fn urls() -> Scope {
    web::scope("/interactions").service(get_problem_like)
}
