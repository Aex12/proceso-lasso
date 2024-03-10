use super::Process;

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