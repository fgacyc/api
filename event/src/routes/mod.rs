use poem::web;
use poem_openapi::{param::Path, payload, OpenApi, Tags};

use crate::database::Database;

mod event;
mod price;
mod registration;
mod session;

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
    /* Event */

    /// Create an event
    #[oai(
        path = "/event",
        method = "post",
        operation_id = "create-event",
        tag = "Tag::Event"
    )]
    async fn create_event(
        &self,
        db: web::Data<&Database>,
        body: payload::Json<event::create::Request>,
    ) -> Result<event::create::Response, event::create::Error> {
        self._create_event(db, body).await
    }

    /// List or search event
    #[oai(
        path = "/event",
        method = "get",
        operation_id = "list-event",
        tag = "Tag::Event"
    )]
    async fn list_event(
        &self,
        db: web::Data<&Database>,
    ) -> Result<event::list::Response, event::list::Error> {
        self._list_event(db).await
    }

    /// Get an event
    #[oai(
        path = "/event/:id",
        method = "get",
        operation_id = "get-event",
        tag = "Tag::Event"
    )]
    async fn get_event(
        &self,
        db: web::Data<&Database>,
        id: Path<String>,
    ) -> Result<event::get::Response, event::get::Error> {
        self._get_event(db, id).await
    }

    /// Update an event
    #[oai(
        path = "/event/:id",
        method = "patch",
        operation_id = "update-event",
        tag = "Tag::Event"
    )]
    async fn update_event(
        &self,
        db: web::Data<&Database>,
        id: Path<String>,
        body: payload::Json<event::update::Request>,
    ) -> Result<event::update::Response, event::update::Error> {
        self._update_event(db, id, body).await
    }

    /// Delete an event
    #[oai(
        path = "/event/:id",
        method = "delete",
        operation_id = "delete-event",
        tag = "Tag::Event"
    )]
    async fn delete_event(
        &self,
        db: web::Data<&Database>,
        id: Path<String>,
    ) -> Result<event::delete::Response, event::delete::Error> {
        self._delete_event(db, id).await
    }

    /// List an event's registrations
    #[oai(
        path = "/event/:id/registration",
        method = "get",
        operation_id = "list-event-registration",
        tag = "Tag::Event"
    )]
    async fn list_event_registration(
        &self,
        db: web::Data<&Database>,
        id: Path<String>,
    ) -> Result<event::list_registration::Response, event::list_registration::Error> {
        self._list_event_registration(db, id).await
    }

    /// List an event's price
    #[oai(
        path = "/event/:id/price",
        method = "get",
        operation_id = "list-event-price",
        tag = "Tag::Event"
    )]
    async fn list_event_price(
        &self,
        db: web::Data<&Database>,
        id: Path<String>,
    ) -> Result<event::list_price::Response, event::list_price::Error> {
        self._list_event_price(db, id).await
    }

    /// List an event's sessions
    #[oai(
        path = "/event/:id/session",
        method = "get",
        operation_id = "list-event-session",
        tag = "Tag::Event"
    )]
    async fn list_event_session(
        &self,
        db: web::Data<&Database>,
        id: Path<String>,
    ) -> Result<event::list_session::Response, event::list_session::Error> {
        self._list_event_session(db, id).await
    }

    /// List an event session's attendance
    #[oai(
        path = "/event/:event_id/session/:session_id/attendance",
        method = "get",
        operation_id = "list-event-session-attendance",
        tag = "Tag::Event"
    )]
    async fn list_event_session_attendance(
        &self,
        db: web::Data<&Database>,
        #[oai(name = "event_id")] _event_id: Path<String>,
        session_id: Path<String>,
    ) -> Result<event::list_session_attendance::Response, event::list_session_attendance::Error>
    {
        self._list_event_session_attendance(db, session_id).await
    }

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

    /* Session */

    /// Create session
    #[oai(
        path = "/session",
        method = "post",
        operation_id = "create-session",
        tag = "Tag::Session"
    )]
    async fn create_session(
        &self,
        db: web::Data<&Database>,
        body: payload::Json<session::create::Request>,
    ) -> Result<session::create::Response, session::create::Error> {
        self._create_session(db, body).await
    }

    /// List or search session
    #[oai(
        path = "/session",
        method = "get",
        operation_id = "list-session",
        tag = "Tag::Session"
    )]
    async fn list_session(
        &self,
        db: web::Data<&Database>,
    ) -> Result<session::list::Response, session::list::Error> {
        self._list_session(db).await
    }

    /// Get a session
    #[oai(
        path = "/session/:id",
        method = "get",
        operation_id = "get-session",
        tag = "Tag::Session"
    )]
    async fn get_session(
        &self,
        db: web::Data<&Database>,
        id: Path<String>,
    ) -> Result<session::get::Response, session::get::Error> {
        self._get_session(db, id).await
    }

    /// Update a session
    #[oai(
        path = "/session/:id",
        method = "patch",
        operation_id = "update-session",
        tag = "Tag::Session"
    )]
    async fn update_session(
        &self,
        db: web::Data<&Database>,
        id: Path<String>,
        body: payload::Json<session::update::Request>,
    ) -> Result<session::update::Response, session::update::Error> {
        self._update_session(db, id, body).await
    }

    /// Delete a session
    #[oai(
        path = "/session/:id",
        method = "delete",
        operation_id = "delete-session",
        tag = "Tag::Session"
    )]
    async fn delete_session(
        &self,
        db: web::Data<&Database>,
        id: Path<String>,
    ) -> Result<session::delete::Response, session::delete::Error> {
        self._delete_session(db, id).await
    }
}
