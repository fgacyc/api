use poem::web;
use poem_openapi::{payload, Object};
use serde::{Deserialize, Serialize};

use crate::{database::Database, entities, error::ErrorResponse};

#[derive(Debug, Clone, Deserialize, Serialize, Object)]
#[oai(rename = "CreateEventRequest")]
pub struct Request {
    name: String,
    description: String,
    event_type: String,
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
    pub async fn _create_event(
        &self,
        db: web::Data<&Database>,
        body: payload::Json<Request>,
    ) -> Result<Response, Error> {
        let event = sqlx::query_as!(
            entities::Event,
            r#"
            INSERT INTO event (
                name, 
                description,
                type
            ) VALUES (
                $1,
                $2,
                $3
            ) 
            RETURNING *
            "#,
            &body.name,
            &body.description,
            &body.event_type,
        )
        .fetch_one(&db.db)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(e)
                if e.is_foreign_key_violation()
                    && e.constraint().is_some_and(|constraint| {
                        constraint == "event_type_fkey"
                    }) =>
            {
                Error::BadRequest(payload::Json(ErrorResponse {
                    message: format!("Event with type '{}' does not exists", body.event_type),
                }))
            }
            _ => Error::InternalServer(payload::Json(ErrorResponse::from(
                &e as &(dyn std::error::Error + Send + Sync),
            ))),
        })?;

        Ok(Response::Ok(payload::Json(event)))
    }
}