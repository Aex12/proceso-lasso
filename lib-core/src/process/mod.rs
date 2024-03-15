mod manager;
mod affinity_mask;
mod list;
pub use manager::ProcessManager;
pub use affinity_mask::AffinityMask;
pub use list::ProcessList;

use std::path::PathBuf;

use super::{Matchable, Matcher};

#[derive(Debug, Clone)]
pub struct Process {
    pub pid: i32,
    pub name: String,
    pub path: Option<PathBuf>,
    pub priority: i32,
}

impl Process {
    pub fn new (pid: i32, name: String, path: Option<PathBuf>, priority: i32) -> Self {
        Process {
            name,
            path,
            pid,
            priority,
        }
    }

    pub fn path_str (&self) -> String {
        self.path.clone().map(|p| p.to_str().unwrap().to_owned()).unwrap_or(String::from(""))
    }
}

impl Matchable for Process {
    #[inline]
    fn matches (&self, matcher: &Matcher) -> bool {
        match matcher {
            Matcher::Path(p) => self.path.as_ref().map(|path| path.starts_with(p)).unwrap_or(false),
            Matcher::Name(n) => self.name == *n,
            Matcher::NoPath => self.path.is_none(),
        }
    }

}

impl PartialEq for Process {
    fn eq (&self, other: &Self) -> bool {
        self.pid == other.pid
    }
}

impl Eq for Process {}