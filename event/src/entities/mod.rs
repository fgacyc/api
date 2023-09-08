use poem_openapi::Object;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono;

#[derive(Debug, Clone, Deserialize, Serialize, Object, sqlx::FromRow)]
pub struct Event {
    pub id: String,
    pub name: String,
    pub description: String,
    #[sqlx(rename = "type")]
    pub r#type: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Object, sqlx::FromRow)]
pub struct Registration {
    pub id: String,
    pub event_id: String,
    pub name: String,
    pub close_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Object, sqlx::FromRow)]
pub struct Price {
    pub event_id: String,
    pub name: String,
    pub fee: i32,
    pub currency_code: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Object, sqlx::FromRow)]
pub struct Session {
    pub id: String,
    pub event_id: String,
    pub name: String,
    pub description: Option<String>,
    pub expected_attendees: i32,
    pub start_at: chrono::DateTime<chrono::Utc>,
    pub end_at: chrono::DateTime<chrono::Utc>,
    pub actual_start_at: Option<chrono::DateTime<chrono::Utc>>,
    pub actual_end_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Object, sqlx::FromRow)]
pub struct EventType {
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

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

