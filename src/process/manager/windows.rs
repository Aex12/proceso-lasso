use std::thread;

use super::super::{Process, ProcessList};
use super::ProcessManager;
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
            path: self.ExecutablePath.map(|p| p.into()),
            pid: self.ProcessId,
            priority: self.Priority,
        }
    }
}


pub struct WindowsProcessManager {
}

impl WindowsProcessManager {
    pub fn new() -> Result<WindowsProcessManager, Box<dyn std::error::Error>> {
        Ok(WindowsProcessManager {})
    }
}

impl ProcessManager for WindowsProcessManager {
    fn getProcessList (&self) -> Result<ProcessList, Box<dyn std::error::Error>> {
        // A new thread is required because the Tao library, which Dioxus uses, utilizes the WinAPI COM library.
        // The WinAPI COM library can only be initialized once per thread, otherwise it panics.
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

    fn setProcessAffinity (&self, _pid: i32, _affinity: u64) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn setProcessPriority (&self, _pid: i32, _priority: i32) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}