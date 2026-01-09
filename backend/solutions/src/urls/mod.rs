use crate::views::{
    add_a_solution, delete_solution_by_id, get_all_solutions, get_solution_by_problem_id,
    update_solution_by_id,
};
use actix_web::{Scope, web};

pub fn urls() -> Scope {
    web::scope("/solutions")
        .service(get_all_solutions)
        .service(add_a_solution)
        .service(update_solution_by_id)
        .service(delete_solution_by_id)
        .service(get_solution_by_problem_id)
}
