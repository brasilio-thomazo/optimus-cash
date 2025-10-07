use std::env;

use crate::error;
use argon2::{
    PasswordHash,
    password_hash::{self, PasswordHasher, PasswordVerifier},
};

pub fn hash(password: &str) -> Result<String, error::Error> {
    let p_cost: u32 = env::var("ARGON2_THREADS")
        .map_err(error::Error::var_error)?
        .parse()
        .map_err(error::Error::parse_error)?;
    let m_cost = env::var("ARGON2_MEMORY")
        .map_err(error::Error::var_error)?
        .parse()
        .map_err(error::Error::parse_error)?;
    let t_cost = env::var("ARGON2_TIME")
        .map_err(error::Error::var_error)?
        .parse()
        .map_err(error::Error::parse_error)?;
    let output_len = env::var("ARGON2_LEN")
        .map_err(error::Error::var_error)?
        .parse()
        .map_err(error::Error::parse_error)?;
    let salt = password_hash::SaltString::generate(&mut password_hash::rand_core::OsRng);
    let params = argon2::Params::new(m_cost, t_cost, p_cost, Some(output_len))
        .map_err(error::Error::argon2_error)?;
    let argon2id = argon2::Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params);
    argon2id
        .hash_password(password.as_bytes(), &salt)
        .map_err(error::Error::hash_error)
        .map(|hash| hash.to_string())
}

pub fn verify(password: &str, hash: &str) -> Result<bool, error::Error> {
    let parsed = PasswordHash::new(hash).map_err(error::Error::hash_error)?;
    argon2::Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .map_err(error::Error::hash_error)
        .map(|_| true)
}
