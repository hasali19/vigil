mod api;
mod db;
mod net;

use actix_web::middleware::Logger;
use actix_web::web::{self, Data};
use actix_web::{App, HttpResponse, HttpServer};

use env_logger::Env;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[actix_rt::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    env_logger::init_from_env(Env::new().default_filter_or("info"));

    let host = std::env::var("VIGIL_HTTP_HOST").unwrap_or_else(|_| "0.0.0.0".to_owned());
    let port = std::env::var("VIGIL_HTTP_PORT").map_or(8080, |v| v.parse().unwrap());
    let db_url = std::env::var("VIGIL_DB_URL").unwrap_or_else(|_| "sqlite://vigil.db".to_owned());

    log::info!("Starting server at http://{}:{}", host, port);

    let db = Data::new(db::connect(&db_url).await?);

    HttpServer::new({
        let db = Data::clone(&db);
        move || {
            App::new()
                .wrap(Logger::default())
                .app_data(Data::clone(&db))
                .route("/", web::get().to(index))
                .service(
                    web::resource("/api/hosts")
                        .route(web::get().to(api::get_hosts))
                        .route(web::post().to(api::create_host)),
                )
                .service(
                    web::resource("/api/hosts/{id}")
                        .route(web::get().to(api::get_host))
                        .route(web::patch().to(api::update_host))
                        .route(web::delete().to(api::delete_host)),
                )
                .route("/api/hosts/{id}/wake", web::post().to(api::wake_host))
        }
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await?;

    db.close().await;

    Ok(())
}

async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello, world!")
}
