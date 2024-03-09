use crate::structs::ProcessList;

pub trait ProcessManager {
    fn getProcessList (&self) -> Result<ProcessList, Box<dyn std::error::Error>>;
}