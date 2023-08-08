use poem::web;
use poem_openapi::{payload, Object};
use serde::{Deserialize, Serialize};
use sqlx::Row;

use crate::{database::Database, error::ErrorResponse};

#[derive(Debug, Clone, Deserialize, Serialize, Object)]
pub struct NewCGRequest {
    name: String,
    user_id: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize, Object)]
pub struct CG {
    id: i32,
    name: String,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(poem_openapi::ApiResponse)]
pub enum NewCGResponse {
    #[oai(status = 200)]
    Ok(payload::Json<CG>),
}

#[derive(poem_openapi::ApiResponse)]
pub enum NewCGResponseError {
    #[oai(status = 400)]
    BadRequest(payload::Json<ErrorResponse>),

    #[oai(status = 404)]
    NotFound(payload::Json<ErrorResponse>),

    #[oai(status = 500)]
    InternalServerError(payload::Json<ErrorResponse>),
}

impl super::Routes {
    pub async fn _new_cg(
        &self,
        db: web::Data<&Database>,
        body: payload::Json<NewCGRequest>,
    ) -> Result<NewCGResponse, NewCGResponseError> {
        let mut tx = db.db.begin().await.map_err(|e| {
            NewCGResponseError::InternalServerError(payload::Json(ErrorResponse::from(
                &e as &(dyn std::error::Error + Send + Sync),
            )))
        })?;

        let cg = sqlx::query(
            r#"
            INSERT INTO cg (name) VALUES ($1) RETURNING id, created_at, updated_at
            "#,
        )
        .bind(&body.name)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(e)
                if e.is_unique_violation()
                    && e.constraint()
                        .is_some_and(|constraint| constraint == "cg_name_key") =>
            {
                NewCGResponseError::BadRequest(payload::Json(ErrorResponse {
                    message: format!("CG with name '{}' already exists", body.name),
                }))
            }
            _ => NewCGResponseError::InternalServerError(payload::Json(ErrorResponse::from(
                &e as &(dyn std::error::Error + Send + Sync),
            ))),
        })?;

        let result = sqlx::query(r#"UPDATE "user" SET cg_id = $1 WHERE id = $2"#)
            .bind(&cg.get::<i32, _>("id"))
            .bind(body.user_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| match e {
                _ => NewCGResponseError::InternalServerError(payload::Json(ErrorResponse::from(
                    &e as &(dyn std::error::Error + Send + Sync),
                ))),
            })?;
        if result.rows_affected() == 0 {
            return Err(NewCGResponseError::NotFound(payload::Json(ErrorResponse {
                message: format!("User with id '{}' not found", body.user_id),
            })));
        }

        tx.commit().await.map_err(|e| {
            NewCGResponseError::InternalServerError(payload::Json(ErrorResponse::from(
                &e as &(dyn std::error::Error + Send + Sync),
            )))
        })?;

        Ok(NewCGResponse::Ok(payload::Json(CG {
            id: cg.get("id"),
            name: body.name.clone(),
            created_at: cg.get("created_at"),
            updated_at: cg.get("updated_at"),
        })))
    }
}
