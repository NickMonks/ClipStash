use crate::domain::clip::ClipError;
use serde::{Deserialize, Serialize};
use rocket::form::{self, FromFormField, ValueField};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Content(String);

impl Content {
    // enforces some business logic to create the Content for each field
    // so we error handle correctly
    pub fn new(content: &str) -> Result<Self, ClipError> {
        if !content.trim().is_empty() {
            Ok(Self(content.to_owned()))
        } else {
            Err(ClipError::EmptyContent)
        }
    }
    
    // retrieve the content, consume the actual Content object
    pub fn into_inner(self) -> String {
        self.0
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[rocket::async_trait]
impl<'r> FromFormField<'r> for Content {
    fn from_value(field: ValueField<'r>) -> form::Result<'r,Self> {
        Ok(Self::new(field.value)
        .map_err(|e| form::Error::validation(format!("{}",e)))?
        )
    }
}