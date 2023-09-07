use poem::web;
use poem_openapi::{param::Path, payload, OpenApi, Tags};

use crate::database::Database;

mod attendance;
mod currency;
mod event_type;

#[derive(Tags)]
enum Tag {
    /// Event related endpoints
    Event,

    /// EventType related endpoints
    EventType,

    /// Currency related endpoints
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
    // /* Atendance */

    /// Create a attendance
    ///
    /// Create a new attendance given its information.
    #[oai(
        path = "/attendances",
        method = "post",
        operation_id = "create-attendance",
        tag = "Tag::Attendance"
    )]
    async fn create_attendance(
        &self,
        db: web::Data<&Database>,
        body: payload::Json<attendance::create::Request>,
    ) -> Result<attendance::create::Response, attendance::create::Error> {
        self._create_attendance(db, body).await
    }

    /// List or search attendance
    ///
    /// Retrieve a list of attendances or search for attendances given a query.
    #[oai(
        path = "/attendances",
        method = "get",
        operation_id = "list-attendances",
        tag = "Tag::Attendance"
    )]
    async fn list_attendance(
        &self,
        db: web::Data<&Database>,
    ) -> Result<attendance::list::Response, attendance::list::Error> {
        self._list_attendance(db).await
    }

    /// Get a attendance
    ///
    /// Retrieve a attendance details given its session_id and user_id.
    #[oai(
        path = "/attendances/get",
        method = "get",
        operation_id = "get-attendance",
        tag = "Tag::Attendance"
    )]
    async fn get_attendance(
        &self,
        db: web::Data<&Database>,
        body: payload::Json<attendance::get::Request>,
    ) -> Result<attendance::get::Response, attendance::get::Error> {
        self._get_attendance(db, body).await
    }

    /// Delete a attendance
    ///
    /// Delete a attendance given its session_id and user_id.
    #[oai(
        path = "/attendances",
        method = "delete",
        operation_id = "delete-attendance",
        tag = "Tag::Attendance"
    )]
    async fn delete_attendance(
        &self,
        db: web::Data<&Database>,
        body: payload::Json<attendance::delete::Request>,
    ) -> Result<attendance::delete::Response, attendance::delete::Error> {
        self._delete_attendance(db, body).await
    }

    /* Currency */

    /// Create a currency
    ///
    /// Create a new currency given its information.
    #[oai(
        path = "/currencies",
        method = "post",
        operation_id = "create-currency",
        tag = "Tag::Currency"
    )]
    async fn create_currency(
        &self,
        db: web::Data<&Database>,
        body: payload::Json<currency::create::Request>,
    ) -> Result<currency::create::Response, currency::create::Error> {
        self._create_currency(db, body).await
    }

    /// List or search currency
    ///
    /// Retrieve a list of currency or search for currency given a query.
    #[oai(
        path = "/currencies",
        method = "get",
        operation_id = "list-currencies",
        tag = "Tag::Currency"
    )]
    async fn list_currencies(
        &self,
        db: web::Data<&Database>,
    ) -> Result<currency::list::Response, currency::list::Error> {
        self._list_currency(db).await
    }

    /// Get a currency
    ///
    /// Retrieve a currency's details given its code.
    #[oai(
        path = "/currencies/:id",
        method = "get",
        operation_id = "get-currency",
        tag = "Tag::Currency"
    )]
    async fn get_currency(
        &self,
        db: web::Data<&Database>,
        id: Path<String>,
    ) -> Result<currency::get::Response, currency::get::Error> {
        self._get_currency(db, id).await
    }

    /// Update a currency
    ///
    /// Update a currency's details given its code and the corresponding fields to update.
    #[oai(
        path = "/currencies/:id",
        method = "patch",
        operation_id = "update-currency",
        tag = "Tag::Currency"
    )]
    async fn update_currency(
        &self,
        db: web::Data<&Database>,
        id: Path<String>,
        body: payload::Json<currency::update::Request>,
    ) -> Result<currency::update::Response, currency::update::Error> {
        self._update_currency(db, id, body).await
    }

    /// Delete a currency
    ///
    /// Delete a currency given its code.
    #[oai(
        path = "/currencies/:id",
        method = "delete",
        operation_id = "delete-currency",
        tag = "Tag::Currency"
    )]
    async fn delete_currency(
        &self,
        db: web::Data<&Database>,
        id: Path<String>,
    ) -> Result<currency::delete::Response, currency::delete::Error> {
        self._delete_currency(db, id).await
    }

    /* Event Type */

    /// Create a event type
    ///
    /// Create a new event type given its information.
    #[oai(
        path = "/event_types",
        method = "post",
        operation_id = "create-event_type",
        tag = "Tag::EventType"
    )]
    async fn create_event_type(
        &self,
        db: web::Data<&Database>,
        body: payload::Json<event_type::create::Request>,
    ) -> Result<event_type::create::Response, event_type::create::Error> {
        self._create_event_type(db, body).await
    }

    /// List or search event_type
    ///
    /// Retrieve a list of event_type or search for event_type given a query.
    #[oai(
        path = "/event_types",
        method = "get",
        operation_id = "list-satellites",
        tag = "Tag::EventType"
    )]
    async fn list_event_types(
        &self,
        db: web::Data<&Database>,
    ) -> Result<event_type::list::Response, event_type::list::Error> {
        self._list_event_type(db).await
    }

    /// Get a event_type
    ///
    /// Retrieve a event_type's details given its name.
    #[oai(
        path = "/event_types/:id",
        method = "get",
        operation_id = "get-event_types",
        tag = "Tag::EventType"
    )]
    async fn get_event_type(
        &self,
        db: web::Data<&Database>,
        id: Path<String>,
    ) -> Result<event_type::get::Response, event_type::get::Error> {
        self._get_event_type(db, id).await
    }

    /// Update a event_type
    ///
    /// Update a event_type's details given its name and the corresponding fields to update.
    #[oai(
        path = "/event_types/:id",
        method = "patch",
        operation_id = "update-event_types",
        tag = "Tag::EventType"
    )]
    async fn update_event_type(
        &self,
        db: web::Data<&Database>,
        id: Path<String>,
        body: payload::Json<event_type::update::Request>,
    ) -> Result<event_type::update::Response, event_type::update::Error> {
        self._update_event_type(db, id, body).await
    }

    /// Delete a event_type
    ///
    /// Delete a event_type given its name.
    #[oai(
        path = "/event_types/:id",
        method = "delete",
        operation_id = "delete-event_type",
        tag = "Tag::EventType"
    )]
    async fn delete_event_type(
        &self,
        db: web::Data<&Database>,
        id: Path<String>,
    ) -> Result<event_type::delete::Response, event_type::delete::Error> {
        self._delete_event_type(db, id).await
    }

}
