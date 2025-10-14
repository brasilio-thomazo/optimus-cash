use sqlx::{Row, types::Json};
pub trait Model {
    const TABLE: &'static str;
    fn from_jsonb<'r, T>(row: &'r sqlx::postgres::PgRow, key: &str) -> sqlx::Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let data: Json<T> = row.try_get(key)?;
        Ok(data.0)
    }
}
