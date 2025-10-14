#[derive(Clone, serde::Serialize, serde::Deserialize, sqlx::Type)]
#[sqlx(type_name = "role_method", rename_all = "UPPERCASE")]
pub enum RoleMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    ANY,
}

#[derive(Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct Role {
    pub id: i32,
    pub name: String,
    pub endpoint: String,
    pub method: RoleMethod,
    pub created_at: i64,
    pub updated_at: i64,
    pub deleted_at: Option<i64>,
}
