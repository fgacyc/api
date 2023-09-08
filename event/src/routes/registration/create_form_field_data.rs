use poem::web;
use poem_openapi::{param::Path, payload, Object};
use serde::{Deserialize, Serialize};

use crate::{database::Database, entities, error::ErrorResponse};

#[derive(Debug, Clone, Deserialize, Serialize, Object)]
#[oai(rename = "CreateRegistrationFormFieldDataRequest")]
pub struct Request {
    name: String,
    user_id: String,
    data: String,
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
    pub async fn _create_registration_form_field_data(
        &self,
        db: web::Data<&Database>,
        registration_id: Path<String>,
        body: payload::Json<Request>,
    ) -> Result<Response, Error> {
        let registration_form_field = sqlx::query_as_unchecked!(
            entities::RegistrationFormFieldData,
            r#"
            INSERT INTO registration_form_field_data (
                registration_id,
                name,
                user_id,
                data
            ) VALUES (
                $1,
                $2,
                $3,
                $4
            ) 
            RETURNING *
            "#,
            &*registration_id,
            &body.name,
            &body.user_id,
            &body.data,
        )
        .fetch_one(&db.db)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(e)
                if e.is_unique_violation()
                    && e.constraint().is_some_and(|constraint| {
                        constraint == "registration_form_field_data_pkey"
                    }) =>
            {
                Error::BadRequest(payload::Json(ErrorResponse {
                    message: format!(
                        "Registration with id '{}', name '{}' and user id '{}' already exists",
                        &*registration_id, body.name, body.user_id
                    ),
                }))
            }
            sqlx::Error::Database(e)
                if e.is_foreign_key_violation()
                    && e.constraint().is_some_and(|constraint| {
                        constraint == "registration_form_field_data_registration_id_name_fkey"
                    }) =>
            {
                Error::BadRequest(payload::Json(ErrorResponse {
                    message: format!(
                        "Registration with id '{}' and name '{}' does not exists",
                        &*registration_id, body.name
                    ),
                }))
            }
            _ => Error::InternalServer(payload::Json(ErrorResponse::from(
                &e as &(dyn std::error::Error + Send + Sync),
            ))),
        })?;

        Ok(Response::Ok(payload::Json(registration_form_field)))
    }
}
