use actix_web::web::{Data, Json, Path};
use actix_web::HttpResponse;

use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::db::{self, Db};
use crate::net::{MacAddress, MagicPacket};

#[derive(Debug, Serialize)]
struct Host {
    id: i32,
    name: String,
    ip_address: String,
    mac_address: String,
}

pub async fn get_hosts(db: Data<Db>) -> HttpResponse {
    // TODO: Actual error handling 😦
    match db::Host::get_all(&*db).await {
        Ok(hosts) => HttpResponse::Ok().json(hosts),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "error": e.to_string()
        })),
    }
}

#[derive(Debug, Deserialize)]
pub struct NewHost {
    name: String,
    ip_address: String,
    mac_address: String,
}

pub async fn create_host(model: Json<NewHost>, db: Data<Db>) -> HttpResponse {
    let model = model.into_inner();
    let host = db::Host::create(&*db, &model.name, &model.ip_address, &model.mac_address).await;
    match host {
        Ok(host) => HttpResponse::Ok().json(host),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "error": e.to_string()
        })),
    }
}

pub async fn get_host(params: Path<(i32,)>, db: Data<Db>) -> HttpResponse {
    let (id,) = *params;
    match db::Host::get_by_id(&*db, id).await {
        Ok(host) => HttpResponse::Ok().json(host),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "error": e.to_string()
        })),
    }
}

pub async fn update_host(
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

pub async fn delete_host(params: Path<(i32,)>, db: Data<Db>) -> HttpResponse {
    let (id,) = *params;
    match db::Host::delete(&*db, id).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "error": e.to_string()
        })),
    }
}

pub async fn wake_host(params: Path<(i32,)>, db: Data<Db>) -> HttpResponse {
    let (id,) = *params;
    let host = match db::Host::get_by_id(&*db, id).await {
        Ok(host) => host,
        Err(e) => {
            return HttpResponse::InternalServerError().json(json!({
                "error": e.to_string()
            }))
        }
    };

    let mac_address: MacAddress = host.mac_address.parse().unwrap();
    let packet = MagicPacket::for_mac_address(&mac_address);

    packet.broadcast().await.unwrap();

    HttpResponse::Ok().finish()
}
