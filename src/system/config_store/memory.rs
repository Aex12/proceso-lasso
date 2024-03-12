use super::{Config, ConfigStore, ConfigStoreError};

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
    fn get (&self) -> Result<Config, ConfigStoreError> {
        Ok(self.config.clone())
    }

    fn put (&mut self, config: Config) -> Result<(), ConfigStoreError> {
        self.config = config;
        Ok(())
    }
}