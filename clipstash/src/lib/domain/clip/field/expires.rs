use derive_more::Constructor;
use serde::{Deserialize, Serialize};

#[derive(Clone, Constructor, Debug, Deserialize, Serialize)] // constructor impls the new fn
pub struct Hits(u64);

impl Hits {
    pub fn into_inner(self) -> u64 {
        self.0
    }
}