use crate::security;

pub async fn run_seed(db: &sqlx::PgPool) -> Result<(), sqlx::Error> {
    seed_roles(db).await?;
    seed_groups(db).await?;
    seed_users(db).await?;
    Ok(())
}

async fn seed_users(db: &sqlx::PgPool) -> Result<(), sqlx::Error> {
    let query = r#"INSERT INTO users 
        (id, name, email, username, hash, is_admin, is_verified, permissions, created_at, updated_at)
        VALUES
        ($1, $2, $3, $4, $5, $6, $7, get_permissions($8), $9, $10) ON CONFLICT DO NOTHING"#;
    let now = chrono::Utc::now().timestamp();
    let hash = security::hash_password("admin").unwrap();
    let id = uuid::Uuid::new_v4();
    let permissions = vec![1];
    tracing::info!("creating admin user");

    sqlx::query(query)
        .bind(id)
        .bind("Administrator")
        .bind("postmaster@localhost")
        .bind("admin")
        .bind(&hash)
        .bind(true)
        .bind(true)
        .bind(permissions)
        .bind(now)
        .bind(now)
        .execute(db)
        .await?;
    Ok(())
}

async fn seed_roles(db: &sqlx::PgPool) -> Result<(), sqlx::Error> {
    let methods = vec!["ANY", "GET", "POST", "PUT", "PATCH", "DELETE"];
    let paths = vec!["/", "/groups", "/users", "/branches", "/branch-accounts"];
    let query = "INSERT INTO roles (id, method, path) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING";
    let mut id = 1;

    for method in methods {
        for path in &paths {
            tracing::info!("creating role {} {}", method, path);
            sqlx::query(query)
                .bind(id)
                .bind(method)
                .bind(path)
                .execute(db)
                .await?;
            id += 1;
        }
    }
    Ok(())
}

async fn seed_groups(db: &sqlx::PgPool) -> Result<(), sqlx::Error> {
    let query = "INSERT INTO groups (id, name, roles) VALUES ($1, $2, get_roles(ARRAY[$3])) ON CONFLICT DO NOTHING";
    let names = vec!["admin", "user"];
    let roles = vec![1, 6];
    let mut id = 1;

    for (name, role) in names.iter().zip(roles.iter()) {
        tracing::info!("creating group {}", name);
        sqlx::query(query)
            .bind(id)
            .bind(name)
            .bind(role)
            .execute(db)
            .await?;
        id += 1;
    }
    Ok(())
}
