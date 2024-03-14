mod config_store;

pub use config_store::{ConfigStoreError, ConfigStore};
use lib_core::ProcessManager;

use lib_windows::WindowsProcessManager;
use config_store::{YamlConfigStore, MemoryConfigStore};

pub fn create_process_manager () -> Box<dyn ProcessManager> {
    Box::new(WindowsProcessManager::new().unwrap())
}

pub fn create_config_store (store: &str, path: &str) -> Box<dyn ConfigStore> {
    match store {
        "yaml" => Box::new(YamlConfigStore::new(path.to_string())),
        "memory" => Box::new(MemoryConfigStore::new()),
        _ => panic!("Invalid config store type"),
    }
}