use poem::web;
use poem_openapi::{param::Path, payload, Object};
use serde::{Deserialize, Serialize};

use crate::{database::Database, error::ErrorResponse};

#[derive(Debug, Clone, Deserialize, Serialize, Object)]
#[oai(rename = "RemoveUsersFromMinistryRequest")]
pub struct Request {
    #[oai(validator(min_items = 1, unique_items = true))]
    users: Vec<String>,
}

#[derive(poem_openapi::ApiResponse)]
pub enum Response {
    #[oai(status = 200)]
    Ok(payload::Json<String>),
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
    pub async fn _remove_users_from_ministry(
        &self,
        db: web::Data<&Database>,
        id: Path<String>,
        body: payload::Json<Request>,
    ) -> Result<Response, Error> {
        sqlx::query!(
            r#"
            DELETE FROM 
                user_ministry um
            WHERE 
                um.ministry_id = $1::TEXT
                    AND um.user_id = ANY($2::TEXT[])
            "#,
            &*id,
            &body.users
        )
        .execute(&db.db)
        .await
        .map_err(|e| match e {
            _ => Error::InternalServer(payload::Json(ErrorResponse::from(
                &e as &(dyn std::error::Error + Send + Sync),
            ))),
        })?;

        Ok(Response::Ok(payload::Json("success".to_string())))
    }
}
