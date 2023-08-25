use poem::web;
use poem_openapi::{param::Path, payload, Object};
use serde::{Deserialize, Serialize};

use crate::{database::Database, entities, error::ErrorResponse};

#[derive(Debug, Clone, Deserialize, Serialize, Object)]
#[oai(rename = "UpdateUserRequest")]
pub struct Request {
    id: String,
    no: Option<i32>,
    name: Option<String>,
    email: Option<String>,
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

    #[oai(status = 404)]
    NotFound(payload::Json<ErrorResponse>),

    #[oai(status = 500)]
    InternalServer(payload::Json<ErrorResponse>),
}

impl crate::routes::Routes {
    pub async fn _update_user(
        &self,
        db: web::Data<&Database>,
        id: Path<String>,
        body: payload::Json<Request>,
    ) -> Result<Response, Error> {
        let user = sqlx::query_as_unchecked!(
            entities::User,
            r#"
            UPDATE "user" SET  
                name                  = COALESCE($1, name),
                email                 = COALESCE($2, email), 
                email_verified        = COALESCE($3, email_verified),
                username              = COALESCE($4, username),
                given_name            = COALESCE($5, given_name),
                family_name           = COALESCE($6, family_name),
                gender                = COALESCE($7, gender), 
                ic_number             = COALESCE($8, ic_number), 
                phone_number          = COALESCE($9, phone_number), 
                phone_number_verified = COALESCE($10, phone_number_verified), 
                nickname              = COALESCE($11, nickname),
                avatar_url            = COALESCE($12, avatar_url),
                address               = COALESCE($13, address),
                date_of_birth         = COALESCE($14, date_of_birth),
                updated_at            = NOW()
            WHERE id = $15
            RETURNING *
            "#,
            &body.name,
            &body.email,
            &body.email_verified,
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
            &*id,
        )
        .fetch_one(&db.db)
        .await
        .map_err(|e| match e {
            sqlx::error::Error::RowNotFound => Error::NotFound(payload::Json(ErrorResponse {
                message: format!("User with id '{}' not found", &*id),
            })),
            _ => Error::InternalServer(payload::Json(ErrorResponse::from(
                &e as &(dyn std::error::Error + Send + Sync),
            ))),
        })?;

        Ok(Response::Ok(payload::Json(user)))
    }
}
