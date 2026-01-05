use crate::views::get_solutions;
use actix_web::{Scope, web};

pub fn urls() -> Scope {
    web::scope("/solutions").service(get_solutions)
}
