use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, PgPool};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub given_name: String,
    pub family_name: String,
    pub name: String,
    pub gender: String,
    pub ic_number: String,
    pub phone_number: String,
    pub nickname: Option<String>,
    pub picture: Option<String>,
    pub cg_id: Option<i32>,
}

#[derive(Debug, Clone)]
pub struct Database {
    pub(crate) db: PgPool,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        // Create a single connection pool for SQLx that's shared across the whole application.
        // This saves us from opening a new connection for every API call, which is wasteful.
        let db = PgPoolOptions::new()
            // CockroachDB recommends 4 connections per CPU core as suggested in the docs.
            // see: https://www.cockroachlabs.com/docs/stable/connection-pooling.html
            .max_connections(num_cpus::get() as u32 * 4)
            .connect(&database_url)
            .await?;

        Ok(Self { db })
    }
}
