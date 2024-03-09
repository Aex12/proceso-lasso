use crate::structs::ProcessList;

pub trait ProcessManager {
    fn getProcessList (&self) -> Result<ProcessList, Box<dyn std::error::Error>>;
    fn setProcessAffinity (&self, pid: i32, affinity: u64) -> Result<(), Box<dyn std::error::Error>>;
    fn setProcessPriority (&self, pid: i32, priority: i32) -> Result<(), Box<dyn std::error::Error>>;
}