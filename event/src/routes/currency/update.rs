use poem::web;
use poem_openapi::{param::Path, payload, Object};

use crate::{database::Database, entities, error::ErrorResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Object)]
#[oai(rename = "UpdateCurrencyRequest")]
pub struct Request {
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
    InternalServer(payload::Json<ErrorResponse>),
}

impl crate::routes::Routes {
    pub async fn _update_currency(
        &self,
        db: web::Data<&Database>,
        code: Path<String>,
        body: payload::Json<Request>,
    ) -> Result<Response, Error> {
        let currency = sqlx::query_as_unchecked!(
            entities::Currency,
            r#"
            UPDATE currency SET
                num                 = COALESCE($1, num),
                denominator         = COALESCE($2, denominator),
                name                = COALESCE($3, name),
                countries           = COALESCE($4, countries)
            WHERE code = $5
            RETURNING *
            "#,
            &body.num,
            &body.denominator,
            &body.name,
            &body.countries,
            &*code,
        )
        .fetch_one(&db.db)
        .await
        .map_err(|e| match e {
            sqlx::error::Error::RowNotFound => Error::NotFound(payload::Json(ErrorResponse {
                message: format!("Currency with code '{}' not found", &*code),
            })),
            _ => Error::InternalServer(payload::Json(ErrorResponse::from(
                &e as &(dyn std::error::Error + Send + Sync),
            ))),
        })?;

        Ok(Response::Ok(payload::Json(currency)))
    }
}
