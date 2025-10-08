use crate::app;

pub fn init() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_max_level(tracing::Level::INFO)
        .with_level(true)
        .with_target(true)
        .init();
    dotenvy::dotenv().ok();
}

pub fn get_var<T: std::str::FromStr>(key: &str, default: Option<T>) -> Result<T, app::Error>
where
    T::Err: std::fmt::Display,
{
    match default {
        Some(d) => Ok(get_var_or(key, d)),
        None => std::env::var(key)
            .map_err(app::Error::var_error)
            .and_then(|value| value.parse().map_err(|err| app::Error::parse_error(err))),
    }
}

pub fn get_var_or<T: std::str::FromStr>(key: &str, default: T) -> T {
    std::env::var(key)
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(default)
}

pub fn open_routes() -> Vec<&'static str> {
    vec!["/auth", "/health"]
}
