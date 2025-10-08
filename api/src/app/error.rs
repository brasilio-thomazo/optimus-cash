use crate::http;

pub struct Error {
    pub message: String,
    pub status: u16,
    pub field: Option<String>,
}

impl Error {
    pub fn new(message: String, status: u16, field: Option<String>) -> Error {
        Error {
            message,
            status,
            field,
        }
    }

    pub fn argon2_error(err: argon2::Error) -> Error {
        Error::new(err.to_string(), 500, None)
    }

    pub fn hash_error(err: password_hash::Error) -> Error {
        Error::new(err.to_string(), 500, None)
    }

    pub fn var_error(err: std::env::VarError) -> Error {
        Error::new(err.to_string(), 500, None)
    }

    pub fn parse_error<E: std::fmt::Display>(err: E) -> Error {
        Error::new(err.to_string(), 500, None)
    }

    pub fn jwt_error(err: jsonwebtoken::errors::Error) -> Error {
        Error::new(err.to_string(), 500, None)
    }

    pub fn sqlx_error(err: sqlx::Error) -> Error {
        Error::new(err.to_string(), 500, None)
    }

    pub fn uuid_error(err: uuid::Error) -> Error {
        Error::new(err.to_string(), 500, None)
    }

    pub fn io_error(err: std::io::Error) -> Error {
        Error::new(err.to_string(), 500, None)
    }

    pub fn unauthorized() -> Error {
        Error::new("unauthorized".to_string(), 401, None)
    }

    pub fn not_found(message: &str) -> Error {
        Error::new(message.to_string(), 404, None)
    }

    pub fn bad_request(message: &str, field: &str) -> Error {
        Error::new(message.to_string(), 400, Some(field.to_string()))
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl actix_web::ResponseError for Error {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::from_u16(self.status).unwrap()
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        let code = actix_web::http::StatusCode::from_u16(self.status).unwrap();
        actix_web::HttpResponse::build(code).json(http::response::ErrorResponse::new(
            self.status,
            self.message.clone(),
            self.field.clone(),
        ))
    }
}
