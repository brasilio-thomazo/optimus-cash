pub async fn run_seed(db: &sqlx::PgPool) -> Result<(), sqlx::Error> {
    seed_roles(db).await?;
    seed_groups(db).await?;
    seed_users(db).await?;
    Ok(())
}

async fn seed_users(db: &sqlx::PgPool) -> Result<(), sqlx::Error> {
    sqlx::query_file!("seed/users.sql").execute(db).await?;
    Ok(())
}

async fn seed_roles(db: &sqlx::PgPool) -> Result<(), sqlx::Error> {
    sqlx::query_file!("seed/roles.sql").execute(db).await?;
    Ok(())
}

async fn seed_groups(db: &sqlx::PgPool) -> Result<(), sqlx::Error> {
    sqlx::query_file!("seed/groups.sql").execute(db).await?;
    Ok(())
}
