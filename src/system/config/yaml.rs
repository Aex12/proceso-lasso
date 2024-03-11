use std::fs;

use super::{Config, ConfigStore};

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
    fn get (&self) -> Result<Config, Box<dyn std::error::Error>> {
        let yaml = fs::read_to_string(&self.path)?;
        let config: Config = serde_yaml::from_str(&yaml).unwrap();
        config.validate().unwrap();
        Ok(config)
    }

    fn put (&mut self, config: Config) -> Result<(), Box<dyn std::error::Error>> {
        let yaml = serde_yaml::to_string(&config)?;
        fs::write(&self.path, yaml)?;
        Ok(())
    }
}