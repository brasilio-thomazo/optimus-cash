pub mod app {
    mod error;
    pub use error::*;
}

pub mod cli;

pub mod config {
    mod config;
    pub use config::*;
}

pub mod controller {
    mod controller;
    pub use controller::*;
    pub mod auth;
    pub mod profile;
    pub mod user;
}

pub mod db {
    mod db;
    mod migration;
    mod seed;
    pub use db::*;
    pub use migration::*;
    pub use seed::*;
}

pub mod http {
    mod server;
    pub mod request {
        mod auth_request;
        mod user_request;
        pub use auth_request::*;
        pub use user_request::*;
    }
    pub mod response {
        mod auth_response;
        mod error_response;
        pub use auth_response::*;
        pub use error_response::*;
    }
    mod routes;
    pub use routes::*;
    pub use server::*;
}

pub mod middleware {
    mod jwt;
    pub use jwt::*;
}

pub mod model {
    mod user;
    pub use user::*;
}

pub mod repository {
    mod repository;
    mod user_repository;
    pub use user_repository::*;
}

pub mod service {
    mod auth_service;
    mod profile_service;
    mod user_service;
    pub use auth_service::*;
    pub use profile_service::*;
    pub use user_service::*;
}

pub mod security {
    mod hash;
    mod jwt;
    pub use hash::*;
    pub use jwt::*;
}
