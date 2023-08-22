use poem::web;
use poem_openapi::{payload, Enum, Object};
use serde::{Deserialize, Serialize};

use crate::{database::Database, error::ErrorResponse};

#[derive(Debug, Clone, Deserialize, Serialize, Object)]
#[serde(rename_all = "lowercase")]
pub struct Address {
    line_one: String,
    line_two: Option<String>,
    city: String,
    state: String,
    country: String,
    postal_code: String,
}

impl ::sqlx::encode::Encode<'_, ::sqlx::Postgres> for Address {
    fn encode_by_ref(
        &self,
        buf: &mut ::sqlx::postgres::PgArgumentBuffer,
    ) -> ::sqlx::encode::IsNull {
        let mut encoder = ::sqlx::postgres::types::PgRecordEncoder::new(buf);
        encoder.encode(&self.line_one);
        encoder.encode(&self.line_two);
        encoder.encode(&self.city);
        encoder.encode(&self.state);
        encoder.encode(&self.country);
        encoder.encode(&self.postal_code);
        encoder.finish();
        ::sqlx::encode::IsNull::No
    }
    fn size_hint(&self) -> ::std::primitive::usize {
        6usize * (4 + 4)
            + <String as ::sqlx::encode::Encode<::sqlx::Postgres>>::size_hint(&self.line_one)
            + <Option<String> as ::sqlx::encode::Encode<::sqlx::Postgres>>::size_hint(
                &self.line_two,
            )
            + <String as ::sqlx::encode::Encode<::sqlx::Postgres>>::size_hint(&self.city)
            + <String as ::sqlx::encode::Encode<::sqlx::Postgres>>::size_hint(&self.state)
            + <String as ::sqlx::encode::Encode<::sqlx::Postgres>>::size_hint(&self.country)
            + <String as ::sqlx::encode::Encode<::sqlx::Postgres>>::size_hint(&self.postal_code)
    }
}

impl<'r> ::sqlx::decode::Decode<'r, ::sqlx::Postgres> for Address {
    fn decode(
        value: ::sqlx::postgres::PgValueRef<'r>,
    ) -> ::std::result::Result<
        Self,
        ::std::boxed::Box<
            dyn ::std::error::Error + 'static + ::std::marker::Send + ::std::marker::Sync,
        >,
    > {
        let mut decoder = ::sqlx::postgres::types::PgRecordDecoder::new(value)?;
        let line_one = decoder.try_decode::<String>()?;
        let line_two = decoder.try_decode::<Option<String>>()?;
        let city = decoder.try_decode::<String>()?;
        let state = decoder.try_decode::<String>()?;
        let country = decoder.try_decode::<String>()?;
        let postal_code = decoder.try_decode::<String>()?;
        ::std::result::Result::Ok(Address {
            line_one,
            line_two,
            city,
            state,
            country,
            postal_code,
        })
    }
}

impl ::sqlx::Type<::sqlx::Postgres> for Address {
    fn type_info() -> ::sqlx::postgres::PgTypeInfo {
        ::sqlx::postgres::PgTypeInfo::with_name("_address")
    }
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize, Enum, sqlx::Type)]
#[sqlx(type_name = "gender", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum Gender {
    #[oai(rename = "male")]
    Male,

    #[oai(rename = "female")]
    Female,
}

#[derive(Debug, Clone, Deserialize, Serialize, Object)]
pub struct NewUserRequest {
    id: String,
    no: Option<i32>,
    name: String,
    email: String,
    email_verified: Option<bool>,
    username: Option<String>,
    given_name: Option<String>,
    family_name: Option<String>,
    gender: Option<Gender>,
    ic_number: Option<String>,
    phone_number: Option<String>,
    phone_number_verified: Option<bool>,
    nickname: Option<String>,
    avatar_url: Option<String>,
    address: Option<Address>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Object, sqlx::FromRow)]
pub struct User {
    id: String,
    no: i32,
    email: String,
    email_verified: bool,
    name: String,
    username: Option<String>,
    given_name: Option<String>,
    family_name: Option<String>,
    gender: Option<Gender>,
    ic_number: Option<String>,
    phone_number: Option<String>,
    phone_number_verified: Option<bool>,
    nickname: Option<String>,
    avatar_url: Option<String>,
    address: Option<Address>,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(poem_openapi::ApiResponse)]
pub enum NewUserResponse {
    #[oai(status = 200)]
    Ok(payload::Json<User>),
}

#[derive(poem_openapi::ApiResponse)]
pub enum NewUserResponseError {
    #[oai(status = 400)]
    BadRequest(payload::Json<ErrorResponse>),

    #[oai(status = 500)]
    InternalServerError(payload::Json<ErrorResponse>),
}

