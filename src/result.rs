use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("database error")]
    SqliteError(sqlx_core::error::Error),
}

impl From<sqlx_core::error::Error> for Error {
    fn from(err: sqlx_core::error::Error) -> Self {
        Error::SqliteError(err)
    }
}
