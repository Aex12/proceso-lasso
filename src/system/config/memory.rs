use super::{Config, ConfigStore};

pub struct MemoryConfigStore {
    config: Config,
}

impl MemoryConfigStore {
    pub fn new () -> MemoryConfigStore {
        MemoryConfigStore {
            config: Config::default(),
        }
    }
}

impl ConfigStore for MemoryConfigStore {
    fn id (&self) -> &str {
        "memory"
    }
    fn get (&self) -> Result<Config, Box<dyn std::error::Error>> {
        Ok(self.config.clone())
    }

    fn put (&mut self, config: Config) -> Result<(), Box<dyn std::error::Error>> {
        self.config = config;
        Ok(())
    }
}