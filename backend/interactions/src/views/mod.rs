use crate::{
    dto::{
        LikeRevoke, ProblemFavourite, ProblemLike, ProblemView, SolutionFavourite, SolutionLike,
    },
    models::{
        ProblemFavourite as ProblemFavouriteModel, ProblemLike as ProblemLikeModel,
        SolutionFavourite as SolutionFavouriteModel, SolutionLike as SolutionLikeModel,
    },
};
use actix_web::{HttpResponse, Responder, get, post, web};
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, r2d2::PooledConnection};
use utils::{
    config::AppState,
    db::schema::{
        problem_favourite, problem_like, problem_view, solution_favourite, solution_like,
    },
    dto::{Claims, DataResponse, Pagination}, models::Role,
};

// FIND A WAY TO MAKE THE LIKE AND DISLIKE FEATURE EXTENDABLE SAME AS FAV/UNFAV
#[get("/problem-likes")]
pub async fn get_problem_likes(
    web::Query(pagination): web::Query<Pagination>,
    claims: web::ReqData<Option<Claims>>,
    state: web::Data<AppState>
) -> impl Responder {
    match claims.into_inner() {
        None => HttpResponse::Unauthorized().finish(),
        Some(claims) => {
            let pagination = pagination.limit_and_offset();

            match state.db_pool.get() {
                Ok(mut connection) => {

                    if claims.role != Role::Admin {
                        return HttpResponse::Forbidden().finish();
                    }

                    let likes: Vec<ProblemLikeModel> = problem_like::dsl::problem_like
                        .order(problem_like::created_at.desc())
                        .limit(pagination.limit)
                        .offset(pagination.offset)
                        .load(&mut connection)
                        .unwrap();

                    HttpResponse::Ok().json(DataResponse::new(likes))
                }
                Err(_) => HttpResponse::InternalServerError().finish(),
            }
        }
    }
}

#[post("/problem-like")]
pub async fn like_a_problem(
    web::Query(revoke): web::Query<LikeRevoke>,
    data: web::Json<ProblemLike>,
    claims: web::ReqData<Option<Claims>>,
    state: web::Data<AppState>
) -> impl Responder {
    match claims.into_inner() {
        None => HttpResponse::Unauthorized(),
        Some(claims) => match state.db_pool.get() {
            Ok(mut connection) => {

                if claims.role != Role::Admin && data.user_id != claims.user_id {
                    return HttpResponse::Forbidden()
                }

                like_and_unlike_a_problem(&data.0, &revoke, &mut connection);
                HttpResponse::Created()
            }
            Err(_) => HttpResponse::InternalServerError(),
        },
    }
}

#[get("/solution-likes")]
pub async fn get_solution_likes(
    web::Query(pagination): web::Query<Pagination>,
    claims: web::ReqData<Option<Claims>>,
    state: web::Data<AppState>
) -> impl Responder {
    match claims.into_inner() {
        None => HttpResponse::Unauthorized().finish(),
        Some(claims) => {
            let pagination = pagination.limit_and_offset();
            match state.db_pool.get() {
                Ok(mut connection) => {

                    if claims.role != Role::Admin {
                        return HttpResponse::Forbidden().finish();
                    }
                    
                    let likes: Vec<SolutionLikeModel> = solution_like::dsl::solution_like
                        .order(solution_like::created_at.desc())
                        .limit(pagination.limit)
                        .offset(pagination.offset)
                        .load(&mut connection)
                        .unwrap();

                    HttpResponse::Ok().json(DataResponse::new(likes))
                }
                Err(_) => HttpResponse::InternalServerError().finish(),
            }
        }
    }
}

#[post("/solution-like")]
pub async fn like_a_solution(
    web::Query(revoke): web::Query<LikeRevoke>,
    data: web::Json<SolutionLike>,
    claims: web::ReqData<Option<Claims>>,
    state: web::Data<AppState>
) -> impl Responder {
    match claims.into_inner() {
        None => HttpResponse::Unauthorized(),
        Some(claims) => match state.db_pool.get() {
            Ok(mut connection) => {

                if claims.role != Role::Admin && data.user_id != claims.user_id {
                    return HttpResponse::Forbidden()
                }

                like_and_unlike_a_solution(&data.0, &revoke, &mut connection);
                HttpResponse::Ok()
            }
            Err(_) => HttpResponse::InternalServerError(),
        },
    }
}

