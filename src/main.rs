use auth0::authentication::signup::Signup;
use clap::Parser;
use database::Database;
use poem::{
    error,
    listener::TcpListener,
    middleware::{Cors, Tracing},
    web, EndpointExt, Result, Route, Server,
};
use poem_openapi::{
    payload::{self, PlainText},
    Object, OpenApi, OpenApiService,
};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::Row;
use std::{collections::HashMap, iter::FromIterator, sync::Arc};

use crate::config::Config;

mod config;
mod database;

#[derive(Debug, Clone, Deserialize, Serialize, Object)]
struct RegisterRequest {
    email: String,
    password: String,
    username: String,
    given_name: String,
    family_name: String,
    name: String,
    gender: String,
    ic_number: String,
    phone_number: String,
    nickname: Option<String>,
    picture: Option<String>,
    cg_id: Option<i32>,
}

#[derive(poem_openapi::ApiResponse)]
enum RegisterResponse {
    #[oai(status = 200)]
    Ok,
}

#[derive(poem_openapi::ApiResponse)]
enum RegisterResponseError {
    #[oai(status = 400)]
    BadRequest(PlainText<String>),

    #[oai(status = 404)]
    NotFound(PlainText<String>),

    #[oai(status = 500)]
    InternalServerError(PlainText<String>),
}

#[derive(Debug, thiserror::Error)]
enum RegisterError {
    #[error("The email address is already in use")]
    EmailAlreadyInUse,

    #[error("The username is already in use")]
    UsernameAlreadyInUse,

    #[error("Failed to create a user: {reason}")]
    FailedToCreateUserInAuth0 { reason: String },

    #[error("Failed to create a user: {0}")]
    FailedToInsertUserIntoDB(#[from] sqlx::Error),

    #[error("Unknown error: {source}")]
    Unknown {
        source: Box<dyn std::error::Error + Send + Sync>,
    },
}

impl error::ResponseError for RegisterError {
    fn status(&self) -> StatusCode {
        match self {
            RegisterError::EmailAlreadyInUse | RegisterError::UsernameAlreadyInUse => {
                StatusCode::NOT_FOUND
            }
            RegisterError::FailedToCreateUserInAuth0 { .. } => StatusCode::BAD_REQUEST,
            RegisterError::FailedToInsertUserIntoDB(_) => StatusCode::INTERNAL_SERVER_ERROR,
            RegisterError::Unknown { .. } => StatusCode::NOT_FOUND,
        }
    }
}

struct Api {
    auth: Arc<auth0::authentication::Api>,
}

#[OpenApi]
impl Api {
    #[oai(path = "/register", method = "post")]
    async fn register(
        &self,
        db: web::Data<&Database>,
        config: web::Data<&Config>,
        body: payload::Json<RegisterRequest>,
    ) -> Result<RegisterResponse, RegisterResponseError> {
        let mut tx = db.db.begin().await.map_err(|e| {
            RegisterResponseError::InternalServerError(PlainText(
                RegisterError::from(e).to_string(),
            ))
        })?;

        // Create a user in the database
        let id = sqlx::query(
            r#"
            INSERT INTO "user" (email, username, given_name, family_name, name, gender, nickname, picture, ic_number, phone_number, cg_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            RETURNING id
            "#,
        )
        .bind(body.email.clone())
        .bind(body.username.clone())
        .bind(body.given_name.clone())
        .bind(body.family_name.clone())
        .bind(body.name.clone())
        .bind(body.gender.clone())
        .bind(body.nickname.clone())
        .bind(body.picture.clone())
        .bind(body.ic_number.clone())
        .bind(body.phone_number.clone())
        .bind(body.cg_id.clone())
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| {
            RegisterResponseError::InternalServerError(PlainText(
                RegisterError::from(e).to_string(),
            ))
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
                    RegisterResponseError::InternalServerError(PlainText(
                        RegisterError::Unknown {
                            source: Box::new(e),
                        }
                        .to_string(),
                    ))
                })?;
                tracing::error!("Failed to create a user in Auth0: {:?}", body);
                return Err(RegisterResponseError::BadRequest(PlainText(
                    RegisterError::FailedToCreateUserInAuth0 { reason: body }.to_string(),
                )));
            }
            Err(e) => {
                tracing::error!("Failed to create a user in Auth0: {}", e);
                return Err(RegisterResponseError::BadRequest(PlainText(
                    RegisterError::FailedToCreateUserInAuth0 {
                        reason: e.to_string(),
                    }
                    .to_string(),
                )));
            }
            Ok(resp) => tracing::info!("Created a user in Auth0: {:?}", resp),
        }

        tx.commit().await.map_err(|e| {
            RegisterResponseError::InternalServerError(PlainText(
                RegisterError::from(e).to_string(),
            ))
        })?;

        Ok(RegisterResponse::Ok)
    }
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // This returns an error if the `.env` file doesn't exist, but that's not what we want
    // since we're not going to use a `.env` file if we deploy this application.
    dotenvy::dotenv().ok();

    // Set up tracing
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug,auth=info");
    }
    tracing_subscriber::fmt::init();

    // Parse configuration
    let config = Config::parse();

    // Make a connection to the database
    let database = Database::new(&config.database_url).await?;

    // Make an api client for Auth0
    let auth = auth0::authentication::Api::init(
        reqwest::Url::parse(&config.auth0_domain)?,
        auth0::authentication::AuthenicationMethod::ClientIDClientSecret(
            config.auth0_client_id.clone(),
            config.auth0_client_secret.clone(),
        ),
    );

    let service = OpenApiService::new(
        Api {
            auth: Arc::new(auth),
        },
        "FGACYC Auth",
        "0.0.1",
    )
    .server(&format!("http://{}:{}", config.address, config.port));
    let swagger = service.swagger_ui();
    let explorer = service.openapi_explorer();

    Server::new(TcpListener::bind(&format!(
        "{}:{}",
        config.address, config.port
    )))
    .run(
        Route::new()
            .nest("/", service)
            .nest("/swagger", swagger)
            .nest("/explorer", explorer)
            .with(Tracing)
            .with(Cors::new())
            .data(database)
            .data(config),
    )
    .await?;

    Ok(())
}
