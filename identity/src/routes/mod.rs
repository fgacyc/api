use poem::web;
use poem_openapi::{param::Path, payload, OpenApi};

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

    // #[oai(path = "/signup", method = "post")]
    // async fn signup(
    //     &self,
    //     db: web::Data<&crate::database::Database>,
    //     config: web::Data<&crate::config::Config>,
    //     body: payload::Json<signup::SignUpRequest>,
    // ) -> Result<signup::SignUpResponse, signup::SignUpResponseError> {
    //     self._signup(db, config, body).await
    // }

    /* User */

    #[oai(path = "/users", method = "post")]
    async fn create_user(&self) -> payload::PlainText<String> {
        payload::PlainText("unimplemented".to_string())
    }

    #[oai(path = "/users", method = "get")]
    async fn list_users(&self) -> payload::PlainText<String> {
        payload::PlainText("unimplemented".to_string())
    }

    #[oai(path = "/users/:id", method = "get")]
    async fn get_user(&self, id: Path<String>) -> payload::PlainText<String> {
        payload::PlainText("unimplemented".to_string())
    }

    #[oai(path = "/users/:id", method = "patch")]
    async fn update_user(&self, id: Path<String>) -> payload::PlainText<String> {
        payload::PlainText("unimplemented".to_string())
    }

    #[oai(path = "/users/:id", method = "delete")]
    async fn delete_user(&self, id: Path<String>) -> payload::PlainText<String> {
        payload::PlainText("unimplemented".to_string())
    }

    #[oai(path = "/users/:id/roles", method = "get")]
    async fn get_user_roles(&self, id: Path<String>) -> payload::PlainText<String> {
        payload::PlainText("unimplemented".to_string())
    }

    #[oai(path = "/users/:id/connect-groups", method = "get")]
    async fn get_user_connect_groups(&self, id: Path<String>) -> payload::PlainText<String> {
        payload::PlainText("unimplemented".to_string())
    }

    #[oai(path = "/users/:id/ministries", method = "get")]
    async fn get_user_ministries(&self, id: Path<String>) -> payload::PlainText<String> {
        payload::PlainText("unimplemented".to_string())
    }

    /* Satellite */

    #[oai(path = "/satellites", method = "post")]
    async fn create_satellite(&self) -> payload::PlainText<String> {
        payload::PlainText("unimplemented".to_string())
    }

    #[oai(path = "/satellites", method = "get")]
    async fn list_satellites(&self) -> payload::PlainText<String> {
        payload::PlainText("unimplemented".to_string())
    }

    #[oai(path = "/satellites/:id", method = "get")]
    async fn get_satellite(&self, id: Path<String>) -> payload::PlainText<String> {
        payload::PlainText("unimplemented".to_string())
    }

    #[oai(path = "/satellites/:id", method = "patch")]
    async fn update_satellite(&self, id: Path<String>) -> payload::PlainText<String> {
        payload::PlainText("unimplemented".to_string())
    }

    #[oai(path = "/satellites/:id", method = "delete")]
    async fn delete_satellite(&self, id: Path<String>) -> payload::PlainText<String> {
        payload::PlainText("unimplemented".to_string())
    }

    /* Connect Group */

    #[oai(path = "/connect-groups", method = "post")]
    async fn create_connect_group(&self) -> payload::PlainText<String> {
        payload::PlainText("unimplemented".to_string())
    }

    #[oai(path = "/connect-groups", method = "get")]
    async fn list_connect_groups(&self) -> payload::PlainText<String> {
        payload::PlainText("unimplemented".to_string())
    }

    #[oai(path = "/connect-groups/:id", method = "get")]
    async fn get_connect_group(&self, id: Path<String>) -> payload::PlainText<String> {
        payload::PlainText("unimplemented".to_string())
    }

    #[oai(path = "/connect-groups/:id", method = "patch")]
    async fn update_connect_group(&self, id: Path<String>) -> payload::PlainText<String> {
        payload::PlainText("unimplemented".to_string())
    }

    #[oai(path = "/connect-groups/:id", method = "delete")]
    async fn delete_connect_group(&self, id: Path<String>) -> payload::PlainText<String> {
        payload::PlainText("unimplemented".to_string())
    }

    #[oai(path = "/connect-groups/:id/users", method = "post")]
    async fn associate_users_with_connect_group(
        &self,
        id: Path<String>,
    ) -> payload::PlainText<String> {
        payload::PlainText("unimplemented".to_string())
    }

    #[oai(path = "/connect-groups/:id/users", method = "get")]
    async fn get_connect_group_users(&self, id: Path<String>) -> payload::PlainText<String> {
        payload::PlainText("unimplemented".to_string())
    }

    #[oai(path = "/connect-groups/:id/users", method = "delete")]
    async fn remove_users_from_connect_group(
        &self,
        id: Path<String>,
    ) -> payload::PlainText<String> {
        payload::PlainText("unimplemented".to_string())
    }

    /* Pastoral Roles */

    #[oai(path = "/pastoral-roles", method = "post")]
    async fn create_pastoral_role(&self) -> payload::PlainText<String> {
        payload::PlainText("unimplemented".to_string())
    }

    #[oai(path = "/pastoral-roles", method = "get")]
    async fn list_pastoral_roles(&self) -> payload::PlainText<String> {
        payload::PlainText("unimplemented".to_string())
    }

    #[oai(path = "/pastoral-roles/:id", method = "get")]
    async fn get_pastoral_role(&self, id: Path<String>) -> payload::PlainText<String> {
        payload::PlainText("unimplemented".to_string())
    }

    #[oai(path = "/pastoral-roles/:id", method = "patch")]
    async fn update_pastoral_role(&self, id: Path<String>) -> payload::PlainText<String> {
        payload::PlainText("unimplemented".to_string())
    }

    #[oai(path = "/pastoral-roles/:id", method = "delete")]
    async fn delete_pastoral_role(&self, id: Path<String>) -> payload::PlainText<String> {
        payload::PlainText("unimplemented".to_string())
    }

    /* Ministry Roles */

    #[oai(path = "/ministry-roles", method = "post")]
    async fn create_ministry_role(&self) -> payload::PlainText<String> {
        payload::PlainText("unimplemented".to_string())
    }

    #[oai(path = "/ministry-roles", method = "get")]
    async fn list_ministry_roles(&self) -> payload::PlainText<String> {
        payload::PlainText("unimplemented".to_string())
    }

    #[oai(path = "/ministry-roles/:id", method = "get")]
    async fn get_ministry_role(&self, id: Path<String>) -> payload::PlainText<String> {
        payload::PlainText("unimplemented".to_string())
    }

    #[oai(path = "/ministry-roles/:id", method = "patch")]
    async fn update_ministry_role(&self, id: Path<String>) -> payload::PlainText<String> {
        payload::PlainText("unimplemented".to_string())
    }

    #[oai(path = "/ministry-roles/:id", method = "delete")]
    async fn delete_ministry_role(&self, id: Path<String>) -> payload::PlainText<String> {
        payload::PlainText("unimplemented".to_string())
    }

    /* Ministry Team */

    #[oai(path = "/ministry-teams", method = "post")]
    async fn create_ministry_team(&self) -> payload::PlainText<String> {
        payload::PlainText("unimplemented".to_string())
    }

    #[oai(path = "/ministry-teams", method = "get")]
    async fn list_ministry_team(&self) -> payload::PlainText<String> {
        payload::PlainText("unimplemented".to_string())
    }

    #[oai(path = "/ministry-teams/:id", method = "get")]
    async fn get_ministry_team(&self, id: Path<String>) -> payload::PlainText<String> {
        payload::PlainText("unimplemented".to_string())
    }

    #[oai(path = "/ministry-teams/:id", method = "patch")]
    async fn update_ministry_team(&self, id: Path<String>) -> payload::PlainText<String> {
        payload::PlainText("unimplemented".to_string())
    }

    #[oai(path = "/ministry-teams/:id", method = "delete")]
    async fn delete_ministry_team(&self, id: Path<String>) -> payload::PlainText<String> {
        payload::PlainText("unimplemented".to_string())
    }

    /* Ministry Departments */

    #[oai(path = "/ministry-departments", method = "post")]
    async fn create_ministry_department(&self) -> payload::PlainText<String> {
        payload::PlainText("unimplemented".to_string())
    }

    #[oai(path = "/ministry-departments", method = "get")]
    async fn list_ministry_department(&self) -> payload::PlainText<String> {
        payload::PlainText("unimplemented".to_string())
    }

    #[oai(path = "/ministry-departments/:id", method = "get")]
    async fn get_ministry_department(&self, id: Path<String>) -> payload::PlainText<String> {
        payload::PlainText("unimplemented".to_string())
    }

    #[oai(path = "/ministry-departments/:id", method = "patch")]
    async fn update_ministry_department(&self, id: Path<String>) -> payload::PlainText<String> {
        payload::PlainText("unimplemented".to_string())
    }

    #[oai(path = "/ministry-departments/:id", method = "delete")]
    async fn delete_ministry_department(&self, id: Path<String>) -> payload::PlainText<String> {
        payload::PlainText("unimplemented".to_string())
    }

    /* Ministry */

    #[oai(path = "/ministries", method = "post")]
    async fn create_ministry(&self) -> payload::PlainText<String> {
        payload::PlainText("unimplemented".to_string())
    }

    #[oai(path = "/ministries", method = "get")]
    async fn list_ministries(&self) -> payload::PlainText<String> {
        payload::PlainText("unimplemented".to_string())
    }

    #[oai(path = "/ministries/:id", method = "get")]
    async fn get_ministry(&self, id: Path<String>) -> payload::PlainText<String> {
        payload::PlainText("unimplemented".to_string())
    }

    #[oai(path = "/ministries/:id", method = "patch")]
    async fn update_ministry(&self, id: Path<String>) -> payload::PlainText<String> {
        payload::PlainText("unimplemented".to_string())
    }

    #[oai(path = "/ministries/:id", method = "delete")]
    async fn delete_ministry(&self, id: Path<String>) -> payload::PlainText<String> {
        payload::PlainText("unimplemented".to_string())
    }

    #[oai(path = "/ministries/:id/users", method = "post")]
    async fn associate_users_with_ministry(&self, id: Path<String>) -> payload::PlainText<String> {
        payload::PlainText("unimplemented".to_string())
    }

    #[oai(path = "/ministries/:id/users", method = "get")]
    async fn get_ministry_users(&self, id: Path<String>) -> payload::PlainText<String> {
        payload::PlainText("unimplemented".to_string())
    }

    #[oai(path = "/ministries/:id/users", method = "delete")]
    async fn remove_users_from_ministry(&self, id: Path<String>) -> payload::PlainText<String> {
        payload::PlainText("unimplemented".to_string())
    }
}