#[post("/problem-view")]
pub async fn view_a_problem(
    data: web::Json<ProblemView>,
    claims: web::ReqData<Option<Claims>>,
    state: web::Data<AppState>
) -> impl Responder {
    match claims.into_inner() {
        None => HttpResponse::Unauthorized(),
        Some(claims) => match state.db_pool.get() {
            Ok(mut connection) => {
                diesel::insert_into(problem_view::dsl::problem_view)
                    .values(&data.0)
                    .execute(&mut connection)
                    .unwrap();
                HttpResponse::Created()
            }
            Err(_) => HttpResponse::InternalServerError(),
        },
    }
}

// IMPROVE TO INCLUDE SOME USER/PROBLEM DATA
#[get("problem-favourites")]
pub async fn get_problems_favourites(
    web::Query(pagination): web::Query<Pagination>,
    claims: web::ReqData<Option<Claims>>,
    state: web::Data<AppState>
) -> impl Responder {
    match claims.into_inner() {
        None => HttpResponse::Unauthorized().finish(),
        Some(claims) => {

            let pagination = pagination.limit_and_offset();
            match state.db_pool.get() {
                Ok(mut connection) => {

                    let favs: Vec<ProblemFavouriteModel> = if claims.role == Role::Admin {
                        problem_favourite::dsl::problem_favourite
                            .order(problem_favourite::created_at.desc())
                            .limit(pagination.limit)
                            .offset(pagination.offset)
                            .load(&mut connection)
                            .unwrap()
                    } else {
                        problem_favourite::dsl::problem_favourite
                            .filter(problem_favourite::user_id.eq(claims.user_id))
                            .order(problem_favourite::created_at.desc())
                            .limit(pagination.limit)
                            .offset(pagination.offset)
                            .load(&mut connection)
                            .unwrap()
                    };

                    HttpResponse::Ok().json(DataResponse::new(favs))
                }
                Err(_) => HttpResponse::InternalServerError().finish(),
            }
        }
    }
}

#[get("problem-favourites/{problem_id}")]
pub async fn get_problem_favourites_by_id(
    path: web::Path<i64>,
    web::Query(pagination): web::Query<Pagination>,
    claims: web::ReqData<Option<Claims>>,
    state: web::Data<AppState>
) -> impl Responder {
    match claims.into_inner() {
        None => HttpResponse::Unauthorized().finish(),
        Some(claims) => {
            if claims.role != Role::Admin {
                return HttpResponse::Forbidden().finish();
            }
            let pagination = pagination.limit_and_offset();
            let problem_id = path.into_inner();
            match state.db_pool.get() {
                Ok(mut connection) => {
                    let favs: Vec<ProblemFavouriteModel> =
                        problem_favourite::dsl::problem_favourite
                            .filter(problem_favourite::problem_id.eq(problem_id))
                            .order(problem_favourite::created_at.desc())
                            .limit(pagination.limit)
                            .offset(pagination.offset)
                            .load(&mut connection)
                            .unwrap();
                    HttpResponse::Ok().json(DataResponse::new(favs))
                }
                Err(_) => HttpResponse::InternalServerError().finish(),
            }
        }
    }
}

#[post("problem-favourite")]
pub async fn favourite_a_problem(
    data: web::Json<ProblemFavourite>,
    claims: web::ReqData<Option<Claims>>,
    state: web::Data<AppState>
) -> impl Responder {
    match claims.into_inner() {
        None => HttpResponse::Unauthorized(),
        Some(claims) => match state.db_pool.get() {
            Ok(mut connection) => {

                if claims.role != Role::Admin && data.user_id != claims.user_id {
                    return HttpResponse::Forbidden()
                }

                favourite_and_unfavourite_a_problem(&data.0, &mut connection);
                HttpResponse::Created()
            }
            Err(_) => HttpResponse::InternalServerError(),
        },
    }
}

