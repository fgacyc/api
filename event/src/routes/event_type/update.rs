use poem::web;
use poem_openapi::{param::Path, payload, Object};

use crate::{database::Database, entities, error::ErrorResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Object)]
#[oai(rename = "UpdateEventTypeRequest")]
pub struct Request {
    name: Option<String>,
}

#[derive(poem_openapi::ApiResponse)]
pub enum Response {
    #[oai(status = 200)]
    Ok(payload::Json<entities::EventType>),
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
    pub async fn _update_event_type(
        &self,
        db: web::Data<&Database>,
        name: Path<String>,
        body: payload::Json<Request>,
    ) -> Result<Response, Error> {
        let event_type = sqlx::query_as_unchecked!(
            entities::EventType,
            r#"
            UPDATE event_type SET
                name       = COALESCE($1, name),
                updated_at = NOW()
            WHERE name = $2
            RETURNING *
            "#,
            &body.name,
            &*name,
        )
        .fetch_one(&db.db)
        .await
        .map_err(|e| match e {
            sqlx::error::Error::RowNotFound => Error::NotFound(payload::Json(ErrorResponse {
                message: format!("Event type with name '{:?}' not found", &body.name),
            })),
            _ => Error::InternalServer(payload::Json(ErrorResponse::from(
                &e as &(dyn std::error::Error + Send + Sync),
            ))),
        })?;

        Ok(Response::Ok(payload::Json(event_type)))
    }
}
