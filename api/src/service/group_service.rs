use crate::{
    app, db,
    http::request::GroupRequest,
    model::Group,
    repository::{GroupRepository, Repository},
};

#[derive(Clone)]
pub struct GroupService {
    repo: GroupRepository,
}

impl GroupService {
    pub fn new(pool: &db::Pool) -> Self {
        Self {
            repo: GroupRepository::new(pool),
        }
    }

    pub async fn find_all(&self) -> Result<Vec<crate::model::Group>, app::Error> {
        self.repo.find_all().await.map_err(app::Error::sqlx_error)
    }

    pub async fn find_by_id(&self, id: uuid::Uuid) -> Result<Group, app::Error> {
        match self.repo.find_by_id(id).await {
            Ok(Some(group)) => Ok(group),
            Ok(None) => Err(app::Error::not_found("group not found")),
            Err(error) => Err(app::Error::sqlx_error(error)),
        }
    }

    pub async fn create(&self, request: GroupRequest) -> Result<Group, app::Error> {
        let model = request.to_model(None);
        self.repo
            .create(&model)
            .await
            .map_err(app::Error::sqlx_error)?;
        Ok(model)
    }

    pub async fn update(&self, id: uuid::Uuid, request: GroupRequest) -> Result<Group, app::Error> {
        let model = request.to_model(Some(id));
        self.repo
            .update(&model)
            .await
            .map_err(app::Error::sqlx_error)?;
        Ok(model)
    }

    pub async fn delete(&self, id: uuid::Uuid) -> Result<(), app::Error> {
        self.repo.delete(id).await.map_err(app::Error::sqlx_error)
    }

    pub async fn remove(&self, id: uuid::Uuid) -> Result<(), app::Error> {
        self.repo.remove(id).await.map_err(app::Error::sqlx_error)
    }

    pub async fn restore(&self, id: uuid::Uuid) -> Result<(), app::Error> {
        self.repo.restore(id).await.map_err(app::Error::sqlx_error)
    }
}
