use actix_web::HttpServer;

use crate::controller::user_controller;

mod controller;
mod http;
mod model;
mod repository;
mod service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    let pool = database().await;
    http(pool).await
}

async fn database() -> sqlx::PgPool {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .unwrap()
}

async fn http(pool: sqlx::PgPool) -> Result<(), std::io::Error> {
    HttpServer::new(move || {
        actix_web::App::new().configure(|cfg| {
            user_controller::init(cfg, pool.clone());
        })
    })
    .bind("0.0.0.0:4001")
    .unwrap()
    .run()
    .await
}
