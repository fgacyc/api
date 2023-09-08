use poem::web;
use poem_openapi::{payload, Object};
use serde::{Deserialize, Serialize};

use crate::{database::Database, entities, error::ErrorResponse};

#[derive(Debug, Clone, Deserialize, Serialize, Object)]
#[oai(rename = "CreateCurrencyRequest")]
pub struct Request {
    code: String,
    num: i32,
    denominator: i32,
    name: String,
    countries: Vec<String>,
}

#[derive(poem_openapi::ApiResponse)]
pub enum Response {
    #[oai(status = 200)]
    Ok(payload::Json<entities::Currency>),
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
    pub async fn _create_currency(
        &self,
        db: web::Data<&Database>,
        body: payload::Json<Request>,
    ) -> Result<Response, Error> {
        let currency = sqlx::query_as!(
            entities::Currency,
            r#"
            INSERT INTO "currency" (
                code,
                num,
                denominator,
                name,
                countries
            ) VALUES (
                $1,
                $2,
                $3,
                $4,
                $5
            )
            RETURNING *
            "#,
            &body.code,
            &body.num,
            &body.denominator,
            &body.name,
            &body.countries,
        )
        .fetch_one(&db.db)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(e)
                if e.is_unique_violation()
                    && e.constraint()
                        .is_some_and(|constraint| constraint == "currency_pkey") =>
            {
                Error::BadRequest(payload::Json(ErrorResponse {
                    message: format!("Code '{}' already exists", body.code),
                }))
            }
            _ => Error::InternalServerError(payload::Json(ErrorResponse::from(
                &e as &(dyn std::error::Error + Send + Sync),
            ))),
        })?;

        Ok(Response::Ok(payload::Json(currency)))
    }
}
