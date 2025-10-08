pub async fn run_seed(db: &sqlx::PgPool) -> Result<(), sqlx::Error> {
    seed_users(db).await?;
    Ok(())
}

async fn seed_users(db: &sqlx::PgPool) -> Result<(), sqlx::Error> {
    sqlx::query_file!("seed/users.sql").execute(db).await?;
    Ok(())
}
