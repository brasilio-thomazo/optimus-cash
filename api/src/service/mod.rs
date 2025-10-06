mod auth_service;
mod profile_service;
mod user_service;

pub use auth_service::AuthService;
pub use profile_service::ProfileService;
pub use user_service::UserService;

#[derive(Debug, Clone)]
pub struct Error {
    pub message: String,
    pub status: u16,
    pub field: Option<String>,
}

impl Error {
    pub fn new(message: String, status: u16) -> Self {
        Self {
            message,
            status,
            field: None,
        }
    }

    pub fn sqlx_error(error: sqlx::Error) -> Self {
        Self::new(error.to_string(), 500)
    }

    pub fn bcrypt_error(error: bcrypt::BcryptError) -> Self {
        Self::new(error.to_string(), 500)
    }

    pub fn io_error(error: std::io::Error) -> Self {
        Self::new(error.to_string(), 500)
    }

    pub fn jwt_error(error: jsonwebtoken::errors::Error) -> Self {
        Self::new(error.to_string(), 500)
    }

    pub fn uuid_error(error: uuid::Error) -> Self {
        Self::new(error.to_string(), 500)
    }

    pub fn not_found() -> Self {
        Self::new("Not Found".to_string(), 404)
    }

    pub fn bad_request(message: &str, field: &str) -> Self {
        Self {
            message: message.to_string(),
            field: Some(field.to_string()),
            status: 400,
        }
    }

    pub fn internal_server_error(message: &str) -> Self {
        Self::new(message.to_string(), 500)
    }

    pub fn unauthorized() -> Self {
        Self::new("Unauthorized".to_string(), 401)
    }
}
