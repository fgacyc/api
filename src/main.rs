use poem::{listener::TcpListener, web::Data, EndpointExt, Route, Server};
use poem_openapi::{payload::PlainText, OpenApi, OpenApiService};

struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/hello", method = "get")]
    async fn index(&self, data: Data<&i32>) -> PlainText<String> {
        PlainText(format!("{}", data.0))
    }
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    tracing_subscriber::fmt::init();

    let service = OpenApiService::new(Api, "FGACYC Auth", "0.0.1").server("http://localhost:3000");
    let openapi_ui = service.swagger_ui();

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(
            Route::new()
                .nest("/", service.data(100i32))
                .nest("/swagger", openapi_ui),
        )
        .await?;

    Ok(())
}
