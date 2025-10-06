use crate::{
    model::User,
    repository::{LIMIT, offset},
};

#[derive(Debug, Clone)]
pub struct UserRepository {
    db: sqlx::PgPool,
}

impl UserRepository {
    pub fn new(db: sqlx::PgPool) -> Self {
        Self { db }
    }

    pub async fn find_all(&self, page: i32) -> Result<Vec<User>, sqlx::Error> {
        let sql = r#"SELECT *
            FROM users
            WHERE deleted_at IS NULL
            LIMIT $1 OFFSET $2"#;

        sqlx::query_as(sql)
            .bind(LIMIT)
            .bind(offset(page))
            .fetch_all(&self.db)
            .await
    }

    pub async fn find_by_id(&self, id: uuid::Uuid) -> Result<Option<User>, sqlx::Error> {
        let sql = r#"SELECT *
            FROM users
            WHERE id = $1 AND deleted_at IS NULL"#;

        sqlx::query_as(sql).bind(id).fetch_optional(&self.db).await
    }
}
