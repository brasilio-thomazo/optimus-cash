use actix_web::HttpServer;

use crate::controller::{auth_controller, health_controller, profile_controller, user_controller};

mod controller;
mod http;
mod middleware;
mod model;
mod repository;
mod security;
mod service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_level(true)
        .with_target(true)
        .init();

    dotenvy::dotenv().ok();
    let pool = database().await;
    init(pool.clone()).await;
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

async fn init(pool: sqlx::PgPool) {
    match sqlx::migrate!().run(&pool.clone()).await {
        Ok(_) => {
            tracing::info!("database migrated");
        }
        Err(err) => {
            tracing::error!("failed to migrate database: {}", err);
        }
    }

    let now = chrono::Utc::now().timestamp();
    let user_repo = crate::repository::UserRepository::new(pool);
    if user_repo.find_all(1).await.unwrap().is_empty() {
        tracing::info!("creating admin user");
        let admin = crate::model::User {
            id: uuid::Uuid::new_v4(),
            name: "admin".to_string(),
            email: "postmaster@localhost".to_string(),
            phone: "".to_string(),
            username: "admin".to_string(),
            is_admin: true,
            is_verified: true,
            hash: security::hash::hash("admin").unwrap(),
            created_at: now,
            updated_at: now,
            deleted_at: None,
        };

        match user_repo.create(&admin).await {
            Ok(_) => {
                tracing::info!("admin user created");
            }
            Err(err) => {
                tracing::error!("failed to create admin user: {}", err);
            }
        }
    }
}

async fn http(pool: sqlx::PgPool) -> Result<(), std::io::Error> {
    HttpServer::new(move || {
        actix_web::App::new().configure(|cfg| {
            user_controller::init(cfg, pool.clone());
            auth_controller::init(cfg, pool.clone());
            profile_controller::init(cfg, pool.clone());
            health_controller::init(cfg);
        })
    })
    .bind("0.0.0.0:4000")
    .unwrap()
    .run()
    .await
}
