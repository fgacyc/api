use poem::web;
use poem_openapi::{param::Path, payload, OpenApi, Tags};

use crate::database::Database;

mod registration;
mod price;

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

	/* Price */

    /// Create price
    #[oai(
        path = "/price",
        method = "post",
        operation_id = "create-price",
        tag = "Tag::Price"
    )]
    async fn create_price(
        &self,
        db: web::Data<&Database>,
        body: payload::Json<price::create::Request>,
    ) -> Result<price::create::Response, price::create::Error> {
        self._create_price(db, body).await
    }

	/// List or search price
    #[oai(
        path = "/price",
        method = "get",
        operation_id = "list-price",
        tag = "Tag::Price"
    )]
    async fn list_price(
        &self,
        db: web::Data<&Database>,
    ) -> Result<price::list::Response, price::list::Error> {
        self._list_price(db).await
    }

	/// Get a price
    #[oai(
        path = "/price/:id",
        method = "get",
        operation_id = "get-price",
        tag = "Tag::Price"
    )]
    async fn get_price(
        &self,
        db: web::Data<&Database>,
        id: Path<String>,
    ) -> Result<price::get::Response, price::get::Error> {
        self._get_price(db, id).await
    }

	/// Update a price
    #[oai(
        path = "/price/:id",
        method = "patch",
        operation_id = "update-price",
        tag = "Tag::Price"
    )]
    async fn update_price(
        &self,
        db: web::Data<&Database>,
        id: Path<String>,
        body: payload::Json<price::update::Request>,
    ) -> Result<price::update::Response, price::update::Error> {
        self._update_price(db, id, body).await
    }

	/// Delete a price
    #[oai(
        path = "/price/:id",
        method = "delete",
        operation_id = "delete-price",
        tag = "Tag::Price"
    )]
    async fn delete_price(
        &self,
        db: web::Data<&Database>,
        id: Path<String>,
    ) -> Result<price::delete::Response, price::delete::Error> {
        self._delete_price(db, id).await
    }
}