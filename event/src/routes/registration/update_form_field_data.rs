use poem::web;
use poem_openapi::{param::Path, payload, Object};

use crate::{database::Database, entities, error::ErrorResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Object)]
#[oai(rename = "UpdateRegistrationFormFieldDataRequest")]
pub struct Request {
    data: Option<String>,
}

#[derive(poem_openapi::ApiResponse)]
pub enum Response {
    #[oai(status = 200)]
    Ok(payload::Json<entities::RegistrationFormFieldData>),
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
    pub async fn _update_registration_form_field_data(
        &self,
        db: web::Data<&Database>,
        registration_id: Path<String>,
        name: Path<String>,
        user_id: Path<String>,
        body: payload::Json<Request>,
    ) -> Result<Response, Error> {
        let registration = sqlx::query_as_unchecked!(
            entities::RegistrationFormFieldData,
            r#"
            UPDATE registration_form_field_data SET
                data       = COALESCE($1, data),
                updated_at = NOW()
            WHERE registration_id = $2 AND name = $3 AND user_id = $4
            RETURNING *
            "#,
            &body.data,
            &*registration_id,
            &*name,
            &*user_id,
        )
        .fetch_one(&db.db)
        .await
        .map_err(|e| match e {
            sqlx::error::Error::RowNotFound => Error::NotFound(payload::Json(ErrorResponse {
                message: format!(
                    "Registration with id '{}', name '{}' and user id '{}' not found",
                    &*registration_id, &*name, &*user_id
                ),
            })),
            _ => Error::InternalServer(payload::Json(ErrorResponse::from(
                &e as &(dyn std::error::Error + Send + Sync),
            ))),
        })?;

        Ok(Response::Ok(payload::Json(registration)))
    }
}
