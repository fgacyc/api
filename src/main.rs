use clap::Parser;
use database::Database;
use poem::{
    listener::TcpListener,
    middleware::{Cors, Tracing},
    EndpointExt, Result, Route, Server,
};
use poem_openapi::OpenApiService;
use std::sync::Arc;

use crate::config::Config;

mod config;
mod database;
mod error;
mod routes;

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

    let service = OpenApiService::new(routes::Routes::new(Arc::new(auth)), "FGACYC Auth", "0.0.1")
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
            .nest("/docs", swagger)
            .nest("/explorer", explorer)
            .with(Tracing)
            .with(Cors::new())
            .data(database)
            .data(config),
    )
    .await?;

    Ok(())
}
