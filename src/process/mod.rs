mod manager;
mod list;
pub use manager::*;
pub use list::ProcessList;

use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Process {
    pub name: String,
    pub path: Option<PathBuf>,
    pub pid: i32,
    pub priority: i32,
}

impl PartialEq for Process {
    fn eq (&self, other: &Self) -> bool {
        self.pid == other.pid
    }
}

impl Eq for Process {
    fn assert_receiver_is_total_eq(&self) {
        // This is a no-op, but it's required because `Eq` is a marker trait.
    }
}