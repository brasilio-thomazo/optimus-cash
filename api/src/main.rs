use api::{cli, config, db};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    config::init();
    let pool = match db::db().await {
        Ok(pool) => pool,
        Err(e) => {
            tracing::error!("failed to connect to database: {}", e);
            std::process::exit(1);
        }
    };

    match cli::run(&pool).await {
        Ok(_) => Ok(()),
        Err(e) => {
            tracing::error!("error: {}", e);
            std::process::exit(1);
        }
    }
}
