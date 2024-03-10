mod process_list;
mod affinity;

use crate::lasso::AffinityMask;
use super::super::ProcessList;
use super::ProcessManager;

use process_list::getProcessList;
use affinity::setProcessAffinity;

pub struct WindowsProcessManager {}

impl WindowsProcessManager {
    pub fn new() -> Result<WindowsProcessManager, Box<dyn std::error::Error>> {
        Ok(WindowsProcessManager {})
    }
}

impl ProcessManager for WindowsProcessManager {
    fn getProcessList (&self) -> Result<ProcessList, Box<dyn std::error::Error>> {
        getProcessList()
    }

    fn setProcessAffinity (&self, pid: i32, affinity: &AffinityMask) -> Result<(), Box<dyn std::error::Error>> {
        setProcessAffinity(pid, affinity)
    }

    fn setProcessPriority (&self, _pid: i32, _priority: i32) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}