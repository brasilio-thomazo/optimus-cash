use actix_web::HttpServer;
use clap::Parser;

use crate::controller::{auth_controller, health_controller, profile_controller, user_controller};

mod controller;
mod http;
mod middleware;
mod model;
mod repository;
mod security;
mod service;

#[derive(Debug, clap::Parser)]
#[command(name = "api")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, clap::Subcommand)]
enum Commands {
    Server,
    Migrate,
    Seed,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_level(true)
        .with_target(true)
        .init();

    dotenvy::dotenv().ok();
    let cli = Cli::parse();
    let pool = database().await;

    match &cli.command {
        Commands::Server => http(pool.clone()).await,
        Commands::Migrate => migrate(pool.clone()).await,
        Commands::Seed => seed(pool.clone()).await,
    }
}

async fn migrate(pool: sqlx::PgPool) -> Result<(), std::io::Error> {
    match sqlx::migrate!().run(&pool.clone()).await {
        Ok(_) => {
            tracing::info!("database migrated");
        }
        Err(err) => {
            tracing::error!("failed to migrate database: {}", err);
        }
    }

    Ok(())
}

async fn seed(pool: sqlx::PgPool) -> Result<(), std::io::Error> {
    let user_repo = crate::repository::UserRepository::new(pool);
    let now = chrono::Utc::now().timestamp();

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

    Ok(())
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
    tracing::info!("starting http server on 0.0.0.0:4000");
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
