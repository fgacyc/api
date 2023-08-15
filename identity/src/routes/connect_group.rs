use poem::web;
use poem_openapi::{payload, Object};
use serde::{Deserialize, Serialize};

use crate::{database::Database, error::ErrorResponse};

#[derive(Debug, Clone, Deserialize, Serialize, Object)]
pub struct CreateConnecGroupRequest {
    name: String,
    #[oai(validator(max_length = 2))]
    variant: String,
    satellite_id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Object, sqlx::FromRow)]
pub struct ConnectGroup {
    id: String,
    no: i32,
    name: String,
    variant: String,
    satellite_id: String,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(poem_openapi::ApiResponse)]
pub enum CreateConnectGroupResponse {
    #[oai(status = 200)]
    Ok(payload::Json<ConnectGroup>),
}

#[derive(poem_openapi::ApiResponse)]
pub enum CreateConnectGroupError {
    #[oai(status = 400)]
    BadRequest(payload::Json<ErrorResponse>),

    #[oai(status = 404)]
    NotFound(payload::Json<ErrorResponse>),

    #[oai(status = 500)]
    InternalServerError(payload::Json<ErrorResponse>),
}

impl super::Routes {
    pub async fn _create_connect_group(
        &self,
        db: web::Data<&Database>,
        body: payload::Json<CreateConnecGroupRequest>,
    ) -> Result<CreateConnectGroupResponse, CreateConnectGroupError> {
        let cg: ConnectGroup = sqlx::query_as(
            r#"
            INSERT INTO connect_group (
                id, 
                name, 
                variant, 
                satellite_id
            ) VALUES (
                $1,
                $2,
                $3,
                $4,
            ) 
            RETURNING *
            "#,
        )
        .bind(&format!("connect_group_{}", ulid::Ulid::new()))
        .bind(&body.name)
        .bind(&body.variant)
        .bind(&body.satellite_id)
        .fetch_one(&db.db)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(e)
                if e.is_unique_violation()
                    && e.constraint().is_some_and(|constraint| {
                        constraint == "connect_group_no_satellite_id_key"
                    }) =>
            {
                CreateConnectGroupError::BadRequest(payload::Json(ErrorResponse {
                    message: format!(
                        "CG with no '{}' and satellite_id '{}' already exists",
                        0, body.satellite_id
                    ),
                }))
            }
            _ => CreateConnectGroupError::InternalServerError(payload::Json(ErrorResponse::from(
                &e as &(dyn std::error::Error + Send + Sync),
            ))),
        })?;

        Ok(CreateConnectGroupResponse::Ok(payload::Json(cg)))
    }
}
