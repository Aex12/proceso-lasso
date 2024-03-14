use super::Process;

#[derive(Debug, Clone, PartialEq, Eq)]
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

impl FromIterator<Process> for ProcessList {
    fn from_iter<I: IntoIterator<Item=Process>> (iter: I) -> Self {
        ProcessList {
            processes: iter.into_iter().collect(),
        }
    }
}

impl<'a> FromIterator<&'a Process> for ProcessList {
    fn from_iter<I: IntoIterator<Item=&'a Process>> (iter: I) -> Self {
        ProcessList {
            processes: iter.into_iter().map(|p| p.clone()).collect(),
        }
    }
}

impl Default for ProcessList {
    fn default () -> ProcessList {
        ProcessList {
            processes: Vec::new(),
        }
    }
}

