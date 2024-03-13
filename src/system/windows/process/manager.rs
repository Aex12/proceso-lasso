use crate::lasso::{ProcessManager, ProcessList, AffinityMask};

use super::getProcessList;
use super::WindowsProcess;
pub struct WindowsProcessManager {}

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

    fn set_process_affinity (&self, pid: i32, affinity: AffinityMask) -> Result<(AffinityMask, AffinityMask), Box<dyn std::error::Error>> {
        let process = WindowsProcess::open(pid)?;
        let res = process.set_affinity(affinity)?;
        Ok(res)
    }

    fn get_process_affinity (&self, pid: i32) -> Result<AffinityMask, Box<dyn std::error::Error>> {
        let process = WindowsProcess::open(pid)?;
        Ok(process.get_affinity()?.process())
    }

    fn set_process_priority (&self, _pid: i32, _priority: i32) -> Result<(), Box<dyn std::error::Error>> {
        unimplemented!("set_process_priority is not implemented for WindowsProcessManager yet")
    }
}

impl PartialEq for WindowsProcessManager {
    fn eq (&self, _other: &Self) -> bool {
        true
    }
}