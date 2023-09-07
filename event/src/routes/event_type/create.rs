use poem::web;
use poem_openapi::{payload, Object};
use serde::{Deserialize, Serialize};

use crate::{database::Database, entities, error::ErrorResponse};

#[derive(Debug, Clone, Deserialize, Serialize, Object)]
#[oai(rename = "CreateEventTypeRequest")]
pub struct Request {
    name: String,
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
    InternalServerError(payload::Json<ErrorResponse>),
}

impl crate::routes::Routes {
    pub async fn _create_event_type(
        &self,
        db: web::Data<&Database>,
        body: payload::Json<Request>,
    ) -> Result<Response, Error> {
        let event_type = sqlx::query_as!(
            entities::EventType,
            r#"
            INSERT INTO event_type(
                name
            ) VALUES (
                $1
            )
            RETURNING *
            "#,
            &body.name,
        )
        .fetch_one(&db.db)
        .await
        .map_err(|e| match e {
            //TODO : Check and see if this pkey check is required for composite pkey relation
            sqlx::Error::Database(e)
                if e.is_unique_violation()
                    && e.constraint()
                        .is_some_and(|constraint| constraint == "name_pkey") =>
            {
                Error::BadRequest(payload::Json(ErrorResponse {
                    message: format!("Name '{}' already exists", body.name),
                }))
            }
            _ => Error::InternalServerError(payload::Json(ErrorResponse::from(
                &e as &(dyn std::error::Error + Send + Sync),
            ))),
        })?;

        Ok(Response::Ok(payload::Json(event_type)))
    }
}
