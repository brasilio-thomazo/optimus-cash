use actix_web::HttpResponse;
use actix_web::body::BoxBody;
use actix_web::{ResponseError, http::StatusCode};
use std::fmt;

use crate::http::ErrorResponse;
use crate::service;

pub mod user_controller;

#[derive(Debug)]
pub struct Error {
    message: String,
    status: u16,
}

impl Error {
    fn new(message: String, status: u16) -> Self {
        Self { message, status }
    }

    fn error(error: service::Error) -> Self {
        Self::new(error.message, error.status)
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        StatusCode::from_u16(self.status).unwrap()
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        let code = StatusCode::from_u16(self.status).unwrap();
        HttpResponse::build(code).json(ErrorResponse::new(self.status, self.message.clone()))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
