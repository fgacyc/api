use poem::web;
use poem_openapi::{payload, Object};
use serde::{Deserialize, Serialize};

use crate::{database::Database, entities, error::ErrorResponse};

#[derive(Debug, Clone, Deserialize, Serialize, Object)]
#[oai(rename = "CreateMinistryRoleRequest")]
pub struct Request {
    name: String,
	description: String,
	weight: i32,
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
    InternalServerError(payload::Json<ErrorResponse>),
}

impl crate::routes::Routes {
    pub async fn _create_ministry_role(
        &self,
        db: web::Data<&Database>,
        body: payload::Json<Request>,
    ) -> Result<Response, Error> {
        let ministry_role: entities::MinistryRole = sqlx::query_as(
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
                $4
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
                    && e.constraint().is_some_and(|constraint: &str| {
                        constraint == "ministry_role_name_key"
                    }) =>
            {
                Error::BadRequest(payload::Json(ErrorResponse {
                    message: format!(
                        "Ministry role with name '{}' already exists", body.name),
                }))
            }
			_ => Error::InternalServerError(payload::Json(ErrorResponse::from(
				&e as &(dyn std::error::Error + Send + Send + Sync),
			  ))),
			})?;

			Ok(Response::Ok(payload::Json(ministry_role)))
    }
}
