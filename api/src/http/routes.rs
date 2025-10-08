use actix_web::{HttpResponse, web};

use crate::{
    controller, db,
    service::{AuthService, ProfileService, UserService},
};
pub fn init(cfg: &mut web::ServiceConfig, pool: db::Pool) {
    cfg.service(health())
        .service(users(&pool))
        .service(profile(&pool))
        .service(auth(&pool));
}

pub fn health() -> actix_web::Resource {
    web::resource("/health").route(web::get().to(|| async { HttpResponse::Ok().finish() }))
}

pub fn auth(pool: &db::Pool) -> actix_web::Scope {
    let srv = AuthService::new(pool);
    let data = web::Data::new(srv);
    web::scope("/auth")
        .service(controller::auth::auth)
        .app_data(data)
}

pub fn users(pool: &db::Pool) -> actix_web::Scope {
    let service = web::Data::new(UserService::new(pool));
    web::scope("/users")
        .service(controller::user::index)
        .service(controller::user::show)
        .service(controller::user::create)
        .service(controller::user::update)
        .service(controller::user::delete)
        .service(controller::user::patch)
        .app_data(service)
}

pub fn profile(pool: &db::Pool) -> actix_web::Scope {
    let service = web::Data::new(ProfileService::new(pool));
    web::scope("/profile")
        .service(controller::profile::index)
        .app_data(service)
}
