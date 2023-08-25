use poem::web;
use poem_openapi::{param::Path, payload, Object};
use serde::{Deserialize, Serialize};

use crate::{database::Database, error::ErrorResponse};

#[derive(Debug, Clone, Deserialize, Serialize, Object, PartialEq, Eq, Hash)]
#[oai(rename = "AssociateUsersWithMinistryRequestUser")]
pub struct UserRole {
    user_id: String,
    role_id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Object)]
#[oai(rename = "AssociateUsersWithMinistryRequest")]
pub struct Request {
    #[oai(validator(min_items = 1, unique_items = true))]
    users: Vec<UserRole>,
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
    pub async fn _associate_users_with_ministry(
        &self,
        db: web::Data<&Database>,
        id: Path<String>,
        body: payload::Json<Request>,
    ) -> Result<Response, Error> {
        sqlx::QueryBuilder::new(
            r#"INSERT INTO user_ministry (
                user_id, 
                ministry_id, 
                user_role
            ) "#,
        )
        .push_values(&body.users, |mut b, user| {
            b.push_bind(&user.user_id)
                .push_bind(&*id)
                .push_bind(&user.role_id);
        })
        .build()
        .execute(&db.db)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(e)
                if e.is_unique_violation()
                    && e.constraint()
                        .is_some_and(|constraint| constraint == "user_ministry_pkey") =>
            {
                Error::BadRequest(payload::Json(ErrorResponse {
                    message: format!(
                        "One of the user in the list is already associated with the ministry '{}'",
                        &*id
                    ),
                }))
            }
            sqlx::Error::Database(e)
                if e.is_foreign_key_violation()
                    && e.constraint().is_some_and(|constraint| {
                        constraint == "user_ministry_ministry_id_fkey"
                    }) =>
            {
                Error::BadRequest(payload::Json(ErrorResponse {
                    message: format!("Ministry with id '{}' does not exists", &*id),
                }))
            }
            sqlx::Error::Database(e)
                if e.is_foreign_key_violation()
                    && e.constraint()
                        .is_some_and(|constraint| constraint == "user_ministry_user_id_fkey") =>
            {
                Error::BadRequest(payload::Json(ErrorResponse {
                    message: format!("One of the user in the list does not exists"),
                }))
            }
            sqlx::Error::Database(e)
                if e.is_foreign_key_violation()
                    && e.constraint()
                        .is_some_and(|constraint| constraint == "user_ministry_user_role_fkey") =>
            {
                Error::BadRequest(payload::Json(ErrorResponse {
                    message: format!("One of the role in the list does not exists"),
                }))
            }
            _ => Error::InternalServer(payload::Json(ErrorResponse::from(
                &e as &(dyn std::error::Error + Send + Sync),
            ))),
        })?;

        Ok(Response::Ok(payload::Json("success".to_string())))
    }
}
