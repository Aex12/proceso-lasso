use super::{LassoConfig, ConfigManager};

pub struct MemoryConfigManager {
    config: LassoConfig,
}

impl MemoryConfigManager {
    pub fn new () -> MemoryConfigManager {
        MemoryConfigManager {
            config: LassoConfig::default(),
        }
    }
}

impl ConfigManager for MemoryConfigManager {
    fn get (&self) -> Result<LassoConfig, Box<dyn std::error::Error>> {
        Ok(self.config.clone())
    }

    fn put (&mut self, config: LassoConfig) -> Result<(), Box<dyn std::error::Error>> {
        self.config = config;
        Ok(())
    }

    fn load (&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.config = LassoConfig::default();
        Ok(())
    }
}