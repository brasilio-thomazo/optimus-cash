use actix_web::{HttpResponse, Responder, web};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/health").route(web::get().to(health)));
}

async fn health() -> impl Responder {
    HttpResponse::Ok().finish()
}
