use sqlx::FromRow;

use crate::{config, model::Model};

#[async_trait::async_trait]
pub trait Repository<T, ID>
where
    T: Model + for<'r> FromRow<'r, sqlx::postgres::PgRow> + Send + Sync + Unpin + 'static,
    ID: Send
        + Sync
        + Unpin
        + 'static
        + Clone
        + for<'q> sqlx::Encode<'q, sqlx::Postgres>
        + sqlx::Type<sqlx::Postgres>,
{
    fn read(&self) -> &sqlx::PgPool;
    fn write(&self) -> &sqlx::PgPool;
    async fn create(&self, model: &T) -> Result<(), sqlx::Error>;
    async fn update(&self, model: &T) -> Result<(), sqlx::Error>;

    fn offset(&self, page: i32) -> i32 {
        let limit = self.limit();
        if page <= 1 {
            return 0;
        }
        (page - 1) * limit
    }

    fn limit(&self) -> i32 {
        config::get_var_or("PAGINATION_LIMIT", 10)
    }

    async fn find_all(&self) -> Result<Vec<T>, sqlx::Error> {
        let sql = format!(
            "SELECT * FROM {} WHERE deleted_at IS NULL ORDER BY created_at DESC",
            T::TABLE
        );
        sqlx::query_as(&sql).fetch_all(self.read()).await
    }

    async fn find_all_paginated(&self, page: i32) -> Result<Vec<T>, sqlx::Error> {
        let sql = format!(
            "SELECT * FROM {} WHERE deleted_at IS NULL ORDER BY created_at DESC LIMIT $1 OFFSET $2",
            T::TABLE
        );
        sqlx::query_as(&sql)
            .bind(self.limit())
            .bind(self.offset(page))
            .fetch_all(self.read())
            .await
    }

    async fn find_by_id(&self, id: ID) -> Result<Option<T>, sqlx::Error> {
        let sql = format!(
            "SELECT * FROM {} WHERE id = $1 AND deleted_at IS NULL",
            T::TABLE
        );
        sqlx::query_as(&sql)
            .bind(id)
            .fetch_optional(self.read())
            .await
    }

    async fn remove(&self, id: ID) -> Result<(), sqlx::Error> {
        let sql = format!("UPDATE {} SET deleted_at = $1 WHERE id = $2", T::TABLE);
        sqlx::query(&sql)
            .bind(chrono::Utc::now().timestamp())
            .bind(id)
            .execute(self.write())
            .await?;
        Ok(())
    }

    async fn restore(&self, id: ID) -> Result<(), sqlx::Error> {
        let sql = format!("UPDATE {} SET deleted_at = NULL WHERE id = $1", T::TABLE);
        sqlx::query(&sql).bind(id).execute(self.write()).await?;
        Ok(())
    }

    async fn delete(&self, id: ID) -> Result<(), sqlx::Error> {
        let sql = format!("DELETE FROM {} WHERE id = $1", T::TABLE);
        sqlx::query(&sql).bind(id).execute(self.write()).await?;
        Ok(())
    }
}
