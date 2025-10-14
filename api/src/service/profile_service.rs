use crate::{
    app, db,
    model::User,
    repository::{Repository, UserRepository},
};

pub struct ProfileService {
    repo: UserRepository,
}

impl ProfileService {
    pub fn new(pool: &db::Pool) -> Self {
        Self {
            repo: UserRepository::new(pool),
        }
    }

    pub async fn get_profile(&self, id: String) -> Result<User, app::Error> {
        let id = uuid::Uuid::parse_str(&id).map_err(app::Error::uuid_error)?;
        match self.repo.find_by_id(id).await {
            Ok(Some(user)) => Ok(user),
            Ok(None) => Err(app::Error::not_found("user not found")),
            Err(error) => Err(app::Error::sqlx_error(error)),
        }
    }
}
