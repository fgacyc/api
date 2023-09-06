use poem_openapi::{Enum, Object};
use serde::{Deserialize, Serialize};
use sqlx::types::chrono;

#[derive(Debug, Clone, Deserialize, Serialize, Object, sqlx::FromRow)]
pub struct Registration {
    pub id: String,
    pub event_id: String,
    pub name: String,
    pub close_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
