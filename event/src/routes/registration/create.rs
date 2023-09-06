use poem::web;
use poem_openapi::{payload, Object};
use serde::{Deserialize, Serialize};

use crate::{database::Database, entities, error::ErrorResponse};

#[derive(Debug, Clone, Deserialize, Serialize, Object)]
#[oai(rename = "CreateRegistrationRequest")]
pub struct Request {
    event_id: String,
    name: String,
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
    pub async fn _create_registration(
        &self,
        db: web::Data<&Database>,
        body: payload::Json<Request>,
    ) -> Result<Response, Error> {
        let registration = sqlx::query_as!(
            entities::Registration,
            r#"
            INSERT INTO registration (
                id, 
                event_id, 
                name
            ) VALUES (
                $1,
                $2,
                $3
            ) 
            RETURNING *
            "#,
            &format!("registration_{}", ulid::Ulid::new()),
            &body.event_id,
            &body.name,
        )
        .fetch_one(&db.db)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(e)
                if e.is_unique_violation()
                    && e.constraint().is_some_and(|constraint| {
                        constraint == "registration_event_id_name_key"
                    }) =>
            {
                Error::BadRequest(payload::Json(ErrorResponse {
                    message: format!(
                        "Registration with event_id '{}' and name '{}' already exists",
                        body.event_id, body.name
                    ),
                }))
            }
            sqlx::Error::Database(e)
                if e.is_foreign_key_violation()
                    && e.constraint().is_some_and(|constraint| {
                        constraint == "registration_event_id_fkey"
                    }) =>
            {
                Error::BadRequest(payload::Json(ErrorResponse {
                    message: format!("Event with id '{}' does not exists", body.event_id),
                }))
            }
            _ => Error::InternalServer(payload::Json(ErrorResponse::from(
                &e as &(dyn std::error::Error + Send + Sync),
            ))),
        })?;

        Ok(Response::Ok(payload::Json(registration)))
    }
}
