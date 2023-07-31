use auth0::authentication::signup::Signup;
use clap::Parser;
use database::Database;
use poem::{
    error,
    listener::TcpListener,
    middleware::{Cors, Tracing},
    web, EndpointExt, Result, Route, Server,
};
use poem_openapi::{payload::Json, Object, OpenApi, OpenApiService};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio_compat_02::FutureExt;

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
    nickname: Option<String>,
    picture: Option<String>,
    ic_number: Option<String>,
    phone_number: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Object)]
struct RegisterResponse {
    name: String,
}

struct Api {
    auth: Arc<auth0::authentication::Api>,
}

#[OpenApi]
impl Api {
    #[oai(path = "/register", method = "post")]
    async fn index(
        &self,
        db: web::Data<&Database>,
        body: web::Json<RegisterRequest>,
    ) -> Result<Json<RegisterResponse>> {
        let tx = db.db.begin().await.map_err(error::InternalServerError)?;

        let auth = self.auth.clone();

        // Create a user on auth0
        match auth
            .signup(auth0::authentication::signup::RequestParameters {
                client_id: "101".to_string(),
                email: body.email.clone(),
                password: body.password.clone(),
                connection: "Username-Password-Authentication".to_string(),
                username: Some(body.username.clone()),
                given_name: Some(body.given_name.clone()),
                family_name: Some(body.family_name.clone()),
                name: Some(body.name.clone()),
                nickname: body.nickname.clone(),
                picture: body.picture.clone(),
                user_metadata: None,
            })
            .send()
            .compat()
            .await
            .map_err(error::InternalServerError)
        {
            Ok(resp) if resp.status() == StatusCode::NOT_FOUND => {
                let body = resp.text().await.map_err(error::InternalServerError)?;
                tracing::error!("Failed to create a user in Auth0: {:?}", body);
            }
            Err(e) => {
                tracing::error!("Failed to create a user in Auth0: {}", e);
            }
            Ok(resp) => tracing::info!("Created a user in Auth0: {:?}", resp),
        }

        let id = sqlx::query!(
            r#"
            INSERT INTO "user" (email, username, given_name, family_name, name, gender, nickname, picture, ic_number, phone_number)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            "#,
            body.email,
            body.username,
            body.given_name,
            body.family_name,
            body.name,
            body.gender,
            body.nickname,
            body.picture,
            body.ic_number,
            body.phone_number,
        )
        .execute(&db.db)
        .await
        .map_err(error::InternalServerError)?;

        tx.commit().await.map_err(error::InternalServerError)?;

        Ok(Json(RegisterResponse {
            name: "hello world".to_string(),
        }))
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
    let openapi_ui = service.swagger_ui();

    Server::new(TcpListener::bind(&format!(
        "{}:{}",
        config.address, config.port
    )))
    .run(
        Route::new()
            .nest("/", service)
            .nest("/swagger", openapi_ui)
            .with(Tracing)
            .with(Cors::new())
            .data(database),
    )
    .await?;

    Ok(())
}
