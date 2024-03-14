use super::AffinityMask;

use crate::Config;
use super::ProcessList;

pub trait ProcessManager {
    fn get_id (&self) -> &str;
    fn get_process_list (&self) -> Result<ProcessList, Box<dyn std::error::Error>>;
    fn set_process_affinity (&self, pid: i32, affinity: AffinityMask) -> Result<(AffinityMask, AffinityMask), Box<dyn std::error::Error>>;
    fn get_process_affinity (&self, pid: i32) -> Result<AffinityMask, Box<dyn std::error::Error>>;
    fn set_process_priority (&self, pid: i32, priority: i32) -> Result<(), Box<dyn std::error::Error>>;

    fn apply (&self, config: &Config) -> Result<(), Box<dyn std::error::Error>> {
        let process_list = self.get_process_list().unwrap();
        process_list.into_iter().for_each(|process| {
            let (preset_name, preset) = config.find_preset(&process);
            if let Some(affinity) = &preset.affinity {
                match self.set_process_affinity(process.pid, *affinity) {
                    Ok(_) => println!("{}: {} {}", process.name, preset_name, affinity),
                    Err(e) => println!("Error setting affinity for process {}: {}", process.name, e),
                }
            }
        });
        Ok(())
    }
}

impl PartialEq for dyn ProcessManager {
    fn eq (&self, _other: &Self) -> bool {
        self.get_id() == _other.get_id()
    }
}