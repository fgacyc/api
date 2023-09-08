use poem::web;
use poem_openapi::{payload, Object};
use serde::{Deserialize, Serialize};

use crate::{database::Database, entities, error::ErrorResponse};

#[derive(Debug, Clone, Deserialize, Serialize, Object)]
#[oai(rename = "GetAttendanceRequest")]
pub struct Request {
    session_id: String,
    user_id: String,
}

#[derive(poem_openapi::ApiResponse)]
pub enum Response {
    #[oai(status = 200)]
    OK(payload::Json<entities::Attendance>),
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
    pub async fn _get_attendance(
        &self,
        db: web::Data<&Database>,
        body: payload::Json<Request>,
    ) -> Result<Response, Error> {
        let attendance = sqlx::query_as!(
            entities::Attendance,
            r#"
            SELECT * from "attendance"
            WHERE session_id = $1::TEXT
            AND user_id = $2::TEXT
            "#,
            &body.session_id,
            &body.user_id
        )
        .fetch_one(&db.db)
        .await
        .map_err(|e| match e {
            sqlx::error::Error::RowNotFound => Error::NotFound(payload::Json(ErrorResponse {
                message: format!(
                    "Attendance with session_id '{}' and user_id '{}' not found",
                    &body.session_id, &body.user_id
                ),
            })),
            _ => Error::InternalServerError(payload::Json(ErrorResponse::from(
                &e as &(dyn std::error::Error + Send + Sync),
            ))),
        })?;

        Ok(Response::OK(payload::Json(attendance)))
    }
}
