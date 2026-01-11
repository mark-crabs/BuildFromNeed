use crate::{
    dto::{AddUser, UpdateUser},
    models::User,
};
use actix_web::HttpMessage;
use actix_web::{HttpRequest, HttpResponse, Responder, delete, get, patch, post, web};
use diesel::{
    ExpressionMethods, RunQueryDsl,
    query_dsl::methods::{FilterDsl, LimitDsl, OffsetDsl, OrderDsl},
};
use jsonwebtoken::TokenData;
use utils::{
    config::AppState,
    db::schema::users,
    dto::{Claims, DataResponse, Pagination},
};

#[get("")]
pub async fn get_users(
    web::Query(pagination): web::Query<Pagination>,
    claims: web::ReqData<Option<Claims>>,
    state: web::Data<AppState>,
    req: HttpRequest,
) -> impl Responder {
    let pagination = pagination.limit_and_offset();

    println!("{:#?}", claims);

    match state.db_pool.get() {
        Ok(mut connection) => {
            let users: Vec<User> = users::dsl::users
                .limit(pagination.limit)
                .offset(pagination.offset)
                .order(users::created_at.desc())
                .load(&mut connection)
                .unwrap();
            HttpResponse::Ok().json(DataResponse::new(users))
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/{user_id}")]
pub async fn get_user_by_id(
    path: web::Path<i64>,
    claims: web::ReqData<Option<Claims>>,
    state: web::Data<AppState>,
    req: HttpRequest,
) -> impl Responder {
    let user_id = path.into_inner();
    match state.db_pool.get() {
        Ok(mut connection) => {
            let user: User = users::dsl::users
                .filter(users::id.eq(user_id))
                .get_result(&mut connection)
                .unwrap();
            HttpResponse::Ok().json(DataResponse::new(user))
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("")]
pub async fn add_user(
    data: web::Json<AddUser>,
    claims: web::ReqData<Option<Claims>>,
    state: web::Data<AppState>,
    req: HttpRequest,
) -> impl Responder {
    match state.db_pool.get() {
        Ok(mut connection) => {
            diesel::insert_into(users::dsl::users)
                .values(&data.0)
                .execute(&mut connection)
                .unwrap();
            HttpResponse::Created()
        }
        Err(_) => HttpResponse::InternalServerError(),
    }
}

// DONT UPDATE IF USER IS ARCHIVED BUT YOU CAN IF YOURE AN ADMIN
#[patch("/{user_id}")]
pub async fn update_user_by_id(
    path: web::Path<i64>,
    data: web::Json<UpdateUser>,
    claims: web::ReqData<Option<Claims>>,
    state: web::Data<AppState>,
    req: HttpRequest,
) -> impl Responder {
    let user_id = path.into_inner();
    match state.db_pool.get() {
        Ok(mut connection) => {
            let mut user: User = users::dsl::users
                .filter(users::id.eq(user_id))
                .first(&mut connection)
                .unwrap();

            data.populate_user(&mut user);

            diesel::update(&user)
                .set(&user)
                .get_result::<User>(&mut connection)
                .unwrap();

            HttpResponse::Ok()
        }
        Err(_) => HttpResponse::InternalServerError(),
    }
}

#[delete("/{user_id}")]
pub async fn delete_user_by_id(
    path: web::Path<i64>,
    claims: web::ReqData<Option<Claims>>,
    state: web::Data<AppState>,
    req: HttpRequest,
) -> impl Responder {
    let user_id = path.into_inner();
    match state.db_pool.get() {
        Ok(mut connection) => {
            if let Err(_) = diesel::update(users::dsl::users)
                .filter(users::id.eq(user_id))
                .set(users::archive.eq(true))
                .execute(&mut connection)
            {
                return HttpResponse::InternalServerError();
            }
            HttpResponse::Ok()
        }
        Err(_) => HttpResponse::InternalServerError(),
    }
}
