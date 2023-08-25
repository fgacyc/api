use poem::web;
use poem_openapi::{param::Path, payload, Object};

use crate::{database::Database, entities, error::ErrorResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Object)]
#[oai(rename = "UpdateMinistryRequest")]
pub struct Request {
    name: Option<String>,
    description: Option<String>,
    department_id: Option<String>,
    team_id: Option<String>,
    satellite_id: Option<String>,
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
    InternalServer(payload::Json<ErrorResponse>),
}

impl crate::routes::Routes {
    pub async fn _update_ministry(
        &self,
        db: web::Data<&Database>,
        id: Path<String>,
        body: payload::Json<Request>,
    ) -> Result<Response, Error> {
        let ministry = sqlx::query_as_unchecked!(
            entities::Ministry,
            r#"
            UPDATE ministry SET
                name          = COALESCE($1, name),
                description   = COALESCE($2, description),
                department_id = COALESCE($3, department_id),
                team_id       = COALESCE($4, team_id),
                satellite_id  = COALESCE($5, satellite_id),
                updated_at    = NOW()
            WHERE id = $6
            RETURNING *
            "#,
            &body.name,
            &body.description,
            &body.department_id,
            &body.team_id,
            &body.satellite_id,
            &*id,
        )
        .fetch_one(&db.db)
        .await
        .map_err(|e| match e {
            sqlx::error::Error::RowNotFound => Error::NotFound(payload::Json(ErrorResponse {
                message: format!("Ministry with id '{}' not found", &*id),
            })),
            _ => Error::InternalServer(payload::Json(ErrorResponse::from(
                &e as &(dyn std::error::Error + Send + Sync),
            ))),
        })?;

        Ok(Response::Ok(payload::Json(ministry)))
    }
}
