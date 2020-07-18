use actix_web::middleware::Logger;
use actix_web::web;
use actix_web::{App, HttpResponse, HttpServer};

use env_logger::Env;

#[actix_rt::main]
async fn main() -> Result<(), impl std::error::Error> {
    dotenv::dotenv().ok();

    env_logger::init_from_env(Env::new().default_filter_or("info"));

    let host = "127.0.0.1";
    let port = 8080;

    log::info!("Starting server at http://{}:{}", host, port);

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .route("/", web::get().to(index))
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}

async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello, world!")
}
