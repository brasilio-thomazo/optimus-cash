use crate::{model::User, repository::UserRepository};

#[derive(Debug, Clone)]
pub struct UserService {
    repo: UserRepository,
}

impl UserService {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self {
            repo: UserRepository::new(pool),
        }
    }

    pub async fn find_all(&self, page: Option<i32>) -> Result<Vec<User>, super::Error> {
        let page = page.unwrap_or(1);
        self.repo
            .find_all(page)
            .await
            .map_err(super::Error::sqlx_error)
    }

    pub async fn find_by_id(&self, id: uuid::Uuid) -> Result<User, super::Error> {
        match self.repo.find_by_id(id).await {
            Ok(Some(user)) => Ok(user),
            Ok(None) => Err(super::Error::not_found()),
            Err(error) => Err(super::Error::sqlx_error(error)),
        }
    }
}
