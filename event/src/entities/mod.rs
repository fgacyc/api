use poem_openapi::{Enum, Object};
use serde::{Deserialize, Serialize};
use sqlx::types::chrono;

#[derive(Debug, Clone, Deserialize, Serialize, Object, sqlx::FromRow)]
pub struct Attendance {
    pub session_id: String,
    pub user_id: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Object, sqlx::FromRow)]
pub struct Currency {
    pub code: String,
    pub num: i32,
    pub denominator: i32,
    pub name: String,
    pub countries: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Object, sqlx::FromRow)]
pub struct EventType {
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
