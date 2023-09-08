use poem::web;
use poem_openapi::{param::Path, payload, Object};

use crate::{database::Database, entities, error::ErrorResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Object)]
#[oai(rename = "UpdateFormFieldTypeRequest")]
pub struct Request {
    r#type: Option<String>,
    description: Option<String>,
}

#[derive(poem_openapi::ApiResponse)]
pub enum Response {
    #[oai(status = 200)]
    Ok(payload::Json<entities::FormFieldType>),
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
    pub async fn _update_form_field_type(
        &self,
        db: web::Data<&Database>,
        r#type: Path<String>,
        body: payload::Json<Request>,
    ) -> Result<Response, Error> {
        let form_field_type = sqlx::query_as_unchecked!(
            entities::FormFieldType,
            r#"
            UPDATE form_field_type SET
                type        = COALESCE($1, type),
                description = COALESCE($2, description),
                updated_at  = NOW()
            WHERE type = $3
            RETURNING *
            "#,
            &body.r#type,
            &body.description,
            &*r#type,
        )
        .fetch_one(&db.db)
        .await
        .map_err(|e| match e {
            sqlx::error::Error::RowNotFound => Error::NotFound(payload::Json(ErrorResponse {
                message: format!("Form field type '{:?}' not found", &body.r#type),
            })),
            _ => Error::InternalServer(payload::Json(ErrorResponse::from(
                &e as &(dyn std::error::Error + Send + Sync),
            ))),
        })?;

        Ok(Response::Ok(payload::Json(form_field_type)))
    }
}
