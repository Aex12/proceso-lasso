mod manager;
mod list;
pub use manager::*;
pub use list::ProcessList;

use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
pub struct Process {
    pub name: String,
    pub path: Option<PathBuf>,
    pub pid: i32,
    pub priority: i32,
}