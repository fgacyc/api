use poem::web;
use poem_openapi::{param::Path, payload, OpenApi, Tags};

use crate::database::Database;

mod event;
mod price;
mod registration;
mod session;
mod attendance;
mod currency;
mod event_type;
mod form_field_type;

#[derive(Tags)]
enum Tag {
    /// Event related endpoints
    Event,

    /// Event Type related endpoints
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

    /// Form related endpoints
    Form,
}

#[allow(unused)]
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

    /// List an event's attendance
    #[oai(
        path = "/event/:id/attendance",
        method = "get",
        operation_id = "list-event-attendance",
        tag = "Tag::Event"
    )]
    async fn list_event_attendance(
        &self,
        db: web::Data<&Database>,
        id: Path<String>,
    ) -> Result<event::list_attendance::Response, event::list_attendance::Error>
    {
        self._list_event_attendance(db, id).await
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

    /// List a session's attendance
    #[oai(
        path = "/session/:id/attendance",
        method = "get",
        operation_id = "list-session-attendance",
        tag = "Tag::Session"
    )]
    async fn list_session_attendance(
        &self,
        db: web::Data<&Database>,
        id: Path<String>,
    ) -> Result<session::list_attendance::Response, session::list_attendance::Error>
    {
        self._list_session_attendance(db, id).await
    }

    /* Atendance */

    /// Create a attendance
    ///
    /// Create a new attendance given its information.
    #[oai(
        path = "/attendance",
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
        path = "/attendance",
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
        path = "/attendance/get",
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
        path = "/attendance",
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
        path = "/currency",
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
        path = "/currency",
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
        path = "/currency/:code",
        method = "get",
        operation_id = "get-currency",
        tag = "Tag::Currency"
    )]
    async fn get_currency(
        &self,
        db: web::Data<&Database>,
        code: Path<String>,
    ) -> Result<currency::get::Response, currency::get::Error> {
        self._get_currency(db, code).await
    }

    /// Update a currency
    ///
    /// Update a currency's details given its code and the corresponding fields to update.
    #[oai(
        path = "/currency/:code",
        method = "patch",
        operation_id = "update-currency",
        tag = "Tag::Currency"
    )]
    async fn update_currency(
        &self,
        db: web::Data<&Database>,
        code: Path<String>,
        body: payload::Json<currency::update::Request>,
    ) -> Result<currency::update::Response, currency::update::Error> {
        self._update_currency(db, code, body).await
    }

    /// Delete a currency
    ///
    /// Delete a currency given its code.
    #[oai(
        path = "/currency/:code",
        method = "delete",
        operation_id = "delete-currency",
        tag = "Tag::Currency"
    )]
    async fn delete_currency(
        &self,
        db: web::Data<&Database>,
        code: Path<String>,
    ) -> Result<currency::delete::Response, currency::delete::Error> {
        self._delete_currency(db, code).await
    }

    /* Event Type */

    /// Create a event type
    ///
    /// Create a new event type given its information.
    #[oai(
        path = "/event-type",
        method = "post",
        operation_id = "create-event-type",
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
        path = "/event-type",
        method = "get",
        operation_id = "list-event-types",
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
        path = "/event-type/:name",
        method = "get",
        operation_id = "get-event-type",
        tag = "Tag::EventType"
    )]
    async fn get_event_type(
        &self,
        db: web::Data<&Database>,
        name: Path<String>,
    ) -> Result<event_type::get::Response, event_type::get::Error> {
        self._get_event_type(db, name).await
    }

    /// Update a event_type
    ///
    /// Update a event_type's details given its name and the corresponding fields to update.
    #[oai(
        path = "/event-type/:name",
        method = "patch",
        operation_id = "update-event-type",
        tag = "Tag::EventType"
    )]
    async fn update_event_type(
        &self,
        db: web::Data<&Database>,
        name: Path<String>,
        body: payload::Json<event_type::update::Request>,
    ) -> Result<event_type::update::Response, event_type::update::Error> {
        self._update_event_type(db, name, body).await
    }

    /// Delete a event_type
    ///
    /// Delete a event_type given its name.
    #[oai(
        path = "/event-type/:name",
        method = "delete",
        operation_id = "delete-event-type",
        tag = "Tag::EventType"
    )]
    async fn delete_event_type(
        &self,
        db: web::Data<&Database>,
        name: Path<String>,
    ) -> Result<event_type::delete::Response, event_type::delete::Error> {
        self._delete_event_type(db, name).await
    }

    /* Form Field Type */
    
    /// Create a new form field type
    ///
    /// Create a form field type given its information.
    #[oai(
        path = "/form-field-type",
        method = "post",
        operation_id = "create-form-field-type",
        tag = "Tag::Form"
    )]
    async fn create_form_field_type(
        &self,
        db: web::Data<&Database>,
        body: payload::Json<form_field_type::create::Request>,
    ) -> Result<form_field_type::create::Response, form_field_type::create::Error> {
        self._create_form_field_type(db, body).await
    }

    /// List or search form field type
    ///
    /// Retrieve a list of form field type.
    #[oai(
        path = "/form-field-type",
        method = "get",
        operation_id = "list-form-field-types",
        tag = "Tag::Form"
    )]
    async fn list_form_field_types(
        &self,
        db: web::Data<&Database>,
    ) -> Result<form_field_type::list::Response, form_field_type::list::Error> {
        self._list_form_field_type(db).await
    }

    /// Get a form field type
    ///
    /// Retrieve a form field type's details given its name.
    #[oai(
        path = "/form-field-type/:type",
        method = "get",
        operation_id = "get-form-field-type",
        tag = "Tag::Form"
    )]
    async fn get_form_field_type(
        &self,
        db: web::Data<&Database>,
        r#type: Path<String>,
    ) -> Result<form_field_type::get::Response, form_field_type::get::Error> {
        self._get_form_field_type(db, r#type).await
    }

    /// Update a form field type
    ///
    /// Update a form field type's details given its name and the corresponding fields to update.
    #[oai(
        path = "/form-field-type/:type",
        method = "patch",
        operation_id = "update-form-field-type",
        tag = "Tag::Form"
    )]
    async fn update_form_field_type(
        &self,
        db: web::Data<&Database>,
        r#type: Path<String>,
        body: payload::Json<form_field_type::update::Request>,
    ) -> Result<form_field_type::update::Response, form_field_type::update::Error> {
        self._update_form_field_type(db, r#type, body).await
    }

    /// Delete a form field type
    ///
    /// Delete a form field type given its name.
    #[oai(
        path = "/form-field-type/:type",
        method = "delete",
        operation_id = "delete-form-field-type",
        tag = "Tag::Form"
    )]
    async fn delete_form_field_type(
        &self,
        db: web::Data<&Database>,
        r#type: Path<String>,
    ) -> Result<form_field_type::delete::Response, form_field_type::delete::Error> {
        self._delete_form_field_type(db, r#type).await
    }
}
