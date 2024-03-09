use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
pub struct Process {
    pub name: String,
    pub path: Option<PathBuf>,
    pub pid: i32,
    pub priority: i32,
}

#[derive(Debug)]
pub struct ProcessList {
    processes: Vec<Process>,
}

impl ProcessList {
    pub fn new (processes: Vec<Process>) -> ProcessList {
        ProcessList {
            processes,
        }
    }

    pub fn processes (&self) -> &Vec<Process> {
        &self.processes
    }
}