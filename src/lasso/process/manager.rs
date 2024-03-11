use super::AffinityMask;

use crate::lasso::Config;
use super::ProcessList;

pub trait ProcessManager {
    fn getProcessList (&self) -> Result<ProcessList, Box<dyn std::error::Error>>;
    fn setProcessAffinity (&self, pid: i32, affinity: &AffinityMask) -> Result<(), Box<dyn std::error::Error>>;
    fn setProcessPriority (&self, pid: i32, priority: i32) -> Result<(), Box<dyn std::error::Error>>;
    fn apply (&self, config: &Config) -> Result<(), Box<dyn std::error::Error>> {
        let process_list = self.getProcessList().unwrap();
        process_list.into_iter().for_each(|process| {
            let (preset_name, preset) = config.find_preset(&process);
            if let Some(affinity) = &preset.affinity {
                match self.setProcessAffinity(process.pid, affinity) {
                    Ok(_) => println!("{}: {} {}", process.name, preset_name, affinity),
                    Err(e) => println!("Error setting affinity for process {}: {}", process.name, e),
                }
            }
        });
        Ok(())
    }
}