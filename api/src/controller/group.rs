use actix_web::{Responder, web};

use crate::{controller, http::request::GroupRequest, service::GroupService};

#[actix_web::get("")]
pub async fn index(srv: web::Data<GroupService>) -> impl Responder {
    srv.find_all().await.map(controller::ok)
}

#[actix_web::get("/{id}")]
pub async fn show(srv: web::Data<GroupService>, id: web::Path<uuid::Uuid>) -> impl Responder {
    let id = id.into_inner();
    srv.find_by_id(id).await.map(controller::ok)
}

#[actix_web::post("")]
pub async fn create(
    srv: web::Data<GroupService>,
    request: web::Json<GroupRequest>,
) -> impl Responder {
    srv.create(request.into_inner()).await.map(controller::ok)
}

#[actix_web::put("/{id}")]
pub async fn update(
    srv: web::Data<GroupService>,
    id: web::Path<uuid::Uuid>,
    request: web::Json<GroupRequest>,
) -> impl Responder {
    let id = id.into_inner();
    srv.update(id, request.into_inner())
        .await
        .map(controller::ok)
}
