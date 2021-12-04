use super::ClipError;
use derive_more::From;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Deserialize, Serialize, From)]
struct ShortCode(String);

impl ShortCode {
    pub fn new() -> Self {
        use rand::prelude::*;
        let allowed_chars = [
            'a','b','c','d','1','2','3','4'
        ];

        let mut rng = thread_rng();
        let mut shortcode = String::with_capacity(10);

        for _ in 0..10 {
           shortcode.push(
               *allowed_chars
               .choose(&mut rng)
               .expect("sampling array should have values"),
           );
        }

        Self(shortcode);

    }
}