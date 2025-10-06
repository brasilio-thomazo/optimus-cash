#[derive(Debug, Clone, serde::Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: crate::model::User,
}

impl AuthResponse {
    pub fn new(token: String, user: crate::model::User) -> Self {
        Self { token, user }
    }
}
