use crate::{app, http::request::UserRequest, model::Model, security};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub phone: String,
    pub email: String,
    pub username: String,
    #[serde(skip)]
    pub hash: String,
    pub is_admin: bool,
    pub is_verified: bool,
    pub created_at: i64,
    pub updated_at: i64,
    pub deleted_at: Option<i64>,
}

impl Model for User {
    const TABLE: &'static str = "users";
}

impl User {
    pub fn new_from(req: UserRequest) -> Result<Self, app::Error> {
        let id = Uuid::new_v4();
        let hash = security::hash_password(&req.password)?;
        let now = chrono::Utc::now();
        let timestamp = now.timestamp();

        Ok(Self {
            id: id,
            name: req.name,
            phone: req.phone,
            email: req.email,
            username: req.username,
            hash: hash,
            is_admin: false,
            is_verified: false,
            created_at: timestamp,
            updated_at: timestamp,
            deleted_at: None,
        })
    }

    pub fn update_from(&mut self, req: UserRequest) -> Result<(), app::Error> {
        let now = chrono::Utc::now();
        let timestamp = now.timestamp();
        let hash = security::hash_password(&req.password)?;

        self.name = req.name;
        self.phone = req.phone;
        self.email = req.email;
        self.username = req.username;
        self.hash = hash;
        self.updated_at = timestamp;

        Ok(())
    }
}
