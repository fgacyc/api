use poem::web;
use poem_openapi::{param::Path, payload, Object};

use crate::{database::Database, entities, error::ErrorResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Object)]
#[oai(rename = "UpdateRegistrationRequest")]
pub struct Request {
    event_id: Option<String>,
    name: Option<i32>,
}

#[derive(poem_openapi::ApiResponse)]
pub enum Response {
    #[oai(status = 200)]
    Ok(payload::Json<entities::Registration>),
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
    pub async fn _update_registration(
        &self,
        db: web::Data<&Database>,
        id: Path<String>,
        body: payload::Json<Request>,
    ) -> Result<Response, Error> {
        let registration = sqlx::query_as_unchecked!(
            entities::Registration,
            r#"
            UPDATE registration SET
                name         = COALESCE($1, name),
                event_id     = COALESCE($2, event_id),
                updated_at   = NOW()
            WHERE id = $3
            RETURNING *
            "#,
            &body.name,
            &body.event_id,
            &*id,
        )
        .fetch_one(&db.db)
        .await
        .map_err(|e| match e {
            sqlx::error::Error::RowNotFound => Error::NotFound(payload::Json(ErrorResponse {
                message: format!("Registration with id '{}' not found", &*id),
            })),
            _ => Error::InternalServer(payload::Json(ErrorResponse::from(
                &e as &(dyn std::error::Error + Send + Sync),
            ))),
        })?;

        Ok(Response::Ok(payload::Json(registration)))
    }
}
