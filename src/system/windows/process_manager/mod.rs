mod process_list;
mod affinity;

use crate::lasso::{ProcessManager, ProcessList, AffinityMask};

use process_list::getProcessList;
use affinity::setProcessAffinity;

pub struct WindowsProcessManager {}

impl PartialEq for WindowsProcessManager {
    fn eq (&self, _other: &Self) -> bool {
        true
    }
}

impl WindowsProcessManager {
    pub fn new() -> Result<WindowsProcessManager, Box<dyn std::error::Error>> {
        Ok(WindowsProcessManager {})
    }
}

impl ProcessManager for WindowsProcessManager {
    fn get_id (&self) -> &str {
        "windows"
    }
    fn get_process_list (&self) -> Result<ProcessList, Box<dyn std::error::Error>> {
        getProcessList()
    }

    fn set_process_affinity (&self, pid: i32, affinity: &AffinityMask) -> Result<(), Box<dyn std::error::Error>> {
        setProcessAffinity(pid, affinity)
    }

    fn set_process_priority (&self, _pid: i32, _priority: i32) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}