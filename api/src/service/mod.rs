mod user_service;
pub use user_service::UserService;

pub struct Error {
    pub message: String,
    pub status: u16,
}

impl Error {
    pub fn new(message: String, status: u16) -> Self {
        Self { message, status }
    }

    pub fn sqlx_error(error: sqlx::Error) -> Self {
        Self::new(error.to_string(), 500)
    }

    pub fn not_found() -> Self {
        Self::new("Not Found".to_string(), 404)
    }
}
