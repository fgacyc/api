use clap::Parser;
use database::Database;
use poem::{
    middleware::{Cors, Tracing},
    EndpointExt, Result, Route,
};
use poem_openapi::OpenApiService;

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
        std::env::set_var("RUST_LOG", "poem=debug,identity=info,auth0=debug");
    }
    tracing_subscriber::fmt::init();

    // Parse configuration
    let config = Config::parse();

    // Make a connection to the database
    let database = Database::new(&config.database_url).await?;

    // Make the api clients for Auth0
    let auth = auth0::authentication::Api::init(
        reqwest::Url::parse(&config.auth0_domain)?,
        auth0::authentication::AuthenticationMethod::ClientIDClientSecret(
            config.auth0_client_id.clone(),
            config.auth0_client_secret.clone(),
        ),
    );
    let management = auth0::management::Api::init(
        reqwest::Url::parse(&config.auth0_domain)?,
        config.auth0_client_id.clone(),
        config.auth0_client_secret.clone(),
    )
    .await?;

    let service = OpenApiService::new(
        routes::Routes::new(auth, management),
        "FGACYC Auth",
        "0.0.1",
    )
    .server(&config.oai_address);
    let rapidoc = service.rapidoc();
    let specs = service.spec();

    let routes = Route::new()
        .nest("/", service)
        .nest("/docs", rapidoc)
        .at("/specs", poem::endpoint::make_sync(move |_| specs.clone()))
        .with(Tracing)
        .with(Cors::new())
        .data(database)
        .data(config.clone());

    #[cfg(not(feature = "lambda"))]
    poem::Server::new(poem::listener::TcpListener::bind(&format!(
        "{}:{}",
        config.address, config.port
    )))
    .run(routes)
    .await?;

    #[cfg(feature = "lambda")]
    poem_lambda::run(routes)
        .await
        .map_err(|e| anyhow::anyhow!("Failed running on lambda: {:?}", e))?;

    Ok(())
}
