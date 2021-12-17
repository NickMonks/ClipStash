use crate::data::DbId;
use crate::domain::clip::field;
use crate::{ClipError, Short, Time};
use chrono::{NaiveDateTime, Utc};
use std::convert::TryFrom;

// sql::FromRow will convert a row into a clip struct
// model will contain all information of the database, which separates
// the domain layer
#[derive(Debug, sqlx::FromRow)]
pub struct Clip {
    pub clip_id: String,
    pub shortcode: String,
    pub content: String,
    pub title: Option<String>,
    pub posted: NaiveDateTime,
    pub expires: Option<NaiveDateTime>,
    pub password: Option<String>,
    pub hits: i64,
}

impl TryFrom<Clip> for crate::domain::Clip {
    type Error = ClipError;
    fn try_from(clip: Clip) -> Result<Self, Self::Error>{
        use crate::domain::clip::field;
        Ok(Self {
                clip_id: field::ClipId::new(DbId::try_from(clip.clip_id.as_str())?),

            }
        )
    }
}