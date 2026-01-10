use crate::views::{
    add_a_problem, delete_problem_by_id, get_problem_by_id, get_problems, update_problem_by_id,
};
use actix_web::{Scope, web};

pub fn urls() -> Scope {
    web::scope("/problems")
        .service(get_problems)
        .service(get_problem_by_id)
        .service(add_a_problem)
        .service(update_problem_by_id)
        .service(delete_problem_by_id)
}
