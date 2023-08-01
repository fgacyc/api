use std::collections::HashMap;

use auth0::{
    authentication::signup::Signup,
    management::{
        roles::{ListRolesRequestParameters, Roles},
        users::{AssignRolesToUserRequestParameters, Users},
    },
};
use poem::{error, web};
use poem_openapi::{payload, Enum, Object};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::Row;

use crate::{config::Config, database::Database, error::ErrorResponse};

#[derive(Debug, Copy, Clone, Deserialize, Serialize, Enum)]
#[serde(rename_all = "lowercase")]
pub enum Gender {
    #[oai(rename = "male")]
    Male,

    #[oai(rename = "female")]
    Female,
}

impl std::fmt::Display for Gender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_value(self).unwrap().as_str().unwrap()
        )
    }
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize, Enum)]
pub enum Role {
    #[oai(rename = "p_team_leader")]
    #[serde(rename = "p_team_leader")]
    PastoralTeamLeader,

    #[oai(rename = "p_coach")]
    #[serde(rename = "p_coach")]
    Coach,

    #[oai(rename = "p_cell_group_leader")]
    #[serde(rename = "p_cell_group_leader")]
    CellGroupLeader,

    #[oai(rename = "p_host")]
    #[serde(rename = "p_host")]
    Host,

    #[oai(rename = "p_ordinary_member")]
    #[serde(rename = "p_ordinary_member")]
    OrdinaryMember,

    #[oai(rename = "p_new_believer")]
    #[serde(rename = "p_new_believer")]
    NewBeliever,

    #[oai(rename = "p_new_friend")]
    #[serde(rename = "p_new_friend")]
    NewFriend,

    #[oai(rename = "p_new_friend")]
    #[serde(rename = "p_new_friend")]
    Visitor,

    #[oai(rename = "m_pic")]
    #[serde(rename = "m_pic")]
    MinistryPersonInCharge,

    #[oai(rename = "m_department_head")]
    #[serde(rename = "m_department_head")]
    MinistryDepartmentHead,

    #[oai(rename = "m_team_lead")]
    #[serde(rename = "m_team_lead")]
    MinistryTeamLead,
}

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_value(self).unwrap().as_str().unwrap()
        )
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Object)]
pub struct SignUpRequest {
    email: String,
    password: String,
    username: String,
    given_name: String,
    family_name: String,
    name: String,
    gender: Gender,
    ic_number: String,
    phone_number: String,
    roles: Option<Vec<Role>>,
    nickname: Option<String>,
    picture: Option<String>,
    cg_id: Option<i32>,
}

#[derive(poem_openapi::ApiResponse)]
pub enum SignUpResponse {
    #[oai(status = 200)]
    Ok,
}

#[derive(poem_openapi::ApiResponse)]
pub enum SignUpResponseError {
    #[oai(status = 400)]
    BadRequest(payload::Json<ErrorResponse>),

    #[oai(status = 404)]
    NotFound(payload::Json<ErrorResponse>),

    #[oai(status = 500)]
    InternalServerError(payload::Json<ErrorResponse>),
}

#[derive(Debug, thiserror::Error)]
pub enum SignUpError {
    #[error("The email address '{email}' is already in use")]
    EmailAlreadyInUse { email: String },

    #[error("The username '{username}' is already in use")]
    UsernameAlreadyInUse { username: String },

    #[error("The gender '{gender}' is not valid, must be either 'male' or 'female'")]
    InvalidGender { gender: Gender },

    #[error("The cg '{cg}' cannot be found")]
    CGNotFound { cg: i32 },

    #[error("Failed to create a user: {reason}")]
    FailedToCreateUserInAuth0 { reason: String },

    #[error("Failed to create a user: {0}")]
    FailedToInsertUserIntoDB(#[from] sqlx::Error),

    #[error("Unknown error: {source}")]
    Unknown {
        source: Box<dyn std::error::Error + Send + Sync>,
    },
}

impl super::Routes {
    pub async fn _signup(
        &self,
        db: web::Data<&Database>,
        config: web::Data<&Config>,
        body: payload::Json<SignUpRequest>,
    ) -> Result<SignUpResponse, SignUpResponseError> {
        let mut tx = db.db.begin().await.map_err(|e| {
            SignUpResponseError::InternalServerError(payload::Json(ErrorResponse::from(
                &SignUpError::from(e) as &(dyn std::error::Error + Send + Sync),
            )))
        })?;

        // Create a user in the database
        let id = sqlx::query(
            r#"
            INSERT INTO "user" (
                email, 
                username, 
                given_name, 
                family_name, 
                name, 
                gender, 
                nickname, 
                picture, 
                ic_number, 
                phone_number, 
                cg_id
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            RETURNING id
            "#,
        )
        .bind(&body.email)
        .bind(&body.username)
        .bind(&body.given_name)
        .bind(&body.family_name)
        .bind(&body.name)
        .bind(&body.gender.to_string())
        .bind(&body.nickname)
        .bind(&body.picture)
        .bind(&body.ic_number)
        .bind(&body.phone_number)
        .bind(&body.cg_id)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(e)
                if e.is_unique_violation()
                    && e.constraint()
                        .is_some_and(|constraint| constraint == "user_email_key") =>
            {
                SignUpResponseError::NotFound(payload::Json(ErrorResponse::from(
                    &SignUpError::EmailAlreadyInUse {
                        email: body.email.clone(),
                    } as &(dyn std::error::Error + Send + Sync),
                )))
            }
            sqlx::Error::Database(e)
                if e.is_unique_violation()
                    && e.constraint()
                        .is_some_and(|constraint| constraint == "user_username_key") =>
            {
                SignUpResponseError::NotFound(payload::Json(ErrorResponse::from(
                    &SignUpError::UsernameAlreadyInUse {
                        username: body.username.clone(),
                    } as &(dyn std::error::Error + Send + Sync),
                )))
            }
            sqlx::Error::Database(e)
                if e.is_foreign_key_violation()
                    && e.constraint()
                        .is_some_and(|constraint| constraint == "user_gender_fkey") =>
            {
                SignUpResponseError::NotFound(payload::Json(ErrorResponse::from(
                    &SignUpError::InvalidGender {
                        gender: body.gender,
                    } as &(dyn std::error::Error + Send + Sync),
                )))
            }
            sqlx::Error::Database(e)
                if e.is_foreign_key_violation()
                    && e.constraint()
                        .is_some_and(|constraint| constraint == "user_cg_id_fkey") =>
            {
                SignUpResponseError::NotFound(payload::Json(ErrorResponse::from(
                    &SignUpError::CGNotFound {
                        cg: body.cg_id.unwrap(),
                    } as &(dyn std::error::Error + Send + Sync),
                )))
            }
            _ => SignUpResponseError::InternalServerError(payload::Json(ErrorResponse::from(
                &SignUpError::from(e) as &(dyn std::error::Error + Send + Sync),
            ))),
        })?
        .get::<i32, _>("id");

