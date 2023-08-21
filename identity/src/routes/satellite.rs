use crate::{database::Database, error::ErrorResponse};
use poem::web;
use poem_openapi::{payload, Enum, Object};
use serde::{Deserialize, Serialize};

use super::users::Address;

#[derive(Debug, Clone, Deserialize, Serialize, Object, sqlx::FromRow)]
pub struct Satellite {
    id: String,
    no: i32,
    name: String,
    address: Address,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Object)]
pub struct NewSatelliteRequest {
    id : String,
    no : i32,
    name: String,
    address: Option<Address>,
}

#[derive(poem_openapi::ApiResponse)]
pub enum NewSatelliteResponse {
    #[oai(status = 200)]
    Ok(payload::Json<Satellite>),
}

#[derive(poem_openapi::ApiResponse)]
pub enum NewSatelliteResponseError {
    #[oai(status = 400)]
    BadRequest(payload::Json<ErrorResponse>),

    #[oai(status = 404)]
    NotFound(payload::Json<ErrorResponse>),

    #[oai(status = 400)]
    InternalServerError(payload::Json<ErrorResponse>),
}

impl super::Routes {
    pub async fn _create_satellite(
        &self,
        db: web::Data<&Database>,
        body: payload::Json<NewSatelliteRequest>,
    ) -> Result<NewSatelliteResponse, NewSatelliteResponseError> {
        let satellite: Satellite = sqlx::query_as(
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
                NewSatelliteResponseError::BadRequest(payload::Json(ErrorResponse {
                    message: format!("Satellite with id '{}' already exists", body.id),
                }))
            }
            _ => NewSatelliteResponseError::InternalServerError(payload::Json(
                ErrorResponse::from(&e as &(dyn std::error::Error + Send + Sync)),
            )),
        })?;

        Ok(NewSatelliteResponse::Ok(payload::Json(satellite)))
    }
}
