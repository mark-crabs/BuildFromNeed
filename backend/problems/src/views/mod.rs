use actix_web::{HttpResponse, Responder, delete, get, patch, post, web};
use diesel::ExpressionMethods;
use diesel::sql_query;
use diesel::sql_types::BigInt;
use diesel::{QueryDsl, RunQueryDsl};
use utils::db::schema::{problem_like};
use utils::dto::Claims;
use utils::models::Role;
use utils::{
    config::AppState,
    db::schema::problem,
    dto::{DataResponse, Pagination},
};

use crate::dto::{
    AddProblem, AddProblemView, ProblemWithUserOverview, UpdateProblem,
};
use crate::models::{Problem, ProblemLike};

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
            let problems: Vec<ProblemWithUserOverview> = sql_query(r#"
                    SELECT
                        p.id,
                        p.anonymous,
                        p.title,
                        p.description,
                        p.flag,
                        p.featured_id,
                        p.category,
                        p.status,
                        p.public,
                        p.archive,
                        p.created_at,
                        p.updated_at,
                
                        u.email,
                        u.picture,
                        u.name,
                
                        -- upvotes
                        (SELECT COUNT(*) FROM problem_like pl WHERE pl.problem_id = p.id AND pl.option = 'Up') AS upvotes,
                        -- downvotes
                        (SELECT COUNT(*) FROM problem_like pl WHERE pl.problem_id = p.id AND pl.option = 'Dowm') AS downvotes,
                        -- solution count
                        (SELECT COUNT(*) FROM solution s WHERE s.problem_id = p.id) AS solution_count
                
                    FROM problem p
                    LEFT JOIN users u ON p.user_id = u.id
                    ORDER BY p.created_at DESC
                    LIMIT $1 OFFSET $2
                "#)
                .bind::<BigInt, _>(pagination.limit)
                .bind::<BigInt, _>(pagination.offset)
                .load(&mut connection)
                .unwrap();
            HttpResponse::Ok().json(DataResponse::new(problems))
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// TODO: get some solutions belonging to the problem
#[get("/{problem_id}")]
pub async fn get_problem_by_id(
    path: web::Path<i64>,
    claims: web::ReqData<Option<Claims>>,
    state: web::Data<AppState>,
) -> impl Responder {
    let problem_id = path.into_inner();

    match state.db_pool.get() {
        Ok(mut connection) => {
            let problem: ProblemWithUserOverview = sql_query(r#"
            SELECT
                p.id,
                p.anonymous,
                p.title,
                p.description,
                p.flag,
                p.featured_id,
                p.category,
                p.sub_category,
                p.status,
                p.public,
                p.archive,
                p.created_at,
                p.updated_at,
        
                u.email,
                u.picture,
                u.name,
        
                -- upvotes
                (SELECT COUNT(*) FROM problem_like pl WHERE pl.problem_id = p.id AND pl.option = 'Up') AS upvotes,
                -- downvotes
                (SELECT COUNT(*) FROM problem_like pl WHERE pl.problem_id = p.id AND pl.option = 'Down') AS downvotes,
                -- solution count
                (SELECT COUNT(*) FROM solution s WHERE s.problem_id = p.id) AS solution_count
        
            FROM problem p
            LEFT JOIN users u ON p.user_id = u.id
            WHERE p.id = $1
        "#)
        .bind::<BigInt, _>(problem_id)
        .get_result(&mut connection)
        .unwrap();

            // TODO: Place this an async task (populates views)
            if let Some(claims) = claims.into_inner() {
                let like: Vec<ProblemLike> = problem_like::dsl::problem_like
                    .filter(problem_like::problem_id.eq(problem_id))
                    .filter(problem_like::user_id.eq(claims.user_id))
                    .load(&mut connection)
                    .unwrap();

                if like.is_empty() {
                    diesel::insert_into(problem_like::dsl::problem_like)
                        .values(AddProblemView {
                            problem_id,
                            user_id: claims.user_id,
                        })
                        .execute(&mut connection)
                        .unwrap();
                }
            }

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
