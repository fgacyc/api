use poem::web;
use poem_openapi::{payload, Object};
use serde::{Deserialize, Serialize};

use crate::{database::Database, entities, error::ErrorResponse};

#[derive(Debug, Clone, Deserialize, Serialize, Object)]
#[oai(rename = "CreateConnectGroupCategoryRequest")]
pub struct Request {
    name: String,
}

#[derive(poem_openapi::ApiResponse)]
pub enum Response {
    #[oai(status = 200)]
    Ok(payload::Json<entities::ConnectGroupCategory>),
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
    pub async fn _create_connect_group_category(
        &self,
        db: web::Data<&Database>,
        body: payload::Json<Request>,
    ) -> Result<Response, Error> {
        let cg = sqlx::query_as!(
            entities::ConnectGroupCategory,
            r#"
            INSERT INTO connect_group_category (
                id,
                name
            ) VALUES (
                $1,
                $2
            ) 
            RETURNING *
            "#,
            &format!("connect_group_category_{}", ulid::Ulid::new()),
            &body.name,
        )
        .fetch_one(&db.db)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(e)
                if e.is_unique_violation()
                    && e.constraint().is_some_and(|constraint| {
                        constraint == "connect_group_category_name_key"
                    }) =>
            {
                Error::BadRequest(payload::Json(ErrorResponse {
                    message: format!("CG category with name '{}' already exists", body.name),
                }))
            }
            _ => Error::InternalServer(payload::Json(ErrorResponse::from(
                &e as &(dyn std::error::Error + Send + Sync),
            ))),
        })?;

        Ok(Response::Ok(payload::Json(cg)))
    }
}
