// exposes to other mods the service layer as "ask", because this is what we actually do
pub mod ask;
pub mod action;

use crate::{ClipError, DataError};

#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("clip error: {0}")]
    Clip(#[from] ClipError),
    #[error("database error: {0}")]
    Data(DataError),
    #[error("not found")]
    NotFound,
    #[error("permissions not met: {0}")]
    PermissionError(String),
}

impl From<DataError> for ServiceError {
// manually implement the from trait for the Data error in Service Error
fn from(err: DataError) -> Self {
        match err {
            // rust allows to do nested matching patterns like this one
            DataError::Database(d) => match d {
                sqlx::Error::RowNotFound => Self::NotFound,
                other => Self::Data(DataError::Database(other)),
            }
        }
    }
}

impl From<sqlx::Error> for ServiceError {
    fn from(err: sqlx::Error) -> Self {
        match err {
           sqlx::Error::RowNotFound => Self::NotFound,
           other => Self::Data(DataError::Database(other)), 
        }
    }
}