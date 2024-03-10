use crate::lasso::{Config, ConfigManager};

pub struct MemoryConfigManager {
    config: Config,
}

impl MemoryConfigManager {
    pub fn new () -> MemoryConfigManager {
        MemoryConfigManager {
            config: Config::default(),
        }
    }
}

impl ConfigManager for MemoryConfigManager {
    fn get (&self) -> Result<Config, Box<dyn std::error::Error>> {
        Ok(self.config.clone())
    }

    fn put (&mut self, config: Config) -> Result<(), Box<dyn std::error::Error>> {
        self.config = config;
        Ok(())
    }

    fn load (&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.config = Config::default();
        Ok(())
    }
}