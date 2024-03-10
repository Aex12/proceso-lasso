use crate::{structs::PLConfig, traits::ConfigManager};

pub struct MemoryConfigManager {
    config: PLConfig,
}

impl MemoryConfigManager {
    pub fn new () -> MemoryConfigManager {
        MemoryConfigManager {
            config: PLConfig::default(),
        }
    }
}

impl ConfigManager for MemoryConfigManager {
    fn get (&self) -> Result<PLConfig, Box<dyn std::error::Error>> {
        Ok(self.config.clone())
    }

    fn put (&mut self, config: PLConfig) -> Result<(), Box<dyn std::error::Error>> {
        self.config = config;
        Ok(())
    }

    fn load (&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.config = PLConfig::default();
        Ok(())
    }
}