use poem::web;
use poem_openapi::{param::Path, payload, Object};
use serde::{Deserialize, Serialize};

use crate::{database::Database, error::ErrorResponse};

#[derive(Debug, Clone, Deserialize, Serialize, Object, PartialEq, Eq, Hash)]
#[oai(rename = "AssociateUsersWithConnectGroupRequestUser")]
pub struct UserRole {
    user_id: String,
    role_id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Object)]
#[oai(rename = "AssociateUsersWithConnectGroupRequest")]
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
    pub async fn _associate_users_with_connect_group(
        &self,
        db: web::Data<&Database>,
        id: Path<String>,
        body: payload::Json<Request>,
    ) -> Result<Response, Error> {
        sqlx::QueryBuilder::new(
            r#"INSERT INTO user_connect_group (
                user_id, 
                connect_group_id, 
                user_role
            ) "#,
        )
        .push_values(&body.users, |mut b, user| {
            b.push_bind(&user.user_id)
                .push_bind(&*id)
                .push_bind(&user.role_id);
        })
        .push("ON CONFLICT (user_id, connect_group_id) DO UPDATE 
                        SET user_role = EXCLUDED.user_role")
        .build()
        .execute(&db.db)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(e)
                if e.is_unique_violation()
                    && e.constraint()
                        .is_some_and(|constraint| constraint == "user_connect_group_pkey") =>
            {
                Error::BadRequest(payload::Json(ErrorResponse {
                    message: format!(
                        "One of the user in the list is already associated with the connect group \
                         '{}'",
                        &*id
                    ),
                }))
            }
            sqlx::Error::Database(e)
                if e.is_foreign_key_violation()
                    && e.constraint().is_some_and(|constraint| {
                        constraint == "user_connect_group_connect_group_id_fkey"
                    }) =>
            {
                Error::BadRequest(payload::Json(ErrorResponse {
                    message: format!("Connect group with id '{}' does not exists", &*id),
                }))
            }
            sqlx::Error::Database(e)
                if e.is_foreign_key_violation()
                    && e.constraint().is_some_and(|constraint| {
                        constraint == "user_connect_group_user_id_fkey"
                    }) =>
            {
                Error::BadRequest(payload::Json(ErrorResponse {
                    message: format!("One of the user in the list does not exists"),
                }))
            }
            sqlx::Error::Database(e)
                if e.is_foreign_key_violation()
                    && e.constraint().is_some_and(|constraint| {
                        constraint == "user_connect_group_user_role_fkey"
                    }) =>
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
