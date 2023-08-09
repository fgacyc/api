/// The configuration parameters for the application.
///
/// These can either be passed on the command line, or pulled from environment variables.
/// The latter is preferred as environment variables are one of the recommended ways to
/// get configuration from Kubernetes Secrets in deployment.
///
/// For development convenience, these can also be read from a `.env` file in the working
/// directory where the application is started.
///
/// See `.env.example` in the repository root for details.
#[derive(clap::Parser, Debug, Clone)]
pub struct Config {
    /// The port number that the application should listen on.
    #[clap(long, env, default_value = "0.0.0.0")]
    pub address: String,

    /// The port number that the application should listen on.
    #[clap(long, env, default_value_t = 8000)]
    pub port: u16,

    /// The OpenAPI URL Address.
    #[clap(long, env, default_value = "http://0.0.0.0:8000")]
    pub oai_address: String,

    /// The connection URL for the Postgres database this application should use.
    #[clap(long, env)]
    pub database_url: String,

    /// The domain for Auth0.
    #[clap(long, env)]
    pub auth0_domain: String,

    /// The client id for Auth0 app.
    #[clap(long, env)]
    pub auth0_client_id: String,

    /// The client secret for Auth0 app.
    #[clap(long, env)]
    pub auth0_client_secret: String,

    /// The name of the database connection for Auth0 app.
    #[clap(long, env)]
    pub auth0_connection: String,
}
