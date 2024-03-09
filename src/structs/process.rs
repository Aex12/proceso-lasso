#[derive(Debug)]
pub struct Process {
    pub name: String,
    pub path: Option<String>,
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
}