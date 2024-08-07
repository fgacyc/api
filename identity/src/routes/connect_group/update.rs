use poem::web;
use poem_openapi::{param::Path, payload, Object};

use crate::{database::Database, entities, error::ErrorResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Object)]
#[oai(rename = "UpdateConnectGroupRequest")]
pub struct Request {
    name: Option<String>,
    no: Option<i32>,
    #[oai(validator(max_length = 2))]
    variant: Option<String>,
    satellite_id: Option<String>,
    category_id: Option<String>,
    active: Option<bool>,
    closed_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(poem_openapi::ApiResponse)]
pub enum Response {
    #[oai(status = 200)]
    Ok(payload::Json<entities::ConnectGroup>),
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
    pub async fn _update_connect_group(
        &self,
        db: web::Data<&Database>,
        id: Path<String>,
        body: payload::Json<Request>,
    ) -> Result<Response, Error> {
        let cg = sqlx::query_as_unchecked!(
            entities::ConnectGroup,
            r#"
            UPDATE connect_group SET
                no           = COALESCE($1, no),
                name         = COALESCE($2, name),
                variant      = COALESCE($3, variant),
                satellite_id = COALESCE($4, satellite_id),
                category_id  = COALESCE($5, category_id),
                active       = COALESCE($6, active),
                closed_at    = COALESCE($7, closed_at),
                updated_at   = NOW()
            WHERE id = $8
            RETURNING *
            "#,
            &body.no,
            &body.name,
            &body.variant,
            &body.satellite_id,
            &body.category_id,
            &body.active,
            &body.closed_at,
            &*id,
        )
        .fetch_one(&db.db)
        .await
        .map_err(|e| match e {
            sqlx::error::Error::RowNotFound => Error::NotFound(payload::Json(ErrorResponse {
                message: format!("Connect group with id '{}' not found", &*id),
            })),
            _ => Error::InternalServer(payload::Json(ErrorResponse::from(
                &e as &(dyn std::error::Error + Send + Sync),
            ))),
        })?;

        Ok(Response::Ok(payload::Json(cg)))
    }
}
