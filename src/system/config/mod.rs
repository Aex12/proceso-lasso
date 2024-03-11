mod yaml;
mod memory;

pub use yaml::YamlConfigStore;
pub use memory::MemoryConfigStore;

use crate::lasso::Config;

pub trait ConfigStore {
    fn get (&self) -> Result<Config, Box<dyn std::error::Error>>;
    fn put (&mut self, config: Config) -> Result<(), Box<dyn std::error::Error>>;
}

pub fn create_config_store (store: &str, path: &str) -> Box<dyn ConfigStore> {
    match store {
        "yaml" => Box::new(YamlConfigStore::new(path.to_string())),
        "memory" => Box::new(MemoryConfigStore::new()),
        _ => panic!("Invalid config store type"),
    }
}