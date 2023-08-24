use poem::web;
use poem_openapi::{param::Path, payload};

use crate::{database::Database, entities, error::ErrorResponse};

#[derive(poem_openapi::ApiResponse)]
pub enum Response {
    #[oai(status = 200)]
    OK(payload::Json<entities::Satellite>),
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
    pub async fn _get_satellite(
        &self,
        db: web::Data<&Database>,
        id: Path<String>,
    ) -> Result<Response, Error> {
        let satellite: entities::Satellite = sqlx::query_as(
            r#"
            SELECT * from "satellite"
            WHERE id = $1::TEXT
            "#,
        )
        .bind(&*id)
        .fetch_one(&db.db)
        .await
        .map_err(|e| match e {
            sqlx::error::Error::RowNotFound => Error::NotFound(payload::Json(ErrorResponse {
                message: format!("Satellite with id '{}' not found", &*id),
            })),
            _ => Error::InternalServerError(payload::Json(
                ErrorResponse::from(&e as &(dyn std::error::Error + Send + Sync)),
            )),
        })?;

        Ok(Response::OK(payload::Json(satellite)))
    }
}