use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Matcher {
    NoPath,
    Path(PathBuf),
    Name(String),
}

pub trait Matchable {
    fn matches (&self, matcher: &Matcher) -> bool;
}

impl Matcher {
    #[inline]
    pub fn matches <T: Matchable> (&self, target: &T) -> bool {
        target.matches(self)
    }
}