use crate::{db, http, middleware};

pub async fn run(pool: db::Pool) -> Result<(), std::io::Error> {
    tracing::info!("starting http server on 0.0.0.0:4000");

    let server = actix_web::HttpServer::new(move || {
        let cors = actix_cors::Cors::default()
            .allowed_origin("http://localhost:5173")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allowed_headers(vec!["Content-Type", "Authorization"])
            .max_age(3600);

        let data_pool = actix_web::web::Data::new(pool.clone());

        actix_web::App::new()
            .app_data(data_pool)
            .configure(|cfg| http::routes::init(cfg, pool.clone()))
            .wrap(cors)
            .wrap(middleware::JwtMiddleware)
    });
    server.bind(("0.0.0.0", 4000))?.run().await
}
