use actix_web::{HttpResponse, Responder, delete, get, patch, post, web};
use diesel::ExpressionMethods;
use diesel::{JoinOnDsl, NullableExpressionMethods, QueryDsl, RunQueryDsl};
use utils::db::schema::users;
use utils::dto::Claims;
use utils::models::Role;
use utils::{
    config::AppState,
    db::schema::problem,
    dto::{DataResponse, Pagination},
};

use crate::dto::{AddProblem, ProblemWithUser, ProblemWithUserOverview, UpdateProblem};
use crate::models::Problem;

// REMEMBER ABOUT PROBLEM LIKE/DIS-LIKES

#[get("")]
pub async fn get_problems(
    web::Query(pagination): web::Query<Pagination>,
    claims: web::ReqData<Option<Claims>>,
    state: web::Data<AppState>,
) -> impl Responder {
    let pagination = pagination.limit_and_offset();
    match state.db_pool.get() {
        Ok(mut connection) => {
            let problems: Vec<ProblemWithUserOverview> = problem::table
                .left_join(users::table.on(problem::user_id.eq(users::id)))
                .select((
                    problem::id,
                    problem::anonymous,
                    problem::title,
                    problem::description,
                    problem::flag,
                    problem::featured_id,
                    problem::category,
                    problem::status,
                    problem::public,
                    problem::archive,
                    problem::created_at,
                    problem::updated_at,
                    users::email.nullable(),
                    users::picture.nullable(),
                    users::name.nullable(),
                ))
                .order(problem::created_at.desc())
                .limit(pagination.limit)
                .offset(pagination.offset)
                .load(&mut connection)
                .unwrap();
            HttpResponse::Ok().json(DataResponse::new(problems))
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// TODO: POPULATE THE VIEWS RECORD ASYNC
#[get("/{problem_id}")]
pub async fn get_problem_by_id(
    path: web::Path<i64>,
    claims: web::ReqData<Option<Claims>>,
    state: web::Data<AppState>,
) -> impl Responder {
    let problem_id = path.into_inner();

    match state.db_pool.get() {
        Ok(mut connection) => {
            let problem: ProblemWithUser = problem::table
                .left_join(users::table.on(problem::user_id.eq(users::id)))
                .select((
                    problem::id,
                    problem::anonymous,
                    problem::title,
                    problem::description,
                    problem::flag,
                    problem::featured_id,
                    problem::category,
                    problem::sub_category,
                    problem::status,
                    problem::public,
                    problem::archive,
                    problem::created_at,
                    problem::updated_at,
                    users::email.nullable(),
                    users::picture.nullable(),
                    users::name.nullable(),
                ))
                .filter(problem::id.eq(problem_id))
                .first(&mut connection)
                .unwrap();

            HttpResponse::Ok().json(problem)
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("")]
pub async fn add_a_problem(
    data: web::Json<AddProblem>,
    claims: web::ReqData<Option<Claims>>,
    state: web::Data<AppState>,
) -> impl Responder {
    match claims.into_inner() {
        None => HttpResponse::Unauthorized(),
        Some(claims) => match state.db_pool.get() {
            Ok(mut connection) => {
                if claims.role != Role::Admin && data.user_id != claims.user_id {
                    return HttpResponse::Forbidden();
                }

                diesel::insert_into(problem::dsl::problem)
                    .values(&data.0)
                    .execute(&mut connection)
                    .unwrap();
                HttpResponse::Created()
            }
            Err(_) => HttpResponse::InternalServerError(),
        },
    }
}

#[patch("/{problem_id}")]
pub async fn update_problem_by_id(
    path: web::Path<i64>,
    data: web::Json<UpdateProblem>,
    claims: web::ReqData<Option<Claims>>,
    state: web::Data<AppState>,
) -> impl Responder {
    match claims.into_inner() {
        None => HttpResponse::Unauthorized(),
        Some(claims) => {
            let problem_id = path.into_inner();
            match state.db_pool.get() {
                Ok(mut connection) => {
                    let mut problem: Problem = problem::dsl::problem
                        .filter(problem::id.eq(problem_id))
                        .first(&mut connection)
                        .unwrap();

                    data.populate_problem(&mut problem);

                    if claims.role == Role::Admin || claims.user_id == problem.user_id {
                        diesel::update(&problem)
                            .set(&problem)
                            .get_result::<Problem>(&mut connection)
                            .unwrap();
                    } else {
                        return HttpResponse::Forbidden();
                    }

                    HttpResponse::Ok()
                }
                Err(_) => HttpResponse::InternalServerError(),
            }
        }
    }
}

#[delete("/{problem_id}")]
pub async fn delete_problem_by_id(
    path: web::Path<i64>,
    claims: web::ReqData<Option<Claims>>,
    state: web::Data<AppState>,
) -> impl Responder {
    match claims.into_inner() {
        None => HttpResponse::Unauthorized(),
        Some(claims) => {
            let problem_id = path.into_inner();
            match state.db_pool.get() {
                Ok(mut connection) => {
                    if claims.role == Role::Admin {
                        if let Err(_) = diesel::update(problem::dsl::problem)
                            .filter(problem::id.eq(problem_id))
                            .set(problem::archive.eq(true))
                            .execute(&mut connection)
                        {
                            return HttpResponse::InternalServerError();
                        }
                    } else {
                        if let Err(_) = diesel::update(problem::dsl::problem)
                            .filter(problem::id.eq(problem_id))
                            .filter(problem::user_id.eq(claims.user_id))
                            .set(problem::archive.eq(true))
                            .execute(&mut connection)
                        {
                            return HttpResponse::InternalServerError();
                        }
                    }

                    HttpResponse::Ok()
                }
                Err(_) => HttpResponse::InternalServerError(),
            }
        }
    }
}
