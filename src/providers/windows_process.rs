use std::thread;

use crate::{
    structs::{Process, ProcessList},
    traits::ProcessManager,
};
use serde::Deserialize;
use wmi::{COMLibrary, WMIConnection};

#[allow(non_camel_case_types)]
#[derive(Deserialize, Debug)]
struct Win32_Process {
    Name: Option<String>,
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
}

impl WindowsProcessManager {
    pub fn new() -> anyhow::Result<WindowsProcessManager> {
        Ok(WindowsProcessManager {})
    }
}

impl ProcessManager for WindowsProcessManager {
    fn getProcessList (&self) -> Result<ProcessList, Box<dyn std::error::Error>> {
        let processes = thread::spawn(|| {
            let com_con = COMLibrary::new().unwrap();
            let wmi_con = WMIConnection::new(com_con).unwrap();
            let processes: Vec<Win32_Process> = wmi_con.query().unwrap();
            processes
        }).join().unwrap();
        let proclistvec: Vec<Process> = processes.into_iter().map(|p| p.into()).collect();
        let proclist = ProcessList::new(proclistvec);
        Ok(proclist)
    }
}