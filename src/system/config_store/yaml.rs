use std::fs;

use super::{Config, ConfigStore, ConfigStoreError};

pub struct YamlConfigStore {
    path: String,
}

impl PartialEq for YamlConfigStore {
    fn eq (&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl YamlConfigStore {
    pub fn new (path: String) -> YamlConfigStore {
        YamlConfigStore { path }
    }
}

impl ConfigStore for YamlConfigStore {
    fn id (&self) -> &str {
        &self.path
    }
    fn get (&self) -> Result<Config, ConfigStoreError> {
        let yaml = fs::read_to_string(&self.path).map_err(|e| {
            match e.kind() {
                std::io::ErrorKind::NotFound => ConfigStoreError::NotInitialized,
                _ => ConfigStoreError::IoError(e),
            }
        })?;
        let config: Config = serde_yaml::from_str(&yaml).map_err(|e| ConfigStoreError::UnknownError(e.to_string()))?;
        config.validate().map_err(|_| ConfigStoreError::SchemaValidationFailed)?;
        Ok(config)
    }

    fn put (&mut self, config: Config) -> Result<(), ConfigStoreError> {
        let yaml = serde_yaml::to_string(&config).map_err(|e| ConfigStoreError::UnknownError(e.to_string()))?;
        fs::write(&self.path, yaml).map_err(|e| ConfigStoreError::IoError(e))?;
        Ok(())
    }
}