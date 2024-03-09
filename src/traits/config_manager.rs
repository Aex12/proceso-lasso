use crate::structs::PLConfig;

pub trait ConfigManager {
    fn get (&self) -> Result<PLConfig, Box<dyn std::error::Error>>;
    fn put (&mut self, config: PLConfig) -> Result<(), Box<dyn std::error::Error>>;
    fn load (&mut self) -> Result<(), Box<dyn std::error::Error>>;
}