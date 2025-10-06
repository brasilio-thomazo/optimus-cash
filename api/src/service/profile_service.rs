use crate::{model::User, repository::UserRepository};

pub struct ProfileService {
    repo: UserRepository,
}

impl ProfileService {
    pub fn new(repo: UserRepository) -> Self {
        Self { repo }
    }

    pub async fn get_profile(&self, id: String) -> Result<User, super::Error> {
        let id = uuid::Uuid::parse_str(&id).map_err(super::Error::uuid_error)?;
        match self.repo.find_by_id(id).await {
            Ok(Some(user)) => Ok(user),
            Ok(None) => Err(super::Error::not_found()),
            Err(error) => Err(super::Error::sqlx_error(error)),
        }
    }
}
