use poem::web;
use poem_openapi::{payload, Object};
use serde::{Deserialize, Serialize};

use crate::{database::Database, entities, error::ErrorResponse};

#[derive(Debug, Clone, Deserialize, Serialize, Object)]
#[oai(rename = "CreateUserRequest")]
pub struct Request {
    id: String,
    no: Option<i32>,
    name: String,
    email: String,
    email_verified: Option<bool>,
    username: Option<String>,
    given_name: Option<String>,
    family_name: Option<String>,
    gender: Option<entities::Gender>,
    ic_number: Option<String>,
    phone_number: Option<String>,
    phone_number_verified: Option<bool>,
    nickname: Option<String>,
    avatar_url: Option<String>,
    address: Option<entities::Address>,
    date_of_birth: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(poem_openapi::ApiResponse)]
pub enum Response {
    #[oai(status = 200)]
    Ok(payload::Json<entities::User>),
}

#[derive(poem_openapi::ApiResponse)]
pub enum Error {
    #[oai(status = 400)]
    BadRequest(payload::Json<ErrorResponse>),

    #[oai(status = 500)]
    InternalServer(payload::Json<ErrorResponse>),
}

impl crate::routes::Routes {
    pub async fn _create_user(
        &self,
        db: web::Data<&Database>,
        body: payload::Json<Request>,
    ) -> Result<Response, Error> {
        let user = sqlx::query_as_unchecked!(
            entities::User,
            r#"
            INSERT INTO "user" (
                id, 
                no, 
                name, 
                email, 
                email_verified, 
                username, 
                given_name, 
                family_name, 
                gender, 
                ic_number, 
                phone_number, 
                phone_number_verified, 
                nickname, 
                avatar_url, 
                address,
                date_of_birth
            ) VALUES (
                $1, 
                $2, 
                $3, 
                $4, 
                $5, 
                $6, 
                $7, 
                $8, 
                $9, 
                $10, 
                $11, 
                $12, 
                $13, 
                $14, 
                $15,
                $16
            ) 
            RETURNING *
            "#,
            &body.id,
            &body.no,
            &body.name,
            &body.email,
            &body.email_verified.unwrap_or(false),
            &body.username,
            &body.given_name,
            &body.family_name,
            &body.gender,
            &body.ic_number,
            &body.phone_number,
            &body.phone_number_verified,
            &body.nickname,
            &body.avatar_url,
            &body.address,
            &body.date_of_birth,
        )
        .fetch_one(&db.db)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(e)
                if e.is_unique_violation()
                    && e.constraint()
                        .is_some_and(|constraint| constraint == "user_pkey") =>
            {
                Error::BadRequest(payload::Json(ErrorResponse {
                    message: format!("User with id '{}' already exists", body.id),
                }))
            }
            sqlx::Error::Database(e)
                if e.is_unique_violation()
                    && e.constraint()
                        .is_some_and(|constraint| constraint == "user_email_key") =>
            {
                Error::BadRequest(payload::Json(ErrorResponse {
                    message: format!("User with email '{}' already exists", body.email),
                }))
            }
            sqlx::Error::Database(e)
                if e.is_unique_violation()
                    && e.constraint()
                        .is_some_and(|constraint| constraint == "user_no_key") =>
            {
                Error::BadRequest(payload::Json(ErrorResponse {
                    message: format!("User with no '{:?}' already exists", body.no),
                }))
            }
            sqlx::Error::Database(e)
                if e.is_unique_violation()
                    && e.constraint()
                        .is_some_and(|constraint| constraint == "user_username_key") =>
            {
                Error::BadRequest(payload::Json(ErrorResponse {
                    message: format!("User with username '{:?}' already exists", body.username),
                }))
            }
            _ => Error::InternalServer(payload::Json(ErrorResponse::from(
                &e as &(dyn std::error::Error + Send + Sync),
            ))),
        })?;

        Ok(Response::Ok(payload::Json(user)))
    }
}