impl super::Routes {
    pub async fn _create_user(
        &self,
        db: web::Data<&Database>,
        body: payload::Json<NewUserRequest>,
    ) -> Result<NewUserResponse, NewUserResponseError> {
        let user: User = sqlx::query_as(
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
                address
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
                $15
            ) RETURNING *
            "#,
        )
        .bind(&body.id)
        .bind(&body.no)
        .bind(&body.name)
        .bind(&body.email)
        .bind(&body.email_verified.unwrap_or(false))
        .bind(&body.username)
        .bind(&body.given_name)
        .bind(&body.family_name)
        .bind(&body.gender)
        .bind(&body.ic_number)
        .bind(&body.phone_number)
        .bind(&body.phone_number_verified)
        .bind(&body.nickname)
        .bind(&body.avatar_url)
        .bind(&body.address)
        .fetch_one(&db.db)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(e)
                if e.is_unique_violation()
                    && e.constraint()
                        .is_some_and(|constraint| constraint == "user_pkey") =>
            {
                NewUserResponseError::BadRequest(payload::Json(ErrorResponse {
                    message: format!("User with id '{}' already exists", body.id),
                }))
            }
            sqlx::Error::Database(e)
                if e.is_unique_violation()
                    && e.constraint()
                        .is_some_and(|constraint| constraint == "user_email_key") =>
            {
                NewUserResponseError::BadRequest(payload::Json(ErrorResponse {
                    message: format!("User with email '{}' already exists", body.email),
                }))
            }
            sqlx::Error::Database(e)
                if e.is_unique_violation()
                    && e.constraint()
                        .is_some_and(|constraint| constraint == "user_no_key") =>
            {
                NewUserResponseError::BadRequest(payload::Json(ErrorResponse {
                    message: format!("User with no '{:?}' already exists", body.no),
                }))
            }
            sqlx::Error::Database(e)
                if e.is_unique_violation()
                    && e.constraint()
                        .is_some_and(|constraint| constraint == "user_username_key") =>
            {
                NewUserResponseError::BadRequest(payload::Json(ErrorResponse {
                    message: format!("User with username '{:?}' already exists", body.username),
                }))
            }
            _ => NewUserResponseError::InternalServerError(payload::Json(ErrorResponse::from(
                &e as &(dyn std::error::Error + Send + Sync),
            ))),
        })?;

        Ok(NewUserResponse::Ok(payload::Json(user)))
    }
}

#[derive(poem_openapi::ApiResponse)]
pub enum ListUsersResponse {
    #[oai(status = 200)]
    Ok(payload::Json<Vec<User>>),
}

#[derive(poem_openapi::ApiResponse)]
pub enum ListUsersResponseError {
    #[oai(status = 500)]
    InternalServerError(payload::Json<ErrorResponse>),
}

impl super::Routes {
    pub async fn _list_users(
        &self,
        db: web::Data<&Database>,
    ) -> Result<ListUsersResponse, ListUsersResponseError> {
        let users: Vec<User> = sqlx::query_as(
            r#"
            SELECT * from "user"
            "#,
        )
        .fetch_all(&db.db)
        .await
        .map_err(|e| match e {
            _ => ListUsersResponseError::InternalServerError(payload::Json(ErrorResponse::from(
                &e as &(dyn std::error::Error + Send + Sync),
            ))),
        })?;

        Ok(ListUsersResponse::Ok(payload::Json(users)))
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Object)]
pub struct GetUserRequest {
    id: String,
}

#[derive(poem_openapi::ApiResponse)]
pub enum GetUserResponse {
    #[oai(status = 200)]
    Ok(payload::Json<User>),
}

#[derive(poem_openapi::ApiResponse)]
pub enum GetUserResponseError {
    #[oai(status = 500)]
    InternalServerError(payload::Json<ErrorResponse>),

    #[oai(status = 400)]
    BadRequest(payload::Json<ErrorResponse>),

    #[oai(status = 404)]
    NotFoundError(payload::Json<ErrorResponse>),
}

impl super::Routes {
    pub async fn _get_user(
        &self,
        db: web::Data<&Database>,
        id: String,
    ) -> Result<GetUserResponse, GetUserResponseError> {
        let user: User = sqlx::query_as(
            r#"
            SELECT * from "user" WHERE id = $1::TEXT
            "#,
        )
        .bind(&id)
        .fetch_one(&db.db)
        .await
        .map_err(|e| match e {
            _ => GetUserResponseError::InternalServerError(payload::Json(ErrorResponse::from(
                &e as &(dyn std::error::Error + Send + Sync),
            ))),
        })?;

        Ok(GetUserResponse::Ok(payload::Json(user)))
    }
}
