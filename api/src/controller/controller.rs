pub fn ok<T: serde::Serialize>(data: T) -> actix_web::HttpResponse {
    actix_web::HttpResponse::Ok().json(data)
}

pub fn created<T: serde::Serialize>(data: T) -> actix_web::HttpResponse {
    actix_web::HttpResponse::Created().json(data)
}

pub fn no_content(_: ()) -> actix_web::HttpResponse {
    actix_web::HttpResponse::NoContent().finish()
}
