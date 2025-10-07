use actix_web::{
    HttpResponse, Responder,
    web::{Data, Json, Path, ServiceConfig, delete, get, post, put, scope},
};

use crate::{http::UserRequest, service::UserService};

pub fn init(cfg: &mut ServiceConfig, pool: sqlx::PgPool) {
    let service = Data::new(UserService::new(pool));
    cfg.service(
        scope("/users")
            .route("", get().to(index))
            .route("/{id}", get().to(show))
            .route("", post().to(create))
            .route("/{id}", put().to(update))
            .route("/{id}", delete().to(hard_delete))
            .route("/soft-delete/{id}", delete().to(soft_delete))
            .route("/undelete/{id}", put().to(undelete)),
    )
    .app_data(service);
}

pub async fn index(service: Data<UserService>) -> impl Responder {
    service
        .find_all(None)
        .await
        .map(|data| HttpResponse::Ok().json(data))
}

pub async fn show(service: Data<UserService>, id: Path<uuid::Uuid>) -> impl Responder {
    let id = id.into_inner();
    service
        .find_by_id(id)
        .await
        .map(|data| HttpResponse::Ok().json(data))
}

pub async fn create(service: Data<UserService>, request: Json<UserRequest>) -> impl Responder {
    service
        .create(request.into_inner())
        .await
        .map(|data| HttpResponse::Created().json(data))
}

pub async fn update(
    service: Data<UserService>,
    id: Path<uuid::Uuid>,
    request: Json<UserRequest>,
) -> impl Responder {
    let id = id.into_inner();
    service
        .update(id, request.into_inner())
        .await
        .map(|data| HttpResponse::Ok().json(data))
}

pub async fn soft_delete(service: Data<UserService>, id: Path<uuid::Uuid>) -> impl Responder {
    let id = id.into_inner();
    service
        .soft_delete(id)
        .await
        .map(|data| HttpResponse::Ok().json(data))
}

pub async fn hard_delete(service: Data<UserService>, id: Path<uuid::Uuid>) -> impl Responder {
    let id = id.into_inner();
    service
        .hard_delete(id)
        .await
        .map(|_| HttpResponse::NoContent().finish())
}

pub async fn undelete(service: Data<UserService>, id: Path<uuid::Uuid>) -> impl Responder {
    let id = id.into_inner();
    service
        .undelete(id)
        .await
        .map(|data| HttpResponse::Ok().json(data))
}
