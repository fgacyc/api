use auth0::management::roles::{Roles, UpdateRoleRequestParameters};
use poem::web;
use poem_openapi::{param::Path, payload, Object};
use reqwest::StatusCode;

use crate::{database::Database, entities, error::ErrorResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Object)]
#[oai(rename = "UpdateMinistryRoleRequest")]
pub struct Request {
    name: Option<String>,
    description: Option<String>,
    weight: Option<i32>,
}

#[derive(poem_openapi::ApiResponse)]
pub enum Response {
    #[oai(status = 200)]
    Ok(payload::Json<entities::MinistryRole>),
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
    pub async fn _update_ministry_role(
        &self,
        db: web::Data<&Database>,
        id: Path<String>,
        body: payload::Json<Request>,
    ) -> Result<Response, Error> {
        let mut tx = db.db.begin().await.map_err(|e| {
            Error::InternalServer(payload::Json(ErrorResponse::from(
                &e as &(dyn std::error::Error + Send + Send + Sync),
            )))
        })?;

        match self
            .management
            .update_role(
                id.clone(),
                UpdateRoleRequestParameters {
                    name: body.name.clone(),
                    description: body.description.clone(),
                },
            )
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
            _ => {
                return Err(Error::InternalServer(payload::Json(ErrorResponse {
                    message: format!("Failed to update ministry role with id '{}' on Auth0", &*id),
                })))
            }
        };

        let ministry_role = sqlx::query_as_unchecked!(
            entities::MinistryRole,
            r#"
            UPDATE ministry_role SET
                name        = COALESCE($1, name),
                description = COALESCE($2, description),
                weight      = COALESCE($3, weight)
            WHERE id = $4
            RETURNING *
            "#,
            &body.name,
            &body.description,
            &body.weight,
            &*id,
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| match e {
            sqlx::error::Error::RowNotFound => Error::NotFound(payload::Json(ErrorResponse {
                message: format!("Ministry role with id '{}' not found", &*id),
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

        Ok(Response::Ok(payload::Json(ministry_role)))
    }
}
