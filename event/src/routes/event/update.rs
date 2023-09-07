use poem::web;
use poem_openapi::{param::Path, payload, Object};

use crate::{database::Database, entities, error::ErrorResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Object)]
#[oai(rename = "UpdateEventRequest")]
pub struct Request {
    name: Option<String>,
    description: Option<String>,
    event_type: Option<String>,
}

#[derive(poem_openapi::ApiResponse)]
pub enum Response {
    #[oai(status = 200)]
    Ok(payload::Json<entities::Event>),
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
    pub async fn _update_event(
        &self,
        db: web::Data<&Database>,
        id: Path<String>,
        body: payload::Json<Request>,
    ) -> Result<Response, Error> {
        let event = sqlx::query_as_unchecked!(
            entities::Event,
            r#"
            UPDATE event SET
                name             = COALESCE($1, name),
                description      = COALESCE($2, description),
                type             = COALESCE($3, type),
                updated_at       = NOW()
            WHERE id = $4
            RETURNING *
            "#,
            &body.name,
            &body.description,
            &body.event_type,
            &*id,
        )
        .fetch_one(&db.db)
        .await
        .map_err(|e| match e {
            sqlx::error::Error::RowNotFound => Error::NotFound(payload::Json(ErrorResponse {
                message: format!("Event with id '{}' not found", &*id),
            })),
            _ => Error::InternalServer(payload::Json(ErrorResponse::from(
                &e as &(dyn std::error::Error + Send + Sync),
            ))),
        })?;

        Ok(Response::Ok(payload::Json(event)))
    }
}
