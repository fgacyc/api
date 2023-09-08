use poem::web;
use poem_openapi::{param::Path, payload, ApiResponse, Object};

use crate::{database::Database, entities, error::ErrorResponse};

#[derive(Object)]
#[oai(rename = "GetUserMinistriesRepsonse")]
pub struct ResponseBody {
    ministry: entities::Ministry,
    role: entities::MinistryRole,
}

#[derive(ApiResponse)]
pub enum Response {
    #[oai(status = 200)]
    Ok(payload::Json<Vec<ResponseBody>>),
}

#[derive(ApiResponse)]
pub enum Error {
    #[oai(status = 400)]
    BadRequest(payload::Json<ErrorResponse>),

    #[oai(status = 404)]
    NotFound(payload::Json<ErrorResponse>),

    #[oai(status = 500)]
    InternalServer(payload::Json<ErrorResponse>),
}

impl crate::routes::Routes {
    pub async fn _get_ministries(
        &self,
        db: web::Data<&Database>,
        id: Path<String>,
    ) -> Result<Response, Error> {
        let results = sqlx::query!(
            r#"
            SELECT 
                m.id AS m_id,
                m.name AS m_name,
                m.description AS m_description,
                m.department_id AS m_department_id,
                m.team_id AS m_team_id,
                m.satellite_id AS m_satellite_id,
                m.updated_at AS m_updated_at,
                m.created_at AS m_created_at,
                mr.id AS mr_id,
                mr.name AS mr_name,
                mr.description AS mr_description,
                mr.weight AS mr_weight
            FROM 
                ministry m 
                    INNER JOIN user_ministry um ON m.id = um.ministry_id
                    INNER JOIN ministry_role mr on mr.id = um.user_role 
            WHERE um.user_id = $1
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

        Ok(Response::Ok(payload::Json(
            results
                .into_iter()
                .map(|result| ResponseBody {
                    ministry: entities::Ministry {
                        id: result.m_id,
                        name: result.m_name,
                        description: result.m_description,
                        department_id: result.m_department_id,
                        team_id: result.m_team_id,
                        satellite_id: result.m_satellite_id,
                        created_at: result.m_created_at,
                        updated_at: result.m_updated_at,
                    },
                    role: entities::MinistryRole {
                        id: result.mr_id,
                        name: result.mr_name,
                        description: result.mr_description,
                        weight: result.mr_weight,
                    },
                })
                .collect(),
        )))
    }
}
