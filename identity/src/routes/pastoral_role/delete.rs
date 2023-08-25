use auth0::management::roles::Roles;
use poem::web;
use poem_openapi::{param::Path, payload};
use reqwest::StatusCode;

use crate::{database::Database, entities, error::ErrorResponse};

#[derive(poem_openapi::ApiResponse)]
pub enum Response {
    #[oai(status = 200)]
    Ok(payload::Json<entities::PastoralRole>),
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
    pub async fn _delete_pastoral_role(
        &self,
        db: web::Data<&Database>,
        id: Path<String>,
    ) -> Result<Response, Error> {
        let mut tx = db.db.begin().await.map_err(|e| {
            Error::InternalServer(payload::Json(ErrorResponse::from(
                &e as &(dyn std::error::Error + Send + Send + Sync),
            )))
        })?;

        match self
            .management
            .delete_role(id.clone())
            .send()
            .await
            .map_err(|e| {
                Error::InternalServer(payload::Json(ErrorResponse::from(
                    &e as &(dyn std::error::Error + Send + Send + Sync),
                )))
            })?
            .status()
        {
            StatusCode::OK => {}
            StatusCode::NOT_FOUND => {
                return Err(Error::NotFound(payload::Json(ErrorResponse {
                    message: format!("Auth0 uole with id '{}' not found", &*id),
                })))
            }
            _ => {
                return Err(Error::InternalServer(payload::Json(ErrorResponse {
                    message: format!("Failed to delete auth0 role with id '{}'", &*id),
                })))
            }
        };

        let pr: entities::PastoralRole = sqlx::query_as(
            r#"
            DELETE FROM pastoral_role 
            WHERE id = $1::TEXT 
            RETURNING *
            "#,
        )
        .bind(&*id)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| match e {
            sqlx::error::Error::RowNotFound => Error::NotFound(payload::Json(ErrorResponse {
                message: format!("Pastoral role with id '{}' not found", &*id),
            })),
            _ => Error::InternalServer(payload::Json(ErrorResponse::from(
                &e as &(dyn std::error::Error + Send + Sync),
            ))),
        })?;

        tx.commit().await.map_err(|e| {
            Error::InternalServer(payload::Json(ErrorResponse::from(
                &e as &(dyn std::error::Error + Send + Send + Sync),
            )))
        })?;

        Ok(Response::Ok(payload::Json(pr)))
    }
}
