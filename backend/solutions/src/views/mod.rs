use crate::{
    dto::{AddSolution, UpdateSolution},
    models::Solution,
};
use actix_web::{HttpResponse, Responder, delete, get, patch, post, web};
use diesel::{
    ExpressionMethods, RunQueryDsl,
    query_dsl::methods::{FilterDsl, LimitDsl, OffsetDsl, OrderDsl},
};
use utils::{
    config::AppState,
    db::schema::solution,
    dto::{DataResponse, Pagination},
};

#[get("")]
pub async fn get_all_solutions(
    web::Query(pagination): web::Query<Pagination>,
    state: web::Data<AppState>,
) -> impl Responder {
    let pagination = pagination.limit_and_offset();
    match state.db_pool.get() {
        Ok(mut connection) => {
            let solutions: Vec<Solution> = solution::dsl::solution
                .order(solution::created_at.desc())
                .limit(pagination.limit)
                .offset(pagination.offset)
                .load(&mut connection)
                .unwrap();

            HttpResponse::Ok().json(DataResponse::new(solutions))
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/p/{problem_id}")]
pub async fn get_solution_by_problem_id(
    path: web::Path<i64>,
    web::Query(pagination): web::Query<Pagination>,
    state: web::Data<AppState>,
) -> impl Responder {
    let pagination = pagination.limit_and_offset();
    let problem_id = path.into_inner();
    match state.db_pool.get() {
        Ok(mut connection) => {
            let results: Vec<Solution> = solution::dsl::solution
                .filter(solution::problem_id.eq(problem_id))
                .order(solution::created_at.desc())
                .limit(pagination.limit)
                .offset(pagination.offset)
                .load(&mut connection)
                .unwrap();

            HttpResponse::Ok().json(DataResponse::new(results))
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("")]
pub async fn add_a_solution(
    data: web::Json<AddSolution>,
    state: web::Data<AppState>,
) -> impl Responder {
    match state.db_pool.get() {
        Ok(mut connection) => {
            diesel::insert_into(solution::dsl::solution)
                .values(&data.0)
                .execute(&mut connection)
                .unwrap();
            HttpResponse::Created()
        }
        Err(_) => HttpResponse::InternalServerError(),
    }
}

#[patch("/{solution_id}")]
pub async fn update_solution_by_id(
    path: web::Path<i64>,
    data: web::Json<UpdateSolution>,
    state: web::Data<AppState>,
) -> impl Responder {
    let solution_id = path.into_inner();

    match state.db_pool.get() {
        Ok(mut connection) => {
            let mut solution: Solution = solution::dsl::solution
                .filter(solution::id.eq(solution_id))
                .first(&mut connection)
                .unwrap();

            data.populate_solution(&mut solution);

            diesel::update(&solution)
                .set(&solution)
                .get_result::<Solution>(&mut connection)
                .unwrap();

            HttpResponse::Ok()
        }
        Err(_) => HttpResponse::InternalServerError(),
    }
}

#[delete("/{solution_id}")]
pub async fn delete_solution_by_id(
    path: web::Path<i64>,
    state: web::Data<AppState>,
) -> impl Responder {
    let solution_id = path.into_inner();
    match state.db_pool.get() {
        Ok(mut connection) => {
            if let Err(_) = diesel::update(solution::dsl::solution)
                .filter(solution::id.eq(solution_id))
                .set(solution::archive.eq(true))
                .execute(&mut connection)
            {
                return HttpResponse::InternalServerError();
            }

            HttpResponse::Ok()
        }
        Err(_) => HttpResponse::InternalServerError(),
    }
}