#[get("solution-favourites")]
pub async fn get_solutions_favourites(
    web::Query(pagination): web::Query<Pagination>,
    claims: web::ReqData<Option<Claims>>,
    state: web::Data<AppState>
) -> impl Responder {
    match claims.into_inner() {
        None => HttpResponse::Unauthorized().finish(),
        Some(claims) => {
            let pagination = pagination.limit_and_offset();
            match state.db_pool.get() {
                Ok(mut connection) => {


                    let favs: Vec<SolutionFavouriteModel> = if claims.role == Role::Admin {
                        solution_favourite::dsl::solution_favourite
                        .order(solution_favourite::created_at.desc())
                        .limit(pagination.limit)
                        .offset(pagination.offset)
                        .load(&mut connection)
                        .unwrap()
                    } else {
                        solution_favourite::dsl::solution_favourite
                        .filter(solution_favourite::user_id.eq(claims.user_id))
                        .order(solution_favourite::created_at.desc())
                        .limit(pagination.limit)
                        .offset(pagination.offset)
                        .load(&mut connection)
                        .unwrap()
                    };
                    HttpResponse::Ok().json(DataResponse::new(favs))
                }
                Err(_) => HttpResponse::InternalServerError().finish(),
            }
        }
    }
}

#[get("solution-favourites/{solution_id}")]
pub async fn get_solution_favourites_by_id(
    path: web::Path<i64>,
    web::Query(pagination): web::Query<Pagination>,
    claims: web::ReqData<Option<Claims>>,
    state: web::Data<AppState>
) -> impl Responder {
    match claims.into_inner() {
        None => HttpResponse::Unauthorized().finish(),
        Some(claims) => {
            if claims.role != Role::Admin {
                return HttpResponse::Forbidden().finish();
            }
            let pagination = pagination.limit_and_offset();
            let solution_id = path.into_inner();
            match state.db_pool.get() {
                Ok(mut connection) => {
                    let favs: Vec<SolutionFavouriteModel> =
                        solution_favourite::dsl::solution_favourite
                            .filter(solution_favourite::solution_id.eq(solution_id))
                            .order(solution_favourite::created_at.desc())
                            .limit(pagination.limit)
                            .offset(pagination.offset)
                            .load(&mut connection)
                            .unwrap();
                    HttpResponse::Ok().json(DataResponse::new(favs))
                }
                Err(_) => HttpResponse::InternalServerError().finish(),
            }
        }
    }
}

#[post("solution-favourite")]
pub async fn favourite_a_solution(
    data: web::Json<SolutionFavourite>,
    claims: web::ReqData<Option<Claims>>,
    state: web::Data<AppState>
) -> impl Responder {
    match claims.into_inner() {
        None => HttpResponse::Unauthorized(),
        Some(claims) => match state.db_pool.get() {
            Ok(mut connection) => {
                if claims.role != Role::Admin && data.user_id != claims.user_id {
                    return HttpResponse::Forbidden()
                }
                favourite_and_unfavourite_a_solution(&data.0, &mut connection);
                HttpResponse::Created()
            }
            Err(_) => HttpResponse::InternalServerError(),
        },
    }
}

