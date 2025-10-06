use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct ErrorResponse {
    pub status: u16,
    pub message: String,
}

impl ErrorResponse {
    pub fn new(status: u16, message: String) -> Self {
        Self { status, message }
    }
}