        let mut metadata = HashMap::from_iter([
            ("id".to_string(), id.to_string()),
            ("ic_number".to_string(), body.ic_number.to_string()),
            ("phone_number".to_string(), body.phone_number.to_string()),
            ("gender".to_string(), body.gender.to_string()),
        ]);
        if let Some(cg_id) = body.cg_id {
            metadata.insert("cg_id".to_string(), cg_id.to_string());
        }

        // Create a user on auth0
        match self
            .auth
            .signup(auth0::authentication::signup::RequestParameters {
                client_id: config.auth0_client_id.clone(),
                email: body.email.clone(),
                password: body.password.clone(),
                connection: config.auth0_connection.clone(),
                username: Some(body.username.clone()),
                given_name: Some(body.given_name.clone()),
                family_name: Some(body.family_name.clone()),
                name: Some(body.name.clone()),
                nickname: body.nickname.clone(),
                picture: body.picture.clone(),
                user_metadata: Some(metadata),
            })
            .send()
            .await
            .map_err(error::InternalServerError)
        {
            Ok(resp) if resp.status() != StatusCode::OK => {
                let body = resp.text().await.map_err(|e| {
                    SignUpResponseError::InternalServerError(payload::Json(ErrorResponse::from(
                        &SignUpError::Unknown {
                            source: Box::new(e),
                        } as &(dyn std::error::Error + Send + Sync),
                    )))
                })?;
                tracing::error!("Failed to create a user in Auth0: {:?}", body);
                return Err(SignUpResponseError::BadRequest(payload::Json(
                    ErrorResponse::from(&SignUpError::FailedToCreateUserInAuth0 { reason: body }
                        as &(dyn std::error::Error + Send + Sync)),
                )));
            }
            Err(e) => {
                tracing::error!("Failed to create a user in Auth0: {}", e);
                return Err(SignUpResponseError::BadRequest(payload::Json(
                    ErrorResponse::from(&SignUpError::FailedToCreateUserInAuth0 {
                        reason: e.to_string(),
                    }
                        as &(dyn std::error::Error + Send + Sync)),
                )));
            }
            Ok(resp) => {
                // Parse the create user response.
                let auth0::authentication::signup::Response { id, .. } = resp
                    .json::<auth0::authentication::signup::Response>()
                    .await
                    .map_err(|e| {
                        SignUpResponseError::InternalServerError(payload::Json(ErrorResponse {
                            message: e.to_string(),
                        }))
                    })?;
                tracing::info!("Created a user in Auth0 with id {}", id);

                // Add roles to this user.
                if let Some(roles) = body.roles.clone() {
                    let roles = roles
                        .into_iter()
                        .map(|r| r.to_string())
                        .collect::<Vec<String>>();
                    // Fetch the role ids from auth0.
                    let roles = self
                        .management
                        .list_roles(ListRolesRequestParameters {
                            page: None,
                            per_page: None,
                            include_totals: None,
                            name_filter: None,
                        })
                        .send()
                        .await
                        .map_err(|e| {
                            SignUpResponseError::InternalServerError(payload::Json(ErrorResponse {
                                message: e.to_string(),
                            }))
                        })?
                        .json::<Vec<auth0::models::Role>>()
                        .await
                        .map_err(|e| {
                            SignUpResponseError::InternalServerError(payload::Json(ErrorResponse {
                                message: e.to_string(),
                            }))
                        })?
                        .into_iter()
                        .filter_map(|role| match roles.contains(&role.name) {
                            true => Some(role.id),
                            false => None,
                        })
                        .collect::<Vec<String>>();

                    // Call auth0 to assign roles to this user.
                    tracing::info!("Assigning roles {:?} to user {}", roles, id);
                    self.management
                        .assign_roles_to_user(
                            format!("auth0|{}", id),
                            AssignRolesToUserRequestParameters {
                                roles: roles
                                    .clone()
                                    .into_iter()
                                    .map(|role| role.to_string())
                                    .collect(),
                            },
                        )
                        .send()
                        .await
                        .map_err(|e| {
                            SignUpResponseError::InternalServerError(payload::Json(ErrorResponse {
                                message: e.to_string(),
                            }))
                        })?;
                    tracing::info!("Assigned roles {:?} to user {}", roles, id);
                }
            }
        }

        tx.commit().await.map_err(|e| {
            SignUpResponseError::InternalServerError(payload::Json(ErrorResponse::from(
                &SignUpError::from(e) as &(dyn std::error::Error + Send + Sync),
            )))
        })?;

        Ok(SignUpResponse::Ok)
    }
}
