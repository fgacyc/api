use poem::web;
use poem_openapi::{param::Path, payload, Object};
use serde::{Deserialize, Serialize};

use crate::{database::Database, entities, error::ErrorResponse};

#[derive(Debug, Clone, Deserialize, Serialize, Object)]
#[oai(rename = "CreateUserRelationshipRequest")]
pub struct Request {
    destination_user_id: String,
    relationship: String,
}

#[derive(poem_openapi::ApiResponse)]
pub enum Response {
    #[oai(status = 200)]
    Ok(payload::Json<entities::UserRelationship>),
}

#[derive(poem_openapi::ApiResponse)]
pub enum Error {
    #[oai(status = 400)]
    BadRequest(payload::Json<ErrorResponse>),

    #[oai(status = 500)]
    InternalServer(payload::Json<ErrorResponse>),
}

impl crate::routes::Routes {
    pub async fn _create_user_relationship(
        &self,
        db: web::Data<&Database>,
        id: Path<String>,
        body: payload::Json<Request>,
    ) -> Result<Response, Error> {
        let user_relationship = sqlx::query_as!(
            entities::UserRelationship,
            r#"
            INSERT INTO user_relationship (
                source_user_id,
                destination_user_id,
                relationship
            ) VALUES (
                $1, 
                $2, 
                $3
            ) 
            RETURNING *
            "#,
            &*id,
            &body.destination_user_id,
            &body.relationship,
        )
        .fetch_one(&db.db)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(e)
                if e.is_unique_violation()
                    && e.constraint()
                        .is_some_and(|constraint| constraint == "user_relationship_pkey") =>
            {
                Error::BadRequest(payload::Json(ErrorResponse {
                    message: format!(
                        "source_user_id({}) and destination_user_id({}) already has a relationship",
                        *id, body.destination_user_id
                    ),
                }))
            }
            sqlx::Error::Database(e)
                if e.constraint()
                    .is_some_and(|constraint| constraint == "source_destination_check") =>
            {
                Error::BadRequest(payload::Json(ErrorResponse {
                    message: format!("source_user_id and destination_user_id must be different"),
                }))
            }
            _ => Error::InternalServer(payload::Json(ErrorResponse::from(
                &e as &(dyn std::error::Error + Send + Sync),
            ))),
        })?;

        Ok(Response::Ok(payload::Json(user_relationship)))
    }
}