// FIND A WAY TO MERGE PROBLEM AND SOLUTION LIKE/DIS and FAV/UNFAV
pub fn like_and_unlike_a_problem(
    data: &ProblemLike,
    revoke: &LikeRevoke,
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
) {
    let exists: Vec<ProblemLikeModel> = problem_like::dsl::problem_like
        .filter(problem_like::problem_id.eq(data.problem_id))
        .filter(problem_like::user_id.eq(data.user_id))
        .load(connection)
        .unwrap();

    if exists.is_empty() {
        if revoke.revoke.is_none() {
            diesel::insert_into(problem_like::dsl::problem_like)
                .values(data)
                .execute(connection)
                .unwrap();
        }
    } else {
        let record = exists.first().unwrap();

        match revoke.revoke {
            Some(revoke) => {
                if revoke {
                    diesel::delete(problem_like::dsl::problem_like)
                        .filter(problem_like::id.eq(record.id))
                        .execute(connection)
                        .unwrap();
                } else {
                    diesel::update(problem_like::dsl::problem_like)
                        .filter(problem_like::problem_id.eq(data.problem_id))
                        .filter(problem_like::user_id.eq(data.user_id))
                        .set(problem_like::option.eq(data.option))
                        .execute(connection)
                        .unwrap();
                }
            }
            None => {
                if record.option != data.option {
                    diesel::update(problem_like::dsl::problem_like)
                        .filter(problem_like::problem_id.eq(data.problem_id))
                        .filter(problem_like::user_id.eq(data.user_id))
                        .set(problem_like::option.eq(data.option))
                        .execute(connection)
                        .unwrap();
                }
            }
        }
    }
}

pub fn like_and_unlike_a_solution(
    data: &SolutionLike,
    revoke: &LikeRevoke,
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
) {
    let exists: Vec<SolutionLikeModel> = solution_like::dsl::solution_like
        .filter(solution_like::solution_id.eq(data.solution_id))
        .filter(solution_like::user_id.eq(data.user_id))
        .load(connection)
        .unwrap();

    if exists.is_empty() {
        diesel::insert_into(solution_like::dsl::solution_like)
            .values(data)
            .execute(connection)
            .unwrap();
    } else {
        let record = exists.first().unwrap();
        match revoke.revoke {
            Some(revoke) => {
                if revoke {
                    diesel::delete(solution_like::dsl::solution_like)
                        .filter(solution_like::id.eq(record.id))
                        .execute(connection)
                        .unwrap();
                } else {
                    diesel::update(solution_like::dsl::solution_like)
                        .filter(solution_like::solution_id.eq(data.solution_id))
                        .filter(solution_like::user_id.eq(data.user_id))
                        .set(solution_like::option.eq(data.option))
                        .execute(connection)
                        .unwrap();
                }
            }
            None => {
                if record.option != data.option {
                    diesel::update(solution_like::dsl::solution_like)
                        .filter(solution_like::solution_id.eq(data.solution_id))
                        .filter(solution_like::user_id.eq(data.user_id))
                        .set(solution_like::option.eq(data.option))
                        .execute(connection)
                        .unwrap();
                }
            }
        }
    }
}

pub fn favourite_and_unfavourite_a_problem(
    data: &ProblemFavourite,
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
) {
    let exists: Vec<ProblemFavouriteModel> = problem_favourite::dsl::problem_favourite
        .filter(problem_favourite::problem_id.eq(data.problem_id))
        .filter(problem_favourite::user_id.eq(data.user_id))
        .load(connection)
        .unwrap();

    if exists.is_empty() {
        diesel::insert_into(problem_favourite::dsl::problem_favourite)
            .values(data)
            .execute(connection)
            .unwrap();
    } else {
        let record = exists.first().unwrap();
        diesel::delete(problem_favourite::dsl::problem_favourite)
            .filter(problem_favourite::id.eq(record.id))
            .execute(connection)
            .unwrap();
    }
}

pub fn favourite_and_unfavourite_a_solution(
    data: &SolutionFavourite,
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
) {
    let exists: Vec<SolutionFavouriteModel> = solution_favourite::dsl::solution_favourite
        .filter(solution_favourite::solution_id.eq(data.solution_id))
        .filter(solution_favourite::user_id.eq(data.user_id))
        .load(connection)
        .unwrap();

    if exists.is_empty() {
        diesel::insert_into(solution_favourite::dsl::solution_favourite)
            .values(data)
            .execute(connection)
            .unwrap();
    } else {
        let record = exists.first().unwrap();
        diesel::delete(solution_favourite::dsl::solution_favourite)
            .filter(solution_favourite::id.eq(record.id))
            .execute(connection)
            .unwrap();
    }
}
