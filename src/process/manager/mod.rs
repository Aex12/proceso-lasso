mod windows;
pub use windows::WindowsProcessManager;

use crate::lasso::AffinityMask;

use super::ProcessList;

pub trait ProcessManager {
    fn getProcessList (&self) -> Result<ProcessList, Box<dyn std::error::Error>>;
    fn setProcessAffinity (&self, pid: i32, affinity: &AffinityMask) -> Result<(), Box<dyn std::error::Error>>;
    fn setProcessPriority (&self, pid: i32, priority: i32) -> Result<(), Box<dyn std::error::Error>>;
}