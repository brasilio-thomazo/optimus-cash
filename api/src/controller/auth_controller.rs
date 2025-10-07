use actix_web::{HttpResponse, Responder, web};

use crate::{http::AuthRequest, service::AuthService};

pub fn init(cfg: &mut web::ServiceConfig, pool: sqlx::PgPool) {
    let service = web::Data::new(AuthService::new(pool));
    cfg.service(web::scope("/auth").route("", web::post().to(auth)))
        .app_data(service);
}

pub async fn auth(service: web::Data<AuthService>, req: web::Json<AuthRequest>) -> impl Responder {
    service
        .auth(req.into_inner())
        .await
        .map(|data| HttpResponse::Ok().json(data))
}
