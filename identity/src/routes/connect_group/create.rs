use poem::web;
use poem_openapi::{payload, Object};
use serde::{Deserialize, Serialize};

use crate::{database::Database, entities, error::ErrorResponse};

#[derive(Debug, Clone, Deserialize, Serialize, Object)]
#[oai(rename = "CreateConnectGroupRequest")]
pub struct Request {
    name: String,
    no: i32,
    #[oai(validator(max_length = 2))]
    variant: String,
    satellite_id: String,
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
    pub async fn _create_connect_group(
        &self,
        db: web::Data<&Database>,
        body: payload::Json<Request>,
    ) -> Result<Response, Error> {
        let cg = sqlx::query_as!(
            entities::ConnectGroup,
            r#"
            INSERT INTO connect_group (
                id, 
                name, 
                no,
                variant, 
                satellite_id
            ) VALUES (
                $1,
                $2,
                $3,
                $4,
                $5
            ) 
            RETURNING *
            "#,
            &format!("connect_group_{}", ulid::Ulid::new()),
            &body.name,
            &body.no,
            &body.variant,
            &body.satellite_id,
        )
        .fetch_one(&db.db)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(e)
                if e.is_unique_violation()
                    && e.constraint().is_some_and(|constraint| {
                        constraint == "connect_group_satellite_id_no_variant_key"
                    }) =>
            {
                Error::BadRequest(payload::Json(ErrorResponse {
                    message: format!(
                        "CG with satellite_id '{}', no '{}' and variant '{}' already exists",
                        body.satellite_id, body.no, body.variant
                    ),
                }))
            }
            sqlx::Error::Database(e)
                if e.is_foreign_key_violation()
                    && e.constraint().is_some_and(|constraint| {
                        constraint == "connect_group_satellite_id_fkey"
                    }) =>
            {
                Error::BadRequest(payload::Json(ErrorResponse {
                    message: format!("Satellite with id '{}' does not exists", body.satellite_id),
                }))
            }
            _ => Error::InternalServer(payload::Json(ErrorResponse::from(
                &e as &(dyn std::error::Error + Send + Sync),
            ))),
        })?;

        Ok(Response::Ok(payload::Json(cg)))
    }
}
