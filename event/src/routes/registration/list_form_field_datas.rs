use poem::web;
use poem_openapi::{param::Path, payload};

use crate::{database::Database, entities, error::ErrorResponse};

#[derive(poem_openapi::ApiResponse)]
pub enum Response {
    #[oai(status = 200)]
    Ok(payload::Json<Vec<entities::RegistrationFormFieldData>>),
}

#[derive(poem_openapi::ApiResponse)]
pub enum Error {
    #[oai(status = 400)]
    BadRequest(payload::Json<ErrorResponse>),

    #[oai(status = 500)]
    InternalServerError(payload::Json<ErrorResponse>),
}

impl crate::routes::Routes {
    pub async fn _list_registration_form_field_datas(
        &self,
        db: web::Data<&Database>,
        registration_id: Path<String>,
        user_id: Path<String>,
    ) -> Result<Response, Error> {
        let registration_form_fields = sqlx::query_as!(
            entities::RegistrationFormFieldData,
            r#"
            SELECT * 
            FROM registration_form_field_data
            WHERE registration_id = $1 AND user_id = $2
            "#,
            &*registration_id,
            &*user_id,
        )
        .fetch_all(&db.db)
        .await
        .map_err(|e| match e {
            _ => Error::InternalServerError(payload::Json(ErrorResponse::from(
                &e as &(dyn std::error::Error + Send + Sync),
            ))),
        })?;

        Ok(Response::Ok(payload::Json(registration_form_fields)))
    }
}
