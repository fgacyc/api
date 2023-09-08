use poem::web;
use poem_openapi::{payload, Object};
use serde::{Deserialize, Serialize};

use crate::{database::Database, entities, error::ErrorResponse};

#[derive(Debug, Clone, Deserialize, Serialize, Object)]
#[oai(rename = "CreatePriceRequest")]
pub struct Request {
    event_id: String,
    name: String,
    fee: i32,
    #[oai(validator(max_length = 3))]
    currency_code: String,
}

#[derive(poem_openapi::ApiResponse)]
pub enum Response {
    #[oai(status = 200)]
    Ok(payload::Json<entities::Price>),
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
    pub async fn _create_price(
        &self,
        db: web::Data<&Database>,
        body: payload::Json<Request>,
    ) -> Result<Response, Error> {
        let price = sqlx::query_as!(
            entities::Price,
            r#"
            INSERT INTO price (
                event_id, 
                name,
                fee,
                currency_code
            ) VALUES (
                $1,
                $2,
                $3,
                $4
            ) 
            RETURNING *
            "#,
            &body.event_id,
            &body.name,
            &body.fee,
            &body.currency_code,
        )
        .fetch_one(&db.db)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(e)
                if e.is_foreign_key_violation()
                    && e.constraint()
                        .is_some_and(|constraint| constraint == "price_event_id_fkey") =>
            {
                Error::BadRequest(payload::Json(ErrorResponse {
                    message: format!("Event with id '{}' does not exists", body.event_id),
                }))
            }
            sqlx::Error::Database(e)
                if e.is_foreign_key_violation()
                    && e.constraint()
                        .is_some_and(|constraint| constraint == "price_currency_code_fkey") =>
            {
                Error::BadRequest(payload::Json(ErrorResponse {
                    message: format!("Currency Code '{}' does not exists", body.currency_code),
                }))
            }
            _ => Error::InternalServer(payload::Json(ErrorResponse::from(
                &e as &(dyn std::error::Error + Send + Sync),
            ))),
        })?;

        Ok(Response::Ok(payload::Json(price)))
    }
}
