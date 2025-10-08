use std::env;

use crate::app;
use argon2::{
    PasswordHash,
    password_hash::{self, PasswordHasher, PasswordVerifier},
};

fn params() -> Result<argon2::Params, app::Error> {
    let p_cost: u32 = env::var("ARGON2_THREADS")
        .map_err(app::Error::var_error)?
        .parse()
        .map_err(app::Error::parse_error)?;
    let m_cost = env::var("ARGON2_MEMORY")
        .map_err(app::Error::var_error)?
        .parse()
        .map_err(app::Error::parse_error)?;
    let t_cost = env::var("ARGON2_TIME")
        .map_err(app::Error::var_error)?
        .parse()
        .map_err(app::Error::parse_error)?;
    let output_len = env::var("ARGON2_LEN")
        .map_err(app::Error::var_error)?
        .parse()
        .map_err(app::Error::parse_error)?;
    let params = argon2::Params::new(m_cost, t_cost, p_cost, Some(output_len))
        .map_err(app::Error::argon2_error)?;
    Ok(params)
}

pub fn hash_password(password: &str) -> Result<String, app::Error> {
    let salt = password_hash::SaltString::generate(&mut password_hash::rand_core::OsRng);
    let params = params()?;
    let argon2id = argon2::Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params);
    argon2id
        .hash_password(password.as_bytes(), &salt)
        .map_err(app::Error::hash_error)
        .map(|hash| hash.to_string())
}

pub fn verify_password(password: &str, hash: &str) -> Result<Option<String>, app::Error> {
    let parsed = PasswordHash::new(hash).map_err(app::Error::hash_error)?;
    argon2::Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .map_err(app::Error::hash_error)?;
    let params = params()?;
    let old_params = argon2::Params::try_from(&parsed).map_err(app::Error::hash_error)?;
    if params != old_params {
        let hash = hash_password(password)?;
        return Ok(Some(hash));
    }
    Ok(None)
}
