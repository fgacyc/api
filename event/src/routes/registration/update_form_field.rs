use poem::web;
use poem_openapi::{param::Path, payload, Object};

use crate::{database::Database, entities, error::ErrorResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Object)]
#[oai(rename = "UpdateRegistrationFormFieldRequest")]
pub struct Request {
    name: Option<String>,
    label: Option<String>,
    description: Option<String>,
    r#type: Option<String>,
    weight: Option<i32>,
}

#[derive(poem_openapi::ApiResponse)]
pub enum Response {
    #[oai(status = 200)]
    Ok(payload::Json<entities::RegistrationFormField>),
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
    pub async fn _update_registration_form_field(
        &self,
        db: web::Data<&Database>,
        registration_id: Path<String>,
        name: Path<String>,
        body: payload::Json<Request>,
    ) -> Result<Response, Error> {
        let registration = sqlx::query_as_unchecked!(
            entities::RegistrationFormField,
            r#"
            UPDATE registration_form_field SET
                name        = COALESCE($1, name),
                label       = COALESCE($2, label),
                description = COALESCE($3, description),
                type        = COALESCE($4, type),
                weight      = COALESCE($5, weight),
                updated_at  = NOW()
            WHERE registration_id = $6 AND name = $7
            RETURNING *
            "#,
            &body.name,
            &body.label,
            &body.description,
            &body.r#type,
            &body.weight,
            &*registration_id,
            &*name,
        )
        .fetch_one(&db.db)
        .await
        .map_err(|e| match e {
            sqlx::error::Error::RowNotFound => Error::NotFound(payload::Json(ErrorResponse {
                message: format!(
                    "Registration  with id '{}' and form field '{}' not found",
                    &*registration_id, &*name,
                ),
            })),
            _ => Error::InternalServer(payload::Json(ErrorResponse::from(
                &e as &(dyn std::error::Error + Send + Sync),
            ))),
        })?;

        Ok(Response::Ok(payload::Json(registration)))
    }
}
