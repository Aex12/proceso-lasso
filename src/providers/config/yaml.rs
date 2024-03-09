use std::fs;

use crate::{structs::PLConfig, traits::ConfigManager};

pub struct YamlConfigManager {
    config: PLConfig,
    config_path: String,
}

impl YamlConfigManager {
    pub fn new (config_path: &str) -> Result<YamlConfigManager, Box<dyn std::error::Error>> {
        let config = match Self::read_from_file(config_path) {
            Ok(config) => config,
            Err(_) => PLConfig::default(),
        };
        Ok(YamlConfigManager {
            config,
            config_path: config_path.to_string(),
        })
    }

    pub fn read_from_file (config_path: &str) -> Result<PLConfig, Box<dyn std::error::Error>> {
        let yaml = fs::read_to_string(config_path)?;
        let config: PLConfig = serde_yaml::from_str(&yaml).unwrap();
        Ok(config)
    }
}

impl ConfigManager for YamlConfigManager {
    fn get (&self) -> Result<PLConfig, Box<dyn std::error::Error>> {
        Ok(self.config.clone())
    }

    fn put (&mut self, config: PLConfig) -> Result<(), Box<dyn std::error::Error>> {
        self.config = config;
        let yaml = serde_yaml::to_string(&self.config)?;
        fs::write(&self.config_path, yaml)?;
        Ok(())
    }

    fn load (&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.config = Self::read_from_file(&self.config_path)?;
        Ok(())
    }
}