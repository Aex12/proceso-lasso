use std::path::PathBuf;
use serde::{Deserialize, Serialize};

use super::process::Process;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Matcher {
    Path(PathBuf),
    Name(String),
}

impl Matcher {
    pub fn matches (&self, process: &Process) -> bool {
        match self {
            Matcher::Path(p) => process.path.as_ref().map(|path| path.starts_with(p)).unwrap_or(false),
            Matcher::Name(n) => process.name == *n,
        }
    }
}

pub struct ProcessMatch {
    pub process: Process,
    pub matcher: Option<Matcher>,
}