use crate::{
    app, db, http::request::AuthRequest, http::response::AuthResponse, repository::UserRepository,
    security,
};

#[derive(Clone)]
pub struct AuthService {
    repo: UserRepository,
}

impl AuthService {
    pub fn new(pool: &db::Pool) -> Self {
        Self {
            repo: UserRepository::new(pool),
        }
    }

    pub async fn auth(&self, req: AuthRequest) -> Result<AuthResponse, app::Error> {
        req.validate()?;
        match self.repo.find_by_username(&req.username).await {
            Ok(Some(data)) => match security::verify_password(&req.password, &data.hash) {
                Ok(Some(hash)) => {
                    let token = security::generate_jwt_token(data.clone())?;
                    self.repo
                        .update_hash(&data.id, &hash)
                        .await
                        .map_err(app::Error::sqlx_error)?;
                    Ok(AuthResponse::new(token, data))
                }
                Ok(None) => {
                    let token = security::generate_jwt_token(data.clone())?;
                    Ok(AuthResponse::new(token, data))
                }
                Err(err) => Err(err),
            },
            Ok(None) => Err(app::Error::unauthorized()),
            Err(error) => Err(app::Error::sqlx_error(error)),
        }
    }
}
