mod yaml;
mod memory;

use thiserror::Error;

pub(super) use yaml::YamlConfigStore;
pub(super) use memory::MemoryConfigStore;

use lib_core::Config;

#[derive(Debug, Error)]
pub enum ConfigStoreError {
    #[error("Config store error: Not initialized")]
    NotInitialized,
    #[error("Config store error: Schema validation failed")]
    SchemaValidationFailed,
    #[error("Config store error: IO error")]
    IoError(#[from] std::io::Error),
    #[error("Config store error: unknown: {0}")]
    UnknownError(String),
}

pub trait ConfigStore {
    fn id (&self) -> &str;
    fn get (&self) -> Result<Config, ConfigStoreError>;
    fn put (&mut self, config: Config) -> Result<(), ConfigStoreError>;
    fn initialize (&mut self) -> Result<Config, ConfigStoreError> {
        Ok(match self.get() {
            Ok(config) => config,
            Err(ConfigStoreError::NotInitialized) => {
                let conf = lib_core::Config::default();
                self.put(conf.clone()).unwrap();
                conf
            },
            Err(err) => panic!("Error reading config: {}", err),
        })
    }
}

impl PartialEq for dyn ConfigStore {
    fn eq (&self, _other: &Self) -> bool {
        self.id() == _other.id()
    }
}