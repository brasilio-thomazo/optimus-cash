use crate::{error, model::User, repository::UserRepository};

pub struct ProfileService {
    repo: UserRepository,
}

impl ProfileService {
    pub fn new(repo: UserRepository) -> Self {
        Self { repo }
    }

    pub async fn get_profile(&self, id: String) -> Result<User, error::Error> {
        let id = uuid::Uuid::parse_str(&id).map_err(error::Error::uuid_error)?;
        match self.repo.find_by_id(id).await {
            Ok(Some(user)) => Ok(user),
            Ok(None) => Err(error::Error::not_found("user not found")),
            Err(error) => Err(error::Error::sqlx_error(error)),
        }
    }
}
