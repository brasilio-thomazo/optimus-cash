use serde::Deserialize;

use crate::service;

#[derive(Debug, Clone, Deserialize)]
pub struct UserRequest {
    pub name: String,
    pub phone: String,
    pub email: String,
    pub username: String,
    pub password: String,
    pub password_confirmation: String,
}

impl UserRequest {
    pub fn validate(&self) -> Result<(), service::Error> {
        if self.name.is_empty() {
            return Err(service::Error::bad_request("name is required", "name"));
        }

        if self.email.is_empty() {
            return Err(service::Error::bad_request("email is required", "email"));
        }

        if self.username.is_empty() {
            return Err(service::Error::bad_request(
                "username is required",
                "username",
            ));
        }

        if self.password.is_empty() {
            return Err(service::Error::bad_request(
                "password is required",
                "password",
            ));
        }

        if self.password != self.password_confirmation {
            return Err(service::Error::bad_request(
                "password does not match",
                "password_confirmation",
            ));
        }

        Ok(())
    }
}
