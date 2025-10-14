use crate::{
    db,
    model::{Model, User},
    repository::repository::Repository,
};

#[derive(Clone)]
pub struct UserRepository {
    pool: db::Pool,
}

#[async_trait::async_trait]
impl Repository<User, uuid::Uuid> for UserRepository {
    fn read(&self) -> &sqlx::PgPool {
        &self.pool.read
    }
    fn write(&self) -> &sqlx::PgPool {
        &self.pool.write
    }

    async fn create(&self, user: &User) -> Result<(), sqlx::Error> {
        let sql = format!(
            r#"INSERT INTO {}
            (id, name, phone, email, username, hash, is_admin, is_verified, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)"#,
            User::TABLE
        );

        sqlx::query(&sql)
            .bind(user.id)
            .bind(&user.name)
            .bind(&user.phone)
            .bind(&user.email)
            .bind(&user.username)
            .bind(&user.hash)
            .bind(user.is_admin)
            .bind(user.is_verified)
            .bind(&user.created_at)
            .bind(&user.updated_at)
            .execute(self.write())
            .await?;
        Ok(())
    }

    async fn update(&self, user: &User) -> Result<(), sqlx::Error> {
        let sql = format!(
            r#"UPDATE {}
            SET name = $1, phone = $2, email = $3, username = $4, hash = $5, updated_at = $6
            WHERE id = $7"#,
            User::TABLE
        );
        sqlx::query(&sql)
            .bind(&user.name)
            .bind(&user.phone)
            .bind(&user.username)
            .bind(&user.email)
            .bind(&user.hash)
            .bind(&user.updated_at)
            .bind(&user.id)
            .execute(&self.pool.write)
            .await?;
        Ok(())
    }
}

impl UserRepository {
    pub fn new(pool: &db::Pool) -> Self {
        Self { pool: pool.clone() }
    }
    pub async fn find_by_username(&self, username: &str) -> Result<Option<User>, sqlx::Error> {
        let sql = r#"SELECT *
            FROM users
            WHERE username = $1 OR email = $1 AND deleted_at IS NULL"#;

        sqlx::query_as(sql)
            .bind(username)
            .fetch_optional(&self.pool.read)
            .await
    }

    pub async fn update_hash(&self, id: &uuid::Uuid, hash: &str) -> Result<(), sqlx::Error> {
        let now = chrono::Utc::now().timestamp();
        let sql = r#"UPDATE users
            SET hash = $1, updated_at = $2
            WHERE id = $3"#;

        sqlx::query(sql)
            .bind(hash)
            .bind(now)
            .bind(id)
            .execute(&self.pool.write)
            .await?;
        Ok(())
    }
}
