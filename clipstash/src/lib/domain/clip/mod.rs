pub mod field;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ClipError {
    #[error("invalid password: {0}")]
    InvalidPassword(String),
    #[error("invalid title: {0}")]
    InvalidTitle(String),
    #[error("error: empty content")]
    EmptyContent,
    #[error("invalid date: {0}")]
    InvalidDate(String),
    #[error("date parse error: {0}")]
    DateParse(#[from] chrono::ParseError),
    #[error("Id error: {0}")]
    Id(#[from] uuid::Error),
    #[error("Hits try error")]
    Hits(#[from] std::num::TryFromIntError)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Clip {
    pub name: field::ClipId,
    pub shortcode: field::ShortCode,
    pub content: field::Content,
    pub title: field::Title,
    pub posted: field::Posted,
    pub expires: field::Expires,
    pub password: field::Password,
    pub hints: field::Hits,
}
