use poem::web;
use poem_openapi::{payload, OpenApi};

mod cg;
mod health;
mod signup;

pub struct Routes {
    auth: auth0::authentication::Api,
    management: auth0::management::Api,
}

impl Routes {
    pub fn new(auth: auth0::authentication::Api, management: auth0::management::Api) -> Self {
        Self { auth, management }
    }
}

#[OpenApi]
impl Routes {
    #[oai(path = "/health", method = "get")]
    async fn health(&self) -> payload::PlainText<String> {
        self._health().await
    }

    #[oai(path = "/signup", method = "post")]
    async fn signup(
        &self,
        db: web::Data<&crate::database::Database>,
        config: web::Data<&crate::config::Config>,
        body: payload::Json<signup::SignUpRequest>,
    ) -> Result<signup::SignUpResponse, signup::SignUpResponseError> {
        self._signup(db, config, body).await
    }

    #[oai(path = "/cg", method = "post")]
    async fn new_cg(
        &self,
        db: web::Data<&crate::database::Database>,
        body: payload::Json<cg::NewCGRequest>,
    ) -> Result<cg::NewCGResponse, cg::NewCGResponseError> {
        self._new_cg(db, body).await
    }
}
