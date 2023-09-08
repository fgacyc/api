use poem::web;
use poem_openapi::{param::Path, payload, Object};

use crate::{database::Database, entities, error::ErrorResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Object)]
#[oai(rename = "UpdateSessionRequest")]
pub struct Request {
    event_id: Option<String>,
    name: Option<String>,
    description: Option<String>,
    expected_attendees: Option<i32>,
    start_at: Option<chrono::DateTime<chrono::Utc>>,
    end_at: Option<chrono::DateTime<chrono::Utc>>,
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
    pub async fn _update_session(
        &self,
        db: web::Data<&Database>,
        id: Path<String>,
        body: payload::Json<Request>,
    ) -> Result<Response, Error> {
        let session = sqlx::query_as_unchecked!(
            entities::Session,
            r#"
            UPDATE session SET
                event_id             = COALESCE($1, event_id),
                name                 = COALESCE($2, name),
                description          = COALESCE($3, description),
                expected_attendees   = COALESCE($4, expected_attendees),
                start_at             = COALESCE($5, start_at),
                end_at               = COALESCE($6, end_at),
                actual_start_at      = COALESCE($7, actual_start_at),
                actual_end_at        = COALESCE($8, actual_end_at),
                updated_at           = NOW()
            WHERE id = $9
            RETURNING *
            "#,
            &body.event_id,
            &body.name,
            &body.description,
            &body.expected_attendees,
            &body.start_at,
            &body.end_at,
            &body.actual_start_at,
            &body.actual_end_at,
            &*id,
        )
        .fetch_one(&db.db)
        .await
        .map_err(|e| match e {
            sqlx::error::Error::RowNotFound => Error::NotFound(payload::Json(ErrorResponse {
                message: format!("Session with id '{}' not found", &*id),
            })),
            _ => Error::InternalServer(payload::Json(ErrorResponse::from(
                &e as &(dyn std::error::Error + Send + Sync),
            ))),
        })?;

        Ok(Response::Ok(payload::Json(session)))
    }
}
