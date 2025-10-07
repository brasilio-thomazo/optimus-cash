use crate::{error, http::UserRequest, model::User, repository::UserRepository};

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

    pub async fn find_all(&self, page: Option<i32>) -> Result<Vec<User>, error::Error> {
        let page = page.unwrap_or(1);
        self.repo
            .find_all(page)
            .await
            .map_err(error::Error::sqlx_error)
    }

    pub async fn find_by_id(&self, id: uuid::Uuid) -> Result<User, error::Error> {
        match self.repo.find_by_id(id).await {
            Ok(Some(user)) => Ok(user),
            Ok(None) => Err(error::Error::not_found("user not found")),
            Err(error) => Err(error::Error::sqlx_error(error)),
        }
    }

    pub async fn create(&self, request: UserRequest) -> Result<User, error::Error> {
        request.validate()?;
        let data = User::new_from(request)?;
        self.repo
            .create(&data)
            .await
            .map_err(error::Error::sqlx_error)
    }

    pub async fn update(&self, id: uuid::Uuid, request: UserRequest) -> Result<User, error::Error> {
        request.validate()?;
        let mut data = self.find_by_id(id).await?;
        data.update_from(request)?;
        self.repo
            .update(&data)
            .await
            .map_err(error::Error::sqlx_error)
    }

    pub async fn soft_delete(&self, id: uuid::Uuid) -> Result<User, error::Error> {
        self.repo
            .soft_delete(id)
            .await
            .map_err(error::Error::sqlx_error)
    }

    pub async fn hard_delete(&self, id: uuid::Uuid) -> Result<(), error::Error> {
        self.repo
            .hard_delete(id)
            .await
            .map_err(error::Error::sqlx_error)
    }

    pub async fn undelete(&self, id: uuid::Uuid) -> Result<User, error::Error> {
        self.repo
            .undelete(id)
            .await
            .map_err(error::Error::sqlx_error)
    }
}
