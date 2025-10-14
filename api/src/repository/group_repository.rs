use sqlx::types::Json;

use crate::{
    db,
    model::{Group, Model},
    repository::Repository,
};

#[derive(Clone)]
pub struct GroupRepository {
    pool: db::Pool,
}

#[async_trait::async_trait]
impl Repository<Group, uuid::Uuid> for GroupRepository {
    fn read(&self) -> &sqlx::PgPool {
        &self.pool.read
    }

    fn write(&self) -> &sqlx::PgPool {
        &self.pool.write
    }

    async fn create(&self, group: &Group) -> Result<(), sqlx::Error> {
        let sql = format!(
            "INSERT INTO {} (id, name, description, roles) VALUES ($1, $2, $3, $4)",
            Group::TABLE
        );

        sqlx::query(&sql)
            .bind(group.id)
            .bind(&group.name)
            .bind(&group.description)
            .bind(Json(&group.roles))
            .execute(self.write())
            .await?;
        Ok(())
    }

    async fn update(&self, group: &Group) -> Result<(), sqlx::Error> {
        let sql = format!(
            "UPDATE {} SET name = $1, description = $2, roles = $3 WHERE id = $4",
            Group::TABLE
        );
        sqlx::query(&sql)
            .bind(&group.name)
            .bind(&group.description)
            .bind(Json(&group.roles))
            .bind(group.id)
            .execute(self.write())
            .await?;
        Ok(())
    }
}

impl GroupRepository {
    pub fn new(pool: &db::Pool) -> Self {
        Self { pool: pool.clone() }
    }
}
