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

    pub fn iter (&self) -> std::slice::Iter<Process> {
        self.processes.iter()
    }
}

impl Iterator for ProcessList {
    type Item = Process;

    fn next (&mut self) -> Option<Self::Item> {
        self.processes.pop()
    }
}

impl Default for ProcessList {
    fn default () -> ProcessList {
        ProcessList {
            processes: Vec::new(),
        }
    }
}

