mod db;

use actix_web::middleware::Logger;
use actix_web::web::{self, Data, Json, Path};
use actix_web::{App, HttpResponse, HttpServer};

use env_logger::Env;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::net::UdpSocket;

use db::Db;

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
                        .route(web::get().to(get_hosts))
                        .route(web::post().to(create_host)),
                )
                .service(
                    web::resource("/api/hosts/{id}")
                        .route(web::get().to(get_host))
                        .route(web::patch().to(update_host))
                        .route(web::delete().to(delete_host)),
                )
                .route("/api/hosts/{id}/wake", web::post().to(wake_host))
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

#[derive(Debug, Serialize)]
struct Host {
    id: i32,
    name: String,
    ip_address: String,
    mac_address: String,
}

async fn get_hosts(db: Data<Db>) -> HttpResponse {
    // TODO: Actual error handling 😦
    match db::Host::get_all(&*db).await {
        Ok(hosts) => HttpResponse::Ok().json(hosts),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "error": e.to_string()
        })),
    }
}

#[derive(Debug, Deserialize)]
struct HostCreate {
    name: String,
    ip_address: String,
    mac_address: String,
}

async fn create_host(model: Json<HostCreate>, db: Data<Db>) -> HttpResponse {
    let model = model.into_inner();
    let host = db::Host::create(&*db, &model.name, &model.ip_address, &model.mac_address).await;
    match host {
        Ok(host) => HttpResponse::Ok().json(host),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "error": e.to_string()
        })),
    }
}

async fn get_host(params: Path<(i32,)>, db: Data<Db>) -> HttpResponse {
    let (id,) = *params;
    match db::Host::get_by_id(&*db, id).await {
        Ok(host) => HttpResponse::Ok().json(host),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "error": e.to_string()
        })),
    }
}

async fn update_host(
    params: Path<(i32,)>,
    update: Json<db::HostUpdate>,
    db: Data<Db>,
) -> HttpResponse {
    let (id,) = *params;
    let update = update.into_inner();

    match db::Host::update(&*db, id, update).await {
        Ok(host) => HttpResponse::Ok().json(host),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "error": e.to_string()
        })),
    }
}

async fn delete_host(params: Path<(i32,)>, db: Data<Db>) -> HttpResponse {
    let (id,) = *params;
    match db::Host::delete(&*db, id).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "error": e.to_string()
        })),
    }
}

async fn wake_host(params: Path<(i32,)>, db: Data<Db>) -> HttpResponse {
    let (id,) = *params;
    let host = match db::Host::get_by_id(&*db, id).await {
        Ok(host) => host,
        Err(e) => {
            return HttpResponse::InternalServerError().json(json!({
                "error": e.to_string()
            }))
        }
    };

    let mut mac_address = [0u8; 6];

    for (i, v) in host.mac_address.split(':').enumerate() {
        mac_address[i] = u8::from_str_radix(v, 16).unwrap();
    }

    let mut packet = [0u8; 102];

    // 6 bytes of 0xff...
    for byte in packet.iter_mut().take(6) {
        *byte = 0xff;
    }

    // ...followed by 16 repetitions of the target mac address
    for i in 0..16 {
        for j in 0..6 {
            packet[6 + i * 6 + j] = mac_address[j];
        }
    }

    let mut socket = UdpSocket::bind("0.0.0.0:0").await.unwrap();

    socket.set_broadcast(true).unwrap();
    socket
        .send_to(
            &packet,
            // std::net::SocketAddrV4::new(std::net::Ipv4Addr::BROADCAST, 7),
            "192.168.1.255:9",
        )
        .await
        .unwrap();

    HttpResponse::Ok().finish()
}
