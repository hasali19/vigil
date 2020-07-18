use std::error::Error;
use std::sync::Mutex;

use actix_web::middleware::Logger;
use actix_web::web::{self, Data, Json, Path};
use actix_web::{App, HttpResponse, HttpServer};

use env_logger::Env;
use serde::{Deserialize, Serialize};
use serde_json::json;

type Db = Mutex<Vec<Host>>;

#[actix_rt::main]
async fn main() -> Result<(), impl Error> {
    dotenv::dotenv().ok();

    env_logger::init_from_env(Env::new().default_filter_or("info"));

    let host = "127.0.0.1";
    let port = 8080;

    log::info!("Starting server at http://{}:{}", host, port);

    let id = Data::new(Mutex::new(0));
    let db = Data::new(Db::new(vec![]));

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(Data::clone(&id))
            .app_data(Data::clone(&db))
            .route("/", web::get().to(index))
            .service(
                web::resource("/api/hosts")
                    .route(web::get().to(get_hosts))
                    .route(web::post().to(create_host)),
            )
            .service(
                web::resource("/api/hosts/{id}")
                    .route(web::get().to(get_host))
                    .route(web::patch().to(update_host))
                    .route(web::delete().to(delete_host)),
            )
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}

async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello, world!")
}

#[derive(Debug, Serialize)]
struct Host {
    id: i32,
    name: String,
    ip_address: String,
    mac_address: String,
}

async fn get_hosts(db: Data<Db>) -> HttpResponse {
    let hosts = db.lock().unwrap();
    HttpResponse::Ok().json(&*hosts)
}

#[derive(Debug, Deserialize)]
struct HostCreate {
    name: String,
    ip_address: String,
    mac_address: String,
}

async fn create_host(model: Json<HostCreate>, id: Data<Mutex<i32>>, db: Data<Db>) -> HttpResponse {
    let model = model.into_inner();
    let db = db.into_inner();
    let mut hosts = db.lock().unwrap();

    let id = {
        let mut id = id.lock().unwrap();
        let next_id = *id + 1;
        *id = next_id;
        next_id
    };

    hosts.push(Host {
        id,
        name: model.name,
        ip_address: model.ip_address,
        mac_address: model.mac_address,
    });

    HttpResponse::Ok().json(hosts.last())
}

async fn get_host(params: Path<(i32,)>, db: Data<Db>) -> HttpResponse {
    let (id,) = *params;
    let db = db.into_inner();
    let hosts = db.lock().unwrap();
    match hosts.iter().find(|h| h.id == id) {
        Some(host) => HttpResponse::Ok().json(host),
        None => HttpResponse::NotFound().json(json!({
            "error": "No host found with the given id"
        })),
    }
}

#[derive(Debug, Deserialize)]
struct HostUpdate {
    name: Option<String>,
    ip_address: Option<String>,
    mac_address: Option<String>,
}

async fn update_host(params: Path<(i32,)>, model: Json<HostUpdate>, db: Data<Db>) -> HttpResponse {
    let (id,) = *params;
    let model = model.into_inner();
    let db = db.into_inner();
    let mut hosts = db.lock().unwrap();

    let host = match hosts.iter_mut().find(|h| h.id == id) {
        Some(host) => host,
        None => return HttpResponse::NotFound().finish(),
    };

    if let Some(name) = model.name {
        host.name = name;
    }

    if let Some(ip_address) = model.ip_address {
        host.ip_address = ip_address;
    }

    if let Some(mac_address) = model.mac_address {
        host.mac_address = mac_address;
    }

    HttpResponse::Ok().json(host)
}

async fn delete_host(params: Path<(i32,)>, db: Data<Db>) -> HttpResponse {
    let (id,) = *params;
    let db = db.into_inner();
    let mut hosts = db.lock().unwrap();
    hosts.retain(|h| h.id != id);
    HttpResponse::NoContent().finish()
}
