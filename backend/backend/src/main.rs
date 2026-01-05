use actix_web::{App, HttpResponse, HttpServer, Responder, get, middleware::Logger, web};
use env_logger::Env;
use utils::config::AppState;
#[get("/")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let app_state = web::Data::new(AppState::new());
    let server_port = app_state.env.server.port;

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(users::urls::urls())
            .service(problems::urls::urls())
            .service(solutions::urls::urls())
            .service(interactions::urls::urls())
            .service(health_check)
            .wrap(Logger::default())
    })
    .bind(("0.0.0.0", server_port))?
    .run()
    .await
}
