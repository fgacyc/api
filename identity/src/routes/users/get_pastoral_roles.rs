use poem::web;
use poem_openapi::{param::Path, payload};

use crate::{database::Database, entities, error::ErrorResponse};

#[derive(poem_openapi::ApiResponse)]
pub enum Response {
    #[oai(status = 200)]
    Ok(payload::Json<Vec<entities::PastoralRole>>),
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
    pub async fn _get_user_pastoral_roles(
        &self,
        db: web::Data<&Database>,
        id: Path<String>,
    ) -> Result<Response, Error> {
        let roles = sqlx::query_as!(
            entities::PastoralRole,
            r#"
            SELECT 
                pr.* 
            FROM 
                pastoral_role pr 
                    INNER JOIN user_connect_group ucg ON pr.id = ucg.user_role
            WHERE ucg.user_id = $1::TEXT
            "#,
            &*id
        )
        .fetch_all(&db.db)
        .await
        .map_err(|e| match e {
            _ => Error::InternalServer(payload::Json(ErrorResponse::from(
                &e as &(dyn std::error::Error + Send + Sync),
            ))),
        })?;

        Ok(Response::Ok(payload::Json(roles)))
    }
}
