use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct ErrorResponse {
    pub status: u16,
    pub message: String,
    pub field: Option<String>,
}

impl ErrorResponse {
    pub fn new(status: u16, message: String, field: Option<String>) -> Self {
        Self {
            status,
            message,
            field,
        }
    }
}
