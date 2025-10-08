use actix_web::{Responder, web};

use crate::{controller, security::Claims, service::ProfileService};

#[actix_web::get("")]
pub async fn index(srv: web::Data<ProfileService>, claims: web::ReqData<Claims>) -> impl Responder {
    let id = claims.sub.clone();
    srv.get_profile(id).await.map(controller::ok)
}
