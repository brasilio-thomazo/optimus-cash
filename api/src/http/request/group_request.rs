use crate::model::{Group, GroupRole};

#[derive(serde::Deserialize, Clone)]
pub struct GroupRequest {
    pub name: String,
    pub description: Option<String>,
    pub roles: Vec<GroupRole>,
}

impl GroupRequest {
    pub fn to_model(&self, id: Option<uuid::Uuid>) -> Group {
        let id = id.unwrap_or_else(uuid::Uuid::new_v4);
        let timestamp = chrono::Utc::now().timestamp();
        Group {
            id,
            name: self.name.clone(),
            description: self.description.clone(),
            roles: self.roles.clone(),
            created_at: timestamp,
            updated_at: timestamp,
            deleted_at: None,
        }
    }
}
