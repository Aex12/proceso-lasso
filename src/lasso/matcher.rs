use std::path::PathBuf;
use serde::{Deserialize, Serialize};

use super::process::Process;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Matcher {
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

pub struct ProcessMatch {
    pub process: Process,
    pub matcher: Option<Matcher>,
}