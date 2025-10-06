mod user_service;
pub use user_service::UserService;

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
}
