use poem::web;
use poem_openapi::{param::Path, payload, OpenApi, Tags};

use crate::database::Database;

mod registration;

#[derive(Tags)]
enum Tag {
    /// Event related endpoints
    Event,

    /// Event Type related endpoints
    Currency,

    /// Registration related endpoints
    Registration,

    /// Price related endpoints
    Price,
        
    /// Session related endpoints
    Session,

    /// Attendance related endpoints
    Attendance,
}

pub struct Routes {
    management: auth0::management::Api,
}

impl Routes {
    pub fn new(management: auth0::management::Api) -> Self {
        Self { management }
    }
}

#[OpenApi]
impl Routes {
    /* Registration */

    /// Create registration
    #[oai(
        path = "/registration",
        method = "post",
        operation_id = "create-registration",
        tag = "Tag::Registration"
    )]
    async fn create_registration(
        &self,
        db: web::Data<&Database>,
        body: payload::Json<registration::create::Request>,
    ) -> Result<registration::create::Response, registration::create::Error> {
        self._create_registration(db, body).await
    }

    /// List or search registration
    #[oai(
        path = "/registration",
        method = "get",
        operation_id = "list-registration",
        tag = "Tag::Registration"
    )]
    async fn list_registration(
        &self,
        db: web::Data<&Database>,
    ) -> Result<registration::list::Response, registration::list::Error> {
        self._list_registrations(db).await
    }

    /// Get a registration
    #[oai(
        path = "/registration/:id",
        method = "get",
        operation_id = "get-registration",
        tag = "Tag::Registration"
    )]
    async fn get_registration(
        &self,
        db: web::Data<&Database>,
        id: Path<String>,
    ) -> Result<registration::get::Response, registration::get::Error> {
        self._get_registration(db, id).await
    }

    /// Update a registration
    #[oai(
        path = "/registration/:id",
        method = "patch",
        operation_id = "update-registration",
        tag = "Tag::Registration"
    )]
    async fn update_registration(
        &self,
        db: web::Data<&Database>,
        id: Path<String>,
        body: payload::Json<registration::update::Request>,
    ) -> Result<registration::update::Response, registration::update::Error> {
        self._update_registration(db, id, body).await
    }

    /// Delete a registration
    #[oai(
        path = "/registration/:id",
        method = "delete",
        operation_id = "delete-registration",
        tag = "Tag::Registration"
    )]
    async fn delete_registration(
        &self,
        db: web::Data<&Database>,
        id: Path<String>,
    ) -> Result<registration::delete::Response, registration::delete::Error> {
        self._delete_registration(db, id).await
    }
}