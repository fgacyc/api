use poem::web;
use poem_openapi::{payload, Object};
use serde::{Deserialize, Serialize};

use crate::{database::Database, entities, error::ErrorResponse};

#[derive(Debug, Clone, Deserialize, Serialize, Object)]
#[oai(rename = "CreateSatelliteRequest")]
pub struct Request {
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
        let satellite = sqlx::query_as_unchecked!(
            entities::Satellite,
            r#"
            INSERT INTO "satellite" (
                id,
                no,
                name,
                address
            ) VALUES (
                $1,
                $2,
                $3,
                $4
            )
            RETURNING *
            "#,
            &format!("satellite_{}", ulid::Ulid::new()),
            &body.no,
            &body.name,
            &body.address,
        )
        .fetch_one(&db.db)
        .await
        .map_err(|e| match e {
            _ => Error::InternalServerError(payload::Json(ErrorResponse::from(
                &e as &(dyn std::error::Error + Send + Sync),
            ))),
        })?;

        Ok(Response::Ok(payload::Json(satellite)))
    }
}
