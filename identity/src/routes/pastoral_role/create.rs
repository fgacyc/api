use auth0::management::roles::{CreateRoleRequestParameters, Roles};
use poem::web;
use poem_openapi::{payload, Object};
use serde::{Deserialize, Serialize};

use crate::{database::Database, entities, error::ErrorResponse};

#[derive(Debug, Clone, Deserialize, Serialize, Object)]
#[oai(rename = "CreatePastoralRoleRequest")]
pub struct Request {
    name: String,
    description: String,
    weight: i32,
}

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
    pub async fn _create_pastoral_role(
        &self,
        db: web::Data<&Database>,
        body: payload::Json<Request>,
    ) -> Result<Response, Error> {
        let mut tx = db.db.begin().await.map_err(|e| {
            Error::InternalServer(payload::Json(ErrorResponse::from(
                &e as &(dyn std::error::Error + Send + Send + Sync),
            )))
        })?;

        let role_id = self
            .management
            .create_role(CreateRoleRequestParameters {
                name: body.name.clone(),
                description: body.description.clone(),
            })
            .send()
            .await
            .map_err(|e| {
                Error::InternalServer(payload::Json(ErrorResponse::from(
                    &e as &(dyn std::error::Error + Send + Send + Sync),
                )))
            })?
            .error_for_status()
            .map_err(|e| {
                tracing::error!("Failed to create role: {}", e);

                Error::InternalServer(payload::Json(ErrorResponse::from(
                    &e as &(dyn std::error::Error + Send + Send + Sync),
                )))
            })?
            .json::<serde_json::Value>()
            .await
            .map_err(|e| {
                Error::InternalServer(payload::Json(ErrorResponse::from(
                    &e as &(dyn std::error::Error + Send + Send + Sync),
                )))
            })?
            .get("id")
            .ok_or(Error::BadRequest(payload::Json(ErrorResponse {
                message: format!("Failed to create a role on Auth0"),
            })))?
            .as_str()
            .ok_or(Error::InternalServer(payload::Json(ErrorResponse {
                message: format!("ID returned from Auth0 is not a string"),
            })))?
            .to_string();

        let pr = sqlx::query_as!(
            entities::PastoralRole,
            r#"
            INSERT INTO pastoral_role (
                id, 
                name, 
                description,
                weight
            ) VALUES (
                $1,
                $2,
                $3,
                $4
            ) 
            RETURNING *
            "#,
            &role_id,
            &body.name,
            &body.description,
            &body.weight,
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(e)
                if e.is_unique_violation()
                    && e.constraint()
                        .is_some_and(|constraint| constraint == "pastoral_role_name_key") =>
            {
                Error::BadRequest(payload::Json(ErrorResponse {
                    message: format!("Pastoral role name of '{}' already exists", body.name),
                }))
            }
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
