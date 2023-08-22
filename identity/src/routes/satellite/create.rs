use poem::web;
use poem_openapi::{payload, Object};
use serde::{Deserialize, Serialize};

use crate::{database::Database, entities, error::ErrorResponse};

#[derive(Debug, Clone, Deserialize, Serialize, Object)]
#[oai(rename = "CreateSatelliteRequest")]
pub struct Request {
    id: String,
    no: i32,
    name: String,
    address: Option<entities::Address>,
}

#[derive(poem_openapi::ApiResponse)]
pub enum Response {
    #[oai(status = 200)]
    Ok(payload::Json<entities::Satellite>),
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
    pub async fn _create_satellite(
        &self,
        db: web::Data<&Database>,
        body: payload::Json<Request>,
    ) -> Result<Response, Error> {
        let satellite: entities::Satellite = sqlx::query_as(
            r#"
            INSERT INTO "satellite" (
                id,
                no,
                name,
                address
            ) VALUES(
                $1,
                $2,
                $3,
                $4
            )
            "#,
        )
        .bind(&body.id)
        .bind(&body.no)
        .bind(&body.name)
        .bind(&body.address)
        .fetch_one(&db.db)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(e)
                if e.is_unique_violation()
                    && e.constraint()
                        .is_some_and(|constraint| constraint == "satellite_pkey") =>
            {
                Error::BadRequest(payload::Json(ErrorResponse {
                    message: format!("Satellite with id '{}' already exists", body.id),
                }))
            }
            _ => Error::InternalServerError(payload::Json(
                ErrorResponse::from(&e as &(dyn std::error::Error + Send + Sync)),
            )),
        })?;

        Ok(Response::Ok(payload::Json(satellite)))
    }
}
