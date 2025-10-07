use actix_web::{HttpResponse, Responder, web};

use crate::{middleware, repository::UserRepository, security::Claims, service::ProfileService};

pub fn init(cfg: &mut web::ServiceConfig, pool: sqlx::PgPool) {
    let repo = UserRepository::new(pool);
    let data = web::Data::new(ProfileService::new(repo));
    cfg.service(
        web::resource("/profile")
            .route(web::get().to(index))
            .wrap(middleware::JwtMiddleware),
    )
    .app_data(data);
}

pub async fn index(srv: web::Data<ProfileService>, claims: web::ReqData<Claims>) -> impl Responder {
    let id = claims.sub.clone();
    srv.get_profile(id)
        .await
        .map(|data| HttpResponse::Ok().json(data))
}
