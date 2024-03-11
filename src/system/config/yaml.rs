use std::fs;

use super::{Config, ConfigStore};

pub struct YamlConfigStore {
    config: Config,
    config_path: String,
}

impl PartialEq for YamlConfigStore {
    fn eq (&self, other: &Self) -> bool {
        self.config_path == other.config_path
    }
}

impl YamlConfigStore {
    pub fn new (config_path: &str) -> YamlConfigStore {
        YamlConfigStore {
            config: Self::read_from_file(config_path).unwrap_or_else(|_| Config::default()),
            config_path: config_path.to_string(),
        }
    }

    fn read_from_file (config_path: &str) -> Result<Config, Box<dyn std::error::Error>> {
        let yaml = fs::read_to_string(config_path)?;
        let config: Config = serde_yaml::from_str(&yaml).unwrap();
        config.validate().unwrap();
        Ok(config)
    }

    fn write_to_file (&self) -> Result<(), Box<dyn std::error::Error>> {
        let yaml = serde_yaml::to_string(&self.config)?;
        fs::write(&self.config_path, yaml)?;
        Ok(())
    }
}

impl ConfigStore for YamlConfigStore {
    fn get (&self) -> Result<Config, Box<dyn std::error::Error>> {
        Ok(self.config.clone())
    }

    fn put (&mut self, config: Config) -> Result<(), Box<dyn std::error::Error>> {
        self.config = config;
        self.write_to_file()
    }
}