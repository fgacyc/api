use poem::web;
use poem_openapi::{param::Path, payload, ApiResponse, Object};

use crate::{database::Database, entities, error::ErrorResponse};

#[derive(Object)]
#[oai(rename = "GetUserConnectGroupsRepsonse")]
pub struct ResponseBody {
    cg: entities::ConnectGroup,
    role: entities::PastoralRole,
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
    pub async fn _get_user_connect_groups(
        &self,
        db: web::Data<&Database>,
        id: Path<String>,
    ) -> Result<Response, Error> {
        let results = sqlx::query!(
            r#"
            SELECT 
                cg.id AS cg_id,
                cg.no AS cg_no,
                cg.name AS cg_name,
                cg.variant AS cg_variant,
                cg.satellite_id AS cg_satellite_id,
                cg.category_id AS cg_category_id,
                cg.active AS cg_active,
                cg.closed_at AS cg_closed_at,
                cg.updated_at AS cg_updated_at,
                cg.created_at AS cg_created_at,
                pr.id AS pr_id,
                pr.name AS pr_name,
                pr.description AS pr_description,
                pr.weight AS pr_weight
            FROM 
                connect_group cg 
                    INNER JOIN user_connect_group ucg ON cg.id = ucg.connect_group_id
                    INNER JOIN pastoral_role pr on pr.id = ucg.user_role 
            WHERE ucg.user_id = $1
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
                    cg: entities::ConnectGroup {
                        id: result.cg_id,
                        no: result.cg_no,
                        name: result.cg_name,
                        variant: result.cg_variant,
                        satellite_id: result.cg_satellite_id,
                        category_id: result.cg_category_id,
                        active: result.cg_active,
                        closed_at: result.cg_closed_at,
                        created_at: result.cg_created_at,
                        updated_at: result.cg_updated_at,
                    },
                    role: entities::PastoralRole {
                        id: result.pr_id,
                        name: result.pr_name,
                        description: result.pr_description,
                        weight: result.pr_weight,
                    },
                })
                .collect(),
        )))
    }
}
