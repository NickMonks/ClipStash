use crate::domain::clip::ClipError;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd)]
pub struct Password(Option<String>); // the user could not grant a password

impl Password {
    // Into trait allow us to use either a string or Option<String>,
    // so the input can be a string itself or an option. This allows to use some
    // overload verison of the new method
    pub fn new<T : Into<Option<String>>>(password: T) -> Result<Self, ClipError> {
        let password : Option<String> = password.into();
        match password {
            Some(password) =>  {
                if !password.trim().is_empty() {
                    Ok(Self(Some(password)))
                } else {
                    Ok(Self(None))
                }
            }
            None => Ok(Self(None))
        }
    }

    pub fn into_inner(self) -> Option<String> {
        self.0
    }

    pub fn has_password(&self) -> bool {
        self.0.is_some()
    }
}

impl Default for Password {
    fn default() -> Self {
        Self(None)
    }
}

impl FromStr for Password {
    type Err = ClipError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s.to_string())
    }

}
