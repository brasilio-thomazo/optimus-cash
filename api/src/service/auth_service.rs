use crate::{
    http::{AuthRequest, AuthResponse},
    repository::UserRepository,
    security,
};

#[derive(Debug, Clone)]
pub struct AuthService {
    repo: UserRepository,
}

impl AuthService {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self {
            repo: UserRepository::new(pool),
        }
    }

    pub async fn auth(&self, req: AuthRequest) -> Result<AuthResponse, super::Error> {
        req.validate()?;
        match self.repo.find_by_username(&req.username).await {
            Ok(Some(data)) => match security::hash::verify(&req.password, &data.hash) {
                Ok(true) => {
                    let token = security::jwt::generate(data.clone())?;
                    Ok(AuthResponse::new(token, data))
                }
                Ok(false) => Err(super::Error::unauthorized()),
                Err(error) => Err(super::Error::bcrypt_error(error)),
            },
            Ok(None) => Err(super::Error::unauthorized()),
            Err(error) => Err(super::Error::sqlx_error(error)),
        }
    }
}
