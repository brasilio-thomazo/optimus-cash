use crate::app;

#[derive(Clone)]
pub struct Pool {
    pub write: sqlx::PgPool,
    pub read: sqlx::PgPool,
}

pub async fn db() -> Result<Pool, app::Error> {
    let write = write_pool().await?;
    let read = read_pool().await?;
    Ok(Pool { write, read })
}

pub async fn write_pool() -> Result<sqlx::PgPool, app::Error> {
    tracing::info!("connecting to write database");
    let url = std::env::var("DATABASE_WRITE_URL").map_err(app::Error::var_error)?;
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await
        .map_err(app::Error::sqlx_error)?;
    tracing::info!("connected to write database");
    Ok(pool)
}

pub async fn read_pool() -> Result<sqlx::PgPool, app::Error> {
    tracing::info!("connecting to read database");
    let url = std::env::var("DATABASE_READ_URL").map_err(app::Error::var_error)?;
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await
        .map_err(app::Error::sqlx_error)?;
    tracing::info!("connected to read database");
    Ok(pool)
}
