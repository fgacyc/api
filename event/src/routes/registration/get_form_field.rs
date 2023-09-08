use poem::web;
use poem_openapi::{param::Path, payload};

use crate::{database::Database, entities, error::ErrorResponse};

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
    pub async fn _get_registration_form_field(
        &self,
        db: web::Data<&Database>,
        registration_id: Path<String>,
        name: Path<String>,
    ) -> Result<Response, Error> {
        let registration = sqlx::query_as!(
            entities::RegistrationFormField,
            r#"
            SELECT * 
            FROM registration_form_field 
            WHERE registration_id = $1 AND name = $2
            "#,
            &*registration_id,
            &*name
        )
        .fetch_one(&db.db)
        .await
        .map_err(|e| match e {
            sqlx::error::Error::RowNotFound => Error::NotFound(payload::Json(ErrorResponse {
                message: format!(
                    "Registration with id '{}' and name '{}' not found",
                    &*registration_id, &*name
                ),
            })),
            _ => Error::InternalServer(payload::Json(ErrorResponse::from(
                &e as &(dyn std::error::Error + Send + Sync),
            ))),
        })?;

        Ok(Response::Ok(payload::Json(registration)))
    }
}
