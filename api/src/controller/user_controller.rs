use actix_web::{
    HttpResponse, Responder,
    web::{Data, Path, ServiceConfig, get, scope},
};

use crate::service::UserService;

pub fn init(cfg: &mut ServiceConfig, pool: sqlx::PgPool) {
    let service = Data::new(UserService::new(pool));
    cfg.service(
        scope("/users")
            .route("", get().to(index))
            .route("/{id}", get().to(show)),
    )
    .app_data(service);
}

pub async fn index(service: Data<UserService>) -> impl Responder {
    service
        .find_all(None)
        .await
        .map(|data| HttpResponse::Ok().json(data))
        .map_err(super::Error::error)
}

pub async fn show(service: Data<UserService>, id: Path<uuid::Uuid>) -> impl Responder {
    let id = id.into_inner();
    service
        .find_by_id(id)
        .await
        .map(|data| HttpResponse::Ok().json(data))
        .map_err(super::Error::error)
}
