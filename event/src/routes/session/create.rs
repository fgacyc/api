use poem::web;
use poem_openapi::{payload, Object};
use serde::{Deserialize, Serialize};

use crate::{database::Database, entities, error::ErrorResponse};

#[derive(Debug, Clone, Deserialize, Serialize, Object)]
#[oai(rename = "CreateSessionRequest")]
pub struct Request {
    event_id: String,
    name: String,
    description: String,
    expected_attendees: i32,
    start_at: chrono::DateTime<chrono::Utc>,
    end_at: chrono::DateTime<chrono::Utc>,
    actual_start_at: Option<chrono::DateTime<chrono::Utc>>,
    actual_end_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(poem_openapi::ApiResponse)]
pub enum Response {
    #[oai(status = 200)]
    Ok(payload::Json<entities::Session>),
}

#[derive(poem_openapi::ApiResponse)]
pub enum Error {
    #[oai(status = 400)]
    BadRequest(payload::Json<ErrorResponse>),

    #[oai(status = 404)]
    NotFound(payload::Json<ErrorResponse>),

    #[oai(status = 500)]
    InternalServer(payload::Json<ErrorResponse>),
}

impl crate::routes::Routes {
    pub async fn _create_session(
        &self,
        db: web::Data<&Database>,
        body: payload::Json<Request>,
    ) -> Result<Response, Error> {
        let session = sqlx::query_as_unchecked!(
            entities::Session,
            r#"
            INSERT INTO session (
                id, 
                event_id,
                name,
                description,
                expected_attendees,
                start_at,
                end_at,
                actual_start_at,
                actual_end_at
            ) VALUES (
                $1,
                $2,
                $3,
                $4,
                $5,
                $6,
                $7,
                $8,
                $9
            ) 
            RETURNING *
            "#,
            &format!("session_{}", ulid::Ulid::new()),
            &body.event_id,
            &body.name,
            &body.description,
            &body.expected_attendees,
            &body.start_at,
            &body.end_at,
            &body.actual_start_at,
            &body.actual_end_at
        )
        .fetch_one(&db.db)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(e)
                if e.is_unique_violation()
                    && e.constraint().is_some_and(|constraint| {
                        constraint == "session_event_id_name_key"
                    }) =>
            {
                Error::BadRequest(payload::Json(ErrorResponse {
                    message: format!(
                        "Session with event_id '{}' and name '{}' already exists",
                        body.event_id, body.name
                    ),
                }))
            }
            sqlx::Error::Database(e)
                if e.is_foreign_key_violation()
                    && e.constraint().is_some_and(|constraint| {
                        constraint == "session_event_id_fkey"
                    }) =>
            {
                Error::BadRequest(payload::Json(ErrorResponse {
                    message: format!("Event id '{}' does not exists", body.event_id),
                }))
            }
            _ => Error::InternalServer(payload::Json(ErrorResponse::from(
                &e as &(dyn std::error::Error + Send + Sync),
            ))),
        })?;

        Ok(Response::Ok(payload::Json(session)))
    }
}