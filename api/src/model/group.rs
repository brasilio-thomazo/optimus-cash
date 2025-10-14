use crate::model::{RoleMethod, model::Model};
use sqlx::Row;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct GroupRole {
    pub endpoint: String,
    pub method: RoleMethod,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Group {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: Option<String>,
    pub roles: Vec<GroupRole>,
    pub created_at: i64,
    pub updated_at: i64,
    pub deleted_at: Option<i64>,
}

impl Model for Group {
    const TABLE: &'static str = "group";
}

impl<'r> sqlx::FromRow<'r, sqlx::postgres::PgRow> for Group {
    fn from_row(row: &'r sqlx::postgres::PgRow) -> sqlx::Result<Self> {
        Ok(Group {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            description: row.try_get("description")?,
            roles: Self::from_jsonb(row, "roles")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            deleted_at: row.try_get("deleted_at")?,
        })
    }
}
