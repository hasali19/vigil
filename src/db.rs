use serde::{Deserialize, Serialize};
use sqlx::arguments::Arguments;
use sqlx::sqlite::{SqliteArguments, SqliteQueryAs};
use sqlx::SqlitePool;

use crate::Result;

pub struct Db(SqlitePool);

pub async fn connect(url: &str) -> Result<Db> {
    let pool = SqlitePool::new(url).await?;

    sqlx::query(include_str!("schema.sql"))
        .execute(&pool)
        .await?;

    Ok(Db(pool))
}

impl Db {
    pub async fn close(&self) {
        self.0.close().await;
    }
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Host {
    pub id: i32,
    pub name: String,
    pub ip_address: String,
    pub mac_address: String,
}

#[derive(Debug, Deserialize)]
pub struct HostUpdate {
    pub name: Option<String>,
    pub ip_address: Option<String>,
    pub mac_address: Option<String>,
}

impl Host {
    pub async fn get_all(db: &Db) -> Result<Vec<Host>> {
        let hosts = sqlx::query_as("SELECT id, name, ip_address, mac_address FROM hosts")
            .fetch_all(&db.0)
            .await?;

        Ok(hosts)
    }

    pub async fn get_by_id(db: &Db, id: i32) -> Result<Host> {
        let host =
            sqlx::query_as("SELECT id, name, ip_address, mac_address FROM hosts WHERE id = ?")
                .bind(id)
                .fetch_one(&db.0)
                .await?;

        Ok(host)
    }

    pub async fn create(db: &Db, name: &str, ip_address: &str, mac_address: &str) -> Result<Host> {
        let (id,) = sqlx::query_as(
            "
            INSERT INTO hosts (name, ip_address, mac_address)
            VALUES (?, ?, ?);
            SELECT last_insert_rowid();
            ",
        )
        .bind(name)
        .bind(ip_address)
        .bind(mac_address)
        .fetch_one(&db.0)
        .await?;

        Ok(Host {
            id,
            name: name.to_owned(),
            ip_address: ip_address.to_owned(),
            mac_address: mac_address.to_owned(),
        })
    }

    pub async fn update(db: &Db, id: i32, update: HostUpdate) -> Result<Host> {
        let mut query = "UPDATE hosts SET".to_owned();
        let mut args = SqliteArguments::default();

        if let Some(name) = update.name {
            query += " name = ?";
            args.add(name);
        }

        if let Some(ip_address) = update.ip_address {
            query += " ip_address = ?";
            args.add(ip_address);
        }

        if let Some(mac_address) = update.mac_address {
            query += " mac_address = ?";
            args.add(mac_address);
        }

        query += " WHERE id = ?";

        let count = sqlx::query(&query).bind_all(args).execute(&db.0).await?;
        if count == 0 {
            return Err("Failed to update host")?;
        }

        Self::get_by_id(db, id).await
    }

    pub async fn delete(db: &Db, id: i32) -> Result<()> {
        sqlx::query("DELETE FROM hosts WHERE id = ?")
            .bind(id)
            .execute(&db.0)
            .await?;

        Ok(())
    }
}
