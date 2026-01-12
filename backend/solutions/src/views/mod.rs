use crate::{
    dto::{AddSolution, SolutionWithOverview, UpdateSolution},
    models::Solution,
};
use actix_web::{HttpResponse, Responder, delete, get, patch, post, web};
use diesel::sql_query;
use diesel::sql_types::BigInt;
use diesel::{
    ExpressionMethods, RunQueryDsl,
    query_dsl::methods::{FilterDsl},
};
use utils::{
    config::AppState,
    db::schema::{solution},
    dto::{Claims, DataResponse, Pagination},
    models::Role,
};

#[get("")]
pub async fn get_all_solutions(
    web::Query(pagination): web::Query<Pagination>,
    claims: web::ReqData<Option<Claims>>,
    state: web::Data<AppState>,
) -> impl Responder {
    match claims.into_inner() {
        None => HttpResponse::Unauthorized().finish(),
        Some(claims) => {
            let pagination = pagination.limit_and_offset();
            match state.db_pool.get() {
                Ok(mut connection) => {
                    let solutions: Vec<SolutionWithOverview> = if claims.role == Role::Admin {
                        sql_query(r#"
    SELECT
        s.id,
        s.problem_id,
        s.user_id,
        s.content,
        s.archive,
        s.created_at,
        s.updated_at,
        -- upvotes
        (SELECT COUNT(*) FROM solution_like sl WHERE sl.solution_id = s.id AND sl.option = 'Up') AS upvotes,
        -- downvotes
        (SELECT COUNT(*) FROM solution_like sl WHERE sl.solution_id = s.id AND sl.option = 'Down') AS downvotes
    FROM solution s
    ORDER BY s.created_at DESC
    LIMIT $1 OFFSET $2
"#)
.bind::<BigInt, _>(pagination.limit)
.bind::<BigInt, _>(pagination.offset)
.load(&mut connection)
.unwrap()
                    } else {
                        sql_query(r#"
                            SELECT
                                s.id,
                                s.problem_id,
                                s.user_id,
                                s.content,
                                s.archive,
                                s.created_at,
                                s.updated_at,
                                (SELECT COUNT(*) FROM solution_like sl WHERE sl.solution_id = s.id AND sl.option = 'Up') AS upvotes,
                                (SELECT COUNT(*) FROM solution_like sl WHERE sl.solution_id = s.id AND sl.option = 'Down') AS downvotes
                            FROM solution s
                            WHERE s.user_id = $1
                            ORDER BY s.created_at DESC
                            LIMIT $2 OFFSET $3
                        "#)
                        .bind::<BigInt, _>(claims.user_id)
                        .bind::<BigInt, _>(pagination.limit)
                        .bind::<BigInt, _>(pagination.offset)
                        .load(&mut connection)
                        .unwrap()
                    };

                    HttpResponse::Ok().json(DataResponse::new(solutions))
                }
                Err(_) => HttpResponse::InternalServerError().finish(),
            }
        }
    }
}

#[get("/p/{problem_id}")]
pub async fn get_solution_by_problem_id(
    path: web::Path<i64>,
    web::Query(pagination): web::Query<Pagination>,
    claims: web::ReqData<Option<Claims>>,
    state: web::Data<AppState>,
) -> impl Responder {
    let pagination = pagination.limit_and_offset();
    let problem_id = path.into_inner();
    match state.db_pool.get() {
        Ok(mut connection) => {
            let results: Vec<SolutionWithOverview> = sql_query(r#"
                    SELECT
                        s.id,
                        s.problem_id,
                        s.user_id,
                        s.content,
                        s.archive,
                        s.created_at,
                        s.updated_at,
                        (SELECT COUNT(*) FROM solution_like sl WHERE sl.solution_id = s.id AND sl.option = 'Up') AS upvotes,
                        (SELECT COUNT(*) FROM solution_like sl WHERE sl.solution_id = s.id AND sl.option = 'Down') AS downvotes
                    FROM solution s
                    WHERE s.problem_id = $1
                    ORDER BY s.created_at DESC
                    LIMIT $2 OFFSET $3
                "#)
                .bind::<BigInt, _>(problem_id)
                .bind::<BigInt, _>(pagination.limit)
                .bind::<BigInt, _>(pagination.offset)
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
                diesel::insert_into(solution::dsl::solution)
                    .values(&data.0)
                    .execute(&mut connection)
                    .unwrap();
                HttpResponse::Created()
            }
            Err(_) => HttpResponse::InternalServerError(),
        },
    }
}

#[patch("/{solution_id}")]
pub async fn update_solution_by_id(
    path: web::Path<i64>,
    data: web::Json<UpdateSolution>,
    claims: web::ReqData<Option<Claims>>,
    state: web::Data<AppState>,
) -> impl Responder {
    match claims.into_inner() {
        None => HttpResponse::Unauthorized(),
        Some(claims) => {
            let solution_id = path.into_inner();

            match state.db_pool.get() {
                Ok(mut connection) => {
                    let mut solution: Solution = solution::dsl::solution
                        .filter(solution::id.eq(solution_id))
                        .first(&mut connection)
                        .unwrap();

                    data.populate_solution(&mut solution);

                    if claims.role != Role::Admin && solution.user_id != claims.user_id {
                        return HttpResponse::Forbidden();
                    }

                    diesel::update(&solution)
                        .set(&solution)
                        .get_result::<Solution>(&mut connection)
                        .unwrap();

                    HttpResponse::Ok()
                }
                Err(_) => HttpResponse::InternalServerError(),
            }
        }
    }
}

#[delete("/{solution_id}")]
pub async fn delete_solution_by_id(
    path: web::Path<i64>,
    claims: web::ReqData<Option<Claims>>,
    state: web::Data<AppState>,
) -> impl Responder {
    match claims.into_inner() {
        None => HttpResponse::Unauthorized(),
        Some(claims) => {
            let solution_id = path.into_inner();
            match state.db_pool.get() {
                Ok(mut connection) => {
                    if claims.role == Role::Admin {
                        if let Err(_) = diesel::update(solution::dsl::solution)
                            .filter(solution::id.eq(solution_id))
                            .set(solution::archive.eq(true))
                            .execute(&mut connection)
                        {
                            return HttpResponse::InternalServerError();
                        }
                    } else {
                        if let Err(_) = diesel::update(solution::dsl::solution)
                            .filter(solution::id.eq(solution_id))
                            .filter(solution::user_id.eq(claims.user_id))
                            .set(solution::archive.eq(true))
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
