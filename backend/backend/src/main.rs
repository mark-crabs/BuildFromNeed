use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, middleware::Logger};
use env_logger::Env;
use utils::files::{
    load_env,
    types::ENV
};


#[get("/")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let env: ENV = load_env().unwrap();
    HttpServer::new(|| {
        App::new()
            .service(health_check)
            .wrap(Logger::default())
    })
    .bind(("0.0.0.0", env.server_port))?
    .run()
    .await
}