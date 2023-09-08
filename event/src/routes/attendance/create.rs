use poem::web;
use poem_openapi::{payload, Object};
use serde::{Deserialize, Serialize};

use crate::{database::Database, entities, error::ErrorResponse};

#[derive(Debug, Clone, Deserialize, Serialize, Object)]
#[oai(rename = "CreateAttendanceRequest")]
pub struct Request {
    session_id: String,
    user_id: String,
}

#[derive(poem_openapi::ApiResponse)]
pub enum Response {
    #[oai(status = 200)]
    Ok(payload::Json<entities::Attendance>),
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
    pub async fn _create_attendance(
        &self,
        db: web::Data<&Database>,
        body: payload::Json<Request>,
    ) -> Result<Response, Error> {
        let attendance = sqlx::query_as!(
            entities::Attendance,
            r#"
            INSERT INTO "attendance" (
                session_id,
                user_id
            ) VALUES (
                $1,
                $2
            )
            RETURNING *
            "#,
            &body.session_id,
            &body.user_id
        )
        .fetch_one(&db.db)
        .await
        .map_err(|e| match e {
            //TODO : Check and see if this pkey check is required for composite pkey relation
            sqlx::Error::Database(e)
                if e.is_unique_violation()
                    && e.constraint()
                        .is_some_and(|constraint| constraint == "session_id_pkey") =>
            {
                Error::BadRequest(payload::Json(ErrorResponse {
                    message: format!("Session id '{}' already exists", body.session_id),
                }))
            }
            sqlx::Error::Database(e)
                if e.is_unique_violation()
                    && e.constraint()
                        .is_some_and(|constraint| constraint == "user_id_pkey") =>
            {
                Error::BadRequest(payload::Json(ErrorResponse {
                    message: format!("User id '{}' already exists", body.user_id),
                }))
            }
            sqlx::Error::Database(e)
                if e.is_foreign_key_violation()
                    && e.constraint()
                        .is_some_and(|constraint| constraint == "session_id_fkey") =>
            {
                Error::BadRequest(payload::Json(ErrorResponse {
                    message: format!("Session id with id '{}' does not exists", body.session_id),
                }))
            }
            _ => Error::InternalServerError(payload::Json(ErrorResponse::from(
                &e as &(dyn std::error::Error + Send + Sync),
            ))),
        })?;

        Ok(Response::Ok(payload::Json(attendance)))
    }
}
