use crate::{db, model::User, repository::repository::Repository};

#[derive(Clone)]
pub struct UserRepository {
    db: db::Pool,
}

impl UserRepository {
    pub fn new(db: &db::Pool) -> Self {
        Self { db: db.clone() }
    }
    pub async fn find_all(&self, page: i32) -> Result<Vec<User>, sqlx::Error> {
        let sql = r#"SELECT *
            FROM users
            WHERE deleted_at IS NULL
            LIMIT $1 OFFSET $2"#;

        sqlx::query_as(sql)
            .bind(self.limit())
            .bind(self.offset(page))
            .fetch_all(&self.db.read)
            .await
    }

    pub async fn find_by_id(&self, id: uuid::Uuid) -> Result<Option<User>, sqlx::Error> {
        let sql = r#"SELECT *
            FROM users
            WHERE id = $1 AND deleted_at IS NULL"#;

        sqlx::query_as(sql)
            .bind(id)
            .fetch_optional(&self.db.read)
            .await
    }

    pub async fn find_by_username(&self, username: &str) -> Result<Option<User>, sqlx::Error> {
        let sql = r#"SELECT *
            FROM users
            WHERE username = $1 OR email = $1 AND deleted_at IS NULL"#;

        sqlx::query_as(sql)
            .bind(username)
            .fetch_optional(&self.db.read)
            .await
    }

    pub async fn create(&self, user: &User) -> Result<(), sqlx::Error> {
        let sql = r#"INSERT INTO users (id, name, phone, email, username, hash, is_admin, is_verified, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)"#;

        sqlx::query(sql)
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
            .execute(&self.db.write)
            .await?;
        Ok(())
    }

    pub async fn update(&self, user: &User) -> Result<(), sqlx::Error> {
        let sql = r#"UPDATE users
            SET name = $1, phone = $2, email = $3, username = $4, hash = $5, updated_at = $6
            WHERE id = $7"#;
        sqlx::query(sql)
            .bind(&user.name)
            .bind(&user.phone)
            .bind(&user.username)
            .bind(&user.email)
            .bind(&user.hash)
            .bind(&user.updated_at)
            .bind(&user.id)
            .execute(&self.db.write)
            .await?;
        Ok(())
    }

    pub async fn soft_delete(&self, id: uuid::Uuid) -> Result<(), sqlx::Error> {
        let sql = r#"UPDATE users
            SET deleted_at = $1
            WHERE id = $2
            RETURNING *"#;
        sqlx::query(sql)
            .bind(chrono::Utc::now().timestamp())
            .bind(id)
            .execute(&self.db.write)
            .await?;
        Ok(())
    }

    pub async fn hard_delete(&self, id: uuid::Uuid) -> Result<(), sqlx::Error> {
        let sql = r#"DELETE FROM users
            WHERE id = $1 AND deleted_at IS NOT NULL"#;
        sqlx::query(sql).bind(id).execute(&self.db.write).await?;
        Ok(())
    }

    pub async fn undelete(&self, id: uuid::Uuid) -> Result<(), sqlx::Error> {
        let sql = r#"UPDATE users
            SET deleted_at = NULL
            WHERE id = $1 AND deleted_at IS NOT NULL"#;
        sqlx::query(sql).bind(id).execute(&self.db.write).await?;
        Ok(())
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
            .execute(&self.db.write)
            .await?;
        Ok(())
    }
}

impl Repository for UserRepository {}
