use poem::web;
use poem_openapi::{param::Path, payload};

use crate::{database::Database, entities, error::ErrorResponse};

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
    pub async fn _delete_registration_form_field_data(
        &self,
        db: web::Data<&Database>,
        id: Path<String>,
        name: Path<String>,
        user_id: Path<String>,
    ) -> Result<Response, Error> {
        let registration_form_field_data = sqlx::query_as!(
            entities::RegistrationFormFieldData,
            r#"
            DELETE FROM registration_form_field_data 
            WHERE registration_id = $1 AND name = $2 AND user_id = $3
            RETURNING *
            "#,
            &*id,
            &*name,
            &*user_id
        )
        .fetch_one(&db.db)
        .await
        .map_err(|e| match e {
            sqlx::error::Error::RowNotFound => Error::NotFound(payload::Json(ErrorResponse {
                message: format!(
                    "Registration with id '{}', name '{}' and user id '{}' not found",
                    &*id, &*name, &*user_id
                ),
            })),
            _ => Error::InternalServer(payload::Json(ErrorResponse::from(
                &e as &(dyn std::error::Error + Send + Sync),
            ))),
        })?;

        Ok(Response::Ok(payload::Json(registration_form_field_data)))
    }
}
