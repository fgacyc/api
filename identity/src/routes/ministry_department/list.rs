use poem::web;
use poem_openapi::payload;

use crate::{database::Database, entities, error::ErrorResponse};

#[derive(poem_openapi::ApiResponse)]
pub enum Response {
    #[oai(status = 200)]
    Ok(payload::Json<Vec<entities::MinistryDepartment>>),
}

#[derive(poem_openapi::ApiResponse)]
pub enum Error {
    #[oai(status = 400)]
    BadRequest(payload::Json<ErrorResponse>),

    #[oai(status = 500)]
    InternalServerError(payload::Json<ErrorResponse>),
}

impl crate::routes::Routes {
    pub async fn _list_ministry_department(
        &self,
        db: web::Data<&Database>,
    ) -> Result<Response, Error> {
        let ministry_departments = sqlx::query_as!(
            entities::MinistryDepartment,
            r#"
            SELECT * from ministry_department
            "#,
        )
        .fetch_all(&db.db)
        .await
        .map_err(|e| match e {
            _ => Error::InternalServerError(payload::Json(ErrorResponse::from(
                &e as &(dyn std::error::Error + Send + Sync),
            ))),
        })?;

        Ok(Response::Ok(payload::Json(ministry_departments)))
    }
}
