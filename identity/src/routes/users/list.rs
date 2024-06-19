use poem::web;
use poem_openapi::{param, payload};

use crate::{database::Database, entities, error::ErrorResponse};

#[derive(poem_openapi::ApiResponse)]
pub enum Response {
    #[oai(status = 200)]
    Ok(payload::Json<Vec<entities::User>>),
}

#[derive(poem_openapi::ApiResponse)]
pub enum Error {
    #[oai(status = 400)]
    BadRequest(payload::Json<ErrorResponse>),

    #[oai(status = 500)]
    InternalServer(payload::Json<ErrorResponse>),
}

impl crate::routes::Routes {
    pub async fn _list_users(
        &self,
        search: param::Query<Option<String>>,
        db: web::Data<&Database>,
    ) -> Result<Response, Error> {
        let users = sqlx::query_as_unchecked!(
            entities::User,
            r#"
            SELECT * 
            FROM "user"
            WHERE 
                to_tsvector(email || ' ' || name || ' ' || username || ' ' || given_name || ' ' || family_name) @@ to_tsquery($1)
            "#,
            search.clone().unwrap_or("".to_string()),
        )
        .fetch_all(&db.db)
        .await
        .map_err(|e| match e {
            _ => Error::InternalServer(payload::Json(ErrorResponse::from(
                &e as &(dyn std::error::Error + Send + Sync),
            ))),
        })?;

        Ok(Response::Ok(payload::Json(users)))
    }
}
