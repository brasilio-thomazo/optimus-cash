use crate::{
    app, db,
    http::request::UserRequest,
    model::User,
    repository::{Repository, UserRepository},
};

#[derive(Clone)]
pub struct UserService {
    repo: UserRepository,
}

impl UserService {
    pub fn new(db: &db::Pool) -> Self {
        Self {
            repo: UserRepository::new(db),
        }
    }

    pub async fn find_all(&self, page: Option<i32>) -> Result<Vec<User>, app::Error> {
        let page = page.unwrap_or(1);
        self.repo
            .find_all_paginated(page)
            .await
            .map_err(app::Error::sqlx_error)
    }

    pub async fn find_by_id(&self, id: uuid::Uuid) -> Result<User, app::Error> {
        match self.repo.find_by_id(id).await {
            Ok(Some(user)) => Ok(user),
            Ok(None) => Err(app::Error::not_found("user not found")),
            Err(error) => Err(app::Error::sqlx_error(error)),
        }
    }

    pub async fn create(&self, request: UserRequest) -> Result<User, app::Error> {
        request.validate()?;
        let data = User::new_from(request)?;
        self.repo
            .create(&data)
            .await
            .map_err(app::Error::sqlx_error)?;
        Ok(data)
    }

    pub async fn update(&self, id: uuid::Uuid, request: UserRequest) -> Result<User, app::Error> {
        request.validate()?;
        let mut data = self.find_by_id(id).await?;
        data.update_from(request)?;
        self.repo
            .update(&data)
            .await
            .map_err(app::Error::sqlx_error)?;
        Ok(data)
    }

    pub async fn remove(&self, id: uuid::Uuid) -> Result<(), app::Error> {
        self.repo.remove(id).await.map_err(app::Error::sqlx_error)
    }

    pub async fn delete(&self, id: uuid::Uuid) -> Result<(), app::Error> {
        self.repo.delete(id).await.map_err(app::Error::sqlx_error)
    }

    pub async fn restore(&self, id: uuid::Uuid) -> Result<(), app::Error> {
        self.repo.restore(id).await.map_err(app::Error::sqlx_error)
    }
}
