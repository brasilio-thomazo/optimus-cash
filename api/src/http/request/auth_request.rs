use crate::app;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct AuthRequest {
    pub username: String,
    pub password: String,
}

impl AuthRequest {
    pub fn validate(&self) -> Result<(), app::Error> {
        if self.username.is_empty() {
            return Err(app::Error::bad_request("username is required", "username"));
        }

        if self.password.is_empty() {
            return Err(app::Error::bad_request("password is required", "password"));
        }

        Ok(())
    }
}
