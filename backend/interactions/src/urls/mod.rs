use crate::views::{
    favourite_a_problem, favourite_a_solution, get_problem_favourites_by_id, get_problem_likes,
    get_problems_favourites, get_solution_favourites_by_id, get_solution_likes,
    get_solutions_favourites, like_a_problem, like_a_solution, view_a_problem,
};
use actix_web::{Scope, web};

pub fn urls() -> Scope {
    web::scope("/interactions")
        .service(view_a_problem)
        .service(like_a_problem)
        .service(like_a_solution)
        .service(favourite_a_problem)
        .service(favourite_a_solution)
        .service(get_problem_likes)
        .service(get_solution_likes)
        .service(get_problems_favourites)
        .service(get_solutions_favourites)
        .service(get_problem_favourites_by_id)
        .service(get_solution_favourites_by_id)
}
