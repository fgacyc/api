use poem::web;
use poem_openapi::{payload, Object};
use serde::{Deserialize, Serialize};

use crate::{database::Database, error::ErrorResponse};

#[derive(Debug, Clone, Deserialize, Serialize, Object)]
pub struct CreateMinistryRoleRequest {
    name: String,
	description: String,
	weight: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize, Object, sqlx::FromRow)]
pub struct MinistryRole {
    id: String,
    name: String,
    description: String,
	weight: i32,
}

#[derive(poem_openapi::ApiResponse)]
pub enum CreateMinistryRoleResponse {
    #[oai(status = 200)]
    Ok(payload::Json<MinistryRole>),
}

#[derive(poem_openapi::ApiResponse)]
pub enum CreateMinistryRoleError {
    #[oai(status = 400)]
    BadRequest(payload::Json<ErrorResponse>),

    #[oai(status = 404)]
    NotFound(payload::Json<ErrorResponse>),

    #[oai(status = 500)]
    InternalServerError(payload::Json<ErrorResponse>),
}

impl super::Routes {
    pub async fn _create_ministry_role(
        &self,
        db: web::Data<&Database>,
        body: payload::Json<CreateMinistryRoleRequest>,
    ) -> Result<CreateMinistryRoleResponse, CreateMinistryRoleError> {
        let ministry_role: MinistryRole = sqlx::query_as(
            r#"
            INSERT INTO ministry_role (
                id, 
                name, 
                description, 
                weight
            ) VALUES (
                $1,
                $2,
                $3,
                $4,
            ) 
            RETURNING *
            "#,
        )
        .bind(&format!("ministry_role_{}", ulid::Ulid::new()))
        .bind(&body.name)
        .bind(&body.description)
        .bind(&body.weight)
        .fetch_one(&db.db)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(e)
                if e.is_unique_violation()
                    && e.constraint().is_some_and(|constraint| {
                        constraint == "ministry_role_name_key"
                    }) =>
            {
                CreateMinistryRoleError::BadRequest(payload::Json(ErrorResponse {
                    message: format!(
                        "Ministry role with name '{}' already exists",
                        body.name
                    ),
                }))
            }
            _ => CreateMinistryRoleError::InternalServerError(payload::Json(ErrorResponse::from(
                &e as &(dyn std::error::Error + Send + Sync),
            ))),
        })?;

        Ok(CreateMinistryRoleResponse::Ok(payload::Json(ministry_role)))
    }
}
