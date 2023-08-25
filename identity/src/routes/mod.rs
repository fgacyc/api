use poem::web;
use poem_openapi::{param::Path, payload, OpenApi, Tags};

use crate::database::Database;

mod connect_group;
mod ministry;
mod ministry_department;
mod ministry_role;
mod ministry_team;
mod pastoral_role;
mod satellite;
mod users;

#[derive(Tags)]
enum Tag {
    /// User related endpoints
    User,

    /// Satellite related endpoints
    Satellite,

    /// Connect group related endpoints
    ConnectGroup,

    /// Ministry team related endpoints
    MinistryTeam,

    /// Ministry department related endpoints
    MinistryDepartment,

    /// Ministry related endpoints
    Ministry,

    /// Pastoral role related endpoints
    PastoralRole,

    /// Ministry role related endpoints
    MinistryRole,
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
    /* User */

    /// Create a user
    ///
    /// Create a new user given its information. This endpoint ideally should be called after a
    /// user is created in Auth0 such as after signup or logging in through social providers.
    #[oai(
        path = "/users",
        method = "post",
        operation_id = "create-user",
        tag = "Tag::User"
    )]
    async fn create_user(
        &self,
        db: web::Data<&Database>,
        body: payload::Json<users::create::Request>,
    ) -> Result<users::create::Response, users::create::Error> {
        self._create_user(db, body).await
    }

    /// List or search users
    ///
    /// Retrieve a list of users or search for users given a query. For now it only supports
    /// retrieving a list of users.
    #[oai(
        path = "/users",
        method = "get",
        operation_id = "list-users",
        tag = "Tag::User"
    )]
    async fn list_users(
        &self,
        db: web::Data<&Database>,
    ) -> Result<users::list::Response, users::list::Error> {
        self._list_users(db).await
    }

    /// Get a user
    ///
    /// Retrieve a user's details given its id.
    #[oai(
        path = "/users/:id",
        method = "get",
        operation_id = "get-user",
        tag = "Tag::User"
    )]
    async fn get_user(
        &self,
        db: web::Data<&Database>,
        id: Path<String>,
    ) -> Result<users::get::Response, users::get::Error> {
        self._get_user(db, id).await
    }

    /// Update a user
    ///
    /// Update a user's details given its id and the corresponding fields to update.
    #[oai(
        path = "/users/:id",
        method = "patch",
        operation_id = "update-user",
        tag = "Tag::User"
    )]
    async fn update_user(
        &self,
        db: web::Data<&Database>,
        id: Path<String>,
        body: payload::Json<users::update::Request>,
    ) -> Result<users::update::Response, users::update::Error> {
        self._update_user(db, id, body).await
    }

    /// Delete a user
    ///
    /// Deletes a user based on the id from the database.
    #[oai(
        path = "/users/:id",
        method = "delete",
        operation_id = "delete-user",
        tag = "Tag::User"
    )]
    async fn delete_user(
        &self,
        db: web::Data<&Database>,
        id: Path<String>,
    ) -> Result<users::delete::Response, users::delete::Error> {
        self._delete_user(db, id).await
    }

    /// Get a user's pastoral roles
    ///
    /// List the pastoral roles associated with a user.
    #[oai(
        path = "/users/:id/pastoral-roles",
        method = "get",
        operation_id = "get-user-pastoral-roles",
        tag = "Tag::User"
    )]
    async fn get_user_pastoral_roles(
        &self,
        db: web::Data<&Database>,
        id: Path<String>,
    ) -> Result<users::get_pastoral_roles::Response, users::get_pastoral_roles::Error> {
        self._get_user_pastoral_roles(db, id).await
    }

    /// Get a user's ministry roles
    ///
    /// List the ministry roles associated with a user.
    #[oai(
        path = "/users/:id/ministry-roles",
        method = "get",
        operation_id = "get-user-ministry-roles",
        tag = "Tag::User"
    )]
    async fn get_user_ministry_roles(
        &self,
        db: web::Data<&Database>,
        id: Path<String>,
    ) -> Result<users::get_ministry_roles::Response, users::get_ministry_roles::Error> {
        self._get_user_ministry_roles(db, id).await
    }

    /// Get a user's connect groups
    ///
    /// List the connect groups that a user is in.
    #[oai(
        path = "/users/:id/connect-groups",
        method = "get",
        operation_id = "get-user-connect-groups",
        tag = "Tag::User"
    )]
    async fn get_user_connect_groups(
        &self,
        db: web::Data<&Database>,
        id: Path<String>,
    ) -> Result<users::get_connect_groups::Response, users::get_connect_groups::Error> {
        self._get_user_connect_groups(db, id).await
    }

    /// Get a user's ministries
    ///
    /// List the ministries that a user is in.
    #[oai(
        path = "/users/:id/ministries",
        method = "get",
        operation_id = "get-user-ministries",
        tag = "Tag::User"
    )]
    async fn get_user_ministries(
        &self,
        db: web::Data<&Database>,
        id: Path<String>,
    ) -> Result<users::get_ministries::Response, users::get_ministries::Error> {
        self._get_ministries(db, id).await
    }

    /* Satellite */

    /// Create a satellite
    ///
    /// Create a new satellite given its information.
    #[oai(
        path = "/satellites",
        method = "post",
        operation_id = "create-satellite",
        tag = "Tag::Satellite"
    )]
    async fn create_satellite(
        &self,
        db: web::Data<&Database>,
        body: payload::Json<satellite::create::Request>,
    ) -> Result<satellite::create::Response, satellite::create::Error> {
        self._create_satellite(db, body).await
    }

    /// List or search satellites
    ///
    /// Retrieve a list of satellites or search for satellites given a query.
    #[oai(
        path = "/satellites",
        method = "get",
        operation_id = "list-satellites",
        tag = "Tag::Satellite"
    )]
    async fn list_satellites(
        &self,
        db: web::Data<&Database>,
    ) -> Result<satellite::list::Response, satellite::list::Error> {
        self._list_satellites(db).await
    }

    /// Get a satellite
    ///
    /// Retrieve a satellite's details given its id.
    #[oai(
        path = "/satellites/:id",
        method = "get",
        operation_id = "get-satellite",
        tag = "Tag::Satellite"
    )]
    async fn get_satellite(
        &self,
        db: web::Data<&Database>,
        id: Path<String>,
    ) -> Result<satellite::get::Response, satellite::get::Error> {
        self._get_satellite(db, id).await
    }

    /// Update a satellite
    ///
    /// Update a satellite's details given its id and the corresponding fields to update.
    #[oai(
        path = "/satellites/:id",
        method = "patch",
        operation_id = "update-satellite",
        tag = "Tag::Satellite"
    )]
    async fn update_satellite(
        &self,
        db: web::Data<&Database>,
        id: Path<String>,
        body: payload::Json<satellite::update::Request>,
    ) -> Result<satellite::update::Response, satellite::update::Error> {
        self._update_satellite(db, id, body).await
    }

    /// Delete a satellite
    ///
    /// Delete a satellite given its id.
    #[oai(
        path = "/satellites/:id",
        method = "delete",
        operation_id = "delete-satellite",
        tag = "Tag::Satellite"
    )]
    async fn delete_satellite(
        &self,
        db: web::Data<&Database>,
        id: Path<String>,
    ) -> Result<satellite::delete::Response, satellite::delete::Error> {
        self._delete_satellite(db, id).await
    }

    /* Connect Group */

    /// Create a connect group
    ///
    /// Create a new connect group given its information.
    #[oai(
        path = "/connect-groups",
        method = "post",
        operation_id = "create-connect-group",
        tag = "Tag::ConnectGroup"
    )]
    async fn create_connect_group(
        &self,
        db: web::Data<&Database>,
        body: payload::Json<connect_group::create::Request>,
    ) -> Result<connect_group::create::Response, connect_group::create::Error> {
        self._create_connect_group(db, body).await
    }

    /// List or search connect groups
    ///
    /// Retrieve a list of connect groups or search for connect groups given a query.
    #[oai(
        path = "/connect-groups",
        method = "get",
        operation_id = "list-connect-groups",
        tag = "Tag::ConnectGroup"
    )]
    async fn list_connect_groups(
        &self,
        db: web::Data<&Database>,
    ) -> Result<connect_group::list::Response, connect_group::list::Error> {
        self._list_connect_groups(db).await
    }

    /// Get a connect group
    ///
    /// Retrieve a connect group's details given its id.
    #[oai(
        path = "/connect-groups/:id",
        method = "get",
        operation_id = "get-connect-group",
        tag = "Tag::ConnectGroup"
    )]
    async fn get_connect_group(
        &self,
        db: web::Data<&Database>,
        id: Path<String>,
    ) -> Result<connect_group::get::Response, connect_group::get::Error> {
        self._get_connect_group(db, id).await
    }

    /// Update a connect group
    ///
    /// Update a connect group's details given its id and the corresponding fields to update.
    #[oai(
        path = "/connect-groups/:id",
        method = "patch",
        operation_id = "update-connect-group",
        tag = "Tag::ConnectGroup"
    )]
    async fn update_connect_group(
        &self,
        db: web::Data<&Database>,
        id: Path<String>,
        body: payload::Json<connect_group::update::Request>,
    ) -> Result<connect_group::update::Response, connect_group::update::Error> {
        self._update_connect_group(db, id, body).await
    }

    /// Delete a connect group
    ///
    /// Delete a connect group given its id.
    #[oai(
        path = "/connect-groups/:id",
        method = "delete",
        operation_id = "delete-connect-group",
        tag = "Tag::ConnectGroup"
    )]
    async fn delete_connect_group(
        &self,
        db: web::Data<&Database>,
        id: Path<String>,
    ) -> Result<connect_group::delete::Response, connect_group::delete::Error> {
        self._delete_connect_group(db, id).await
    }

    /// Associate users with a connect group
    ///
    /// Associate users with a connect group given the connect group's id and the users' ids.
    #[oai(
        path = "/connect-groups/:id/users",
        method = "post",
        operation_id = "associate-users-with-connect-group",
        tag = "Tag::ConnectGroup"
    )]
    async fn associate_users_with_connect_group(
        &self,
        id: Path<String>,
    ) -> payload::PlainText<String> {
        payload::PlainText("unimplemented".to_string())
    }

    /// Get connect group users
    ///
    /// Retrieve a list of users associated with a connect group given the connect group's id.
    #[oai(
        path = "/connect-groups/:id/users",
        method = "get",
        operation_id = "get-connect-group-users",
        tag = "Tag::ConnectGroup"
    )]
    async fn get_connect_group_users(&self, id: Path<String>) -> payload::PlainText<String> {
        payload::PlainText("unimplemented".to_string())
    }

    /// Remove users from a connect group
    ///
    /// Remove users from a connect group given the connect group's id and the users' ids.
    #[oai(
        path = "/connect-groups/:id/users",
        method = "delete",
        operation_id = "remove-users-from-connect-group",
        tag = "Tag::ConnectGroup"
    )]
    async fn remove_users_from_connect_group(
        &self,
        id: Path<String>,
    ) -> payload::PlainText<String> {
        payload::PlainText("unimplemented".to_string())
    }

    /* Pastoral Roles */

    /// Create a pastoral role
    ///
    /// Create a new pastoral role given its information.
    #[oai(
        path = "/pastoral-roles",
        method = "post",
        operation_id = "create-pastoral-role",
        tag = "Tag::PastoralRole"
    )]
    async fn create_pastoral_role(
        &self,
        db: web::Data<&Database>,
        body: payload::Json<pastoral_role::create::Request>,
    ) -> Result<pastoral_role::create::Response, pastoral_role::create::Error> {
        self._create_pastoral_role(db, body).await
    }

    /// List or search pastoral roles
    ///
    /// Retrieve a list of pastoral roles or search for pastoral roles given a query.
    #[oai(
        path = "/pastoral-roles",
        method = "get",
        operation_id = "list-pastoral-roles",
        tag = "Tag::PastoralRole"
    )]
    async fn list_pastoral_roles(
        &self,
        db: web::Data<&Database>,
    ) -> Result<pastoral_role::list::Response, pastoral_role::list::Error> {
        self._list_pastoral_roles(db).await
    }

    /// Get a pastoral role
    ///
    /// Retrieve a pastoral role's details given its id.
    #[oai(
        path = "/pastoral-roles/:id",
        method = "get",
        operation_id = "get-pastoral-role",
        tag = "Tag::PastoralRole"
    )]
    async fn get_pastoral_role(
        &self,
        db: web::Data<&Database>,
        id: Path<String>,
    ) -> Result<pastoral_role::get::Response, pastoral_role::get::Error> {
        self._get_pastoral_role(db, id).await
    }

    /// Update a pastoral role
    ///
    /// Update a pastoral role's details given its id and the corresponding fields to update.
    #[oai(
        path = "/pastoral-roles/:id",
        method = "patch",
        operation_id = "update-pastoral-role",
        tag = "Tag::PastoralRole"
    )]
    async fn update_pastoral_role(
        &self,
        db: web::Data<&Database>,
        id: Path<String>,
        body: payload::Json<pastoral_role::update::Request>,
    ) -> Result<pastoral_role::update::Response, pastoral_role::update::Error> {
        self._update_pastoral_role(db, id, body).await
    }

    /// Delete a pastoral role
    ///
    /// Delete a pastoral role given its id.
    #[oai(
        path = "/pastoral-roles/:id",
        method = "delete",
        operation_id = "delete-pastoral-role",
        tag = "Tag::PastoralRole"
    )]
    async fn delete_pastoral_role(
        &self,
        db: web::Data<&Database>,
        id: Path<String>,
    ) -> Result<pastoral_role::delete::Response, pastoral_role::delete::Error> {
        self._delete_pastoral_role(db, id).await
    }

    /* Ministry Roles */

    /// Create a ministry role
    ///
    /// Create a new ministry role given its information.
    #[oai(
        path = "/ministry-roles",
        method = "post",
        operation_id = "create-ministry-role",
        tag = "Tag::MinistryRole"
    )]
    async fn create_ministry_role(
        &self,
        db: web::Data<&Database>,
        body: payload::Json<ministry_role::create::Request>,
    ) -> Result<ministry_role::create::Response, ministry_role::create::Error> {
        self._create_ministry_role(db, body).await
    }

    /// List or search ministry roles
    ///
    /// Retrieve a list of ministry roles or search for ministry roles given a query.
    #[oai(
        path = "/ministry-roles",
        method = "get",
        operation_id = "list-ministry-roles",
        tag = "Tag::MinistryRole"
    )]
    async fn list_ministry_roles(
        &self,
        db: web::Data<&Database>,
    ) -> Result<ministry_role::list::Response, ministry_role::list::Error> {
        self._list_ministry_roles(db).await
    }

    /// Get a ministry role
    ///
    /// Retrieve a ministry role's details given its id.
    #[oai(
        path = "/ministry-roles/:id",
        method = "get",
        operation_id = "get-ministry-role",
        tag = "Tag::MinistryRole"
    )]
    async fn get_ministry_role(
        &self,
        db: web::Data<&Database>,
        id: Path<String>,
    ) -> Result<ministry_role::get::Response, ministry_role::get::Error> {
        self._get_ministry_role(db, id).await
    }

    /// Update a ministry role
    ///
    /// Update a ministry role's details given its id and the corresponding fields to update.
    #[oai(
        path = "/ministry-roles/:id",
        method = "patch",
        operation_id = "update-ministry-role",
        tag = "Tag::MinistryRole"
    )]
    async fn update_ministry_role(
        &self,
        db: web::Data<&Database>,
        id: Path<String>,
        body: payload::Json<ministry_role::update::Request>,
    ) -> Result<ministry_role::update::Response, ministry_role::update::Error> {
        self._update_ministry_role(db, id, body).await
    }

    /// Delete a ministry role
    ///
    /// Delete a ministry role given its id.
    #[oai(
        path = "/ministry-roles/:id",
        method = "delete",
        operation_id = "delete-ministry-role",
        tag = "Tag::MinistryRole"
    )]
    async fn delete_ministry_role(
        &self,
        db: web::Data<&Database>,
        id: Path<String>,
    ) -> Result<ministry_role::delete::Response, ministry_role::delete::Error> {
        self._delete_ministry_role(db, id).await
    }

    /* Ministry Team */

    /// Create a ministry team
    ///
    /// Create a new ministry team given its information.
    #[oai(
        path = "/ministry-teams",
        method = "post",
        operation_id = "create-ministry-team",
        tag = "Tag::MinistryTeam"
    )]
    async fn create_ministry_team(
        &self,
        db: web::Data<&Database>,
        body: payload::Json<ministry_team::create::Request>,
    ) -> Result<ministry_team::create::Response, ministry_team::create::Error> {
        self._create_ministry_team(db, body).await
    }

    /// List or search ministry teams
    ///
    /// Retrieve a list of ministry teams or search for ministry teams given a query.
    #[oai(
        path = "/ministry-teams",
        method = "get",
        operation_id = "list-ministry-team",
        tag = "Tag::MinistryTeam"
    )]
    async fn list_ministry_team(
        &self,
        db: web::Data<&Database>,
    ) -> Result<ministry_team::list::Response, ministry_team::list::Error> {
        self._list_ministry_team(db).await
    }

    /// Get a ministry team
    ///
    /// Retrieve a ministry team's details given its id.
    #[oai(
        path = "/ministry-teams/:id",
        method = "get",
        operation_id = "get-ministry-team",
        tag = "Tag::MinistryTeam"
    )]
    async fn get_ministry_team(
        &self,
        db: web::Data<&Database>,
        id: Path<String>,
    ) -> Result<ministry_team::get::Response, ministry_team::get::Error> {
        self._get_ministry_team(db, id).await
    }

    /// Update a ministry team
    ///
    /// Update a ministry team's details given its id and the corresponding fields to update.
    #[oai(
        path = "/ministry-teams/:id",
        method = "patch",
        operation_id = "update-ministry-team",
        tag = "Tag::MinistryTeam"
    )]
    async fn update_ministry_team(
        &self,
        db: web::Data<&Database>,
        id: Path<String>,
        body: payload::Json<ministry_team::update::Request>,
    ) -> Result<ministry_team::update::Response, ministry_team::update::Error> {
        self._update_ministry_team(db, id, body).await
    }

    /// Delete a ministry team
    ///
    /// Delete a ministry team given its id.
    #[oai(
        path = "/ministry-teams/:id",
        method = "delete",
        operation_id = "delete-ministry-team",
        tag = "Tag::MinistryTeam"
    )]
    async fn delete_ministry_team(
        &self,
        db: web::Data<&Database>,
        id: Path<String>,
    ) -> Result<ministry_team::delete::Response, ministry_team::delete::Error> {
        self._delete_ministry_team(db, id).await
    }

    /* Ministry Departments */

    /// Create a ministry department
    ///
    /// Create a new ministry department given its information.
    #[oai(
        path = "/ministry-departments",
        method = "post",
        operation_id = "create-ministry-department",
        tag = "Tag::MinistryDepartment"
    )]
    async fn create_ministry_department(
        &self,
        db: web::Data<&Database>,
        body: payload::Json<ministry_department::create::Request>,
    ) -> Result<ministry_department::create::Response, ministry_department::create::Error> {
        self._create_ministry_department(db, body).await
    }

    /// List or search ministry departments
    ///
    /// Retrieve a list of ministry departments or search for ministry departments given a query.
    #[oai(
        path = "/ministry-departments",
        method = "get",
        operation_id = "list-ministry-department",
        tag = "Tag::MinistryDepartment"
    )]
    async fn list_ministry_department(
        &self,
        db: web::Data<&Database>,
    ) -> Result<ministry_department::list::Response, ministry_department::list::Error> {
        self._list_ministry_department(db).await
    }

    /// Get a ministry department
    ///
    /// Retrieve a ministry department's details given its id.
    #[oai(
        path = "/ministry-departments/:id",
        method = "get",
        operation_id = "get-ministry-department",
        tag = "Tag::MinistryDepartment"
    )]
    async fn get_ministry_department(
        &self,
        db: web::Data<&Database>,
        id: Path<String>,
    ) -> Result<ministry_department::get::Response, ministry_department::get::Error> {
        self._get_ministry_department(db, id).await
    }

    /// Update a ministry department
    ///
    /// Update a ministry department's details given its id and the corresponding fields to update.
    #[oai(
        path = "/ministry-departments/:id",
        method = "patch",
        operation_id = "update-ministry-department",
        tag = "Tag::MinistryDepartment"
    )]
    async fn update_ministry_department(
        &self,
        db: web::Data<&Database>,
        id: Path<String>,
        body: payload::Json<ministry_department::update::Request>,
    ) -> Result<ministry_department::update::Response, ministry_department::update::Error> {
        self._update_ministry_department(db, id, body).await
    }
    /// Delete a ministry department
    ///
    /// Delete a ministry department given its id.
    #[oai(
        path = "/ministry-departments/:id",
        method = "delete",
        operation_id = "delete-ministry-department",
        tag = "Tag::MinistryDepartment"
    )]
    async fn delete_ministry_department(
        &self,
        db: web::Data<&Database>,
        id: Path<String>,
    ) -> Result<ministry_department::delete::Response, ministry_department::delete::Error> {
        self._delete_ministry_department(db, id).await
    }

    /* Ministry */

    /// Create a ministry
    ///
    /// Create a new ministry given its information.
    #[oai(
        path = "/ministries",
        method = "post",
        operation_id = "create-ministry",
        tag = "Tag::Ministry"
    )]
    async fn create_ministry(
        &self,
        db: web::Data<&Database>,
        body: payload::Json<ministry::create::Request>,
    ) -> Result<ministry::create::Response, ministry::create::Error> {
        self._create_minitry(db, body).await
    }

    /// List or search ministries
    ///
    /// Retrieve a list of ministries or search for ministries given a query.
    #[oai(
        path = "/ministries",
        method = "get",
        operation_id = "list-ministries",
        tag = "Tag::Ministry"
    )]
    async fn list_ministries(
        &self,
        db: web::Data<&Database>,
    ) -> Result<ministry::list::Response, ministry::list::Error> {
        self._list_ministries(db).await
    }

    /// Get a ministry
    ///
    /// Retrieve a ministry's details given its id.
    #[oai(
        path = "/ministries/:id",
        method = "get",
        operation_id = "get-ministry",
        tag = "Tag::Ministry"
    )]
    async fn get_ministry(
        &self,
        db: web::Data<&Database>,
        id: Path<String>,
    ) -> Result<ministry::get::Response, ministry::get::Error> {
        self._get_ministry(db, id).await
    }

    /// Update a ministry
    ///
    /// Update a ministry's details given its id and the corresponding fields to update.
    #[oai(
        path = "/ministries/:id",
        method = "patch",
        operation_id = "update-ministry",
        tag = "Tag::Ministry"
    )]
    async fn update_ministry(&self, id: Path<String>) -> payload::PlainText<String> {
        payload::PlainText("unimplemented".to_string())
    }

    /// Delete a ministry
    ///
    /// Delete a ministry given its id.
    #[oai(
        path = "/ministries/:id",
        method = "delete",
        operation_id = "delete-ministry",
        tag = "Tag::Ministry"
    )]
    async fn delete_ministry(
        &self,
        db: web::Data<&Database>,
        id: Path<String>,
    ) -> Result<ministry::delete::Response, ministry::delete::Error> {
        self._delete_ministry(db, id).await
    }

    /// Associate users with a ministry
    ///
    /// Associate users with a ministry given the ministry's id and the users' ids.
    #[oai(
        path = "/ministries/:id/users",
        method = "post",
        operation_id = "associate-users-with-ministry",
        tag = "Tag::Ministry"
    )]
    async fn associate_users_with_ministry(&self, id: Path<String>) -> payload::PlainText<String> {
        payload::PlainText("unimplemented".to_string())
    }

    /// Get users associated with a ministry
    ///
    /// Retrieve a list of users associated with a ministry given the ministry's id.
    #[oai(
        path = "/ministries/:id/users",
        method = "get",
        operation_id = "get-ministry-users",
        tag = "Tag::Ministry"
    )]
    async fn get_ministry_users(&self, id: Path<String>) -> payload::PlainText<String> {
        payload::PlainText("unimplemented".to_string())
    }

    /// Remove users from a ministry
    ///
    /// Remove users from a ministry given the ministry's id and the users' ids.
    #[oai(
        path = "/ministries/:id/users",
        method = "delete",
        operation_id = "remove-users-from-ministry",
        tag = "Tag::Ministry"
    )]
    async fn remove_users_from_ministry(&self, id: Path<String>) -> payload::PlainText<String> {
        payload::PlainText("unimplemented".to_string())
    }
}
