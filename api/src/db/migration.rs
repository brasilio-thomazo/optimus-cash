use crate::db;

pub async fn run_migration(pool: &db::Pool) -> Result<(), sqlx::Error> {
    sqlx::migrate!("./migrations")
        .run(&pool.write)
        .await
        .map_err(|e| e.into())
}

pub async fn run_rollback(pool: &db::Pool) -> Result<(), sqlx::Error> {
    sqlx::migrate!("./migrations")
        .run(&pool.write)
        .await
        .map_err(|e| e.into())
}
