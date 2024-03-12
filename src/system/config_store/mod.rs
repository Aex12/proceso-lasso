mod yaml;
mod memory;

use thiserror::Error;

pub(super) use yaml::YamlConfigStore;
pub(super) use memory::MemoryConfigStore;

use crate::lasso::Config;

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
}

impl PartialEq for dyn ConfigStore {
    fn eq (&self, _other: &Self) -> bool {
        self.id() == _other.id()
    }
}