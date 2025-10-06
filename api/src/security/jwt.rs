use crate::{model::User, service};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Claims {
    pub aud: String,
    pub iss: String,
    pub sub: String,
    pub exp: i64,
}

fn read_private_key() -> Result<Vec<u8>, service::Error> {
    let path = std::env::var("PRIVATE_KEY_PATH")
        .map_err(|_| service::Error::internal_server_error("PRIVATE_KEY_PATH is not set"))?;
    std::fs::read(path).map_err(service::Error::io_error)
}

fn read_public_key() -> Result<Vec<u8>, service::Error> {
    let path = std::env::var("PUBLIC_KEY_PATH")
        .map_err(|_| service::Error::internal_server_error("PUBLIC_KEY_PATH is not set"))?;
    std::fs::read(path).map_err(service::Error::io_error)
}

pub fn generate(data: User) -> Result<String, service::Error> {
    let now = chrono::Utc::now().timestamp();
    let exp = now + 60 * 60 * 24; // 1 day
    let claims = Claims {
        aud: "optimus-cash".to_string(),
        iss: "optimus-cash".to_string(),
        sub: data.id.to_string(),
        exp: exp,
    };
    let header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::EdDSA);
    let private_key = read_private_key()?;
    let key = jsonwebtoken::EncodingKey::from_ed_pem(&private_key.as_slice())
        .map_err(service::Error::jwt_error)?;
    jsonwebtoken::encode(&header, &claims, &key).map_err(service::Error::jwt_error)
}

pub fn verify(token: &str) -> Result<Claims, service::Error> {
    let public_key = read_public_key()?;
    let key = jsonwebtoken::DecodingKey::from_ed_pem(&public_key.as_slice())
        .map_err(service::Error::jwt_error)?;
    let mut validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::EdDSA);
    validation.set_audience(&["optimus-cash".to_string()]);

    jsonwebtoken::decode(token, &key, &validation)
        .map_err(service::Error::jwt_error)
        .map(|data| data.claims)
}
