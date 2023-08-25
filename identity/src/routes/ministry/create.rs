use poem::web;
use poem_openapi::{payload, Object};
use serde::{Deserialize, Serialize};

use crate::{database::Database, entities, error::ErrorResponse};

#[derive(Debug, Clone, Deserialize, Serialize, Object)]
#[oai(rename = "CreateMinistryRequest")]
pub struct Request {
    name: String,
    description: String,
    department_id: String,
    team_id: String,
    satellite_id: String,
}

#[derive(poem_openapi::ApiResponse)]
pub enum Response {
    #[oai(status = 200)]
    Ok(payload::Json<entities::Ministry>),
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
    pub async fn _create_minitry(
        &self,
        db: web::Data<&Database>,
        body: payload::Json<Request>,
    ) -> Result<Response, Error> {
        let ministry = sqlx::query_as!(
            entities::Ministry,
            r#"
            INSERT INTO ministry (
                id, 
                name, 
                description,
                department_id,
                team_id,
                satellite_id
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
            &format!("ministry_{}", ulid::Ulid::new()),
            &body.name,
            &body.description,
            &body.department_id,
            &body.team_id,
            &body.satellite_id,
        )
        .fetch_one(&db.db)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(e)
                if e.is_foreign_key_violation()
                    && e.constraint()
                        .is_some_and(|constraint| constraint == "ministry_department_id_fkey") =>
            {
                Error::BadRequest(payload::Json(ErrorResponse {
                    message: format!(
                        "Ministry department with id '{}' does not exists",
                        body.department_id
                    ),
                }))
            }
            sqlx::Error::Database(e)
                if e.is_foreign_key_violation()
                    && e.constraint()
                        .is_some_and(|constraint| constraint == "ministry_satellite_id_fkey") =>
            {
                Error::BadRequest(payload::Json(ErrorResponse {
                    message: format!("Satellite with id '{}' does not exists", body.satellite_id),
                }))
            }
            sqlx::Error::Database(e)
                if e.is_foreign_key_violation()
                    && e.constraint()
                        .is_some_and(|constraint| constraint == "ministry_team_id_fkey") =>
            {
                Error::BadRequest(payload::Json(ErrorResponse {
                    message: format!("Ministry team with id '{}' does not exists", body.team_id),
                }))
            }
            _ => Error::InternalServerError(payload::Json(ErrorResponse::from(
                &e as &(dyn std::error::Error + Send + Sync),
            ))),
        })?;

        Ok(Response::Ok(payload::Json(ministry)))
    }
}
