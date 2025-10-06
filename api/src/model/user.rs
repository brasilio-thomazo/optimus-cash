use crate::{http::UserRequest, security::hash::hash, service};
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

impl User {
    pub fn new_from(req: UserRequest) -> Result<Self, service::Error> {
        let id = Uuid::new_v4();
        let hash = hash(&req.password).map_err(service::Error::bcrypt_error)?;
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

    pub fn update_from(&mut self, req: UserRequest) -> Result<(), service::Error> {
        let now = chrono::Utc::now();
        let timestamp = now.timestamp();
        let hash = hash(&req.password).map_err(service::Error::bcrypt_error)?;

        self.name = req.name;
        self.phone = req.phone;
        self.email = req.email;
        self.username = req.username;
        self.hash = hash;
        self.updated_at = timestamp;

        Ok(())
    }
}
