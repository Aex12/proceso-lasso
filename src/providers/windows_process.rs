use crate::{
    structs::{Process, ProcessList},
    traits::ProcessManager,
};
use serde::Deserialize;
use wmi::{COMLibrary, WMIConnection};

#[allow(non_camel_case_types)]
#[derive(Deserialize, Debug)]
struct Win32_Process {
    Name: String,
    ExecutablePath: Option<String>,
    ProcessId: i32,
    Priority: i32,
}
impl Into<Process> for Win32_Process {
    fn into(self) -> Process {
        Process {
            name: self.Name,
            path: self.ExecutablePath,
            pid: self.ProcessId,
            priority: self.Priority,
        }
    }
}


pub struct WindowsProcessManager {
    wmicon: WMIConnection,
}

impl WindowsProcessManager {
    pub fn new() -> anyhow::Result<WindowsProcessManager> {
        let wmicon = WMIConnection::new(COMLibrary::new()?)?;

        Ok(WindowsProcessManager {
            wmicon,
        })
    }
}

impl ProcessManager for WindowsProcessManager {
    fn getProcessList (&self) -> Result<ProcessList, Box<dyn std::error::Error>> {
        let procs: Vec<Win32_Process> = self.wmicon.query()?;
        let proclistvec: Vec<Process> = procs.into_iter().map(|p| p.into()).collect();
        let proclist = ProcessList::new(proclistvec);
        Ok(proclist)
    }
}