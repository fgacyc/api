use poem::web;
use poem_openapi::{param::Path, payload, Object};
use serde::{Deserialize, Serialize};

use crate::{database::Database, entities, error::ErrorResponse};

#[derive(Debug, Clone, Deserialize, Serialize, Object)]
#[oai(rename = "CreateRegistrationFormFieldRequest")]
pub struct Request {
    name: String,
    label: String,
    description: Option<String>,
    r#type: String,
    weight: i32,
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
    pub async fn _create_registration_form_field(
        &self,
        db: web::Data<&Database>,
        registration_id: Path<String>,
        body: payload::Json<Request>,
    ) -> Result<Response, Error> {
        let registration_form_field = sqlx::query_as_unchecked!(
            entities::RegistrationFormField,
            r#"
            INSERT INTO registration_form_field (
                registration_id,
                name,
                label,
                description,
                type,
                weight
            ) VALUES (
                $1,
                $2,
                $3,
                $4,
                $5,
                $6
            ) 
            RETURNING *
            "#,
            &*registration_id,
            &body.name,
            &body.label,
            &body.description,
            &body.r#type,
            &body.weight,
        )
        .fetch_one(&db.db)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(e)
                if e.is_unique_violation()
                    && e.constraint()
                        .is_some_and(|constraint| constraint == "registration_form_field_pkey") =>
            {
                Error::BadRequest(payload::Json(ErrorResponse {
                    message: format!(
                        "Registration with id '{}' and name '{}' already exists",
                        &*registration_id, body.name
                    ),
                }))
            }
            sqlx::Error::Database(e)
                if e.is_unique_violation()
                    && e.constraint().is_some_and(|constraint| {
                        constraint == "registration_form_field_registration_id_weight_key"
                    }) =>
            {
                Error::BadRequest(payload::Json(ErrorResponse {
                    message: format!(
                        "Registration with id '{}' and weight '{}' already exists",
                        &*registration_id, body.weight
                    ),
                }))
            }
            sqlx::Error::Database(e)
                if e.is_foreign_key_violation()
                    && e.constraint().is_some_and(|constraint| {
                        constraint == "registration_form_field_registration_id_fkey"
                    }) =>
            {
                Error::BadRequest(payload::Json(ErrorResponse {
                    message: format!(
                        "Registration with id '{}' does not exists",
                        &*registration_id
                    ),
                }))
            }
            sqlx::Error::Database(e)
                if e.is_foreign_key_violation()
                    && e.constraint().is_some_and(|constraint| {
                        constraint == "registration_form_field_type_fkey"
                    }) =>
            {
                Error::BadRequest(payload::Json(ErrorResponse {
                    message: format!("Form field type '{}' does not exists", &*body.r#type),
                }))
            }
            _ => Error::InternalServer(payload::Json(ErrorResponse::from(
                &e as &(dyn std::error::Error + Send + Sync),
            ))),
        })?;

        Ok(Response::Ok(payload::Json(registration_form_field)))
    }
}
