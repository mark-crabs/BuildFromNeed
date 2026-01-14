pub mod dto;
pub mod middleware;
pub mod urls;
pub mod views;

use self::middleware::AuthorizationBase;
use self::urls::auth_urls;
use actix_web::{App, HttpResponse, HttpServer, Responder, get, http::header, middleware::Logger, web};
use env_logger::Env;
use utils::{config::AppState, ssl::ssl_builder_creation};
use actix_cors::Cors;
 
#[get("/")]
async fn health_check() -> impl Responder {
    // some();
    HttpResponse::Ok().body("Up")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let app_state = web::Data::new(AppState::new());
    let server_port = app_state.env.server.port;

    let builder = ssl_builder_creation()
        .await
        .expect("Failed to create ssl builder.");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(&[header::AUTHORIZATION, header::ACCEPT, header::CONTENT_TYPE])
            .max_age(3600);
        App::new()
            .wrap(cors)
            .wrap(AuthorizationBase)
            .app_data(app_state.clone())
            .service(auth_urls())
            .service(users::urls::urls())
            .service(problems::urls::urls())
            .service(solutions::urls::urls())
            .service(interactions::urls::urls())
            .service(health_check)
            .wrap(Logger::default())
    })
    .bind_openssl(format!("0.0.0.0:{:?}", server_port), builder)?
    .run()
    .await
}
