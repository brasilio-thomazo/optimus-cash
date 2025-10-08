use actix_web::{Responder, web};

use crate::{controller, http::request::AuthRequest, service::AuthService};

#[actix_web::post("")]
pub async fn auth(service: web::Data<AuthService>, req: web::Json<AuthRequest>) -> impl Responder {
    service.auth(req.into_inner()).await.map(controller::ok)
}
