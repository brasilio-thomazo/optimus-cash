use actix_web::{Responder, web};

use crate::{controller, http::request::UserRequest, service::UserService};

#[actix_web::get("")]
pub async fn index(srv: web::Data<UserService>) -> impl Responder {
    srv.find_all(None).await.map(controller::ok)
}

#[actix_web::get("/{id}")]
pub async fn show(srv: web::Data<UserService>, id: web::Path<uuid::Uuid>) -> impl Responder {
    let id = id.into_inner();
    srv.find_by_id(id).await.map(controller::ok)
}

#[actix_web::post("")]
pub async fn create(srv: web::Data<UserService>, body: web::Json<UserRequest>) -> impl Responder {
    srv.create(body.into_inner()).await.map(controller::created)
}

#[actix_web::put("/{id}")]
pub async fn update(
    srv: web::Data<UserService>,
    id: web::Path<uuid::Uuid>,
    body: web::Json<UserRequest>,
) -> impl Responder {
    let id = id.into_inner();
    srv.update(id, body.into_inner()).await.map(controller::ok)
}

#[actix_web::patch("/{id}")]
pub async fn patch(srv: web::Data<UserService>, id: web::Path<uuid::Uuid>) -> impl Responder {
    let id = id.into_inner();
    let user = srv.find_by_id(id).await?;
    if user.deleted_at.is_none() {
        srv.remove(id).await.map(controller::ok)
    } else {
        srv.restore(id).await.map(controller::ok)
    }
}

#[actix_web::delete("/{id}")]
pub async fn delete(srv: web::Data<UserService>, id: web::Path<uuid::Uuid>) -> impl Responder {
    let id = id.into_inner();
    srv.delete(id).await.map(controller::no_content)
}

pub async fn soft_delete(srv: web::Data<UserService>, id: web::Path<uuid::Uuid>) -> impl Responder {
    let id = id.into_inner();
    srv.remove(id).await.map(controller::ok)
}

pub async fn undelete(srv: web::Data<UserService>, id: web::Path<uuid::Uuid>) -> impl Responder {
    let id = id.into_inner();
    srv.restore(id).await.map(controller::ok)
}
